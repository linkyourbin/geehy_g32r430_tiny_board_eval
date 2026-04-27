use apmg32r430xx_pac::Peripherals;
use embedded_hal::{
    delay::DelayNs,
    spi::{ErrorType, Operation, SpiBus, SpiDevice},
};

pub use crate::spi::SpiError;

#[derive(Clone, Copy, Debug, Default)]
pub struct Delay;

impl DelayNs for Delay {
    fn delay_ns(&mut self, ns: u32) {
        let mut inner = crate::spi::BusyDelay;
        inner.delay_ns(ns);
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Spi1;

impl Spi1 {
    pub(crate) fn new() -> Self {
        Self
    }

    pub fn init_default(&mut self) {
        let dp = unsafe { Peripherals::steal() };
        crate::spi::init_spi1_for_sh1106(&dp);
    }
}

impl ErrorType for Spi1 {
    type Error = SpiError;
}

impl SpiBus for Spi1 {
    fn read(&mut self, words: &mut [u8]) -> Result<(), Self::Error> {
        let dp = unsafe { Peripherals::steal() };
        let mut dev = crate::spi::G32Spi1::new(&dp);
        dev.transaction(&mut [Operation::Read(words)])
    }

    fn write(&mut self, words: &[u8]) -> Result<(), Self::Error> {
        let dp = unsafe { Peripherals::steal() };
        let mut dev = crate::spi::G32Spi1::new(&dp);
        dev.transaction(&mut [Operation::Write(words)])
    }

    fn transfer(&mut self, read: &mut [u8], write: &[u8]) -> Result<(), Self::Error> {
        let dp = unsafe { Peripherals::steal() };
        let mut dev = crate::spi::G32Spi1::new(&dp);
        dev.transaction(&mut [Operation::Transfer(read, write)])
    }

    fn transfer_in_place(&mut self, words: &mut [u8]) -> Result<(), Self::Error> {
        let dp = unsafe { Peripherals::steal() };
        let mut dev = crate::spi::G32Spi1::new(&dp);
        dev.transaction(&mut [Operation::TransferInPlace(words)])
    }

    fn flush(&mut self) -> Result<(), Self::Error> {
        let dp = unsafe { Peripherals::steal() };
        let mut dev = crate::spi::G32Spi1::new(&dp);
        dev.transaction(&mut [])
    }
}

impl SpiDevice for Spi1 {
    fn transaction(&mut self, operations: &mut [Operation<'_, u8>]) -> Result<(), Self::Error> {
        let dp = unsafe { Peripherals::steal() };
        let mut dev = crate::spi::G32Spi1::new(&dp);
        dev.transaction(operations)
    }
}
