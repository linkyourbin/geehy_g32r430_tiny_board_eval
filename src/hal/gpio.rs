use core::{convert::Infallible, marker::PhantomData, ptr::{read_volatile, write_volatile}};

use embedded_hal::digital::{ErrorType, InputPin, OutputPin, StatefulOutputPin};

#[derive(Clone, Copy, Debug)]
pub enum Pull {
    None = 0,
    Up = 1,
    Down = 2,
}

#[derive(Clone, Copy, Debug)]
pub enum Speed {
    Low = 1,
    Medium = 2,
    High = 3,
}

#[derive(Clone, Copy, Debug)]
pub enum OutputType {
    PushPull = 0,
    OpenDrain = 1,
}

#[derive(Clone, Copy, Debug)]
pub struct Input;
#[derive(Clone, Copy, Debug)]
pub struct Output;
#[derive(Clone, Copy, Debug)]
pub struct Analog;
#[derive(Clone, Copy, Debug)]
pub struct Alternate<const AF: u8>;

#[derive(Clone, Copy, Debug)]
pub struct Gpio;

#[derive(Clone, Copy, Debug)]
pub struct Pin<const PORT: u8, const N: u8, MODE> {
    _mode: PhantomData<MODE>,
}

const GPIO_BASE: usize = 0x4002_1800;

const MODER_BASE: usize = 0x00;
const PUPDR_BASE: usize = 0x10;
const OTYPER_BASE: usize = 0x20;
const OSPEEDR_BASE: usize = 0x28;
const IDR_BASE: usize = 0x38;
const ODR_BASE: usize = 0x40;
const AFR_BASE: usize = 0x48;
const BSRR_BASE: usize = 0x60;

const MODE_ANALOG: u32 = 0b00;
const MODE_INPUT: u32 = 0b01;
const MODE_OUTPUT: u32 = 0b10;
const MODE_ALTERNATE: u32 = 0b11;

impl Gpio {
    pub(crate) fn new() -> Self {
        Self
    }

    pub fn pin<const PORT: u8, const N: u8>(&self) -> Pin<PORT, N, Input> {
        debug_assert!(PORT < 4);
        debug_assert!(N < 16);
        Pin::new()
    }
}

impl<const PORT: u8, const N: u8, MODE> Pin<PORT, N, MODE> {
    const fn new() -> Self {
        Self {
            _mode: PhantomData,
        }
    }

    fn reg32(base: usize) -> *mut u32 {
        (GPIO_BASE + base + PORT as usize * 4) as *mut u32
    }

    fn reg16(base: usize) -> *mut u16 {
        (GPIO_BASE + base + PORT as usize * 2) as *mut u16
    }

    fn modify2bit(base: usize, value: u32) {
        let reg = Self::reg32(base);
        let shift = N as u32 * 2;
        let mut bits = unsafe { read_volatile(reg) };
        bits &= !(0b11 << shift);
        bits |= value << shift;
        unsafe { write_volatile(reg, bits) };
    }

    fn modify1bit_16(base: usize, bit: bool) {
        let reg = Self::reg16(base);
        let shift = N as u16;
        let mut bits = unsafe { read_volatile(reg) };
        bits &= !(1 << shift);
        bits |= (bit as u16) << shift;
        unsafe { write_volatile(reg, bits) };
    }

    pub fn set_pull(&mut self, pull: Pull) {
        Self::modify2bit(PUPDR_BASE, pull as u32);
    }

    pub fn set_speed(&mut self, speed: Speed) {
        Self::modify2bit(OSPEEDR_BASE, speed as u32);
    }

    pub fn set_output_type(&mut self, output_type: OutputType) {
        Self::modify1bit_16(OTYPER_BASE, matches!(output_type, OutputType::OpenDrain));
    }

    pub fn into_input(mut self) -> Pin<PORT, N, Input> {
        Self::modify2bit(MODER_BASE, MODE_INPUT);
        self.set_pull(Pull::None);
        Pin::new()
    }

    pub fn into_output(mut self) -> Pin<PORT, N, Output> {
        Self::modify2bit(MODER_BASE, MODE_OUTPUT);
        self.set_pull(Pull::None);
        self.set_output_type(OutputType::PushPull);
        self.set_speed(Speed::High);
        Pin::new()
    }

    pub fn into_analog(self) -> Pin<PORT, N, Analog> {
        Self::modify2bit(MODER_BASE, MODE_ANALOG);
        Pin::new()
    }

    pub fn into_alternate<const AF: u8>(mut self) -> Pin<PORT, N, Alternate<AF>> {
        let reg = Self::reg32(AFR_BASE);
        let shift = N as u32 * 2;
        let mut bits = unsafe { read_volatile(reg) };
        bits &= !(0b11 << shift);
        bits |= (AF as u32 & 0b11) << shift;
        unsafe { write_volatile(reg, bits) };

        Self::modify2bit(MODER_BASE, MODE_ALTERNATE);
        self.set_pull(Pull::None);
        self.set_output_type(OutputType::PushPull);
        self.set_speed(Speed::High);
        Pin::new()
    }
}

impl<const PORT: u8, const N: u8> ErrorType for Pin<PORT, N, Output> {
    type Error = Infallible;
}

impl<const PORT: u8, const N: u8> OutputPin for Pin<PORT, N, Output> {
    fn set_low(&mut self) -> Result<(), Self::Error> {
        let reg = Self::reg32(BSRR_BASE);
        unsafe { write_volatile(reg, 1u32 << (N + 16)) };
        Ok(())
    }

    fn set_high(&mut self) -> Result<(), Self::Error> {
        let reg = Self::reg32(BSRR_BASE);
        unsafe { write_volatile(reg, 1u32 << N) };
        Ok(())
    }
}

impl<const PORT: u8, const N: u8> StatefulOutputPin for Pin<PORT, N, Output> {
    fn is_set_high(&mut self) -> Result<bool, Self::Error> {
        let reg = Self::reg16(ODR_BASE);
        Ok((unsafe { read_volatile(reg) } & (1u16 << N)) != 0)
    }

    fn is_set_low(&mut self) -> Result<bool, Self::Error> {
        Ok(!self.is_set_high()?)
    }
}

impl<const PORT: u8, const N: u8> ErrorType for Pin<PORT, N, Input> {
    type Error = Infallible;
}

impl<const PORT: u8, const N: u8> InputPin for Pin<PORT, N, Input> {
    fn is_high(&mut self) -> Result<bool, Self::Error> {
        let reg = Self::reg16(IDR_BASE);
        Ok((unsafe { read_volatile(reg) } & (1u16 << N)) != 0)
    }

    fn is_low(&mut self) -> Result<bool, Self::Error> {
        Ok(!self.is_high()?)
    }
}
