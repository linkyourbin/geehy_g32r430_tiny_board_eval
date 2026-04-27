#![no_std]
#![no_main]

use apmg32r430xx_pac::Peripherals;
use embedded_hal::digital::OutputPin;
use g32r430_hal_examples::{board, chip, hal::prelude::*, i2c::FAST_MODE_400KHZ};
use cortex_m_rt::entry;
use embedded_graphics::{
    mono_font::{MonoTextStyle, MonoTextStyleBuilder, ascii::FONT_5X7},
    pixelcolor::BinaryColor,
    prelude::*,
    text::{Baseline, Text},
};
use panic_halt as _;
use rtt_target::{rprintln, rtt_init_print};
use sh110x::{Builder, displaysize::DisplaySize, mode::GraphicsMode};
use ssd1306::{I2CDisplayInterface, Ssd1306, prelude::*};

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

fn draw_content<D>(
    display: &mut D,
    text_style: MonoTextStyle<'static, BinaryColor>,
    uid: [u32; 3],
    temp_c: f32,
    uid0_buf: &mut [u8; 13],
    uid1_buf: &mut [u8; 13],
    uid2_buf: &mut [u8; 13],
    temp_buf: &mut [u8; 14],
) -> Result<(), D::Error>
where
    D: DrawTarget<Color = BinaryColor>,
{
    Text::with_baseline("Powered by", Point::new(0, 0), text_style, Baseline::Top).draw(display)?;
    Text::with_baseline("Rust", Point::new(60, 0), text_style, Baseline::Top).draw(display)?;
    Text::with_baseline(
        "Geehy G32R430",
        Point::new(0, 8),
        text_style,
        Baseline::Top,
    )
    .draw(display)?;
    Text::with_baseline("Tiny Board", Point::new(0, 16), text_style, Baseline::Top).draw(display)?;
    Text::with_baseline(
        format_temp_line(temp_c, temp_buf),
        Point::new(0, 26),
        text_style,
        Baseline::Top,
    )
    .draw(display)?;
    Text::with_baseline(
        format_uid_line(0, uid[0], uid0_buf),
        Point::new(0, 36),
        text_style,
        Baseline::Top,
    )
    .draw(display)?;
    Text::with_baseline(
        format_uid_line(1, uid[1], uid1_buf),
        Point::new(0, 46),
        text_style,
        Baseline::Top,
    )
    .draw(display)?;
    Text::with_baseline(
        format_uid_line(2, uid[2], uid2_buf),
        Point::new(0, 56),
        text_style,
        Baseline::Top,
    )
    .draw(display)?;

    Ok(())
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
    let mut uart = parts.usart1;
    uart.init_default(115_200);
    let mut i2c1 = parts.i2c1;
    i2c1.init_default(board::SYSCLK_HZ, FAST_MODE_400KHZ);
    let mut spi1 = parts.spi1;
    spi1.init_default();

    let uid = chip::read_uid_words();

    rprintln!("OLED dual-display example started");
    rprintln!("Powered by Rust");
    rprintln!("Geehy G32R430 Tiny Board");
    rprintln!("SSD1306: I2C1 PD5/PD9 addr=0x3C");
    rprintln!("SH1106: SPI1 PC5/PC11 DC=PC4 CS=PC6 RST=PC10");
    rprintln!("UID0 = 0x{:08X}", uid[0]);
    rprintln!("UID1 = 0x{:08X}", uid[1]);
    rprintln!("UID2 = 0x{:08X}", uid[2]);

    let mut uart_uid0_buf = [0u8; 13];
    let mut uart_uid1_buf = [0u8; 13];
    let mut uart_uid2_buf = [0u8; 13];
    uart.write_str_blocking("\r\nPowered by Rust\r\n");
    uart.write_str_blocking("Geehy G32R430 Tiny Board\r\n");
    uart.write_str_blocking("OLED demo: SSD1306(I2C) + SH1106(SPI)\r\n");
    uart.write_str_blocking(format_uid_line(0, uid[0], &mut uart_uid0_buf));
    uart.write_str_blocking("\r\n");
    uart.write_str_blocking(format_uid_line(1, uid[1], &mut uart_uid1_buf));
    uart.write_str_blocking("\r\n");
    uart.write_str_blocking(format_uid_line(2, uid[2], &mut uart_uid2_buf));
    uart.write_str_blocking("\r\n");
    uart.flush_blocking();

    let ssd1306_interface = I2CDisplayInterface::new(i2c1);
    let mut ssd1306 = Ssd1306::new(
        ssd1306_interface,
        DisplaySize128x64,
        DisplayRotation::Rotate0,
    )
    .into_buffered_graphics_mode();

    if let Err(error) = ssd1306.init() {
        rprintln!("ssd1306 init failed: {:?}", error);
        loop {
            board::led_toggle(&dp);
            board::delay_ms(100);
        }
    }

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
    let mut sh1106: GraphicsMode<_> = raw.into();

    let mut delay = Delay::default();
    if let Err(error) = sh1106.reset(&mut rst, &mut delay) {
        rprintln!("sh1106 reset failed: {:?}", error);
        loop {
            board::led_toggle(&dp);
            board::delay_ms(100);
        }
    }
    if let Err(error) = sh1106.init() {
        rprintln!("sh1106 init failed: {:?}", error);
        loop {
            board::led_toggle(&dp);
            board::delay_ms(100);
        }
    }

    let text_style = MonoTextStyleBuilder::new()
        .font(&FONT_5X7)
        .text_color(BinaryColor::On)
        .build();

    let mut ssd_uid0_buf = [0u8; 13];
    let mut ssd_uid1_buf = [0u8; 13];
    let mut ssd_uid2_buf = [0u8; 13];
    let mut ssd_temp_buf = [0u8; 14];

    let mut sh_uid0_buf = [0u8; 13];
    let mut sh_uid1_buf = [0u8; 13];
    let mut sh_uid2_buf = [0u8; 13];
    let mut sh_temp_buf = [0u8; 14];

    let mut uart_temp_buf = [0u8; 14];

    loop {
        let temp_c = chip::read_temperature_c(&dp).unwrap_or(0.0);

        ssd1306.clear(BinaryColor::Off).unwrap();
        draw_content(
            &mut ssd1306,
            text_style,
            uid,
            temp_c,
            &mut ssd_uid0_buf,
            &mut ssd_uid1_buf,
            &mut ssd_uid2_buf,
            &mut ssd_temp_buf,
        )
        .unwrap();
        if let Err(error) = ssd1306.flush() {
            rprintln!("ssd1306 flush failed: {:?}", error);
        }

        sh1106.clear();
        draw_content(
            &mut sh1106,
            text_style,
            uid,
            temp_c,
            &mut sh_uid0_buf,
            &mut sh_uid1_buf,
            &mut sh_uid2_buf,
            &mut sh_temp_buf,
        )
        .unwrap();
        if let Err(error) = sh1106.flush() {
            rprintln!("sh1106 flush failed: {:?}", error);
        }

        uart.write_str_blocking("Powered by Rust | ");
        uart.write_str_blocking("Geehy G32R430 Tiny Board | ");
        uart.write_str_blocking("TEMP=");
        uart.write_str_blocking(format_temp_line(temp_c, &mut uart_temp_buf));
        uart.write_str_blocking(" | OLED=SSD1306+SH1106\r\n");
        uart.flush_blocking();

        board::led_toggle(&dp);
        board::delay_ms(1000);
    }
}
