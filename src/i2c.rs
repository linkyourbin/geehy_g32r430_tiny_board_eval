use apmg32r430xx_pac::Peripherals;
use embedded_hal::i2c::{ErrorKind, ErrorType, I2c, NoAcknowledgeSource, Operation};

use crate::board::{
    GPIO_AF_0, GPIO_MODE_ALTERNATE, GPIO_SPEED_HIGH, I2C_OWN_ADDRESS, rcm_lock, rcm_unlock,
};

pub const STANDARD_MODE_100KHZ: u32 = 100_000;
pub const FAST_MODE_400KHZ: u32 = 400_000;

const I2C_TIMEOUT: u32 = 200_000;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum I2cError {
    Timeout,
    Acknowledge,
    Bus,
    Unsupported,
}

fn rise_time(freq_mhz: u32, speed_hz: u32) -> u8 {
    if speed_hz <= STANDARD_MODE_100KHZ {
        (freq_mhz + 1) as u8
    } else {
        (((freq_mhz * 300) / 1000) + 1) as u8
    }
}

fn clock_control(periph_clk_hz: u32, speed_hz: u32) -> (u16, bool, bool) {
    if speed_hz > STANDARD_MODE_100KHZ {
        let clks = core::cmp::max(1, periph_clk_hz / (speed_hz * 3));
        (clks as u16, true, false)
    } else {
        let clks = core::cmp::max(4, periph_clk_hz / (speed_hz * 2));
        (clks as u16, false, false)
    }
}

fn clear_addr_flag(dp: &Peripherals) {
    let _ = dp.i2c1.sts1().read().bits();
    let _ = dp.i2c1.sts2().read().bits();
}

fn clear_error_flags(dp: &Peripherals) {
    dp.i2c1.sts1().modify(|_, w| {
        w.aeflg()
            .clear_bit()
            .berrflg()
            .clear_bit()
            .alflg()
            .clear_bit()
            .ovrurflg()
            .clear_bit()
            .peceflg()
            .clear_bit()
            .tteflg()
            .clear_bit()
            .smbaltflg()
            .clear_bit()
    });
}

pub fn init_i2c1(dp: &Peripherals, periph_clk_hz: u32, speed_hz: u32) {
    rcm_unlock(dp);
    dp.rcm.ahbcg().modify(|_, w| w.gpiocen().set_bit());
    dp.rcm.apbcg().modify(|_, w| w.i2ccen().set_bit());
    rcm_lock(dp);

    dp.gpio
        .otyper2()
        .modify(|_, w| w.pd5_ot().set_bit().pd9_ot().set_bit());
    dp.gpio.pupdr4().modify(|_, w| {
        w.pd5_pue()
            .clear_bit()
            .pd5_pus()
            .clear_bit()
            .pd9_pue()
            .clear_bit()
            .pd9_pus()
            .clear_bit()
    });
    dp.gpio.ospeedr4().modify(|_, w| unsafe {
        w.pd5_os()
            .bits(GPIO_SPEED_HIGH)
            .pd9_os()
            .bits(GPIO_SPEED_HIGH)
    });
    dp.gpio.afr4().modify(|_, w| unsafe {
        w.pd5_afsel().bits(GPIO_AF_0).pd9_afsel().bits(GPIO_AF_0)
    });
    dp.gpio.moder4().modify(|_, w| unsafe {
        w.pd5_md()
            .bits(GPIO_MODE_ALTERNATE)
            .pd9_md()
            .bits(GPIO_MODE_ALTERNATE)
    });

    dp.i2c1.ctrl1().modify(|_, w| w.swrst().set_bit());
    dp.i2c1.ctrl1().modify(|_, w| w.swrst().clear_bit());
    dp.i2c1.ctrl1().modify(|_, w| w.i2cen().clear_bit());

    let freq_mhz = periph_clk_hz / 1_000_000;
    let (clks, speedcfg, fdutycfg) = clock_control(periph_clk_hz, speed_hz);

    dp.i2c1
        .ctrl2()
        .modify(|_, w| unsafe { w.clkfcfg().bits(freq_mhz as u8) });
    dp.i2c1
        .risetmax()
        .modify(|_, w| unsafe { w.risetmax().bits(rise_time(freq_mhz, speed_hz)) });
    dp.i2c1.clkctrl().modify(|_, w| unsafe {
        w.clks()
            .bits(clks)
            .fdutycfg()
            .bit(fdutycfg)
            .speedcfg()
            .bit(speedcfg)
    });
    dp.i2c1.saddr1().write(|w| unsafe {
        w.addr7()
            .bits(I2C_OWN_ADDRESS)
            .addrlen()
            .clear_bit()
            .addr0()
            .clear_bit()
    });
    dp.i2c1.ctrl1().modify(|_, w| w.acken().set_bit());
    dp.i2c1.ctrl1().modify(|_, w| w.i2cen().set_bit());

    clear_error_flags(dp);
}

