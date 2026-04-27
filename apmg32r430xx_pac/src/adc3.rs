#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    isr: Isr,
    ier: Ier,
    cr: Cr,
    cfgr: Cfgr,
    _reserved4: [u8; 0x04],
    smpr1: Smpr1,
    smpr2: Smpr2,
    _reserved6: [u8; 0x04],
    tr1: Tr1,
    _reserved7: [u8; 0x08],
    sqr1: Sqr1,
    sqr2: Sqr2,
    sqr3: Sqr3,
    dr: Dr,
    jsqr: Jsqr,
    jdr1: Jdr1,
    jdr2: Jdr2,
    jdr3: Jdr3,
    jdr4: Jdr4,
    _reserved16: [u8; 0x08],
    joffset1: Joffset1,
    joffset2: Joffset2,
    joffset3: Joffset3,
    joffset4: Joffset4,
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
    #[doc = "0x14 - ADC sampling time register 1"]
    #[inline(always)]
    pub const fn smpr1(&self) -> &Smpr1 {
        &self.smpr1
    }
    #[doc = "0x18 - ADC sampling time register 2"]
    #[inline(always)]
    pub const fn smpr2(&self) -> &Smpr2 {
        &self.smpr2
    }
    #[doc = "0x20 - ADC analog watchdog 1 threshold register"]
    #[inline(always)]
    pub const fn tr1(&self) -> &Tr1 {
        &self.tr1
    }
    #[doc = "0x2c - ADC group regular sequencer register 1"]
    #[inline(always)]
    pub const fn sqr1(&self) -> &Sqr1 {
        &self.sqr1
    }
    #[doc = "0x30 - ADC group regular sequencer register 2"]
    #[inline(always)]
    pub const fn sqr2(&self) -> &Sqr2 {
        &self.sqr2
    }
    #[doc = "0x34 - ADC group regular sequencer register 3"]
    #[inline(always)]
    pub const fn sqr3(&self) -> &Sqr3 {
        &self.sqr3
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
    #[doc = "0x58 - ADC offset register 1"]
    #[inline(always)]
    pub const fn joffset1(&self) -> &Joffset1 {
        &self.joffset1
    }
    #[doc = "0x5c - ADC offset register 2"]
    #[inline(always)]
    pub const fn joffset2(&self) -> &Joffset2 {
        &self.joffset2
    }
    #[doc = "0x60 - ADC offset register 3"]
    #[inline(always)]
    pub const fn joffset3(&self) -> &Joffset3 {
        &self.joffset3
    }
    #[doc = "0x64 - ADC offset register 4"]
    #[inline(always)]
    pub const fn joffset4(&self) -> &Joffset4 {
        &self.joffset4
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
#[doc = "SMPR1 (rw) register accessor: ADC sampling time register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`smpr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smpr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smpr1`] module"]
#[doc(alias = "SMPR1")]
pub type Smpr1 = crate::Reg<smpr1::Smpr1Spec>;
#[doc = "ADC sampling time register 1"]
pub mod smpr1;
#[doc = "SMPR2 (rw) register accessor: ADC sampling time register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`smpr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smpr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smpr2`] module"]
#[doc(alias = "SMPR2")]
pub type Smpr2 = crate::Reg<smpr2::Smpr2Spec>;
#[doc = "ADC sampling time register 2"]
pub mod smpr2;
#[doc = "TR1 (rw) register accessor: ADC analog watchdog 1 threshold register\n\nYou can [`read`](crate::Reg::read) this register and get [`tr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tr1`] module"]
#[doc(alias = "TR1")]
pub type Tr1 = crate::Reg<tr1::Tr1Spec>;
#[doc = "ADC analog watchdog 1 threshold register"]
pub mod tr1;
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
#[doc = "SQR3 (rw) register accessor: ADC group regular sequencer register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`sqr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sqr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sqr3`] module"]
#[doc(alias = "SQR3")]
pub type Sqr3 = crate::Reg<sqr3::Sqr3Spec>;
#[doc = "ADC group regular sequencer register 3"]
pub mod sqr3;
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
#[doc = "JOFFSET1 (rw) register accessor: ADC offset register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`joffset1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`joffset1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@joffset1`] module"]
#[doc(alias = "JOFFSET1")]
pub type Joffset1 = crate::Reg<joffset1::Joffset1Spec>;
#[doc = "ADC offset register 1"]
pub mod joffset1;
#[doc = "JOFFSET2 (rw) register accessor: ADC offset register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`joffset2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`joffset2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@joffset2`] module"]
#[doc(alias = "JOFFSET2")]
pub type Joffset2 = crate::Reg<joffset2::Joffset2Spec>;
#[doc = "ADC offset register 2"]
pub mod joffset2;
#[doc = "JOFFSET3 (rw) register accessor: ADC offset register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`joffset3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`joffset3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@joffset3`] module"]
#[doc(alias = "JOFFSET3")]
pub type Joffset3 = crate::Reg<joffset3::Joffset3Spec>;
#[doc = "ADC offset register 3"]
pub mod joffset3;
#[doc = "JOFFSET4 (rw) register accessor: ADC offset register 4\n\nYou can [`read`](crate::Reg::read) this register and get [`joffset4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`joffset4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@joffset4`] module"]
#[doc(alias = "JOFFSET4")]
pub type Joffset4 = crate::Reg<joffset4::Joffset4Spec>;
#[doc = "ADC offset register 4"]
pub mod joffset4;
