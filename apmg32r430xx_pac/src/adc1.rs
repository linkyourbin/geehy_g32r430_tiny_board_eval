#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    isr: Isr,
    ier: Ier,
    cr: Cr,
    cfgr: Cfgr,
    cfgr2: Cfgr2,
    smpr1: Smpr1,
    _reserved6: [u8; 0x08],
    tr1: Tr1,
    tr2: Tr2,
    tr3: Tr3,
    _reserved9: [u8; 0x04],
    sqr1: Sqr1,
    sqr2: Sqr2,
    dr: Dr,
    jsqr: Jsqr,
    jdr1: Jdr1,
    jdr2: Jdr2,
    jdr3: Jdr3,
    jdr4: Jdr4,
    awd2cr: Awd2cr,
    awd3cr: Awd3cr,
    offset0: Offset0,
    offset1: Offset1,
    offset2: Offset2,
    offset3: Offset3,
    offset4: Offset4,
    offset5: Offset5,
    _reserved25: [u8; 0x10],
    cal: Cal,
}
impl RegisterBlock {
    #[doc = "0x00 - ADC interrupt and status register"]
    #[inline(always)]
    pub const fn isr(&self) -> &Isr {
        &self.isr
    }
    #[doc = "0x04 - ADC interrupt enable register"]
    #[inline(always)]
    pub const fn ier(&self) -> &Ier {
        &self.ier
    }
    #[doc = "0x08 - ADC control register"]
    #[inline(always)]
    pub const fn cr(&self) -> &Cr {
        &self.cr
    }
    #[doc = "0x0c - ADC configuration register 1"]
    #[inline(always)]
    pub const fn cfgr(&self) -> &Cfgr {
        &self.cfgr
    }
    #[doc = "0x10 - ADC configuration register 2"]
    #[inline(always)]
    pub const fn cfgr2(&self) -> &Cfgr2 {
        &self.cfgr2
    }
    #[doc = "0x14 - ADC sampling time register 1"]
    #[inline(always)]
    pub const fn smpr1(&self) -> &Smpr1 {
        &self.smpr1
    }
    #[doc = "0x20 - ADC analog watchdog 1 threshold register"]
    #[inline(always)]
    pub const fn tr1(&self) -> &Tr1 {
        &self.tr1
    }
    #[doc = "0x24 - ADC analog watchdog 2 threshold register"]
    #[inline(always)]
    pub const fn tr2(&self) -> &Tr2 {
        &self.tr2
    }
    #[doc = "0x28 - ADC analog watchdog 3 threshold register"]
    #[inline(always)]
    pub const fn tr3(&self) -> &Tr3 {
        &self.tr3
    }
    #[doc = "0x30 - ADC group regular sequencer register 1"]
    #[inline(always)]
    pub const fn sqr1(&self) -> &Sqr1 {
        &self.sqr1
    }
    #[doc = "0x34 - ADC group regular sequencer register 2"]
    #[inline(always)]
    pub const fn sqr2(&self) -> &Sqr2 {
        &self.sqr2
    }
    #[doc = "0x38 - ADC group regular data register"]
    #[inline(always)]
    pub const fn dr(&self) -> &Dr {
        &self.dr
    }
    #[doc = "0x3c - ADC group injected sequencer register"]
    #[inline(always)]
    pub const fn jsqr(&self) -> &Jsqr {
        &self.jsqr
    }
    #[doc = "0x40 - ADC group injected rank 1 data register"]
    #[inline(always)]
    pub const fn jdr1(&self) -> &Jdr1 {
        &self.jdr1
    }
    #[doc = "0x44 - ADC group injected rank 2 data register"]
    #[inline(always)]
    pub const fn jdr2(&self) -> &Jdr2 {
        &self.jdr2
    }
    #[doc = "0x48 - ADC group injected rank 3 data register"]
    #[inline(always)]
    pub const fn jdr3(&self) -> &Jdr3 {
        &self.jdr3
    }
    #[doc = "0x4c - ADC group injected rank 4 data register"]
    #[inline(always)]
    pub const fn jdr4(&self) -> &Jdr4 {
        &self.jdr4
    }
    #[doc = "0x50 - Simulation Watchdog 2 Control Register"]
    #[inline(always)]
    pub const fn awd2cr(&self) -> &Awd2cr {
        &self.awd2cr
    }
    #[doc = "0x54 - Simulation Watchdog 3 Control Register"]
    #[inline(always)]
    pub const fn awd3cr(&self) -> &Awd3cr {
        &self.awd3cr
    }
    #[doc = "0x58 - ADC offset register 0"]
    #[inline(always)]
    pub const fn offset0(&self) -> &Offset0 {
        &self.offset0
    }
    #[doc = "0x5c - ADC offset register 1"]
    #[inline(always)]
    pub const fn offset1(&self) -> &Offset1 {
        &self.offset1
    }
    #[doc = "0x60 - ADC offset register 2"]
    #[inline(always)]
    pub const fn offset2(&self) -> &Offset2 {
        &self.offset2
    }
    #[doc = "0x64 - ADC offset register 3"]
    #[inline(always)]
    pub const fn offset3(&self) -> &Offset3 {
        &self.offset3
    }
    #[doc = "0x68 - ADC offset register 4"]
    #[inline(always)]
    pub const fn offset4(&self) -> &Offset4 {
        &self.offset4
    }
    #[doc = "0x6c - ADC offset register 5"]
    #[inline(always)]
    pub const fn offset5(&self) -> &Offset5 {
        &self.offset5
    }
    #[doc = "0x80 - ADC Calibration Register"]
    #[inline(always)]
    pub const fn cal(&self) -> &Cal {
        &self.cal
    }
}
#[doc = "ISR (rw) register accessor: ADC interrupt and status register\n\nYou can [`read`](crate::Reg::read) this register and get [`isr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`isr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isr`] module"]
#[doc(alias = "ISR")]
pub type Isr = crate::Reg<isr::IsrSpec>;
#[doc = "ADC interrupt and status register"]
pub mod isr;
#[doc = "IER (rw) register accessor: ADC interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`ier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ier`] module"]
#[doc(alias = "IER")]
pub type Ier = crate::Reg<ier::IerSpec>;
#[doc = "ADC interrupt enable register"]
pub mod ier;
#[doc = "CR (rw) register accessor: ADC control register\n\nYou can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`] module"]
#[doc(alias = "CR")]
pub type Cr = crate::Reg<cr::CrSpec>;
#[doc = "ADC control register"]
pub mod cr;
#[doc = "CFGR (rw) register accessor: ADC configuration register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`cfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfgr`] module"]
#[doc(alias = "CFGR")]
pub type Cfgr = crate::Reg<cfgr::CfgrSpec>;
#[doc = "ADC configuration register 1"]
pub mod cfgr;
#[doc = "CFGR2 (rw) register accessor: ADC configuration register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`cfgr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfgr2`] module"]
#[doc(alias = "CFGR2")]
pub type Cfgr2 = crate::Reg<cfgr2::Cfgr2Spec>;
#[doc = "ADC configuration register 2"]
pub mod cfgr2;
#[doc = "SMPR1 (rw) register accessor: ADC sampling time register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`smpr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smpr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smpr1`] module"]
#[doc(alias = "SMPR1")]
pub type Smpr1 = crate::Reg<smpr1::Smpr1Spec>;
#[doc = "ADC sampling time register 1"]
pub mod smpr1;
#[doc = "TR1 (rw) register accessor: ADC analog watchdog 1 threshold register\n\nYou can [`read`](crate::Reg::read) this register and get [`tr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tr1`] module"]
#[doc(alias = "TR1")]
pub type Tr1 = crate::Reg<tr1::Tr1Spec>;
#[doc = "ADC analog watchdog 1 threshold register"]
pub mod tr1;
#[doc = "TR2 (rw) register accessor: ADC analog watchdog 2 threshold register\n\nYou can [`read`](crate::Reg::read) this register and get [`tr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tr2`] module"]
#[doc(alias = "TR2")]
pub type Tr2 = crate::Reg<tr2::Tr2Spec>;
#[doc = "ADC analog watchdog 2 threshold register"]
pub mod tr2;
#[doc = "TR3 (rw) register accessor: ADC analog watchdog 3 threshold register\n\nYou can [`read`](crate::Reg::read) this register and get [`tr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tr3`] module"]
#[doc(alias = "TR3")]
pub type Tr3 = crate::Reg<tr3::Tr3Spec>;
#[doc = "ADC analog watchdog 3 threshold register"]
pub mod tr3;
#[doc = "SQR1 (rw) register accessor: ADC group regular sequencer register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`sqr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sqr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sqr1`] module"]
#[doc(alias = "SQR1")]
pub type Sqr1 = crate::Reg<sqr1::Sqr1Spec>;
#[doc = "ADC group regular sequencer register 1"]
pub mod sqr1;
#[doc = "SQR2 (rw) register accessor: ADC group regular sequencer register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`sqr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sqr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sqr2`] module"]
#[doc(alias = "SQR2")]
pub type Sqr2 = crate::Reg<sqr2::Sqr2Spec>;
#[doc = "ADC group regular sequencer register 2"]
pub mod sqr2;
#[doc = "DR (r) register accessor: ADC group regular data register\n\nYou can [`read`](crate::Reg::read) this register and get [`dr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr`] module"]
#[doc(alias = "DR")]
pub type Dr = crate::Reg<dr::DrSpec>;
#[doc = "ADC group regular data register"]
pub mod dr;
#[doc = "JSQR (rw) register accessor: ADC group injected sequencer register\n\nYou can [`read`](crate::Reg::read) this register and get [`jsqr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`jsqr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@jsqr`] module"]
#[doc(alias = "JSQR")]
pub type Jsqr = crate::Reg<jsqr::JsqrSpec>;
#[doc = "ADC group injected sequencer register"]
pub mod jsqr;
#[doc = "JDR1 (r) register accessor: ADC group injected rank 1 data register\n\nYou can [`read`](crate::Reg::read) this register and get [`jdr1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@jdr1`] module"]
#[doc(alias = "JDR1")]
pub type Jdr1 = crate::Reg<jdr1::Jdr1Spec>;
#[doc = "ADC group injected rank 1 data register"]
pub mod jdr1;
#[doc = "JDR2 (r) register accessor: ADC group injected rank 2 data register\n\nYou can [`read`](crate::Reg::read) this register and get [`jdr2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@jdr2`] module"]
#[doc(alias = "JDR2")]
pub type Jdr2 = crate::Reg<jdr2::Jdr2Spec>;
#[doc = "ADC group injected rank 2 data register"]
pub mod jdr2;
#[doc = "JDR3 (r) register accessor: ADC group injected rank 3 data register\n\nYou can [`read`](crate::Reg::read) this register and get [`jdr3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@jdr3`] module"]
#[doc(alias = "JDR3")]
pub type Jdr3 = crate::Reg<jdr3::Jdr3Spec>;
#[doc = "ADC group injected rank 3 data register"]
pub mod jdr3;
#[doc = "JDR4 (r) register accessor: ADC group injected rank 4 data register\n\nYou can [`read`](crate::Reg::read) this register and get [`jdr4::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@jdr4`] module"]
#[doc(alias = "JDR4")]
pub type Jdr4 = crate::Reg<jdr4::Jdr4Spec>;
#[doc = "ADC group injected rank 4 data register"]
pub mod jdr4;
#[doc = "AWD2CR (rw) register accessor: Simulation Watchdog 2 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`awd2cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`awd2cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@awd2cr`] module"]
#[doc(alias = "AWD2CR")]
pub type Awd2cr = crate::Reg<awd2cr::Awd2crSpec>;
#[doc = "Simulation Watchdog 2 Control Register"]
pub mod awd2cr;
#[doc = "AWD3CR (rw) register accessor: Simulation Watchdog 3 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`awd3cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`awd3cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@awd3cr`] module"]
#[doc(alias = "AWD3CR")]
pub type Awd3cr = crate::Reg<awd3cr::Awd3crSpec>;
#[doc = "Simulation Watchdog 3 Control Register"]
pub mod awd3cr;
#[doc = "OFFSET0 (rw) register accessor: ADC offset register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`offset0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`offset0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@offset0`] module"]
#[doc(alias = "OFFSET0")]
pub type Offset0 = crate::Reg<offset0::Offset0Spec>;
#[doc = "ADC offset register 0"]
pub mod offset0;
#[doc = "OFFSET1 (rw) register accessor: ADC offset register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`offset1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`offset1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@offset1`] module"]
#[doc(alias = "OFFSET1")]
pub type Offset1 = crate::Reg<offset1::Offset1Spec>;
#[doc = "ADC offset register 1"]
pub mod offset1;
#[doc = "OFFSET2 (rw) register accessor: ADC offset register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`offset2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`offset2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@offset2`] module"]
#[doc(alias = "OFFSET2")]
pub type Offset2 = crate::Reg<offset2::Offset2Spec>;
#[doc = "ADC offset register 2"]
pub mod offset2;
#[doc = "OFFSET3 (rw) register accessor: ADC offset register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`offset3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`offset3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@offset3`] module"]
#[doc(alias = "OFFSET3")]
pub type Offset3 = crate::Reg<offset3::Offset3Spec>;
#[doc = "ADC offset register 3"]
pub mod offset3;
#[doc = "OFFSET4 (rw) register accessor: ADC offset register 4\n\nYou can [`read`](crate::Reg::read) this register and get [`offset4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`offset4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@offset4`] module"]
#[doc(alias = "OFFSET4")]
pub type Offset4 = crate::Reg<offset4::Offset4Spec>;
#[doc = "ADC offset register 4"]
pub mod offset4;
#[doc = "OFFSET5 (rw) register accessor: ADC offset register 5\n\nYou can [`read`](crate::Reg::read) this register and get [`offset5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`offset5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@offset5`] module"]
#[doc(alias = "OFFSET5")]
pub type Offset5 = crate::Reg<offset5::Offset5Spec>;
#[doc = "ADC offset register 5"]
pub mod offset5;
#[doc = "CAL (rw) register accessor: ADC Calibration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cal::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cal::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cal`] module"]
#[doc(alias = "CAL")]
pub type Cal = crate::Reg<cal::CalSpec>;
#[doc = "ADC Calibration Register"]
pub mod cal;
