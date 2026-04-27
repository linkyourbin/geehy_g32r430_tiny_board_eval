use apmg32r430xx_pac as pac;

use crate::hal::{native, pac::Peripherals};

const ADC_READY_TIMEOUT: u32 = 1_000_000;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AdcError {
    Timeout,
    Overrun,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Adc16Channel {
    Ch0 = 0,
    Ch1 = 1,
    Ch2 = 2,
    Ch3 = 3,
    Ch4 = 4,
    Ch5 = 5,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Adc12Channel {
    Ch0 = 0,
    Ch1 = 1,
    Ch2 = 2,
    Ch3 = 3,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Adc16SampleTime {
    Cycles10 = 0b1001,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Adc12SampleTime {
    Cycles128 = 0b0111,
    Cycles256 = 0b1000,
}

fn enable_adc16_clock(adc_index: u8) {
    let dp = unsafe { Peripherals::steal() };
    crate::board::rcm_unlock(&dp);
    dp.rcm
        .adcccr()
        .modify(|_, w| unsafe { w.adcaclksel().set_bit().adc16adiv().bits(0) });
    dp.rcm.ahbcg().modify(|_, w| match adc_index {
        1 => w.adc1en().set_bit(),
        2 => w.adc2en().set_bit(),
        _ => w,
    });
    crate::board::rcm_lock(&dp);
}

fn wait_adc16_clock_ready(dp: &Peripherals) -> Result<(), AdcError> {
    let mut timeout = ADC_READY_TIMEOUT;
    while dp.rcm.adcccr().read().adc16aclkrdy().bit_is_clear() {
        if timeout == 0 {
            return Err(AdcError::Timeout);
        }
        timeout -= 1;
    }
    Ok(())
}

fn clear_regular_flags(adc: &pac::adc1::RegisterBlock) {
    adc.isr().write(|w| {
        w.eoc().set_bit()
            .eos().set_bit()
            .ovr().set_bit()
            .eosmp()
            .set_bit()
    });
}

impl native::Adc1 {
    pub fn init_single_ended_continuous(
        &mut self,
        channel: Adc16Channel,
        sample_time: Adc16SampleTime,
    ) -> Result<(), AdcError> {
        enable_adc16_clock(1);
        let regs = self.regs();

        regs.cr().modify(|_, w| w.addis().set_bit());
        regs.cfgr().modify(|_, w| unsafe {
            w.exten()
                .bits(0)
                .cont()
                .set_bit()
                .discen()
                .clear_bit()
                .dmaen()
                .clear_bit()
                .ovrmod()
                .set_bit()
        });
        regs.cfgr2().modify(|_, w| w.difsel().clear_bit());

        match channel {
            Adc16Channel::Ch0 => {
                regs.smpr1().modify(|_, w| unsafe { w.smp0().bits(sample_time as u8) });
            }
            Adc16Channel::Ch1 => {
                regs.smpr1().modify(|_, w| unsafe { w.smp1().bits(sample_time as u8) });
            }
            Adc16Channel::Ch2 => {
                regs.smpr1().modify(|_, w| unsafe { w.smp2().bits(sample_time as u8) });
            }
            Adc16Channel::Ch3 => {
                regs.smpr1().modify(|_, w| unsafe { w.smp3().bits(sample_time as u8) });
            }
            Adc16Channel::Ch4 => {
                regs.smpr1().modify(|_, w| unsafe { w.smp4().bits(sample_time as u8) });
            }
            Adc16Channel::Ch5 => {
                regs.smpr1().modify(|_, w| unsafe { w.smp5().bits(sample_time as u8) });
            }
        }

        regs.sqr1().modify(|_, w| unsafe {
            w.rl().bits(0).sq1().bits(channel as u8)
        });
        regs.cal().modify(|_, w| w.ref_sel().clear_bit());
        clear_regular_flags(regs);
        regs.cr().modify(|_, w| w.aden().set_bit());

        let dp = unsafe { Peripherals::steal() };
        wait_adc16_clock_ready(&dp)?;
        Ok(())
    }

    pub fn start_regular_conversion(&mut self) {
        self.regs().cr().modify(|_, w| w.adstart().set_bit());
    }

    pub fn read_regular_data(&mut self) -> Result<u16, AdcError> {
        let regs = self.regs();
        let mut timeout = ADC_READY_TIMEOUT;
        while regs.isr().read().eoc().bit_is_clear() {
            if regs.isr().read().ovr().bit_is_set() {
                return Err(AdcError::Overrun);
            }
            if timeout == 0 {
                return Err(AdcError::Timeout);
            }
            timeout -= 1;
        }

        let value = regs.dr().read().rdata().bits();
        regs.isr().write(|w| w.eoc().set_bit());
        Ok(value)
    }
}

impl native::Adc2 {
    pub fn init_single_ended_continuous(
        &mut self,
        channel: Adc16Channel,
        sample_time: Adc16SampleTime,
    ) -> Result<(), AdcError> {
        enable_adc16_clock(2);
        let regs = self.regs();

        regs.cr().modify(|_, w| w.addis().set_bit());
        regs.cfgr().modify(|_, w| unsafe {
            w.exten()
                .bits(0)
                .cont()
                .set_bit()
                .discen()
                .clear_bit()
                .dmaen()
                .clear_bit()
                .ovrmod()
                .set_bit()
        });

        match channel {
            Adc16Channel::Ch0 => {
                regs.smpr1().modify(|_, w| unsafe { w.smp0().bits(sample_time as u8) });
            }
            Adc16Channel::Ch1 => {
                regs.smpr1().modify(|_, w| unsafe { w.smp1().bits(sample_time as u8) });
            }
            Adc16Channel::Ch2 => {
                regs.smpr1().modify(|_, w| unsafe { w.smp2().bits(sample_time as u8) });
            }
            Adc16Channel::Ch3 => {
                regs.smpr1().modify(|_, w| unsafe { w.smp3().bits(sample_time as u8) });
            }
            Adc16Channel::Ch4 => {
                regs.smpr1().modify(|_, w| unsafe { w.smp4().bits(sample_time as u8) });
            }
            Adc16Channel::Ch5 => {
                regs.smpr1().modify(|_, w| unsafe { w.smp5().bits(sample_time as u8) });
            }
        }

        regs.sqr1().modify(|_, w| unsafe {
            w.rl().bits(0).sq1().bits(channel as u8)
        });
        regs.cal().modify(|_, w| w.ref_sel().clear_bit());
        clear_regular_flags(regs);
        regs.cr().modify(|_, w| w.aden().set_bit());

        let dp = unsafe { Peripherals::steal() };
        wait_adc16_clock_ready(&dp)?;
        Ok(())
    }

    pub fn start_regular_conversion(&mut self) {
        self.regs().cr().modify(|_, w| w.adstart().set_bit());
    }

    pub fn read_regular_data(&mut self) -> Result<u16, AdcError> {
        let regs = self.regs();
        let mut timeout = ADC_READY_TIMEOUT;
        while regs.isr().read().eoc().bit_is_clear() {
            if regs.isr().read().ovr().bit_is_set() {
                return Err(AdcError::Overrun);
            }
            if timeout == 0 {
                return Err(AdcError::Timeout);
            }
            timeout -= 1;
        }

        let value = regs.dr().read().rdata().bits();
        regs.isr().write(|w| w.eoc().set_bit());
        Ok(value)
    }
}

// ADC3 support is intentionally deferred until its PAC differences are mapped cleanly
// against the vendor ADC12 examples.
