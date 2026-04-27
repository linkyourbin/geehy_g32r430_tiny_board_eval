#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    key: Key,
    optkey: Optkey,
    sr: Sr,
    cr1: Cr1,
    cr2: Cr2,
    _reserved5: [u8; 0x04],
    tmcr: Tmcr,
    _reserved6: [u8; 0x04],
    obsr1: Obsr1,
    obsr2: Obsr2,
    obcr1: Obcr1,
    obcr2: Obcr2,
}
impl RegisterBlock {
    #[doc = "0x00 - key register"]
    #[inline(always)]
    pub const fn key(&self) -> &Key {
        &self.key
    }
    #[doc = "0x04 - option byte key register"]
    #[inline(always)]
    pub const fn optkey(&self) -> &Optkey {
        &self.optkey
    }
    #[doc = "0x08 - status register"]
    #[inline(always)]
    pub const fn sr(&self) -> &Sr {
        &self.sr
    }
    #[doc = "0x0c - Control register 1"]
    #[inline(always)]
    pub const fn cr1(&self) -> &Cr1 {
        &self.cr1
    }
    #[doc = "0x10 - Control register 2"]
    #[inline(always)]
    pub const fn cr2(&self) -> &Cr2 {
        &self.cr2
    }
    #[doc = "0x18 - TMCR"]
    #[inline(always)]
    pub const fn tmcr(&self) -> &Tmcr {
        &self.tmcr
    }
    #[doc = "0x20 - Option Byte Status Register 1"]
    #[inline(always)]
    pub const fn obsr1(&self) -> &Obsr1 {
        &self.obsr1
    }
    #[doc = "0x24 - Option Byte Status Register 2"]
    #[inline(always)]
    pub const fn obsr2(&self) -> &Obsr2 {
        &self.obsr2
    }
    #[doc = "0x28 - Option Byte Control Register 1"]
    #[inline(always)]
    pub const fn obcr1(&self) -> &Obcr1 {
        &self.obcr1
    }
    #[doc = "0x2c - Option Byte Control Register 2"]
    #[inline(always)]
    pub const fn obcr2(&self) -> &Obcr2 {
        &self.obcr2
    }
}
#[doc = "KEY (w) register accessor: key register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`key::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@key`] module"]
#[doc(alias = "KEY")]
pub type Key = crate::Reg<key::KeySpec>;
#[doc = "key register"]
pub mod key;
#[doc = "OPTKEY (w) register accessor: option byte key register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`optkey::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@optkey`] module"]
#[doc(alias = "OPTKEY")]
pub type Optkey = crate::Reg<optkey::OptkeySpec>;
#[doc = "option byte key register"]
pub mod optkey;
#[doc = "SR (rw) register accessor: status register\n\nYou can [`read`](crate::Reg::read) this register and get [`sr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr`] module"]
#[doc(alias = "SR")]
pub type Sr = crate::Reg<sr::SrSpec>;
#[doc = "status register"]
pub mod sr;
#[doc = "CR1 (rw) register accessor: Control register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`cr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr1`] module"]
#[doc(alias = "CR1")]
pub type Cr1 = crate::Reg<cr1::Cr1Spec>;
#[doc = "Control register 1"]
pub mod cr1;
#[doc = "CR2 (rw) register accessor: Control register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`cr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr2`] module"]
#[doc(alias = "CR2")]
pub type Cr2 = crate::Reg<cr2::Cr2Spec>;
#[doc = "Control register 2"]
pub mod cr2;
#[doc = "TMCR (rw) register accessor: TMCR\n\nYou can [`read`](crate::Reg::read) this register and get [`tmcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tmcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tmcr`] module"]
#[doc(alias = "TMCR")]
pub type Tmcr = crate::Reg<tmcr::TmcrSpec>;
#[doc = "TMCR"]
pub mod tmcr;
#[doc = "OBSR1 (r) register accessor: Option Byte Status Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`obsr1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@obsr1`] module"]
#[doc(alias = "OBSR1")]
pub type Obsr1 = crate::Reg<obsr1::Obsr1Spec>;
#[doc = "Option Byte Status Register 1"]
pub mod obsr1;
#[doc = "OBSR2 (r) register accessor: Option Byte Status Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`obsr2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@obsr2`] module"]
#[doc(alias = "OBSR2")]
pub type Obsr2 = crate::Reg<obsr2::Obsr2Spec>;
#[doc = "Option Byte Status Register 2"]
pub mod obsr2;
#[doc = "OBCR1 (rw) register accessor: Option Byte Control Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`obcr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`obcr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@obcr1`] module"]
#[doc(alias = "OBCR1")]
pub type Obcr1 = crate::Reg<obcr1::Obcr1Spec>;
#[doc = "Option Byte Control Register 1"]
pub mod obcr1;
#[doc = "OBCR2 (rw) register accessor: Option Byte Control Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`obcr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`obcr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@obcr2`] module"]
#[doc(alias = "OBCR2")]
pub type Obcr2 = crate::Reg<obcr2::Obcr2Spec>;
#[doc = "Option Byte Control Register 2"]
pub mod obcr2;
