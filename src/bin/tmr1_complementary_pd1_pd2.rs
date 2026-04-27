#![no_std]
#![no_main]

use apmg32r430xx_pac::Peripherals;
use cortex_m_rt::entry;
use g32r430_hal_examples::{board, hal::prelude::*};
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
    board::init_led(&dp);

    board::rcm_unlock(&dp);
    dp.rcm.ahbcg().modify(|_, w| w.gpiocen().set_bit());
    board::rcm_lock(&dp);

    let _pd1 = parts.gpio.pin::<3, 1>().into_alternate::<1>();
    let _pd2 = parts.gpio.pin::<3, 2>().into_alternate::<2>();

    let pwm = parts
        .tmr1
        .complementary_pwm_builder(TmrChannel::Ch1)
        .unwrap()
        .prescaler(127)
        .period(99)
        .duty(50)
        .counter_mode(TmrCounterMode::Up)
        .clock_division(TmrClockDivision::Div1)
        .output_polarity(TmrPolarity::ActiveHigh)
        .complementary_output_polarity(TmrPolarity::ActiveHigh)
        .output_idle_state(TmrIdleState::Low)
        .complementary_output_idle_state(TmrIdleState::Low)
        .dead_time(2)
        .off_state_idle(true)
        .off_state_run(true)
        .build()
        .unwrap();

    rprintln!("TMR1 complementary PWM started");
    rprintln!("Pins: PD1=TMR1_CH1 AF1, PD2=TMR1_CH1N AF2");
    rprintln!("Timer clock: 128 MHz / (127 + 1) = 1 MHz");
    rprintln!("ARR=99 => 10 kHz PWM, CCR1=50 => 50% duty, dead-time=2 ticks");
    rprintln!(
        "key={:#010x} ahbcg={:#010x} apbcg={:#010x} apbrst={:#010x} ctrl1={:#010x} psc={} arr={} ccr1={} ccen={:#06x} bdt={:#06x}",
        dp.rcm.key().read().bits(),
        dp.rcm.ahbcg().read().bits(),
        dp.rcm.apbcg().read().bits(),
        dp.rcm.apbrst().read().bits(),
        dp.tmr1.ctrl1().read().bits(),
        dp.tmr1.psc().read().bits(),
        dp.tmr1.autorld().read().bits(),
        pwm.duty(),
        dp.tmr1.ccen().read().bits(),
        dp.tmr1.bdt().read().bits(),
    );

    loop {
        board::led_toggle(&dp);
        rprintln!(
            "cnt={} arr={} ccr1={} ccen={:#06x} bdt={:#06x} moe={}",
            dp.tmr1.cnt().read().bits(),
            dp.tmr1.autorld().read().bits(),
            pwm.duty(),
            dp.tmr1.ccen().read().bits(),
            dp.tmr1.bdt().read().bits(),
            ((dp.tmr1.bdt().read().bits() >> 15) & 1) as u8,
        );
        board::delay_ms(1000);
    }
}
