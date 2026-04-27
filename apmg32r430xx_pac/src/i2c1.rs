#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ctrl1: Ctrl1,
    ctrl2: Ctrl2,
    saddr1: Saddr1,
    saddr2: Saddr2,
    data: Data,
    sts1: Sts1,
    sts2: Sts2,
    clkctrl: Clkctrl,
    risetmax: Risetmax,
    filter: Filter,
}
impl RegisterBlock {
    #[doc = "0x00 - Control register 1"]
    #[inline(always)]
    pub const fn ctrl1(&self) -> &Ctrl1 {
        &self.ctrl1
    }
    #[doc = "0x04 - Control register 2"]
    #[inline(always)]
    pub const fn ctrl2(&self) -> &Ctrl2 {
        &self.ctrl2
    }
    #[doc = "0x08 - Own address register 1"]
    #[inline(always)]
    pub const fn saddr1(&self) -> &Saddr1 {
        &self.saddr1
    }
    #[doc = "0x0c - Own address register 2"]
    #[inline(always)]
    pub const fn saddr2(&self) -> &Saddr2 {
        &self.saddr2
    }
    #[doc = "0x10 - Data register"]
    #[inline(always)]
    pub const fn data(&self) -> &Data {
        &self.data
    }
    #[doc = "0x14 - Status register 1"]
    #[inline(always)]
    pub const fn sts1(&self) -> &Sts1 {
        &self.sts1
    }
    #[doc = "0x18 - Status register 2"]
    #[inline(always)]
    pub const fn sts2(&self) -> &Sts2 {
        &self.sts2
    }
    #[doc = "0x1c - Clock control register"]
    #[inline(always)]
    pub const fn clkctrl(&self) -> &Clkctrl {
        &self.clkctrl
    }
    #[doc = "0x20 - Maximum Rise Time register"]
    #[inline(always)]
    pub const fn risetmax(&self) -> &Risetmax {
        &self.risetmax
    }
    #[doc = "0x24 - Filter control register"]
    #[inline(always)]
    pub const fn filter(&self) -> &Filter {
        &self.filter
    }
}
#[doc = "CTRL1 (rw) register accessor: Control register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl1`] module"]
#[doc(alias = "CTRL1")]
pub type Ctrl1 = crate::Reg<ctrl1::Ctrl1Spec>;
#[doc = "Control register 1"]
pub mod ctrl1;
#[doc = "CTRL2 (rw) register accessor: Control register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl2`] module"]
#[doc(alias = "CTRL2")]
pub type Ctrl2 = crate::Reg<ctrl2::Ctrl2Spec>;
#[doc = "Control register 2"]
pub mod ctrl2;
#[doc = "SADDR1 (rw) register accessor: Own address register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`saddr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`saddr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@saddr1`] module"]
#[doc(alias = "SADDR1")]
pub type Saddr1 = crate::Reg<saddr1::Saddr1Spec>;
#[doc = "Own address register 1"]
pub mod saddr1;
#[doc = "SADDR2 (rw) register accessor: Own address register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`saddr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`saddr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@saddr2`] module"]
#[doc(alias = "SADDR2")]
pub type Saddr2 = crate::Reg<saddr2::Saddr2Spec>;
#[doc = "Own address register 2"]
pub mod saddr2;
#[doc = "DATA (rw) register accessor: Data register\n\nYou can [`read`](crate::Reg::read) this register and get [`data::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data`] module"]
#[doc(alias = "DATA")]
pub type Data = crate::Reg<data::DataSpec>;
#[doc = "Data register"]
pub mod data;
#[doc = "STS1 (rw) register accessor: Status register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`sts1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sts1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sts1`] module"]
#[doc(alias = "STS1")]
pub type Sts1 = crate::Reg<sts1::Sts1Spec>;
#[doc = "Status register 1"]
pub mod sts1;
#[doc = "STS2 (r) register accessor: Status register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`sts2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sts2`] module"]
#[doc(alias = "STS2")]
pub type Sts2 = crate::Reg<sts2::Sts2Spec>;
#[doc = "Status register 2"]
pub mod sts2;
#[doc = "CLKCTRL (rw) register accessor: Clock control register\n\nYou can [`read`](crate::Reg::read) this register and get [`clkctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clkctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clkctrl`] module"]
#[doc(alias = "CLKCTRL")]
pub type Clkctrl = crate::Reg<clkctrl::ClkctrlSpec>;
#[doc = "Clock control register"]
pub mod clkctrl;
#[doc = "RISETMAX (rw) register accessor: Maximum Rise Time register\n\nYou can [`read`](crate::Reg::read) this register and get [`risetmax::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`risetmax::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@risetmax`] module"]
#[doc(alias = "RISETMAX")]
pub type Risetmax = crate::Reg<risetmax::RisetmaxSpec>;
#[doc = "Maximum Rise Time register"]
pub mod risetmax;
#[doc = "FILTER (rw) register accessor: Filter control register\n\nYou can [`read`](crate::Reg::read) this register and get [`filter::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`filter::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@filter`] module"]
#[doc(alias = "FILTER")]
pub type Filter = crate::Reg<filter::FilterSpec>;
#[doc = "Filter control register"]
pub mod filter;
