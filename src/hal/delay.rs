use embedded_hal::delay::DelayNs;

/// Enable the core DWT cycle counter for busy-wait delays.
pub fn enable_cycle_counter(cp: &mut cortex_m::Peripherals) {
    crate::board::enable_cycle_counter(cp);
}

/// Busy wait delay backed by the ARM DWT cycle counter.
#[derive(Clone, Copy, Debug, Default)]
pub struct Delay;

impl DelayNs for Delay {
    fn delay_ns(&mut self, ns: u32) {
        crate::board::delay_ns(ns);
    }
}
