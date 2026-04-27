use core::convert::Infallible;

use apmg32r430xx_pac::Peripherals;
use embedded_hal::{
    delay::DelayNs,
    digital::{ErrorType as DigitalErrorType, OutputPin},
    spi::{ErrorKind, ErrorType, Operation, SpiDevice},
};

use crate::board::{GPIO_MODE_ALTERNATE, GPIO_MODE_OUTPUT, GPIO_SPEED_HIGH, rcm_lock, rcm_unlock};

const SPI_TIMEOUT: u32 = 200_000;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SpiError {
    Timeout,
    Overrun,
    ModeFault,
}

impl embedded_hal::spi::Error for SpiError {
    fn kind(&self) -> ErrorKind {
        match self {
            Self::Timeout => ErrorKind::Other,
            Self::Overrun => ErrorKind::Overrun,
            Self::ModeFault => ErrorKind::ModeFault,
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum ControlPin {
    Pc4,
    Pc6,
    Pc10,
}

pub struct G32OutputPin<'a> {
    dp: &'a Peripherals,
    pin: ControlPin,
}

impl<'a> G32OutputPin<'a> {
    pub fn pc4(dp: &'a Peripherals) -> Self {
        Self { dp, pin: ControlPin::Pc4 }
    }

    pub fn pc6(dp: &'a Peripherals) -> Self {
        Self { dp, pin: ControlPin::Pc6 }
    }

    pub fn pc10(dp: &'a Peripherals) -> Self {
        Self { dp, pin: ControlPin::Pc10 }
    }
}

impl DigitalErrorType for G32OutputPin<'_> {
    type Error = Infallible;
}

impl OutputPin for G32OutputPin<'_> {
    fn set_low(&mut self) -> Result<(), Self::Error> {
        self.dp.gpio.odr2().modify(|_, w| match self.pin {
            ControlPin::Pc4 => w.pc4_dout().clear_bit(),
            ControlPin::Pc6 => w.pc6_dout().clear_bit(),
            ControlPin::Pc10 => w.pc10_dout().clear_bit(),
        });
        Ok(())
    }

    fn set_high(&mut self) -> Result<(), Self::Error> {
        self.dp.gpio.odr2().modify(|_, w| match self.pin {
            ControlPin::Pc4 => w.pc4_dout().set_bit(),
            ControlPin::Pc6 => w.pc6_dout().set_bit(),
            ControlPin::Pc10 => w.pc10_dout().set_bit(),
        });
        Ok(())
    }
}

pub struct BusyDelay;

impl DelayNs for BusyDelay {
    fn delay_ns(&mut self, ns: u32) {
        crate::board::delay_ns(ns);
    }
}

pub struct G32Spi1<'a> {
    dp: &'a Peripherals,
}

impl<'a> G32Spi1<'a> {
    pub fn new(dp: &'a Peripherals) -> Self {
        Self { dp }
    }

    fn flush_bus(&self) -> Result<(), SpiError> {
        let mut timeout = SPI_TIMEOUT;
        while self.dp.spi1.sts().read().bsyflg().bit_is_set() {
            if timeout == 0 {
                return Err(SpiError::Timeout);
            }
            timeout -= 1;
        }
        Ok(())
    }

    fn transfer_byte(&self, tx: u8) -> Result<u8, SpiError> {
        let mut timeout = SPI_TIMEOUT;
        while self.dp.spi1.sts().read().txbeflg().bit_is_clear() {
            if timeout == 0 {
                return Err(SpiError::Timeout);
            }
            timeout -= 1;
        }

        self.dp
            .spi1
            .data()
            .write(|w| unsafe { w.data().bits(tx as u16) });

        timeout = SPI_TIMEOUT;
        while self.dp.spi1.sts().read().rxbneflg().bit_is_clear() {
            let sts = self.dp.spi1.sts().read();
            if sts.ovrflg().bit_is_set() {
                return Err(SpiError::Overrun);
            }
            if sts.meflg().bit_is_set() {
                return Err(SpiError::ModeFault);
            }
            if timeout == 0 {
                return Err(SpiError::Timeout);
            }
            timeout -= 1;
        }

        Ok(self.dp.spi1.data().read().data().bits() as u8)
    }
}

impl ErrorType for G32Spi1<'_> {
    type Error = SpiError;
}

