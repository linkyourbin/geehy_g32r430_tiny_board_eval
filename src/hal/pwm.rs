use core::{convert::Infallible, marker::PhantomData};

use apmg32r430xx_pac as pac;
use embedded_hal::pwm::{ErrorType, SetDutyCycle};

use crate::hal::native;

#[derive(Clone, Copy, Debug)]
pub enum Mode {
    Pwm1 = 6,
    Pwm2 = 7,
}

#[derive(Clone, Copy, Debug)]
pub struct Tim2;
#[derive(Clone, Copy, Debug)]
pub struct Tim3;
#[derive(Clone, Copy, Debug)]
pub struct Tim4;

pub struct Channel<TIM, const CH: u8> {
    _tim: PhantomData<TIM>,
}

pub type Tmr2Pwm<const CH: u8> = Channel<Tim2, CH>;
pub type Tmr3Pwm<const CH: u8> = Channel<Tim3, CH>;
pub type Tmr4Pwm<const CH: u8> = Channel<Tim4, CH>;

pub trait TimerRegs {
    fn regs() -> &'static pac::tmr2::RegisterBlock;
    fn enable_clock();
}

impl TimerRegs for Tim2 {
    fn regs() -> &'static pac::tmr2::RegisterBlock {
        unsafe { &*pac::Tmr2::ptr() }
    }

    fn enable_clock() {
        let dp = unsafe { pac::Peripherals::steal() };
        crate::board::rcm_unlock(&dp);
        dp.rcm.apbcg().modify(|_, w| w.tmr2cen().set_bit());
        crate::board::rcm_lock(&dp);
    }
}

impl TimerRegs for Tim3 {
    fn regs() -> &'static pac::tmr2::RegisterBlock {
        unsafe { &*pac::Tmr3::ptr() }
    }

    fn enable_clock() {
        let dp = unsafe { pac::Peripherals::steal() };
        crate::board::rcm_unlock(&dp);
        dp.rcm.apbcg().modify(|_, w| w.tmr3cen().set_bit());
        crate::board::rcm_lock(&dp);
    }
}

impl TimerRegs for Tim4 {
    fn regs() -> &'static pac::tmr2::RegisterBlock {
        unsafe { &*pac::Tmr4::ptr() }
    }

    fn enable_clock() {
        let dp = unsafe { pac::Peripherals::steal() };
        crate::board::rcm_unlock(&dp);
        dp.rcm.apbcg().modify(|_, w| w.tmr4cen().set_bit());
        crate::board::rcm_lock(&dp);
    }
}

impl<TIM, const CH: u8> Channel<TIM, CH>
where
    TIM: TimerRegs,
{
    pub fn new() -> Self {
        debug_assert!((1..=4).contains(&CH));
        TIM::enable_clock();
        Self { _tim: PhantomData }
    }

    pub fn configure(&mut self, prescaler: u16, period: u16, mode: Mode) {
        let regs = TIM::regs();

        regs.ctrl1().modify(|_, w| w.cnten().clear_bit().arpen().set_bit());
        regs.psc().write(|w| unsafe { w.psc().bits(prescaler) });
        regs.autorld()
            .write(|w| unsafe { w.autorld().bits(period as u32) });

        match CH {
            1 => {
                regs.ccm1_compare().modify(|_, w| unsafe {
                    w.cc1sel()
                        .bits(0)
                        .oc1pen()
                        .set_bit()
                        .oc1mod()
                        .bits(mode as u8)
                });
                regs.ccen()
                    .modify(|_, w| w.cc1pol().clear_bit().cc1en().set_bit());
            }
            2 => {
                regs.ccm1_compare().modify(|_, w| unsafe {
                    w.cc2sel()
                        .bits(0)
                        .oc2pen()
                        .set_bit()
                        .oc2mod()
                        .bits(mode as u8)
                });
                regs.ccen()
                    .modify(|_, w| w.cc2pol().clear_bit().cc2en().set_bit());
            }
            3 => {
                regs.ccm2_compare().modify(|_, w| unsafe {
                    w.cc3sel()
                        .bits(0)
                        .oc3pen()
                        .set_bit()
                        .oc3mod()
                        .bits(mode as u8)
                });
                regs.ccen()
                    .modify(|_, w| w.cc3pol().clear_bit().cc3en().set_bit());
            }
            4 => {
                regs.ccm2_compare().modify(|_, w| unsafe {
                    w.cc4sel()
                        .bits(0)
                        .oc4pen()
                        .set_bit()
                        .oc4mod()
                        .bits(mode as u8)
                });
                regs.ccen()
                    .modify(|_, w| w.cc4pol().clear_bit().cc4en().set_bit());
            }
            _ => unreachable!(),
        }

        regs.ceg().write(|w| w.ueg().set_bit());
        regs.ctrl1().modify(|_, w| w.cnten().set_bit());
    }
}

impl native::Tmr2 {
    pub fn into_pwm<const CH: u8>(self) -> Tmr2Pwm<CH> {
        Tmr2Pwm::new()
    }
}

impl native::Tmr3 {
    pub fn into_pwm<const CH: u8>(self) -> Tmr3Pwm<CH> {
        Tmr3Pwm::new()
    }
}

impl native::Tmr4 {
    pub fn into_pwm<const CH: u8>(self) -> Tmr4Pwm<CH> {
        Tmr4Pwm::new()
    }
}

impl<TIM, const CH: u8> ErrorType for Channel<TIM, CH> {
    type Error = Infallible;
}

impl<TIM, const CH: u8> SetDutyCycle for Channel<TIM, CH>
where
    TIM: TimerRegs,
{
    fn max_duty_cycle(&self) -> u16 {
        TIM::regs().autorld().read().autorld().bits() as u16
    }

    fn set_duty_cycle(&mut self, duty: u16) -> Result<(), Self::Error> {
        match CH {
            1 => TIM::regs()
                .cc1()
                .write(|w| unsafe { w.cc1_l().bits(duty).cc1_h().bits(0) }),
            2 => TIM::regs()
                .cc2()
                .write(|w| unsafe { w.cc2_l().bits(duty).cc2_h().bits(0) }),
            3 => TIM::regs()
                .cc3()
                .write(|w| unsafe { w.cc3_l().bits(duty).cc3_h().bits(0) }),
            4 => TIM::regs()
                .cc4()
                .write(|w| unsafe { w.cc4_l().bits(duty).cc4_h().bits(0) }),
            _ => unreachable!(),
        };
        Ok(())
    }
}
