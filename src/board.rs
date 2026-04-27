use apmg32r430xx_pac::Peripherals;
use cortex_m::peripheral::{DCB, DWT};

pub const SYSCLK_HZ: u32 = 128_000_000;
pub const RCM_UNLOCK_KEY: u16 = 0x87E4;
pub const GPIO_MODE_OUTPUT: u8 = 0b10;
pub const GPIO_MODE_ALTERNATE: u8 = 0b11;
pub const GPIO_SPEED_HIGH: u8 = 0b11;
pub const GPIO_AF_0: u8 = 0;
pub const I2C_OWN_ADDRESS: u8 = 0x1A;

fn delay_cycles(mut cycles: u64) {
    while cycles != 0 {
        let chunk = cycles.min(u64::from(u32::MAX)) as u32;
        let start = DWT::cycle_count();

        while DWT::cycle_count().wrapping_sub(start) < chunk {}

        cycles -= u64::from(chunk);
    }
}

pub fn enable_cycle_counter(cp: &mut cortex_m::Peripherals) {
    DCB::enable_trace(&mut cp.DCB);
    cp.DWT.enable_cycle_counter();
    cp.DWT.set_cycle_count(0);
}

pub fn delay_ms(ms: u32) {
    let cycles = (u64::from(SYSCLK_HZ) * u64::from(ms)).div_ceil(1_000);
    delay_cycles(cycles);
}

pub fn delay_ns(ns: u32) {
    let cycles = (u64::from(SYSCLK_HZ) * u64::from(ns)).div_ceil(1_000_000_000);
    delay_cycles(cycles);
}

pub fn rcm_unlock(dp: &Peripherals) {
    dp.rcm
        .key()
        .write(|w| unsafe { w.bits(RCM_UNLOCK_KEY as u32) });

    while dp.rcm.key().read().keyst().bit_is_clear() {}
}

pub fn rcm_lock(dp: &Peripherals) {
    dp.rcm.key().write(|w| w.keyst().set_bit());

    while dp.rcm.key().read().keyst().bit_is_set() {}
}

pub fn init_system_clock(dp: &Peripherals) {
    rcm_unlock(dp);

    dp.rcm.rccr().modify(|_, w| w.hseon().set_bit());
    while dp.rcm.rccr().read().hserdy().bit_is_clear() {}

    dp.fmc.cr2().modify(|_, w| unsafe { w.rdwait().bits(3) });

    dp.rcm.rccr().modify(|_, w| w.pllon().clear_bit());
    while dp.rcm.rccr().read().pllrdy().bit_is_set() {}

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

pub fn init_led(dp: &Peripherals) {
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

pub fn led_on(dp: &Peripherals) {
    dp.gpio.odr1().modify(|_, w| w.pa1_dout().clear_bit());
}

pub fn led_off(dp: &Peripherals) {
    dp.gpio.odr1().modify(|_, w| w.pa1_dout().set_bit());
}

pub fn led_toggle(dp: &Peripherals) {
    if dp.gpio.odr1().read().pa1_dout().bit_is_set() {
        led_on(dp);
    } else {
        led_off(dp);
    }
}
