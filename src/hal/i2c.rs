use apmg32r430xx_pac::Peripherals;
use embedded_hal::i2c::{ErrorType, I2c, Operation};

pub use crate::i2c::{FAST_MODE_400KHZ, I2cError, STANDARD_MODE_100KHZ};

#[derive(Clone, Copy, Debug)]
pub struct I2c1;

impl I2c1 {
    pub(crate) fn new() -> Self {
        Self
    }

    pub fn init_default(&mut self, periph_clk_hz: u32, speed_hz: u32) {
        let dp = unsafe { Peripherals::steal() };
        crate::i2c::init_i2c1(&dp, periph_clk_hz, speed_hz);
    }
}

impl ErrorType for I2c1 {
    type Error = I2cError;
}

impl I2c for I2c1 {
    fn transaction(
        &mut self,
        address: u8,
        operations: &mut [Operation<'_>],
    ) -> Result<(), Self::Error> {
        let dp = unsafe { Peripherals::steal() };
        let mut inner = crate::i2c::G32I2c::new(&dp);
        inner.transaction(address, operations)
    }
}