fn write_7bit(dp: &Peripherals, address: u8, bytes: &[u8]) -> Result<(), I2cError> {
    let mut timeout = I2C_TIMEOUT;
    while dp.i2c1.sts2().read().busbsyflg().bit_is_set() {
        if timeout == 0 {
            return Err(I2cError::Timeout);
        }
        timeout -= 1;
    }

    clear_error_flags(dp);
    dp.i2c1.ctrl1().modify(|_, w| w.start().set_bit());

    timeout = I2C_TIMEOUT;
    while dp.i2c1.sts1().read().startflg().bit_is_clear() {
        if timeout == 0 {
            return Err(I2cError::Timeout);
        }
        timeout -= 1;
    }

    dp.i2c1
        .data()
        .write(|w| unsafe { w.data().bits(address << 1) });

    timeout = I2C_TIMEOUT;
    loop {
        let sts1 = dp.i2c1.sts1().read();
        if sts1.addrflg().bit_is_set() {
            break;
        }
        if sts1.aeflg().bit_is_set() {
            clear_error_flags(dp);
            dp.i2c1.ctrl1().modify(|_, w| w.stop().set_bit());
            return Err(I2cError::Acknowledge);
        }
        if sts1.berrflg().bit_is_set() || sts1.alflg().bit_is_set() {
            clear_error_flags(dp);
            dp.i2c1.ctrl1().modify(|_, w| w.stop().set_bit());
            return Err(I2cError::Bus);
        }
        if timeout == 0 {
            return Err(I2cError::Timeout);
        }
        timeout -= 1;
    }

    clear_addr_flag(dp);

    for &byte in bytes {
        timeout = I2C_TIMEOUT;
        loop {
            let sts1 = dp.i2c1.sts1().read();
            if sts1.txbeflg().bit_is_set() {
                break;
            }
            if sts1.aeflg().bit_is_set() {
                clear_error_flags(dp);
                dp.i2c1.ctrl1().modify(|_, w| w.stop().set_bit());
                return Err(I2cError::Acknowledge);
            }
            if timeout == 0 {
                return Err(I2cError::Timeout);
            }
            timeout -= 1;
        }

        dp.i2c1.data().write(|w| unsafe { w.data().bits(byte) });

        timeout = I2C_TIMEOUT;
        loop {
            let sts1 = dp.i2c1.sts1().read();
            if sts1.btcflg().bit_is_set() {
                break;
            }
            if sts1.aeflg().bit_is_set() {
                clear_error_flags(dp);
                dp.i2c1.ctrl1().modify(|_, w| w.stop().set_bit());
                return Err(I2cError::Acknowledge);
            }
            if timeout == 0 {
                return Err(I2cError::Timeout);
            }
            timeout -= 1;
        }
    }

    dp.i2c1.ctrl1().modify(|_, w| w.stop().set_bit());
    Ok(())
}

impl embedded_hal::i2c::Error for I2cError {
    fn kind(&self) -> ErrorKind {
        match self {
            Self::Timeout => ErrorKind::Other,
            Self::Acknowledge => ErrorKind::NoAcknowledge(NoAcknowledgeSource::Unknown),
            Self::Bus => ErrorKind::Bus,
            Self::Unsupported => ErrorKind::Other,
        }
    }
}

pub struct G32I2c<'a> {
    dp: &'a Peripherals,
}

impl<'a> G32I2c<'a> {
    pub fn new(dp: &'a Peripherals) -> Self {
        Self { dp }
    }
}

impl ErrorType for G32I2c<'_> {
    type Error = I2cError;
}

impl I2c for G32I2c<'_> {
    fn transaction(
        &mut self,
        address: u8,
        operations: &mut [Operation<'_>],
    ) -> Result<(), Self::Error> {
        if operations.is_empty() {
            return Ok(());
        }

        for operation in operations {
            match operation {
                Operation::Write(bytes) => write_7bit(self.dp, address, bytes)?,
                Operation::Read(_) => return Err(I2cError::Unsupported),
            }
        }

        Ok(())
    }
}
