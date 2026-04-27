#![no_std]
#![no_main]

use apmg32r430xx_pac::Peripherals;
use g32r430_hal_examples::{board, hal::{native::Adc1, prelude::*}};
use cortex_m_rt::entry;
use panic_halt as _;
use rtt_target::{rprintln, rtt_init_print};

#[entry]
fn main() -> ! {
    let parts = Parts::take().unwrap();
    let dp = unsafe { Peripherals::steal() };

    rtt_init_print!();

    board::init_system_clock(&dp);
    board::init_led(&dp);

    board::rcm_unlock(&dp);
    dp.rcm.ahbcg().modify(|_, w| w.gpiocen().set_bit());
    board::rcm_lock(&dp);

    let _pa1 = parts.gpio.pin::<0, 1>().into_analog();
    let mut adc1: Adc1 = parts.adc1;
    adc1
        .init_single_ended_continuous(
            g32r430_hal_examples::hal::adc::Adc16Channel::Ch1,
            g32r430_hal_examples::hal::adc::Adc16SampleTime::Cycles10,
        )
        .unwrap();
    adc1.start_regular_conversion();

    loop {
        if let Ok(raw) = adc1.read_regular_data() {
            let mv = (raw as u32 * 3300) / 65535;
            rprintln!("ADC1 PA1 raw={} mv={}", raw, mv);
        }
        board::delay_ms(500);
    }
}
