#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ctrl1: Ctrl1,
    ctrl2: Ctrl2,
    smctrl: Smctrl,
    dien: Dien,
    sts: Sts,
    ceg: Ceg,
    _reserved_6_ccm: [u8; 0x04],
    _reserved_7_ccm: [u8; 0x04],
    ccen: Ccen,
    cnt: Cnt,
    psc: Psc,
    autorld: Autorld,
    repcnt: Repcnt,
    cc1: Cc1,
    cc2: Cc2,
    cc3: Cc3,
    cc4: Cc4,
    bdt: Bdt,
    dctrl: Dctrl,
    dmaddr: Dmaddr,
}
impl RegisterBlock {
    #[doc = "0x00 - control register 1"]
    #[inline(always)]
    pub const fn ctrl1(&self) -> &Ctrl1 {
        &self.ctrl1
    }
    #[doc = "0x04 - control register 2"]
    #[inline(always)]
    pub const fn ctrl2(&self) -> &Ctrl2 {
        &self.ctrl2
    }
    #[doc = "0x08 - slave mode control register"]
    #[inline(always)]
    pub const fn smctrl(&self) -> &Smctrl {
        &self.smctrl
    }
    #[doc = "0x0c - DMA/Interrupt enable register"]
    #[inline(always)]
    pub const fn dien(&self) -> &Dien {
        &self.dien
    }
    #[doc = "0x10 - status register"]
    #[inline(always)]
    pub const fn sts(&self) -> &Sts {
        &self.sts
    }
    #[doc = "0x14 - event generation register"]
    #[inline(always)]
    pub const fn ceg(&self) -> &Ceg {
        &self.ceg
    }
    #[doc = "0x18 - capture/compare mode register 1 (input mode)"]
    #[inline(always)]
    pub const fn ccm1_capture(&self) -> &Ccm1Capture {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(24).cast() }
    }
    #[doc = "0x18 - capture/compare mode register 1 (output mode)"]
    #[inline(always)]
    pub const fn ccm1(&self) -> &Ccm1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(24).cast() }
    }
    #[doc = "0x1c - capture/compare mode register 2 (input mode)"]
    #[inline(always)]
    pub const fn ccm2_capture(&self) -> &Ccm2Capture {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(28).cast() }
    }
    #[doc = "0x1c - capture/compare mode register 2 (output mode)"]
    #[inline(always)]
    pub const fn ccm2(&self) -> &Ccm2 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(28).cast() }
    }
    #[doc = "0x20 - capture/compare enable register"]
    #[inline(always)]
    pub const fn ccen(&self) -> &Ccen {
        &self.ccen
    }
    #[doc = "0x24 - counter"]
    #[inline(always)]
    pub const fn cnt(&self) -> &Cnt {
        &self.cnt
    }
    #[doc = "0x28 - prescaler"]
    #[inline(always)]
    pub const fn psc(&self) -> &Psc {
        &self.psc
    }
    #[doc = "0x2c - auto-reload register"]
    #[inline(always)]
    pub const fn autorld(&self) -> &Autorld {
        &self.autorld
    }
    #[doc = "0x30 - repetition counter register"]
    #[inline(always)]
    pub const fn repcnt(&self) -> &Repcnt {
        &self.repcnt
    }
    #[doc = "0x34 - capture/compare register 1"]
    #[inline(always)]
    pub const fn cc1(&self) -> &Cc1 {
        &self.cc1
    }
    #[doc = "0x38 - capture/compare register 2"]
    #[inline(always)]
    pub const fn cc2(&self) -> &Cc2 {
        &self.cc2
    }
    #[doc = "0x3c - capture/compare register 3"]
    #[inline(always)]
    pub const fn cc3(&self) -> &Cc3 {
        &self.cc3
    }
    #[doc = "0x40 - capture/compare register 4"]
    #[inline(always)]
    pub const fn cc4(&self) -> &Cc4 {
        &self.cc4
    }
    #[doc = "0x44 - break and dead-time register"]
    #[inline(always)]
    pub const fn bdt(&self) -> &Bdt {
        &self.bdt
    }
    #[doc = "0x48 - DMA control register"]
    #[inline(always)]
    pub const fn dctrl(&self) -> &Dctrl {
        &self.dctrl
    }
    #[doc = "0x4c - DMA address for full transfer"]
    #[inline(always)]
    pub const fn dmaddr(&self) -> &Dmaddr {
        &self.dmaddr
    }
}
#[doc = "CTRL1 (rw) register accessor: control register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl1`] module"]
#[doc(alias = "CTRL1")]
pub type Ctrl1 = crate::Reg<ctrl1::Ctrl1Spec>;
#[doc = "control register 1"]
pub mod ctrl1;
#[doc = "CTRL2 (rw) register accessor: control register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl2`] module"]
#[doc(alias = "CTRL2")]
pub type Ctrl2 = crate::Reg<ctrl2::Ctrl2Spec>;
#[doc = "control register 2"]
pub mod ctrl2;
#[doc = "SMCTRL (rw) register accessor: slave mode control register\n\nYou can [`read`](crate::Reg::read) this register and get [`smctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smctrl`] module"]
#[doc(alias = "SMCTRL")]
pub type Smctrl = crate::Reg<smctrl::SmctrlSpec>;
#[doc = "slave mode control register"]
pub mod smctrl;
#[doc = "DIEN (rw) register accessor: DMA/Interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`dien::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dien::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dien`] module"]
#[doc(alias = "DIEN")]
pub type Dien = crate::Reg<dien::DienSpec>;
#[doc = "DMA/Interrupt enable register"]
pub mod dien;
#[doc = "STS (rw) register accessor: status register\n\nYou can [`read`](crate::Reg::read) this register and get [`sts::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sts::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sts`] module"]
#[doc(alias = "STS")]
pub type Sts = crate::Reg<sts::StsSpec>;
#[doc = "status register"]
pub mod sts;
#[doc = "CEG (w) register accessor: event generation register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ceg::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ceg`] module"]
#[doc(alias = "CEG")]
pub type Ceg = crate::Reg<ceg::CegSpec>;
#[doc = "event generation register"]
pub mod ceg;
#[doc = "CCM1 (rw) register accessor: capture/compare mode register 1 (output mode)\n\nYou can [`read`](crate::Reg::read) this register and get [`ccm1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccm1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccm1`] module"]
#[doc(alias = "CCM1")]
pub type Ccm1 = crate::Reg<ccm1::Ccm1Spec>;
#[doc = "capture/compare mode register 1 (output mode)"]
pub mod ccm1;
#[doc = "CCM1_CAPTURE (rw) register accessor: capture/compare mode register 1 (input mode)\n\nYou can [`read`](crate::Reg::read) this register and get [`ccm1_capture::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccm1_capture::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccm1_capture`] module"]
#[doc(alias = "CCM1_CAPTURE")]
pub type Ccm1Capture = crate::Reg<ccm1_capture::Ccm1CaptureSpec>;
#[doc = "capture/compare mode register 1 (input mode)"]
pub mod ccm1_capture;
#[doc = "CCM2 (rw) register accessor: capture/compare mode register 2 (output mode)\n\nYou can [`read`](crate::Reg::read) this register and get [`ccm2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccm2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccm2`] module"]
#[doc(alias = "CCM2")]
pub type Ccm2 = crate::Reg<ccm2::Ccm2Spec>;
#[doc = "capture/compare mode register 2 (output mode)"]
pub mod ccm2;
#[doc = "CCM2_CAPTURE (rw) register accessor: capture/compare mode register 2 (input mode)\n\nYou can [`read`](crate::Reg::read) this register and get [`ccm2_capture::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccm2_capture::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccm2_capture`] module"]
#[doc(alias = "CCM2_CAPTURE")]
pub type Ccm2Capture = crate::Reg<ccm2_capture::Ccm2CaptureSpec>;
#[doc = "capture/compare mode register 2 (input mode)"]
pub mod ccm2_capture;
#[doc = "CCEN (rw) register accessor: capture/compare enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`ccen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccen`] module"]
#[doc(alias = "CCEN")]
pub type Ccen = crate::Reg<ccen::CcenSpec>;
#[doc = "capture/compare enable register"]
pub mod ccen;
#[doc = "CNT (rw) register accessor: counter\n\nYou can [`read`](crate::Reg::read) this register and get [`cnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cnt`] module"]
#[doc(alias = "CNT")]
pub type Cnt = crate::Reg<cnt::CntSpec>;
#[doc = "counter"]
pub mod cnt;
#[doc = "PSC (rw) register accessor: prescaler\n\nYou can [`read`](crate::Reg::read) this register and get [`psc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@psc`] module"]
#[doc(alias = "PSC")]
pub type Psc = crate::Reg<psc::PscSpec>;
#[doc = "prescaler"]
pub mod psc;
#[doc = "AUTORLD (rw) register accessor: auto-reload register\n\nYou can [`read`](crate::Reg::read) this register and get [`autorld::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`autorld::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@autorld`] module"]
#[doc(alias = "AUTORLD")]
pub type Autorld = crate::Reg<autorld::AutorldSpec>;
#[doc = "auto-reload register"]
pub mod autorld;
#[doc = "REPCNT (rw) register accessor: repetition counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`repcnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`repcnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@repcnt`] module"]
#[doc(alias = "REPCNT")]
pub type Repcnt = crate::Reg<repcnt::RepcntSpec>;
#[doc = "repetition counter register"]
pub mod repcnt;
#[doc = "CC1 (rw) register accessor: capture/compare register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`cc1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cc1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cc1`] module"]
#[doc(alias = "CC1")]
pub type Cc1 = crate::Reg<cc1::Cc1Spec>;
#[doc = "capture/compare register 1"]
pub mod cc1;
#[doc = "CC2 (rw) register accessor: capture/compare register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`cc2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cc2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cc2`] module"]
#[doc(alias = "CC2")]
pub type Cc2 = crate::Reg<cc2::Cc2Spec>;
#[doc = "capture/compare register 2"]
pub mod cc2;
#[doc = "CC3 (rw) register accessor: capture/compare register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`cc3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cc3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cc3`] module"]
#[doc(alias = "CC3")]
pub type Cc3 = crate::Reg<cc3::Cc3Spec>;
#[doc = "capture/compare register 3"]
pub mod cc3;
#[doc = "CC4 (rw) register accessor: capture/compare register 4\n\nYou can [`read`](crate::Reg::read) this register and get [`cc4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cc4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cc4`] module"]
#[doc(alias = "CC4")]
pub type Cc4 = crate::Reg<cc4::Cc4Spec>;
#[doc = "capture/compare register 4"]
pub mod cc4;
#[doc = "BDT (rw) register accessor: break and dead-time register\n\nYou can [`read`](crate::Reg::read) this register and get [`bdt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bdt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bdt`] module"]
#[doc(alias = "BDT")]
pub type Bdt = crate::Reg<bdt::BdtSpec>;
#[doc = "break and dead-time register"]
pub mod bdt;
#[doc = "DCTRL (rw) register accessor: DMA control register\n\nYou can [`read`](crate::Reg::read) this register and get [`dctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dctrl`] module"]
#[doc(alias = "DCTRL")]
pub type Dctrl = crate::Reg<dctrl::DctrlSpec>;
#[doc = "DMA control register"]
pub mod dctrl;
#[doc = "DMADDR (rw) register accessor: DMA address for full transfer\n\nYou can [`read`](crate::Reg::read) this register and get [`dmaddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmaddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmaddr`] module"]
#[doc(alias = "DMADDR")]
pub type Dmaddr = crate::Reg<dmaddr::DmaddrSpec>;
#[doc = "DMA address for full transfer"]
pub mod dmaddr;
