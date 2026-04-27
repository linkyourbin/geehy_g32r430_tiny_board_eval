use apmg32r430xx_pac::Peripherals;

use crate::board::{
    GPIO_AF_0, GPIO_MODE_ALTERNATE, GPIO_SPEED_HIGH, SYSCLK_HZ, rcm_lock, rcm_unlock,
};

pub const DEFAULT_BAUD_RATE: u32 = 115_200;

fn encode_brr(periph_clk_hz: u32, baud_rate: u32) -> u16 {
    let int_div = (25 * periph_clk_hz) / (4 * baud_rate);
    let mut brr = ((int_div / 100) << 4) as u16;
    let fractional_div = int_div - (100 * u32::from(brr >> 4));

    brr |= ((((fractional_div * 16) + 50) / 100) as u16) & 0x0f;
    brr
}

pub fn init_usart1(dp: &Peripherals, baud_rate: u32) {
    rcm_unlock(dp);
    dp.rcm.ahbcg().modify(|_, w| w.gpiocen().set_bit());
    dp.rcm.apbcg().modify(|_, w| w.usart1cen().set_bit());
    rcm_lock(dp);

    dp.gpio
        .otyper1()
        .modify(|_, w| w.pb6_ot().clear_bit().pb9_ot().clear_bit());
    dp.gpio.pupdr2().modify(|_, w| {
        w.pb6_pue()
            .clear_bit()
            .pb6_pus()
            .clear_bit()
            .pb9_pue()
            .clear_bit()
            .pb9_pus()
            .clear_bit()
    });
    dp.gpio.ospeedr2().modify(|_, w| unsafe {
        w.pb6_os()
            .bits(GPIO_SPEED_HIGH)
            .pb9_os()
            .bits(GPIO_SPEED_HIGH)
    });
    dp.gpio.afr2().modify(|_, w| unsafe {
        w.pb6_afsel().bits(GPIO_AF_0).pb9_afsel().bits(GPIO_AF_0)
    });
    dp.gpio.moder2().modify(|_, w| unsafe {
        w.pb6_md()
            .bits(GPIO_MODE_ALTERNATE)
            .pb9_md()
            .bits(GPIO_MODE_ALTERNATE)
    });

    dp.usart1.ctrl1().modify(|_, w| w.uen().clear_bit());
    dp.usart1
        .ctrl2()
        .modify(|_, w| unsafe { w.stopcfg().bits(0) });

    let brr = encode_brr(SYSCLK_HZ, baud_rate);
    dp.usart1.br().write(|w| unsafe {
        w.fbr()
            .bits((brr & 0x0f) as u8)
            .ibr()
            .bits((brr >> 4) & 0x0fff)
    });

    dp.usart1.ctrl1().modify(|_, w| {
        w.osmcfg()
            .clear_bit()
            .dblcfg()
            .clear_bit()
            .pcen()
            .clear_bit()
            .rxen()
            .set_bit()
            .txen()
            .set_bit()
            .uen()
            .set_bit()
    });
}

pub fn write_byte(dp: &Peripherals, byte: u8) {
    while dp.usart1.sts().read().txbeflg().bit_is_clear() {}
    dp.usart1
        .data()
        .write(|w| unsafe { w.data().bits(byte as u16) });
}

pub fn write_str(dp: &Peripherals, message: &str) {
    for byte in message.bytes() {
        write_byte(dp, byte);
    }
}

pub fn flush(dp: &Peripherals) {
    while dp.usart1.sts().read().txcflg().bit_is_clear() {}
}

pub fn read_byte_blocking(dp: &Peripherals) -> u8 {
    while dp.usart1.sts().read().rxbneflg().bit_is_clear() {}
    dp.usart1.data().read().data().bits() as u8
}
