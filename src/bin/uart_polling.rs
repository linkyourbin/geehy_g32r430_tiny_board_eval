#![no_std]
#![no_main]

use apmg32r430xx_pac::Peripherals;
use embedded_hal::digital::OutputPin;
use g32r430_hal_examples::{board, hal::prelude::*};
use cortex_m_rt::entry;
use panic_halt as _;
use rtt_target::{rprintln, rtt_init_print};

#[entry]
fn main() -> ! {
    let mut cp = cortex_m::Peripherals::take().unwrap();
    let parts = Parts::take().unwrap();
    let dp = unsafe { Peripherals::steal() };

    rtt_init_print!();

    board::init_system_clock(&dp);
    enable_cycle_counter(&mut cp);
    board::rcm_unlock(&dp);
    dp.rcm.ahbcg().modify(|_, w| w.gpiocen().set_bit());
    board::rcm_lock(&dp);
    let mut led = parts.gpio.pin::<0, 1>().into_output();
    let _ = led.set_high();
    let mut uart = parts.usart1;
    uart.init_default(115_200);

    rprintln!("USART1 polling example started");
    uart.write_str_blocking("\r\nG32R430 USART1 polling example\r\n");
    uart.write_str_blocking("Pins: PB9 = TX, PB6 = RX\r\n");
    uart.flush_blocking();

    loop {
        let _ = led.set_low();
        uart.write_str_blocking("hello rust from geehy G32R430\r\n");
        uart.flush_blocking();
        board::delay_ms(500);
        let _ = led.set_high();
    }
}
