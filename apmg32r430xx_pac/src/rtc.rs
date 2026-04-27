#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ctrl: Ctrl,
    csts: Csts,
    pscrldh: Pscrldh,
    pscrldl: Pscrldl,
    divh: Divh,
    divl: Divl,
    cnth: Cnth,
    cntl: Cntl,
    alrh: Alrh,
    alrl: Alrl,
}
impl RegisterBlock {
    #[doc = "0x00 - Control register"]
    #[inline(always)]
    pub const fn ctrl(&self) -> &Ctrl {
        &self.ctrl
    }
    #[doc = "0x04 - Control and State register"]
    #[inline(always)]
    pub const fn csts(&self) -> &Csts {
        &self.csts
    }
    #[doc = "0x08 - RTC predivision loading register High Bit"]
    #[inline(always)]
    pub const fn pscrldh(&self) -> &Pscrldh {
        &self.pscrldh
    }
    #[doc = "0x0c - RTC predivision loading register Low Bit"]
    #[inline(always)]
    pub const fn pscrldl(&self) -> &Pscrldl {
        &self.pscrldl
    }
    #[doc = "0x10 - RTC predivider remainder register High Bit"]
    #[inline(always)]
    pub const fn divh(&self) -> &Divh {
        &self.divh
    }
    #[doc = "0x14 - RTC predivider remainder register Low Bit"]
    #[inline(always)]
    pub const fn divl(&self) -> &Divl {
        &self.divl
    }
    #[doc = "0x18 - RTC count register High Bit"]
    #[inline(always)]
    pub const fn cnth(&self) -> &Cnth {
        &self.cnth
    }
    #[doc = "0x1c - RTC count register Low Bit"]
    #[inline(always)]
    pub const fn cntl(&self) -> &Cntl {
        &self.cntl
    }
    #[doc = "0x20 - RTC alarm clock register High Bit"]
    #[inline(always)]
    pub const fn alrh(&self) -> &Alrh {
        &self.alrh
    }
    #[doc = "0x24 - RTC alarm clock register Low Bit"]
    #[inline(always)]
    pub const fn alrl(&self) -> &Alrl {
        &self.alrl
    }
}
#[doc = "CTRL (rw) register accessor: Control register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`] module"]
#[doc(alias = "CTRL")]
pub type Ctrl = crate::Reg<ctrl::CtrlSpec>;
#[doc = "Control register"]
pub mod ctrl;
#[doc = "CSTS (rw) register accessor: Control and State register\n\nYou can [`read`](crate::Reg::read) this register and get [`csts::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csts::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csts`] module"]
#[doc(alias = "CSTS")]
pub type Csts = crate::Reg<csts::CstsSpec>;
#[doc = "Control and State register"]
pub mod csts;
#[doc = "PSCRLDH (w) register accessor: RTC predivision loading register High Bit\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pscrldh::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pscrldh`] module"]
#[doc(alias = "PSCRLDH")]
pub type Pscrldh = crate::Reg<pscrldh::PscrldhSpec>;
#[doc = "RTC predivision loading register High Bit"]
pub mod pscrldh;
#[doc = "PSCRLDL (w) register accessor: RTC predivision loading register Low Bit\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pscrldl::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pscrldl`] module"]
#[doc(alias = "PSCRLDL")]
pub type Pscrldl = crate::Reg<pscrldl::PscrldlSpec>;
#[doc = "RTC predivision loading register Low Bit"]
pub mod pscrldl;
#[doc = "DIVH (r) register accessor: RTC predivider remainder register High Bit\n\nYou can [`read`](crate::Reg::read) this register and get [`divh::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@divh`] module"]
#[doc(alias = "DIVH")]
pub type Divh = crate::Reg<divh::DivhSpec>;
#[doc = "RTC predivider remainder register High Bit"]
pub mod divh;
#[doc = "DIVL (r) register accessor: RTC predivider remainder register Low Bit\n\nYou can [`read`](crate::Reg::read) this register and get [`divl::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@divl`] module"]
#[doc(alias = "DIVL")]
pub type Divl = crate::Reg<divl::DivlSpec>;
#[doc = "RTC predivider remainder register Low Bit"]
pub mod divl;
#[doc = "CNTH (rw) register accessor: RTC count register High Bit\n\nYou can [`read`](crate::Reg::read) this register and get [`cnth::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cnth::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cnth`] module"]
#[doc(alias = "CNTH")]
pub type Cnth = crate::Reg<cnth::CnthSpec>;
#[doc = "RTC count register High Bit"]
pub mod cnth;
#[doc = "CNTL (rw) register accessor: RTC count register Low Bit\n\nYou can [`read`](crate::Reg::read) this register and get [`cntl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cntl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cntl`] module"]
#[doc(alias = "CNTL")]
pub type Cntl = crate::Reg<cntl::CntlSpec>;
#[doc = "RTC count register Low Bit"]
pub mod cntl;
#[doc = "ALRH (w) register accessor: RTC alarm clock register High Bit\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`alrh::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@alrh`] module"]
#[doc(alias = "ALRH")]
pub type Alrh = crate::Reg<alrh::AlrhSpec>;
#[doc = "RTC alarm clock register High Bit"]
pub mod alrh;
#[doc = "ALRL (w) register accessor: RTC alarm clock register Low Bit\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`alrl::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@alrl`] module"]
#[doc(alias = "ALRL")]
pub type Alrl = crate::Reg<alrl::AlrlSpec>;
#[doc = "RTC alarm clock register Low Bit"]
pub mod alrl;
