#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cr: Cr,
    swtrgr: Swtrgr,
    dhr10r: Dhr10r,
    dhr10l: Dhr10l,
    dor: Dor,
    str: Str,
    stmodr: Stmodr,
    sr: Sr,
}
impl RegisterBlock {
    #[doc = "0x00 - Control register (DAC_CTRL)"]
    #[inline(always)]
    pub const fn cr(&self) -> &Cr {
        &self.cr
    }
    #[doc = "0x04 - DAC software trigger register(DAC_SWTRG)"]
    #[inline(always)]
    pub const fn swtrgr(&self) -> &Swtrgr {
        &self.swtrgr
    }
    #[doc = "0x08 - DAC channel1 10-bit right-aligned data holding register(DAC_DH12R1)"]
    #[inline(always)]
    pub const fn dhr10r(&self) -> &Dhr10r {
        &self.dhr10r
    }
    #[doc = "0x0c - DAC channel1 10-bit left aligned data holding register (DAC_DH12L1)"]
    #[inline(always)]
    pub const fn dhr10l(&self) -> &Dhr10l {
        &self.dhr10l
    }
    #[doc = "0x10 - DAC channel data output register"]
    #[inline(always)]
    pub const fn dor(&self) -> &Dor {
        &self.dor
    }
    #[doc = "0x14 - DAC channel sawtooth wave register."]
    #[inline(always)]
    pub const fn str(&self) -> &Str {
        &self.str
    }
    #[doc = "0x18 - DAC channel sawtooth wave trigger register"]
    #[inline(always)]
    pub const fn stmodr(&self) -> &Stmodr {
        &self.stmodr
    }
    #[doc = "0x1c - DAC status register"]
    #[inline(always)]
    pub const fn sr(&self) -> &Sr {
        &self.sr
    }
}
#[doc = "CR (rw) register accessor: Control register (DAC_CTRL)\n\nYou can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`] module"]
#[doc(alias = "CR")]
pub type Cr = crate::Reg<cr::CrSpec>;
#[doc = "Control register (DAC_CTRL)"]
pub mod cr;
#[doc = "SWTRGR (w) register accessor: DAC software trigger register(DAC_SWTRG)\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swtrgr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swtrgr`] module"]
#[doc(alias = "SWTRGR")]
pub type Swtrgr = crate::Reg<swtrgr::SwtrgrSpec>;
#[doc = "DAC software trigger register(DAC_SWTRG)"]
pub mod swtrgr;
#[doc = "DHR10R (rw) register accessor: DAC channel1 10-bit right-aligned data holding register(DAC_DH12R1)\n\nYou can [`read`](crate::Reg::read) this register and get [`dhr10r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhr10r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dhr10r`] module"]
#[doc(alias = "DHR10R")]
pub type Dhr10r = crate::Reg<dhr10r::Dhr10rSpec>;
#[doc = "DAC channel1 10-bit right-aligned data holding register(DAC_DH12R1)"]
pub mod dhr10r;
#[doc = "DHR10L (rw) register accessor: DAC channel1 10-bit left aligned data holding register (DAC_DH12L1)\n\nYou can [`read`](crate::Reg::read) this register and get [`dhr10l::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhr10l::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dhr10l`] module"]
#[doc(alias = "DHR10L")]
pub type Dhr10l = crate::Reg<dhr10l::Dhr10lSpec>;
#[doc = "DAC channel1 10-bit left aligned data holding register (DAC_DH12L1)"]
pub mod dhr10l;
#[doc = "DOR (r) register accessor: DAC channel data output register\n\nYou can [`read`](crate::Reg::read) this register and get [`dor::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dor`] module"]
#[doc(alias = "DOR")]
pub type Dor = crate::Reg<dor::DorSpec>;
#[doc = "DAC channel data output register"]
pub mod dor;
#[doc = "STR (rw) register accessor: DAC channel sawtooth wave register.\n\nYou can [`read`](crate::Reg::read) this register and get [`str::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`str::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@str`] module"]
#[doc(alias = "STR")]
pub type Str = crate::Reg<str::StrSpec>;
#[doc = "DAC channel sawtooth wave register."]
pub mod str;
#[doc = "STMODR (rw) register accessor: DAC channel sawtooth wave trigger register\n\nYou can [`read`](crate::Reg::read) this register and get [`stmodr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stmodr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stmodr`] module"]
#[doc(alias = "STMODR")]
pub type Stmodr = crate::Reg<stmodr::StmodrSpec>;
#[doc = "DAC channel sawtooth wave trigger register"]
pub mod stmodr;
#[doc = "SR (rw) register accessor: DAC status register\n\nYou can [`read`](crate::Reg::read) this register and get [`sr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr`] module"]
#[doc(alias = "SR")]
pub type Sr = crate::Reg<sr::SrSpec>;
#[doc = "DAC status register"]
pub mod sr;
