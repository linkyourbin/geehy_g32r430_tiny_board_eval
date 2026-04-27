#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cr: Cr,
    wkpcr: Wkpcr,
    sr: Sr,
    _reserved3: [u8; 0x04],
    pucra: Pucra,
    pdcra: Pdcra,
    _reserved5: [u8; 0x04],
    evscr: Evscr,
    pvdcr: Pvdcr,
}
impl RegisterBlock {
    #[doc = "0x00 - Control Register"]
    #[inline(always)]
    pub const fn cr(&self) -> &Cr {
        &self.cr
    }
    #[doc = "0x04 - Low Power Consumption Wake-Up Control Register"]
    #[inline(always)]
    pub const fn wkpcr(&self) -> &Wkpcr {
        &self.wkpcr
    }
    #[doc = "0x08 - Status Register"]
    #[inline(always)]
    pub const fn sr(&self) -> &Sr {
        &self.sr
    }
    #[doc = "0x10 - Wake up IO pull-up control register"]
    #[inline(always)]
    pub const fn pucra(&self) -> &Pucra {
        &self.pucra
    }
    #[doc = "0x14 - Wake-up IO Pull-down Control Register"]
    #[inline(always)]
    pub const fn pdcra(&self) -> &Pdcra {
        &self.pdcra
    }
    #[doc = "0x1c - Power-off Wake-up Control Register"]
    #[inline(always)]
    pub const fn evscr(&self) -> &Evscr {
        &self.evscr
    }
    #[doc = "0x20 - PVD Control Register"]
    #[inline(always)]
    pub const fn pvdcr(&self) -> &Pvdcr {
        &self.pvdcr
    }
}
#[doc = "CR (rw) register accessor: Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`] module"]
#[doc(alias = "CR")]
pub type Cr = crate::Reg<cr::CrSpec>;
#[doc = "Control Register"]
pub mod cr;
#[doc = "WKPCR (rw) register accessor: Low Power Consumption Wake-Up Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`wkpcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wkpcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wkpcr`] module"]
#[doc(alias = "WKPCR")]
pub type Wkpcr = crate::Reg<wkpcr::WkpcrSpec>;
#[doc = "Low Power Consumption Wake-Up Control Register"]
pub mod wkpcr;
#[doc = "SR (rw) register accessor: Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr`] module"]
#[doc(alias = "SR")]
pub type Sr = crate::Reg<sr::SrSpec>;
#[doc = "Status Register"]
pub mod sr;
#[doc = "PUCRA (rw) register accessor: Wake up IO pull-up control register\n\nYou can [`read`](crate::Reg::read) this register and get [`pucra::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pucra::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pucra`] module"]
#[doc(alias = "PUCRA")]
pub type Pucra = crate::Reg<pucra::PucraSpec>;
#[doc = "Wake up IO pull-up control register"]
pub mod pucra;
#[doc = "PDCRA (rw) register accessor: Wake-up IO Pull-down Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pdcra::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdcra::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pdcra`] module"]
#[doc(alias = "PDCRA")]
pub type Pdcra = crate::Reg<pdcra::PdcraSpec>;
#[doc = "Wake-up IO Pull-down Control Register"]
pub mod pdcra;
#[doc = "EVSCR (rw) register accessor: Power-off Wake-up Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`evscr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`evscr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@evscr`] module"]
#[doc(alias = "EVSCR")]
pub type Evscr = crate::Reg<evscr::EvscrSpec>;
#[doc = "Power-off Wake-up Control Register"]
pub mod evscr;
#[doc = "PVDCR (rw) register accessor: PVD Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pvdcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pvdcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pvdcr`] module"]
#[doc(alias = "PVDCR")]
pub type Pvdcr = crate::Reg<pvdcr::PvdcrSpec>;
#[doc = "PVD Control Register"]
pub mod pvdcr;
