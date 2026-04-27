use core::fmt;

use apmg32r430xx_pac::{Peripherals, usart1::RegisterBlock as UartRegisterBlock};
use embedded_hal_nb::{nb, serial};
use embedded_io::{Error as IoError, ErrorKind as IoErrorKind, ErrorType as IoErrorType, Read as IoRead, ReadReady, Write as IoWrite, WriteReady};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UartError {
    Overrun,
    FrameFormat,
    Parity,
    Noise,
    Other,
}

impl serial::Error for UartError {
    fn kind(&self) -> serial::ErrorKind {
        match self {
            Self::Overrun => serial::ErrorKind::Overrun,
            Self::FrameFormat => serial::ErrorKind::FrameFormat,
            Self::Parity => serial::ErrorKind::Parity,
            Self::Noise => serial::ErrorKind::Noise,
            Self::Other => serial::ErrorKind::Other,
        }
    }
}

impl fmt::Display for UartError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

impl core::error::Error for UartError {}

impl IoError for UartError {
    fn kind(&self) -> IoErrorKind {
        match self {
            Self::Overrun => IoErrorKind::Other,
            Self::FrameFormat => IoErrorKind::InvalidData,
            Self::Parity => IoErrorKind::InvalidData,
            Self::Noise => IoErrorKind::InvalidData,
            Self::Other => IoErrorKind::Other,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Usart1;

#[derive(Clone, Copy, Debug)]
pub struct Usart2;

fn regs_usart1() -> &'static UartRegisterBlock {
    unsafe { &*apmg32r430xx_pac::Usart1::ptr() }
}

fn regs_usart2() -> &'static UartRegisterBlock {
    unsafe { &*apmg32r430xx_pac::Usart2::ptr() }
}

fn map_status_error(sts: &apmg32r430xx_pac::usart1::sts::R) -> Option<UartError> {
    if sts.ovreflg().bit_is_set() {
        Some(UartError::Overrun)
    } else if sts.feflg().bit_is_set() {
        Some(UartError::FrameFormat)
    } else if sts.peflg().bit_is_set() {
        Some(UartError::Parity)
    } else if sts.neflg().bit_is_set() {
        Some(UartError::Noise)
    } else {
        None
    }
}

impl Usart1 {
    pub(crate) fn new() -> Self {
        Self
    }

    pub fn init_default(&mut self, baud: u32) {
        let dp = unsafe { Peripherals::steal() };
        crate::uart::init_usart1(&dp, baud);
    }

    pub fn write_byte_blocking(&mut self, byte: u8) {
        let dp = unsafe { Peripherals::steal() };
        crate::uart::write_byte(&dp, byte);
    }

    pub fn write_str_blocking(&mut self, s: &str) {
        let dp = unsafe { Peripherals::steal() };
        crate::uart::write_str(&dp, s);
    }

    pub fn flush_blocking(&mut self) {
        let dp = unsafe { Peripherals::steal() };
        crate::uart::flush(&dp);
    }
}

impl Usart2 {
    pub(crate) fn new() -> Self {
        Self
    }
}

impl serial::ErrorType for Usart1 {
    type Error = UartError;
}

impl serial::Read<u8> for Usart1 {
    fn read(&mut self) -> nb::Result<u8, Self::Error> {
        let regs = regs_usart1();
        let sts = regs.sts().read();

        if let Some(error) = map_status_error(&sts) {
            Err(nb::Error::Other(error))
        } else if sts.rxbneflg().bit_is_set() {
            Ok(regs.data().read().data().bits() as u8)
        } else {
            Err(nb::Error::WouldBlock)
        }
    }
}

impl serial::Write<u8> for Usart1 {
    fn write(&mut self, word: u8) -> nb::Result<(), Self::Error> {
        let regs = regs_usart1();
        let sts = regs.sts().read();

        if let Some(error) = map_status_error(&sts) {
            Err(nb::Error::Other(error))
        } else if sts.txbeflg().bit_is_set() {
            regs.data().write(|w| unsafe { w.data().bits(word as u16) });
            Ok(())
        } else {
            Err(nb::Error::WouldBlock)
        }
    }

    fn flush(&mut self) -> nb::Result<(), Self::Error> {
        let regs = regs_usart1();
        let sts = regs.sts().read();

        if let Some(error) = map_status_error(&sts) {
            Err(nb::Error::Other(error))
        } else if sts.txcflg().bit_is_set() {
            Ok(())
        } else {
            Err(nb::Error::WouldBlock)
        }
    }
}

impl fmt::Write for Usart1 {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_str_blocking(s);
        Ok(())
    }
}

impl IoErrorType for Usart1 {
    type Error = UartError;
}

impl IoWrite for Usart1 {
    fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error> {
        for &byte in buf {
            nb::block!(serial::Write::write(self, byte))?;
        }
        Ok(buf.len())
    }

    fn flush(&mut self) -> Result<(), Self::Error> {
        nb::block!(serial::Write::flush(self))
    }
}

impl IoRead for Usart1 {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error> {
        if buf.is_empty() {
            return Ok(0);
        }

        buf[0] = nb::block!(serial::Read::read(self))?;
        Ok(1)
    }
}

impl ReadReady for Usart1 {
    fn read_ready(&mut self) -> Result<bool, Self::Error> {
        Ok(regs_usart1().sts().read().rxbneflg().bit_is_set())
    }
}

impl WriteReady for Usart1 {
    fn write_ready(&mut self) -> Result<bool, Self::Error> {
        Ok(regs_usart1().sts().read().txbeflg().bit_is_set())
    }
}

impl serial::ErrorType for Usart2 {
    type Error = UartError;
}

impl serial::Read<u8> for Usart2 {
    fn read(&mut self) -> nb::Result<u8, Self::Error> {
        let regs = regs_usart2();
        let sts = regs.sts().read();

        if let Some(error) = map_status_error(&sts) {
            Err(nb::Error::Other(error))
        } else if sts.rxbneflg().bit_is_set() {
            Ok(regs.data().read().data().bits() as u8)
        } else {
            Err(nb::Error::WouldBlock)
        }
    }
}

impl serial::Write<u8> for Usart2 {
    fn write(&mut self, word: u8) -> nb::Result<(), Self::Error> {
        let regs = regs_usart2();
        let sts = regs.sts().read();

        if let Some(error) = map_status_error(&sts) {
            Err(nb::Error::Other(error))
        } else if sts.txbeflg().bit_is_set() {
            regs.data().write(|w| unsafe { w.data().bits(word as u16) });
            Ok(())
        } else {
            Err(nb::Error::WouldBlock)
        }
    }

    fn flush(&mut self) -> nb::Result<(), Self::Error> {
        let regs = regs_usart2();
        let sts = regs.sts().read();

        if let Some(error) = map_status_error(&sts) {
            Err(nb::Error::Other(error))
        } else if sts.txcflg().bit_is_set() {
            Ok(())
        } else {
            Err(nb::Error::WouldBlock)
        }
    }
}
