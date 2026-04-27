#![no_std]
#![no_main]

use apmg32r430xx_pac::Peripherals;
// Data Watchpoint and Trace
// https://developer.arm.com/documentation/ddi0403/d/Debug-Architecture/ARMv7-M-Debug/The-Data-Watchpoint-and-Trace-unit
use cortex_m::peripheral::{DCB, DWT};
use cortex_m_rt::entry;
use panic_halt as _;
use rtt_target::*;

// from DLL provided by Geehy
const SYSCLK_HZ: u32 = 128_000_000;
const BLINK_HALF_PERIOD_MS: u32 = 50;
const GPIO_MODE_OUTPUT: u8 = 0b10;
const GPIO_SPEED_HIGH: u8 = 0b11;
const RCM_UNLOCK_KEY: u16 = 0x87E4;

fn delay_ms(ms: u32) {
    let ticks = (SYSCLK_HZ / 1_000) * ms;
    let start = DWT::cycle_count();
    while DWT::cycle_count().wrapping_sub(start) < ticks {}
}

fn rcm_unlock(dp: &Peripherals) {
    dp.rcm.key().write(|w| unsafe { w.bits(RCM_UNLOCK_KEY as u32) });
}

fn rcm_lock(dp: &Peripherals) {
    dp.rcm.key().modify(|_, w| w.keyst().set_bit());
}

fn init_system_clock(dp: &Peripherals) {
    rcm_unlock(dp);

    dp.rcm.rccr().modify(|_, w| w.hseon().set_bit());
    while dp.rcm.rccr().read().hserdy().bit_is_clear() {}

    dp.fmc.cr2().modify(|_, w| unsafe { w.rdwait().bits(3) });

    dp.rcm.rccr().modify(|_, w| w.pllon().clear_bit());
    while dp.rcm.rccr().read().pllrdy().bit_is_set() {}

    // set max_clock = 8MHz * 16 = 128MHz
    // changing the value of mul could do some overclock tests
    dp.rcm.pllcr().modify(|_, w| unsafe {
        w.pllmul().bits(16).pllp().bits(0).pllsrc().bits(3)
    });

    dp.rcm.rccr().modify(|_, w| w.pllon().set_bit());
    while dp.rcm.rccr().read().pllrdy().bit_is_clear() {}

    dp.rcm
        .sccr()
        .modify(|_, w| unsafe { w.sw().bits(2).hdiv().bits(0).pdiv().bits(0) });

    dp.rcm.mccr().modify(|_, w| w.csen().set_bit());
    while dp.rcm.mccr().read().swdone().bit_is_clear() {}
    dp.rcm.mccr().modify(|_, w| w.csen().clear_bit());

    rcm_lock(dp);
}

fn init_led(dp: &Peripherals) {
    rcm_unlock(dp);
    dp.rcm.ahbcg().modify(|_, w| w.gpiocen().set_bit());
    rcm_lock(dp);

    dp.gpio.otyper1().modify(|_, w| w.pa1_ot().clear_bit());
    dp.gpio
        .pupdr1()
        .modify(|_, w| w.pa1_pue().clear_bit().pa1_pus().clear_bit());
    dp.gpio
        .ospeedr1()
        .modify(|_, w| unsafe { w.pa1_os().bits(GPIO_SPEED_HIGH) });
    dp.gpio
        .moder1()
        .modify(|_, w| unsafe { w.pa1_md().bits(GPIO_MODE_OUTPUT) });

    led_off(dp);
}

fn led_on(dp: &Peripherals) {
    dp.gpio.odr1().modify(|_, w| w.pa1_dout().clear_bit());
}

fn led_off(dp: &Peripherals) {
    dp.gpio.odr1().modify(|_, w| w.pa1_dout().set_bit());
}

fn led_toggle(dp: &Peripherals) {
    if dp.gpio.odr1().read().pa1_dout().bit_is_set() {
        led_on(dp);
    } else {
        led_off(dp);
    }
}

#[entry]
fn main() -> ! {
    let mut cp = cortex_m::Peripherals::take().unwrap();
    let dp = Peripherals::take().unwrap();

    rtt_init_print!();

    init_system_clock(&dp);

    DCB::enable_trace(&mut cp.DCB);
    cp.DWT.enable_cycle_counter();
    cp.DWT.set_cycle_count(0);
    init_led(&dp);

    loop {
        rprintln!("led toggling");
        led_toggle(&dp);
        delay_ms(BLINK_HALF_PERIOD_MS);
    }
}
