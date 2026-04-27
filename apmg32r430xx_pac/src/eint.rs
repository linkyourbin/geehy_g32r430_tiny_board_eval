#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    imask: Imask,
    emask: Emask,
    rten: Rten,
    ften: Ften,
    swinte: Swinte,
    ipend: Ipend,
    ioselr1: Ioselr1,
    ioselr2: Ioselr2,
}
impl RegisterBlock {
    #[doc = "0x00 - Interrupt mask register"]
    #[inline(always)]
    pub const fn imask(&self) -> &Imask {
        &self.imask
    }
    #[doc = "0x04 - Event mask register"]
    #[inline(always)]
    pub const fn emask(&self) -> &Emask {
        &self.emask
    }
    #[doc = "0x08 - Rising Trigger selection register"]
    #[inline(always)]
    pub const fn rten(&self) -> &Rten {
        &self.rten
    }
    #[doc = "0x0c - Falling Trigger selection register"]
    #[inline(always)]
    pub const fn ften(&self) -> &Ften {
        &self.ften
    }
    #[doc = "0x10 - Software interrupt event register"]
    #[inline(always)]
    pub const fn swinte(&self) -> &Swinte {
        &self.swinte
    }
    #[doc = "0x14 - Pending register"]
    #[inline(always)]
    pub const fn ipend(&self) -> &Ipend {
        &self.ipend
    }
    #[doc = "0x18 - Configure Register 1."]
    #[inline(always)]
    pub const fn ioselr1(&self) -> &Ioselr1 {
        &self.ioselr1
    }
    #[doc = "0x1c - Configure Register 2."]
    #[inline(always)]
    pub const fn ioselr2(&self) -> &Ioselr2 {
        &self.ioselr2
    }
}
#[doc = "IMASK (rw) register accessor: Interrupt mask register\n\nYou can [`read`](crate::Reg::read) this register and get [`imask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`imask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@imask`] module"]
#[doc(alias = "IMASK")]
pub type Imask = crate::Reg<imask::ImaskSpec>;
#[doc = "Interrupt mask register"]
pub mod imask;
#[doc = "EMASK (rw) register accessor: Event mask register\n\nYou can [`read`](crate::Reg::read) this register and get [`emask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`emask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@emask`] module"]
#[doc(alias = "EMASK")]
pub type Emask = crate::Reg<emask::EmaskSpec>;
#[doc = "Event mask register"]
pub mod emask;
#[doc = "RTEN (rw) register accessor: Rising Trigger selection register\n\nYou can [`read`](crate::Reg::read) this register and get [`rten::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rten::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rten`] module"]
#[doc(alias = "RTEN")]
pub type Rten = crate::Reg<rten::RtenSpec>;
#[doc = "Rising Trigger selection register"]
pub mod rten;
#[doc = "FTEN (rw) register accessor: Falling Trigger selection register\n\nYou can [`read`](crate::Reg::read) this register and get [`ften::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ften::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ften`] module"]
#[doc(alias = "FTEN")]
pub type Ften = crate::Reg<ften::FtenSpec>;
#[doc = "Falling Trigger selection register"]
pub mod ften;
#[doc = "SWINTE (rw) register accessor: Software interrupt event register\n\nYou can [`read`](crate::Reg::read) this register and get [`swinte::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swinte::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swinte`] module"]
#[doc(alias = "SWINTE")]
pub type Swinte = crate::Reg<swinte::SwinteSpec>;
#[doc = "Software interrupt event register"]
pub mod swinte;
#[doc = "IPEND (rw) register accessor: Pending register\n\nYou can [`read`](crate::Reg::read) this register and get [`ipend::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipend::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ipend`] module"]
#[doc(alias = "IPEND")]
pub type Ipend = crate::Reg<ipend::IpendSpec>;
#[doc = "Pending register"]
pub mod ipend;
#[doc = "IOSELR1 (rw) register accessor: Configure Register 1.\n\nYou can [`read`](crate::Reg::read) this register and get [`ioselr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ioselr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ioselr1`] module"]
#[doc(alias = "IOSELR1")]
pub type Ioselr1 = crate::Reg<ioselr1::Ioselr1Spec>;
#[doc = "Configure Register 1."]
pub mod ioselr1;
#[doc = "IOSELR2 (rw) register accessor: Configure Register 2.\n\nYou can [`read`](crate::Reg::read) this register and get [`ioselr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ioselr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ioselr2`] module"]
#[doc(alias = "IOSELR2")]
pub type Ioselr2 = crate::Reg<ioselr2::Ioselr2Spec>;
#[doc = "Configure Register 2."]
pub mod ioselr2;
