#![no_std]
#![no_main]

use apmg32r430xx_pac::Peripherals;
use embedded_hal::digital::OutputPin;
use g32r430_hal_examples::{board, chip, hal::prelude::*};
use cortex_m_rt::entry;
use embedded_graphics::{
    mono_font::{MonoTextStyleBuilder, ascii::FONT_6X10},
    pixelcolor::BinaryColor,
    prelude::*,
    text::{Baseline, Text},
};
use panic_halt as _;
use rtt_target::{rprintln, rtt_init_print};
use sh110x::{
    Builder,
    displaysize::DisplaySize,
    mode::GraphicsMode,
};

fn hex_nibble(value: u8) -> u8 {
    match value & 0x0f {
        0..=9 => b'0' + (value & 0x0f),
        v => b'A' + (v - 10),
    }
}

fn format_uid_line(index: u8, value: u32, buf: &mut [u8; 13]) -> &str {
    buf[0] = b'U';
    buf[1] = b'I';
    buf[2] = b'D';
    buf[3] = b'0' + index;
    buf[4] = b' ';

    let mut shift = 28;
    let mut pos = 5;
    while pos < 13 {
        buf[pos] = hex_nibble((value >> shift) as u8);
        pos += 1;
        shift -= 4;
    }

    unsafe { core::str::from_utf8_unchecked(&buf[..]) }
}

fn format_temp_line(temp_c: f32, buf: &mut [u8; 14]) -> &str {
    buf.fill(b' ');
    buf[0] = b'T';
    buf[1] = b'E';
    buf[2] = b'M';
    buf[3] = b'P';
    buf[4] = b' ';

    let negative = temp_c < 0.0;
    let scaled = if negative {
        (-temp_c * 100.0 + 0.5) as u32
    } else {
        (temp_c * 100.0 + 0.5) as u32
    };
    let whole = scaled / 100;
    let frac = scaled % 100;

    let mut pos = 5;
    if negative {
        buf[pos] = b'-';
        pos += 1;
    }

    let mut digits = [0u8; 4];
    let mut len = 0;
    let mut value = whole;
    loop {
        digits[len] = b'0' + (value % 10) as u8;
        len += 1;
        value /= 10;
        if value == 0 {
            break;
        }
    }

    for i in 0..len {
        buf[pos + i] = digits[len - 1 - i];
    }
    pos += len;
    buf[pos] = b'.';
    buf[pos + 1] = b'0' + ((frac / 10) as u8);
    buf[pos + 2] = b'0' + ((frac % 10) as u8);
    buf[pos + 3] = b'C';

    unsafe { core::str::from_utf8_unchecked(&buf[..pos + 4]) }
}

#[entry]
fn main() -> ! {
    let mut cp = cortex_m::Peripherals::take().unwrap();
    let parts = Parts::take().unwrap();
    let dp = unsafe { Peripherals::steal() };

    rtt_init_print!();

    board::init_system_clock(&dp);
    enable_cycle_counter(&mut cp);
    board::init_led(&dp);
    chip::init_temperature_sensor(&dp);
    let mut spi1 = parts.spi1;
    spi1.init_default();

    rprintln!("SH1106 SPI example started");
    rprintln!("SPI1 SCK=PC5 AF2, MOSI=PC11 AF1");
    rprintln!("Control pins: DC=PC4, CS=PC6, RST=PC10");
    rprintln!("UID base: 0x0802020C");

    let uid = chip::read_uid_words();
    rprintln!("UID0 = 0x{:08X}", uid[0]);
    rprintln!("UID1 = 0x{:08X}", uid[1]);
    rprintln!("UID2 = 0x{:08X}", uid[2]);

    board::rcm_unlock(&dp);
    dp.rcm.ahbcg().modify(|_, w| w.gpiocen().set_bit());
    board::rcm_lock(&dp);

    let dc = parts.gpio.pin::<2, 4>().into_output();
    let cs = parts.gpio.pin::<2, 6>().into_output();
    let mut rst = parts.gpio.pin::<2, 10>().into_output();
    let _ = rst.set_high();

    let raw = Builder::new()
        .with_size(DisplaySize::Display128x64)
        .connect_spi(spi1, dc, cs);
    let mut display: GraphicsMode<_> = raw.into();

    let mut delay = Delay::default();
    if let Err(error) = display.reset(&mut rst, &mut delay) {
        rprintln!("sh1106 reset failed: {:?}", error);
        loop {
            board::led_toggle(&dp);
            board::delay_ms(100);
        }
    }
    if let Err(error) = display.init() {
        rprintln!("sh1106 init failed: {:?}", error);
        loop {
            board::led_toggle(&dp);
            board::delay_ms(100);
        }
    }

    let text_style = MonoTextStyleBuilder::new()
        .font(&FONT_6X10)
        .text_color(BinaryColor::On)
        .build();
    let mut uid0_buf = [0u8; 13];
    let mut uid1_buf = [0u8; 13];
    let mut uid2_buf = [0u8; 13];
    let mut temp_buf = [0u8; 14];

    loop {
        let temp_c = chip::read_temperature_c(&dp).unwrap_or(0.0);

        display.clear();

        Text::with_baseline(
            format_temp_line(temp_c, &mut temp_buf),
            Point::new(0, 2),
            text_style,
            Baseline::Top,
        )
        .draw(&mut display)
        .unwrap();
        Text::with_baseline(
            format_uid_line(0, uid[0], &mut uid0_buf),
            Point::new(0, 18),
            text_style,
            Baseline::Top,
        )
        .draw(&mut display)
        .unwrap();
        Text::with_baseline(
            format_uid_line(1, uid[1], &mut uid1_buf),
            Point::new(0, 30),
            text_style,
            Baseline::Top,
        )
        .draw(&mut display)
        .unwrap();
        Text::with_baseline(
            format_uid_line(2, uid[2], &mut uid2_buf),
            Point::new(0, 42),
            text_style,
            Baseline::Top,
        )
        .draw(&mut display)
        .unwrap();

        if let Err(error) = display.flush() {
            rprintln!("sh1106 flush failed: {:?}", error);
            board::delay_ms(100);
        }

        board::led_toggle(&dp);
        board::delay_ms(1000);
    }
}
