#![no_std]
#![no_main]

use apmg32r430xx_pac::Peripherals;
use cortex_m::asm;
use cortex_m_rt::entry;
use panic_halt as _;

use rtt_target::{rtt_init_print, rprintln};

const LED_DELAY_OUTER: u32 = 100;
const LED_DELAY_INNER: u32 = 100;
const GPIO_MODE_OUTPUT: u8 = 0b10;
const GPIO_SPEED_HIGH: u8 = 0b11;
const RCM_UNLOCK_KEY: u16 = 0x87E4;

fn delay() {
    for _ in 0..LED_DELAY_OUTER {
        for _ in 0..LED_DELAY_INNER {
            asm::nop();
        }
    }
}

fn rcm_unlock(dp: &Peripherals) {
    // The generated LOCKKEY field is only 15 bits wide, but Geehy's DDL writes
    // the full 16-bit value 0x87E4 to RCM->KEY. Write the raw register value
    // so the unlock key is not truncated to 0x07E4.
    dp.rcm.key().write(|w| unsafe { w.bits(RCM_UNLOCK_KEY as u32) });
}

fn rcm_lock(dp: &Peripherals) {
    dp.rcm.key().modify(|_, w| w.keyst().set_bit());
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

// G32R430 Tiny vendor blinky example toggles PA1.
// LED is active-low in the vendor board support package.

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
    rtt_init_print!();

    let dp = Peripherals::take().unwrap();

    init_led(&dp);

    rprintln!(
        "ahbcg={:#010x} moder1={:#010x} otyper1={:#010x} ospeedr1={:#010x} pupdr1={:#010x} odr1={:#010x} idr1={:#010x}",
        dp.rcm.ahbcg().read().bits(),
        dp.gpio.moder1().read().bits(),
        dp.gpio.otyper1().read().bits(),
        dp.gpio.ospeedr1().read().bits(),
        dp.gpio.pupdr1().read().bits(),
        dp.gpio.odr1().read().bits(),
        dp.gpio.idr1().read().bits(),
    );

    loop {
        led_toggle(&dp);
        rprintln!(
            "odr1={:#010x} idr1={:#010x} pa1_dout={} pa1_din={}",
            dp.gpio.odr1().read().bits(),
            dp.gpio.idr1().read().bits(),
            dp.gpio.odr1().read().pa1_dout().bit_is_set() as u8,
            dp.gpio.idr1().read().pa1_din().bit_is_set() as u8,
        );
        delay();
    }
}
