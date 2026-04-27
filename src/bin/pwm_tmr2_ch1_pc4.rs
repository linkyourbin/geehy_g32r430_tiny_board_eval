#![no_std]
#![no_main]

use apmg32r430xx_pac::Peripherals;
use embedded_hal::pwm::SetDutyCycle;
use g32r430_hal_examples::{board, hal::prelude::*};
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

    let _pc4 = parts.gpio.pin::<2, 4>().into_alternate::<0>();
    let mut pwm = parts.tmr2.into_pwm::<1>();
    // PWM for logic-analyzer verification:
    // 128 MHz / (127 + 1) = 1 M timer clock
    // ARR = 9 => 1 MHz / 10 = 100 KHz PWM
    // CCR = 1 => 10% duty
    pwm.configure(127, 9, PwmMode::Pwm1);
    let _ = pwm.set_duty_cycle(1);

    rprintln!("TMR2 CH1 PWM on PC4 started");

    loop {
        board::delay_ms(1000);
    }
}
