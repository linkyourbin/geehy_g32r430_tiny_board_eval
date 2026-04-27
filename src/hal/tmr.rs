use core::convert::Infallible;

use apmg32r430xx_pac as pac;
use embedded_hal::pwm::{ErrorType, SetDutyCycle};

use crate::{board, hal::native};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TmrChannel {
    Ch1,
    Ch2,
    Ch3,
    Ch4,
}

impl TmrChannel {
    const fn ccen_shift(self) -> u32 {
        match self {
            Self::Ch1 => 0,
            Self::Ch2 => 4,
            Self::Ch3 => 8,
            Self::Ch4 => 12,
        }
    }

    const fn idle_shift(self) -> u32 {
        match self {
            Self::Ch1 => 8,
            Self::Ch2 => 10,
            Self::Ch3 => 12,
            Self::Ch4 => 14,
        }
    }

    const fn supports_complementary_output(self) -> bool {
        !matches!(self, Self::Ch4)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TmrCounterMode {
    Up,
    Down,
    CenterAligned1,
    CenterAligned2,
    CenterAligned3,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TmrClockDivision {
    Div1 = 0,
    Div2 = 1,
    Div4 = 2,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TmrUpdateRequestSource {
    Any = 0,
    CounterOnly = 1,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TmrOutputCompareMode {
    Frozen = 0,
    ActiveOnMatch = 1,
    InactiveOnMatch = 2,
    Toggle = 3,
    ForceInactive = 4,
    ForceActive = 5,
    Pwm1 = 6,
    Pwm2 = 7,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TmrPolarity {
    ActiveHigh = 0,
    ActiveLow = 1,
}

impl TmrPolarity {
    const fn bit(self) -> u32 {
        self as u32
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TmrIdleState {
    Low = 0,
    High = 1,
}

impl TmrIdleState {
    const fn bit(self) -> u32 {
        self as u32
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TmrInputSelection {
    Direct = 1,
    Indirect = 2,
    Trc = 3,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TmrInputPrescaler {
    Div1 = 0,
    Div2 = 1,
    Div4 = 2,
    Div8 = 3,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TmrCapturePolarity {
    Rising,
    Falling,
    BothEdges,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TmrSlaveMode {
    Disabled = 0,
    EncoderTi1 = 1,
    EncoderTi2 = 2,
    EncoderTi12 = 3,
    Reset = 4,
    Gated = 5,
    Trigger = 6,
    ExternalClock = 7,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TmrTriggerSource {
    Itr0 = 0,
    Itr1 = 1,
    Itr2 = 2,
    Itr3 = 3,
    Ti1EdgeDetector = 4,
    FilteredTi1 = 5,
    FilteredTi2 = 6,
    External = 7,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TmrMasterMode {
    Reset = 0,
    Enable = 1,
    Update = 2,
    ComparePulse = 3,
    Oc1Ref = 4,
    Oc2Ref = 5,
    Oc3Ref = 6,
    Oc4Ref = 7,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TmrExternalTriggerPrescaler {
    Div1 = 0,
    Div2 = 1,
    Div4 = 2,
    Div8 = 3,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TmrLockLevel {
    Off = 0,
    Level1 = 1,
    Level2 = 2,
    Level3 = 3,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TmrInterrupt {
    Update,
    CaptureCompare1,
    CaptureCompare2,
    CaptureCompare3,
    CaptureCompare4,
    Commutation,
    Trigger,
    Break,
}

impl TmrInterrupt {
    const fn mask(self) -> u32 {
        match self {
            Self::Update => 1 << 0,
            Self::CaptureCompare1 => 1 << 1,
            Self::CaptureCompare2 => 1 << 2,
            Self::CaptureCompare3 => 1 << 3,
            Self::CaptureCompare4 => 1 << 4,
            Self::Commutation => 1 << 5,
            Self::Trigger => 1 << 6,
            Self::Break => 1 << 7,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TmrDmaRequest {
    Update,
    CaptureCompare1,
    CaptureCompare2,
    CaptureCompare3,
    CaptureCompare4,
    Commutation,
    Trigger,
}

impl TmrDmaRequest {
    const fn mask(self) -> u32 {
        match self {
            Self::Update => 1 << 8,
            Self::CaptureCompare1 => 1 << 9,
            Self::CaptureCompare2 => 1 << 10,
            Self::CaptureCompare3 => 1 << 11,
            Self::CaptureCompare4 => 1 << 12,
            Self::Commutation => 1 << 13,
            Self::Trigger => 1 << 14,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TmrFlag {
    Update,
    CaptureCompare1,
    CaptureCompare2,
    CaptureCompare3,
    CaptureCompare4,
    Commutation,
    Trigger,
    Break,
    Overcapture1,
    Overcapture2,
    Overcapture3,
    Overcapture4,
}

impl TmrFlag {
    const fn mask(self) -> u32 {
        match self {
            Self::Update => 1 << 0,
            Self::CaptureCompare1 => 1 << 1,
            Self::CaptureCompare2 => 1 << 2,
            Self::CaptureCompare3 => 1 << 3,
            Self::CaptureCompare4 => 1 << 4,
            Self::Commutation => 1 << 5,
            Self::Trigger => 1 << 6,
            Self::Break => 1 << 7,
            Self::Overcapture1 => 1 << 9,
            Self::Overcapture2 => 1 << 10,
            Self::Overcapture3 => 1 << 11,
            Self::Overcapture4 => 1 << 12,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TmrEvent {
    Update,
    CaptureCompare1,
    CaptureCompare2,
    CaptureCompare3,
    CaptureCompare4,
    Commutation,
    Trigger,
    Break,
}

impl TmrEvent {
    const fn mask(self) -> u32 {
        match self {
            Self::Update => 1 << 0,
            Self::CaptureCompare1 => 1 << 1,
            Self::CaptureCompare2 => 1 << 2,
            Self::CaptureCompare3 => 1 << 3,
            Self::CaptureCompare4 => 1 << 4,
            Self::Commutation => 1 << 5,
            Self::Trigger => 1 << 6,
            Self::Break => 1 << 7,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TmrDmaBurstBase {
    Ctrl1 = 0x00,
    Ctrl2 = 0x01,
    Smctrl = 0x02,
    Dien = 0x03,
    Sts = 0x04,
    Ceg = 0x05,
    Ccm1 = 0x06,
    Ccm2 = 0x07,
    Ccen = 0x08,
    Cnt = 0x09,
    Psc = 0x0A,
    Autorld = 0x0B,
    Repcnt = 0x0C,
    Cc1 = 0x0D,
    Cc2 = 0x0E,
    Cc3 = 0x0F,
    Cc4 = 0x10,
    Bdt = 0x11,
    Dctrl = 0x12,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TmrError {
    ComplementaryOutputUnavailable,
    BothEdgeCaptureUnavailable,
    HallSensorBothEdgeUnsupported,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TmrTimeBase {
    pub prescaler: u16,
    pub auto_reload: u16,
    pub repetition_counter: u8,
    pub counter_mode: TmrCounterMode,
    pub clock_division: TmrClockDivision,
    pub auto_reload_preload: bool,
    pub update_disable: bool,
    pub update_request_source: TmrUpdateRequestSource,
    pub one_pulse_mode: bool,
}

impl Default for TmrTimeBase {
    fn default() -> Self {
        Self {
            prescaler: 0,
            auto_reload: 0xffff,
            repetition_counter: 0,
            counter_mode: TmrCounterMode::Up,
            clock_division: TmrClockDivision::Div1,
            auto_reload_preload: false,
            update_disable: false,
            update_request_source: TmrUpdateRequestSource::Any,
            one_pulse_mode: false,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TmrOutputCompareConfig {
    pub mode: TmrOutputCompareMode,
    pub compare: u16,
    pub enable_output: bool,
    pub enable_complementary_output: bool,
    pub output_polarity: TmrPolarity,
    pub complementary_output_polarity: TmrPolarity,
    pub output_idle_state: TmrIdleState,
    pub complementary_output_idle_state: TmrIdleState,
    pub preload: bool,
    pub fast_enable: bool,
    pub clear_enable: bool,
}

impl Default for TmrOutputCompareConfig {
    fn default() -> Self {
        Self {
            mode: TmrOutputCompareMode::Frozen,
            compare: 0,
            enable_output: false,
            enable_complementary_output: false,
            output_polarity: TmrPolarity::ActiveHigh,
            complementary_output_polarity: TmrPolarity::ActiveHigh,
            output_idle_state: TmrIdleState::Low,
            complementary_output_idle_state: TmrIdleState::Low,
            preload: false,
            fast_enable: false,
            clear_enable: false,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TmrInputCaptureConfig {
    pub selection: TmrInputSelection,
    pub prescaler: TmrInputPrescaler,
    pub filter: u8,
    pub polarity: TmrCapturePolarity,
    pub enable: bool,
}

impl Default for TmrInputCaptureConfig {
    fn default() -> Self {
        Self {
            selection: TmrInputSelection::Direct,
            prescaler: TmrInputPrescaler::Div1,
            filter: 0,
            polarity: TmrCapturePolarity::Rising,
            enable: false,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TmrEncoderConfig {
    pub mode: TmrSlaveMode,
    pub ch1_selection: TmrInputSelection,
    pub ch1_prescaler: TmrInputPrescaler,
    pub ch1_filter: u8,
    pub ch1_polarity: TmrCapturePolarity,
    pub ch2_selection: TmrInputSelection,
    pub ch2_prescaler: TmrInputPrescaler,
    pub ch2_filter: u8,
    pub ch2_polarity: TmrCapturePolarity,
}

impl Default for TmrEncoderConfig {
    fn default() -> Self {
        Self {
            mode: TmrSlaveMode::EncoderTi12,
            ch1_selection: TmrInputSelection::Direct,
            ch1_prescaler: TmrInputPrescaler::Div1,
            ch1_filter: 0,
            ch1_polarity: TmrCapturePolarity::Rising,
            ch2_selection: TmrInputSelection::Direct,
            ch2_prescaler: TmrInputPrescaler::Div1,
            ch2_filter: 0,
            ch2_polarity: TmrCapturePolarity::Rising,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TmrHallSensorConfig {
    pub prescaler: TmrInputPrescaler,
    pub filter: u8,
    pub polarity: TmrCapturePolarity,
    pub commutation_delay: u16,
}

impl Default for TmrHallSensorConfig {
    fn default() -> Self {
        Self {
            prescaler: TmrInputPrescaler::Div1,
            filter: 0,
            polarity: TmrCapturePolarity::Rising,
            commutation_delay: 0,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TmrSlaveConfig {
    pub mode: TmrSlaveMode,
    pub trigger: TmrTriggerSource,
    pub master_slave_mode: bool,
    pub external_trigger_filter: u8,
    pub external_trigger_prescaler: TmrExternalTriggerPrescaler,
    pub external_clock_mode: bool,
    pub external_trigger_polarity: TmrPolarity,
}

impl Default for TmrSlaveConfig {
    fn default() -> Self {
        Self {
            mode: TmrSlaveMode::Disabled,
            trigger: TmrTriggerSource::Itr0,
            master_slave_mode: false,
            external_trigger_filter: 0,
            external_trigger_prescaler: TmrExternalTriggerPrescaler::Div1,
            external_clock_mode: false,
            external_trigger_polarity: TmrPolarity::ActiveHigh,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TmrBreakDeadTimeConfig {
    pub dead_time: u8,
    pub lock_level: TmrLockLevel,
    pub off_state_idle: bool,
    pub off_state_run: bool,
    pub break_enable: bool,
    pub break_polarity: TmrPolarity,
    pub automatic_output: bool,
    pub main_output: bool,
}

impl Default for TmrBreakDeadTimeConfig {
    fn default() -> Self {
        Self {
            dead_time: 0,
            lock_level: TmrLockLevel::Off,
            off_state_idle: false,
            off_state_run: false,
            break_enable: false,
            break_polarity: TmrPolarity::ActiveHigh,
            automatic_output: false,
            main_output: false,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Tmr1ComplementaryPwm {
    tmr: native::Tmr1,
    channel: TmrChannel,
}

impl Tmr1ComplementaryPwm {
    pub fn channel(&self) -> TmrChannel {
        self.channel
    }

    pub fn duty(&self) -> u16 {
        self.tmr.compare(self.channel)
    }

    pub fn set_duty(&mut self, duty: u16) {
        self.tmr.set_compare(self.channel, duty);
    }

    pub fn max_duty(&self) -> u16 {
        self.tmr.auto_reload()
    }

    pub fn dead_time(&self) -> u8 {
        (self.tmr.regs().bdt().read().bits() & 0xff) as u8
    }

    pub fn set_dead_time(&mut self, dead_time: u8) {
        let regs = self.tmr.regs();
        let mut bits = regs.bdt().read().bits();
        bits &= !0xff;
        bits |= u32::from(dead_time);
        regs.bdt().write(|w| unsafe { w.bits(bits) });
    }

    pub fn start(&mut self) {
        self.tmr.enable_main_output();
        self.tmr.enable_counter();
    }

    pub fn stop(&mut self) {
        self.tmr.disable_main_output();
        self.tmr.disable_counter();
    }

    pub fn enable_primary_output(&mut self) {
        self.tmr.set_output_enabled(self.channel, true);
    }

    pub fn disable_primary_output(&mut self) {
        self.tmr.set_output_enabled(self.channel, false);
    }

    pub fn enable_complementary_output(&mut self) -> Result<(), TmrError> {
        self.tmr.set_complementary_output_enabled(self.channel, true)
    }

    pub fn disable_complementary_output(&mut self) -> Result<(), TmrError> {
        self.tmr.set_complementary_output_enabled(self.channel, false)
    }
}

impl ErrorType for Tmr1ComplementaryPwm {
    type Error = Infallible;
}

impl SetDutyCycle for Tmr1ComplementaryPwm {
    fn max_duty_cycle(&self) -> u16 {
        self.max_duty()
    }

    fn set_duty_cycle(&mut self, duty: u16) -> Result<(), Self::Error> {
        self.set_duty(duty);
        Ok(())
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Tmr1ComplementaryPwmBuilder {
    tmr: native::Tmr1,
    channel: TmrChannel,
    time_base: TmrTimeBase,
    output: TmrOutputCompareConfig,
    break_dead_time: TmrBreakDeadTimeConfig,
    generate_update_event: bool,
    start_immediately: bool,
}

impl Tmr1ComplementaryPwmBuilder {
    fn new(tmr: native::Tmr1, channel: TmrChannel) -> Result<Self, TmrError> {
        if !channel.supports_complementary_output() {
            return Err(TmrError::ComplementaryOutputUnavailable);
        }

        Ok(Self {
            tmr,
            channel,
            time_base: TmrTimeBase {
                auto_reload_preload: true,
                ..Default::default()
            },
            output: TmrOutputCompareConfig {
                mode: TmrOutputCompareMode::Pwm1,
                enable_output: true,
                enable_complementary_output: true,
                preload: true,
                ..Default::default()
            },
            break_dead_time: TmrBreakDeadTimeConfig {
                automatic_output: true,
                main_output: true,
                ..Default::default()
            },
            generate_update_event: true,
            start_immediately: true,
        })
    }

    pub fn time_base(mut self, config: TmrTimeBase) -> Self {
        self.time_base = config;
        self
    }

    pub fn output(mut self, config: TmrOutputCompareConfig) -> Self {
        self.output = config;
        self
    }

    pub fn break_dead_time(mut self, config: TmrBreakDeadTimeConfig) -> Self {
        self.break_dead_time = config;
        self
    }

    pub fn prescaler(mut self, value: u16) -> Self {
        self.time_base.prescaler = value;
        self
    }

    pub fn period(mut self, value: u16) -> Self {
        self.time_base.auto_reload = value;
        self
    }

    pub fn repetition_counter(mut self, value: u8) -> Self {
        self.time_base.repetition_counter = value;
        self
    }

    pub fn counter_mode(mut self, mode: TmrCounterMode) -> Self {
        self.time_base.counter_mode = mode;
        self
    }

    pub fn clock_division(mut self, division: TmrClockDivision) -> Self {
        self.time_base.clock_division = division;
        self
    }

    pub fn auto_reload_preload(mut self, enabled: bool) -> Self {
        self.time_base.auto_reload_preload = enabled;
        self
    }

    pub fn duty(mut self, value: u16) -> Self {
        self.output.compare = value;
        self
    }

    pub fn mode(mut self, mode: TmrOutputCompareMode) -> Self {
        self.output.mode = mode;
        self
    }

    pub fn dead_time(mut self, value: u8) -> Self {
        self.break_dead_time.dead_time = value;
        self
    }

    pub fn output_polarity(mut self, polarity: TmrPolarity) -> Self {
        self.output.output_polarity = polarity;
        self
    }

    pub fn complementary_output_polarity(mut self, polarity: TmrPolarity) -> Self {
        self.output.complementary_output_polarity = polarity;
        self
    }

    pub fn output_idle_state(mut self, state: TmrIdleState) -> Self {
        self.output.output_idle_state = state;
        self
    }

    pub fn complementary_output_idle_state(mut self, state: TmrIdleState) -> Self {
        self.output.complementary_output_idle_state = state;
        self
    }

    pub fn off_state_idle(mut self, enabled: bool) -> Self {
        self.break_dead_time.off_state_idle = enabled;
        self
    }

    pub fn off_state_run(mut self, enabled: bool) -> Self {
        self.break_dead_time.off_state_run = enabled;
        self
    }

    pub fn automatic_output(mut self, enabled: bool) -> Self {
        self.break_dead_time.automatic_output = enabled;
        self
    }

    pub fn main_output(mut self, enabled: bool) -> Self {
        self.break_dead_time.main_output = enabled;
        self
    }

    pub fn break_input(mut self, polarity: TmrPolarity) -> Self {
        self.break_dead_time.break_enable = true;
        self.break_dead_time.break_polarity = polarity;
        self
    }

    pub fn lock_level(mut self, level: TmrLockLevel) -> Self {
        self.break_dead_time.lock_level = level;
        self
    }

    pub fn generate_update_event(mut self, enabled: bool) -> Self {
        self.generate_update_event = enabled;
        self
    }

    pub fn start_immediately(mut self, enabled: bool) -> Self {
        self.start_immediately = enabled;
        self
    }

    pub fn build(self) -> Result<Tmr1ComplementaryPwm, TmrError> {
        self.tmr.reset();
        self.tmr.configure_time_base(self.time_base);
        self.tmr.configure_output_channel(self.channel, self.output)?;
        self.tmr.configure_break_dead_time(self.break_dead_time);

        if self.generate_update_event {
            self.tmr.generate_event(TmrEvent::Update);
        }
        if self.start_immediately {
            self.tmr.enable_counter();
        }

        Ok(Tmr1ComplementaryPwm {
            tmr: self.tmr,
            channel: self.channel,
        })
    }
}

fn output_polarity_bits(channel: TmrChannel, polarity: TmrPolarity, complementary: bool) -> Result<u32, TmrError> {
    let shift = channel.ccen_shift();
    if complementary {
        if !channel.supports_complementary_output() {
            return Err(TmrError::ComplementaryOutputUnavailable);
        }
        Ok(polarity.bit() << (shift + 3))
    } else {
        Ok(polarity.bit() << (shift + 1))
    }
}

fn capture_polarity_bits(channel: TmrChannel, polarity: TmrCapturePolarity) -> Result<u32, TmrError> {
    let shift = channel.ccen_shift();
    match polarity {
        TmrCapturePolarity::Rising => Ok(0),
        TmrCapturePolarity::Falling => Ok(1 << (shift + 1)),
        TmrCapturePolarity::BothEdges => {
            if !channel.supports_complementary_output() {
                Err(TmrError::BothEdgeCaptureUnavailable)
            } else {
                Ok((1 << (shift + 1)) | (1 << (shift + 3)))
            }
        }
    }
}

impl native::Tmr1 {
    pub fn complementary_pwm_builder(
        self,
        channel: TmrChannel,
    ) -> Result<Tmr1ComplementaryPwmBuilder, TmrError> {
        Tmr1ComplementaryPwmBuilder::new(self, channel)
    }

    pub fn enable_clock(&self) {
        let dp = unsafe { pac::Peripherals::steal() };
        board::rcm_unlock(&dp);
        dp.rcm.apbcg().modify(|_, w| w.tmr1cen().set_bit());
        while dp.rcm.apbcg().read().tmr1cen().bit_is_clear() {}
        board::rcm_lock(&dp);
    }

    pub fn reset(&self) {
        self.enable_clock();
        let dp = unsafe { pac::Peripherals::steal() };
        board::rcm_unlock(&dp);
        // On G32R430 APB reset bits are active-low:
        // clearing the bit asserts reset, setting it releases reset.
        dp.rcm.apbrst().modify(|_, w| w.tmr1rst().clear_bit());
        while dp.rcm.apbrst().read().tmr1rst().bit_is_set() {}
        dp.rcm.apbrst().modify(|_, w| w.tmr1rst().set_bit());
        while dp.rcm.apbrst().read().tmr1rst().bit_is_clear() {}
        board::rcm_lock(&dp);
    }

    pub fn configure_time_base(&self, config: TmrTimeBase) {
        self.enable_clock();
        self.disable_counter();

        let regs = self.regs();
        regs.psc()
            .write(|w| unsafe { w.bits(u32::from(config.prescaler)) });
        regs.autorld()
            .write(|w| unsafe { w.bits(u32::from(config.auto_reload)) });
        regs.repcnt()
            .write(|w| unsafe { w.bits(u32::from(config.repetition_counter)) });

        let (cntdir, camsel) = match config.counter_mode {
            TmrCounterMode::Up => (0, 0),
            TmrCounterMode::Down => (1, 0),
            TmrCounterMode::CenterAligned1 => (0, 1),
            TmrCounterMode::CenterAligned2 => (0, 2),
            TmrCounterMode::CenterAligned3 => (0, 3),
        };

        let mut bits = 0u32;
        bits |= (config.update_disable as u32) << 1;
        bits |= (config.update_request_source as u32) << 2;
        bits |= (config.one_pulse_mode as u32) << 3;
        bits |= cntdir << 4;
        bits |= camsel << 5;
        bits |= (config.auto_reload_preload as u32) << 7;
        bits |= (config.clock_division as u32) << 8;
        regs.ctrl1().write(|w| unsafe { w.bits(bits) });
    }

    pub fn set_master_mode(&self, mode: TmrMasterMode) {
        let regs = self.regs();
        let mut bits = regs.ctrl2().read().bits();
        bits &= !(0x7 << 4);
        bits |= (mode as u32) << 4;
        regs.ctrl2().write(|w| unsafe { w.bits(bits) });
    }

    pub fn set_capture_compare_preloaded_control(&self, enabled: bool) {
        let regs = self.regs();
        let mut bits = regs.ctrl2().read().bits();
        bits &= !(1 << 0);
        bits |= (enabled as u32) << 0;
        regs.ctrl2().write(|w| unsafe { w.bits(bits) });
    }

    pub fn set_capture_compare_update_selection(&self, enabled: bool) {
        let regs = self.regs();
        let mut bits = regs.ctrl2().read().bits();
        bits &= !(1 << 2);
        bits |= (enabled as u32) << 2;
        regs.ctrl2().write(|w| unsafe { w.bits(bits) });
    }

    pub fn set_capture_compare_dma_selection(&self, enabled: bool) {
        let regs = self.regs();
        let mut bits = regs.ctrl2().read().bits();
        bits &= !(1 << 3);
        bits |= (enabled as u32) << 3;
        regs.ctrl2().write(|w| unsafe { w.bits(bits) });
    }

    pub fn set_ti1_xor_enabled(&self, enabled: bool) {
        let regs = self.regs();
        let mut bits = regs.ctrl2().read().bits();
        bits &= !(1 << 7);
        bits |= (enabled as u32) << 7;
        regs.ctrl2().write(|w| unsafe { w.bits(bits) });
    }

    pub fn configure_slave_mode(&self, config: TmrSlaveConfig) {
        let regs = self.regs();
        let mut bits = regs.smctrl().read().bits();
        bits &= !((0x7 << 0) | (0x7 << 4) | (1 << 7) | (0x0f << 8) | (0x3 << 12) | (1 << 14) | (1 << 15));
        bits |= (config.mode as u32) << 0;
        bits |= (config.trigger as u32) << 4;
        bits |= (config.master_slave_mode as u32) << 7;
        bits |= u32::from(config.external_trigger_filter & 0x0f) << 8;
        bits |= (config.external_trigger_prescaler as u32) << 12;
        bits |= (config.external_clock_mode as u32) << 14;
        bits |= config.external_trigger_polarity.bit() << 15;
        regs.smctrl().write(|w| unsafe { w.bits(bits) });
    }

    pub fn configure_output_channel(
        &self,
        channel: TmrChannel,
        config: TmrOutputCompareConfig,
    ) -> Result<(), TmrError> {
        self.enable_clock();

        if config.enable_complementary_output && !channel.supports_complementary_output() {
            return Err(TmrError::ComplementaryOutputUnavailable);
        }

        self.set_compare(channel, config.compare);

        let regs = self.regs();
        match channel {
            TmrChannel::Ch1 => {
                let mut bits = regs.ccm1().read().bits();
                bits &= !0x00ff;
                bits |= (config.fast_enable as u32) << 2;
                bits |= (config.preload as u32) << 3;
                bits |= (config.mode as u32) << 4;
                bits |= (config.clear_enable as u32) << 7;
                regs.ccm1().write(|w| unsafe { w.bits(bits) });
            }
            TmrChannel::Ch2 => {
                let mut bits = regs.ccm1().read().bits();
                bits &= !0xff00;
                bits |= (config.fast_enable as u32) << 10;
                bits |= (config.preload as u32) << 11;
                bits |= (config.mode as u32) << 12;
                bits |= (config.clear_enable as u32) << 15;
                regs.ccm1().write(|w| unsafe { w.bits(bits) });
            }
            TmrChannel::Ch3 => {
                let mut bits = regs.ccm2().read().bits();
                bits &= !0x00ff;
                bits |= (config.fast_enable as u32) << 2;
                bits |= (config.preload as u32) << 3;
                bits |= (config.mode as u32) << 4;
                bits |= (config.clear_enable as u32) << 7;
                regs.ccm2().write(|w| unsafe { w.bits(bits) });
            }
            TmrChannel::Ch4 => {
                let mut bits = regs.ccm2().read().bits();
                bits &= !0xff00;
                bits |= (config.fast_enable as u32) << 10;
                bits |= (config.preload as u32) << 11;
                bits |= (config.mode as u32) << 12;
                bits |= (config.clear_enable as u32) << 15;
                regs.ccm2().write(|w| unsafe { w.bits(bits) });
            }
        }

        let idle_shift = channel.idle_shift();
        let mut ctrl2 = regs.ctrl2().read().bits();
        let idle_mask = if channel.supports_complementary_output() {
            0x3 << idle_shift
        } else {
            0x1 << idle_shift
        };
        ctrl2 &= !idle_mask;
        ctrl2 |= config.output_idle_state.bit() << idle_shift;
        if channel.supports_complementary_output() {
            ctrl2 |= config.complementary_output_idle_state.bit() << (idle_shift + 1);
        }
        regs.ctrl2().write(|w| unsafe { w.bits(ctrl2) });

        let shift = channel.ccen_shift();
        let mut ccen = regs.ccen().read().bits();
        ccen &= if channel.supports_complementary_output() {
            !(0x0f << shift)
        } else {
            !(0x03 << shift)
        };
        ccen |= (config.enable_output as u32) << shift;
        ccen |= output_polarity_bits(channel, config.output_polarity, false)?;
        if channel.supports_complementary_output() {
            ccen |= (config.enable_complementary_output as u32) << (shift + 2);
            ccen |= output_polarity_bits(channel, config.complementary_output_polarity, true)?;
        }
        regs.ccen().write(|w| unsafe { w.bits(ccen) });

        Ok(())
    }

    pub fn configure_input_channel(
        &self,
        channel: TmrChannel,
        config: TmrInputCaptureConfig,
    ) -> Result<(), TmrError> {
        self.enable_clock();

        let regs = self.regs();
        match channel {
            TmrChannel::Ch1 => {
                let mut bits = regs.ccm1_capture().read().bits();
                bits &= !0x00ff;
                bits |= (config.selection as u32) << 0;
                bits |= (config.prescaler as u32) << 2;
                bits |= u32::from(config.filter & 0x0f) << 4;
                regs.ccm1_capture().write(|w| unsafe { w.bits(bits) });
            }
            TmrChannel::Ch2 => {
                let mut bits = regs.ccm1_capture().read().bits();
                bits &= !0xff00;
                bits |= (config.selection as u32) << 8;
                bits |= (config.prescaler as u32) << 10;
                bits |= u32::from(config.filter & 0x0f) << 12;
                regs.ccm1_capture().write(|w| unsafe { w.bits(bits) });
            }
            TmrChannel::Ch3 => {
                let mut bits = regs.ccm2_capture().read().bits();
                bits &= !0x00ff;
                bits |= (config.selection as u32) << 0;
                bits |= (config.prescaler as u32) << 2;
                bits |= u32::from(config.filter & 0x0f) << 4;
                regs.ccm2_capture().write(|w| unsafe { w.bits(bits) });
            }
            TmrChannel::Ch4 => {
                let mut bits = regs.ccm2_capture().read().bits();
                bits &= !0xff00;
                bits |= (config.selection as u32) << 8;
                bits |= (config.prescaler as u32) << 10;
                bits |= u32::from(config.filter & 0x0f) << 12;
                regs.ccm2_capture().write(|w| unsafe { w.bits(bits) });
            }
        }

        let shift = channel.ccen_shift();
        let mut ccen = regs.ccen().read().bits();
        ccen &= if channel.supports_complementary_output() {
            !(0x0f << shift)
        } else {
            !(0x03 << shift)
        };
        ccen |= capture_polarity_bits(channel, config.polarity)?;
        ccen |= (config.enable as u32) << shift;
        regs.ccen().write(|w| unsafe { w.bits(ccen) });

        Ok(())
    }

    pub fn configure_encoder(&self, config: TmrEncoderConfig) -> Result<(), TmrError> {
        self.enable_clock();

        let regs = self.regs();

        let mut ccen = regs.ccen().read().bits();
        ccen &= !0x00ff;
        ccen |= capture_polarity_bits(TmrChannel::Ch1, config.ch1_polarity)?;
        ccen |= capture_polarity_bits(TmrChannel::Ch2, config.ch2_polarity)?;
        ccen |= 1 << 0;
        ccen |= 1 << 4;
        regs.ccen().write(|w| unsafe { w.bits(ccen) });

        let mut ccm1 = regs.ccm1_capture().read().bits();
        ccm1 &= !0xffff;
        ccm1 |= (config.ch1_selection as u32) << 0;
        ccm1 |= (config.ch1_prescaler as u32) << 2;
        ccm1 |= u32::from(config.ch1_filter & 0x0f) << 4;
        ccm1 |= (config.ch2_selection as u32) << 8;
        ccm1 |= (config.ch2_prescaler as u32) << 10;
        ccm1 |= u32::from(config.ch2_filter & 0x0f) << 12;
        regs.ccm1_capture().write(|w| unsafe { w.bits(ccm1) });

        let mut smctrl = regs.smctrl().read().bits();
        smctrl &= !0x7;
        smctrl |= match config.mode {
            TmrSlaveMode::EncoderTi1 | TmrSlaveMode::EncoderTi2 | TmrSlaveMode::EncoderTi12 => {
                config.mode as u32
            }
            _ => TmrSlaveMode::EncoderTi12 as u32,
        };
        regs.smctrl().write(|w| unsafe { w.bits(smctrl) });

        Ok(())
    }

    pub fn configure_hall_sensor(&self, config: TmrHallSensorConfig) -> Result<(), TmrError> {
        if matches!(config.polarity, TmrCapturePolarity::BothEdges) {
            return Err(TmrError::HallSensorBothEdgeUnsupported);
        }

        self.enable_clock();
        let regs = self.regs();

        let mut ctrl2 = regs.ctrl2().read().bits();
        ctrl2 &= !((0x7 << 4) | (1 << 7));
        ctrl2 |= (TmrMasterMode::Oc2Ref as u32) << 4;
        ctrl2 |= 1 << 7;
        regs.ctrl2().write(|w| unsafe { w.bits(ctrl2) });

        let mut smctrl = regs.smctrl().read().bits();
        smctrl &= !((0x7 << 0) | (0x7 << 4));
        smctrl |= (TmrSlaveMode::Reset as u32) << 0;
        smctrl |= (TmrTriggerSource::Ti1EdgeDetector as u32) << 4;
        regs.smctrl().write(|w| unsafe { w.bits(smctrl) });

        let mut ccm1 = regs.ccm1_capture().read().bits();
        ccm1 &= !0xffff;
        ccm1 |= (TmrInputSelection::Trc as u32) << 0;
        ccm1 |= (config.prescaler as u32) << 2;
        ccm1 |= u32::from(config.filter & 0x0f) << 4;
        ccm1 |= (TmrOutputCompareMode::Pwm2 as u32) << 12;
        regs.ccm1_capture().write(|w| unsafe { w.bits(ccm1) });

        let mut ccen = regs.ccen().read().bits();
        ccen &= !0x00ff;
        ccen |= capture_polarity_bits(TmrChannel::Ch1, config.polarity)?;
        ccen |= 1 << 0;
        ccen |= 1 << 4;
        regs.ccen().write(|w| unsafe { w.bits(ccen) });

        self.set_compare(TmrChannel::Ch2, config.commutation_delay);

        Ok(())
    }

    pub fn configure_break_dead_time(&self, config: TmrBreakDeadTimeConfig) {
        self.enable_clock();

        let mut bits = 0u32;
        bits |= u32::from(config.dead_time);
        bits |= (config.lock_level as u32) << 8;
        bits |= (config.off_state_idle as u32) << 10;
        bits |= (config.off_state_run as u32) << 11;
        bits |= (config.break_enable as u32) << 12;
        bits |= config.break_polarity.bit() << 13;
        bits |= (config.automatic_output as u32) << 14;
        bits |= (config.main_output as u32) << 15;
        self.regs().bdt().write(|w| unsafe { w.bits(bits) });
    }

    pub fn set_compare(&self, channel: TmrChannel, value: u16) {
        let regs = self.regs();
        let bits = u32::from(value);
        match channel {
            TmrChannel::Ch1 => regs.cc1().write(|w| unsafe { w.bits(bits) }),
            TmrChannel::Ch2 => regs.cc2().write(|w| unsafe { w.bits(bits) }),
            TmrChannel::Ch3 => regs.cc3().write(|w| unsafe { w.bits(bits) }),
            TmrChannel::Ch4 => regs.cc4().write(|w| unsafe { w.bits(bits) }),
        };
    }

    pub fn compare(&self, channel: TmrChannel) -> u16 {
        let regs = self.regs();
        match channel {
            TmrChannel::Ch1 => regs.cc1().read().bits() as u16,
            TmrChannel::Ch2 => regs.cc2().read().bits() as u16,
            TmrChannel::Ch3 => regs.cc3().read().bits() as u16,
            TmrChannel::Ch4 => regs.cc4().read().bits() as u16,
        }
    }

    pub fn set_output_enabled(&self, channel: TmrChannel, enabled: bool) {
        let shift = channel.ccen_shift();
        let regs = self.regs();
        let mut bits = regs.ccen().read().bits();
        bits &= !(1 << shift);
        bits |= (enabled as u32) << shift;
        regs.ccen().write(|w| unsafe { w.bits(bits) });
    }

    pub fn set_complementary_output_enabled(
        &self,
        channel: TmrChannel,
        enabled: bool,
    ) -> Result<(), TmrError> {
        if !channel.supports_complementary_output() {
            return Err(TmrError::ComplementaryOutputUnavailable);
        }

        let shift = channel.ccen_shift() + 2;
        let regs = self.regs();
        let mut bits = regs.ccen().read().bits();
        bits &= !(1 << shift);
        bits |= (enabled as u32) << shift;
        regs.ccen().write(|w| unsafe { w.bits(bits) });
        Ok(())
    }

    pub fn enable_counter(&self) {
        let regs = self.regs();
        let mut bits = regs.ctrl1().read().bits();
        bits |= 1;
        regs.ctrl1().write(|w| unsafe { w.bits(bits) });
    }

    pub fn disable_counter(&self) {
        let regs = self.regs();
        let mut bits = regs.ctrl1().read().bits();
        bits &= !1;
        regs.ctrl1().write(|w| unsafe { w.bits(bits) });
    }

    pub fn counter(&self) -> u16 {
        self.regs().cnt().read().bits() as u16
    }

    pub fn set_counter(&self, value: u16) {
        self.regs()
            .cnt()
            .write(|w| unsafe { w.bits(u32::from(value)) });
    }

    pub fn auto_reload(&self) -> u16 {
        self.regs().autorld().read().bits() as u16
    }

    pub fn set_auto_reload(&self, value: u16) {
        self.regs()
            .autorld()
            .write(|w| unsafe { w.bits(u32::from(value)) });
    }

    pub fn prescaler(&self) -> u16 {
        self.regs().psc().read().bits() as u16
    }

    pub fn set_prescaler(&self, value: u16) {
        self.regs()
            .psc()
            .write(|w| unsafe { w.bits(u32::from(value)) });
    }

    pub fn repetition_counter(&self) -> u8 {
        self.regs().repcnt().read().bits() as u8
    }

    pub fn set_repetition_counter(&self, value: u8) {
        self.regs()
            .repcnt()
            .write(|w| unsafe { w.bits(u32::from(value)) });
    }

    pub fn enable_main_output(&self) {
        let regs = self.regs();
        let mut bits = regs.bdt().read().bits();
        bits |= 1 << 15;
        regs.bdt().write(|w| unsafe { w.bits(bits) });
    }

    pub fn disable_main_output(&self) {
        let regs = self.regs();
        let mut bits = regs.bdt().read().bits();
        bits &= !(1 << 15);
        regs.bdt().write(|w| unsafe { w.bits(bits) });
    }

    pub fn main_output_enabled(&self) -> bool {
        (self.regs().bdt().read().bits() & (1 << 15)) != 0
    }

    pub fn enable_interrupt(&self, interrupt: TmrInterrupt) {
        let regs = self.regs();
        let mut bits = regs.dien().read().bits();
        bits |= interrupt.mask();
        regs.dien().write(|w| unsafe { w.bits(bits) });
    }

    pub fn disable_interrupt(&self, interrupt: TmrInterrupt) {
        let regs = self.regs();
        let mut bits = regs.dien().read().bits();
        bits &= !interrupt.mask();
        regs.dien().write(|w| unsafe { w.bits(bits) });
    }

    pub fn interrupt_enabled(&self, interrupt: TmrInterrupt) -> bool {
        (self.regs().dien().read().bits() & interrupt.mask()) != 0
    }

    pub fn enable_dma_request(&self, request: TmrDmaRequest) {
        let regs = self.regs();
        let mut bits = regs.dien().read().bits();
        bits |= request.mask();
        regs.dien().write(|w| unsafe { w.bits(bits) });
    }

    pub fn disable_dma_request(&self, request: TmrDmaRequest) {
        let regs = self.regs();
        let mut bits = regs.dien().read().bits();
        bits &= !request.mask();
        regs.dien().write(|w| unsafe { w.bits(bits) });
    }

    pub fn dma_request_enabled(&self, request: TmrDmaRequest) -> bool {
        (self.regs().dien().read().bits() & request.mask()) != 0
    }

    pub fn is_flag_set(&self, flag: TmrFlag) -> bool {
        (self.regs().sts().read().bits() & flag.mask()) != 0
    }

    pub fn clear_flag(&self, flag: TmrFlag) {
        self.regs()
            .sts()
            .write(|w| unsafe { w.bits(!flag.mask()) });
    }

    pub fn generate_event(&self, event: TmrEvent) {
        self.regs()
            .ceg()
            .write(|w| unsafe { w.bits(event.mask()) });
    }

    pub fn configure_dma_burst(&self, base: TmrDmaBurstBase, burst_length: u8) {
        let bits = (base as u32) | (u32::from(burst_length & 0x1f) << 8);
        self.regs().dctrl().write(|w| unsafe { w.bits(bits) });
    }

    pub fn dma_burst_base(&self) -> u8 {
        (self.regs().dctrl().read().bits() & 0x1f) as u8
    }

    pub fn dma_burst_length(&self) -> u8 {
        ((self.regs().dctrl().read().bits() >> 8) & 0x1f) as u8
    }

    pub fn set_dma_burst_data(&self, value: u32) {
        self.regs().dmaddr().write(|w| unsafe { w.bits(value) });
    }

    pub fn dma_burst_data(&self) -> u32 {
        self.regs().dmaddr().read().bits()
    }
}
