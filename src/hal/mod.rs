//! G32R430 HAL foundation.
//!
//! This module turns the raw PAC into a typed HAL layer.
//! `embedded-hal` 1.0 only standardizes a subset of peripherals:
//! digital GPIO, delay, I2C, PWM and SPI.
//! For the rest of the chip peripherals this HAL exposes typed native wrappers.

pub use apmg32r430xx_pac as pac;

pub mod clocks;
pub mod adc;
pub mod delay;
pub mod gpio;
pub mod i2c;
pub mod native;
pub mod prelude;
pub mod pwm;
pub mod spi;
pub mod tmr;
pub mod uart;

/// Split peripheral tokens for the whole chip.
pub struct Parts {
    pub dma: native::Dma,
    pub rcm: native::Rcm,
    pub fmc: native::Fmc,
    pub adc1: native::Adc1,
    pub adc2: native::Adc2,
    pub adc3: native::Adc3,
    pub gpio: gpio::Gpio,
    pub tmr1: native::Tmr1,
    pub tmr2: native::Tmr2,
    pub tmr3: native::Tmr3,
    pub tmr4: native::Tmr4,
    pub spi1: spi::Spi1,
    pub usart1: uart::Usart1,
    pub usart2: uart::Usart2,
    pub i2c1: i2c::I2c1,
    pub wwdt: native::Wwdt,
    pub iwdt: native::Iwdt,
    pub eint: native::Eint,
    pub dac1: native::Dac1,
    pub dac2: native::Dac2,
    pub comp: native::Comp,
    pub pmu: native::Pmu,
    pub bakpr: native::Bakpr,
    pub rtc: native::Rtc,
    pub lptmr: native::Lptmr,
    pub ts: native::Ts,
    pub dbgmcu: native::Dbgmcu,
}

impl Parts {
    /// Take all chip peripherals once.
    pub fn take() -> Option<Self> {
        pac::Peripherals::take().map(Self::from_pac)
    }

    /// Construct HAL tokens from PAC tokens.
    pub fn from_pac(_pac: pac::Peripherals) -> Self {
        Self {
            dma: native::Dma::new(),
            rcm: native::Rcm::new(),
            fmc: native::Fmc::new(),
            adc1: native::Adc1::new(),
            adc2: native::Adc2::new(),
            adc3: native::Adc3::new(),
            gpio: gpio::Gpio::new(),
            tmr1: native::Tmr1::new(),
            tmr2: native::Tmr2::new(),
            tmr3: native::Tmr3::new(),
            tmr4: native::Tmr4::new(),
            spi1: spi::Spi1::new(),
            usart1: uart::Usart1::new(),
            usart2: uart::Usart2::new(),
            i2c1: i2c::I2c1::new(),
            wwdt: native::Wwdt::new(),
            iwdt: native::Iwdt::new(),
            eint: native::Eint::new(),
            dac1: native::Dac1::new(),
            dac2: native::Dac2::new(),
            comp: native::Comp::new(),
            pmu: native::Pmu::new(),
            bakpr: native::Bakpr::new(),
            rtc: native::Rtc::new(),
            lptmr: native::Lptmr::new(),
            ts: native::Ts::new(),
            dbgmcu: native::Dbgmcu::new(),
        }
    }
}