impl SpiDevice for G32Spi1<'_> {
    fn transaction(&mut self, operations: &mut [Operation<'_, u8>]) -> Result<(), Self::Error> {
        for operation in operations {
            match operation {
                Operation::Read(words) => {
                    for word in words.iter_mut() {
                        *word = self.transfer_byte(0xff)?;
                    }
                }
                Operation::Write(words) => {
                    for &word in words.iter() {
                        let _ = self.transfer_byte(word)?;
                    }
                }
                Operation::Transfer(read, write) => {
                    let len = core::cmp::max(read.len(), write.len());
                    for i in 0..len {
                        let tx = write.get(i).copied().unwrap_or(0xff);
                        let rx = self.transfer_byte(tx)?;
                        if let Some(slot) = read.get_mut(i) {
                            *slot = rx;
                        }
                    }
                }
                Operation::TransferInPlace(words) => {
                    for word in words.iter_mut() {
                        *word = self.transfer_byte(*word)?;
                    }
                }
                Operation::DelayNs(ns) => {
                    let mut delay = BusyDelay;
                    delay.delay_ns(*ns);
                }
            }
        }

        self.flush_bus()
    }
}

pub fn init_spi1_for_sh1106(dp: &Peripherals) {
    rcm_unlock(dp);
    dp.rcm.ahbcg().modify(|_, w| w.gpiocen().set_bit());
    dp.rcm.apbcg().modify(|_, w| w.spicen().set_bit());
    rcm_lock(dp);

    // PC5 = SCK (AF2), PC11 = MOSI (AF1)
    // PC4 = DC, PC6 = CS, PC10 = RST
    dp.gpio.otyper2().modify(|_, w| {
        w.pc4_ot()
            .clear_bit()
            .pc5_ot()
            .clear_bit()
            .pc6_ot()
            .clear_bit()
            .pc10_ot()
            .clear_bit()
            .pc11_ot()
            .clear_bit()
    });
    dp.gpio.pupdr3().modify(|_, w| {
        w.pc4_pue()
            .clear_bit()
            .pc4_pus()
            .clear_bit()
            .pc5_pue()
            .clear_bit()
            .pc5_pus()
            .clear_bit()
            .pc6_pue()
            .clear_bit()
            .pc6_pus()
            .clear_bit()
            .pc10_pue()
            .clear_bit()
            .pc10_pus()
            .clear_bit()
            .pc11_pue()
            .clear_bit()
            .pc11_pus()
            .clear_bit()
    });
    dp.gpio.ospeedr3().modify(|_, w| unsafe {
        w.pc4_os()
            .bits(GPIO_SPEED_HIGH)
            .pc5_os()
            .bits(GPIO_SPEED_HIGH)
            .pc6_os()
            .bits(GPIO_SPEED_HIGH)
            .pc10_os()
            .bits(GPIO_SPEED_HIGH)
            .pc11_os()
            .bits(GPIO_SPEED_HIGH)
    });
    dp.gpio.afr3().modify(|_, w| unsafe {
        w.pc5_afsel().bits(2).pc11_afsel().bits(1)
    });
    dp.gpio.moder3().modify(|_, w| unsafe {
        w.pc4_md()
            .bits(GPIO_MODE_OUTPUT)
            .pc5_md()
            .bits(GPIO_MODE_ALTERNATE)
            .pc6_md()
            .bits(GPIO_MODE_OUTPUT)
            .pc10_md()
            .bits(GPIO_MODE_OUTPUT)
            .pc11_md()
            .bits(GPIO_MODE_ALTERNATE)
    });

    dp.gpio.odr2().modify(|_, w| {
        w.pc4_dout()
            .clear_bit()
            .pc6_dout()
            .set_bit()
            .pc10_dout()
            .set_bit()
    });

    dp.spi1.ctrl1().modify(|_, w| w.spien().clear_bit());
    dp.spi1.ctrl1().modify(|_, w| unsafe {
        w.cpha()
            .clear_bit()
            .cpol()
            .clear_bit()
            .msmcfg()
            .set_bit()
            .brsel()
            .bits(3)
            .lsbsel()
            .clear_bit()
            .issel()
            .set_bit()
            .ssen()
            .set_bit()
            .rxomen()
            .clear_bit()
            .dflsel()
            .clear_bit()
            .bmoen()
            .clear_bit()
            .bmen()
            .clear_bit()
    });
    dp.spi1.ctrl2().modify(|_, w| {
        w.ssoen()
            .clear_bit()
            .frfcfg()
            .clear_bit()
            .rxden()
            .clear_bit()
            .txden()
            .clear_bit()
    });
    dp.spi1.ctrl1().modify(|_, w| w.spien().set_bit());
}
