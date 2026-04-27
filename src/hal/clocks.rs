use crate::board;

/// Frozen clock description.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Clocks {
    pub sysclk_hz: u32,
    pub hclk_hz: u32,
    pub pclk_hz: u32,
}

impl Clocks {
    pub const fn new(sysclk_hz: u32, hclk_hz: u32, pclk_hz: u32) -> Self {
        Self {
            sysclk_hz,
            hclk_hz,
            pclk_hz,
        }
    }
}

/// Configure the board to the known 128 MHz system clock.
pub fn init_128mhz() -> Clocks {
    let dp = unsafe { crate::hal::pac::Peripherals::steal() };
    board::init_system_clock(&dp);
    Clocks::new(128_000_000, 128_000_000, 128_000_000)
}
