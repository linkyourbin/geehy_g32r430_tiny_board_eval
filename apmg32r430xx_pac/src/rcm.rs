#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    key: Key,
    rccr: Rccr,
    pllcr: Pllcr,
    sccr: Sccr,
    mccr: Mccr,
    cier: Cier,
    cicr: Cicr,
    _reserved7: [u8; 0x04],
    ahbrst: Ahbrst,
    apbrst: Apbrst,
    ahbcg: Ahbcg,
    _reserved10: [u8; 0x04],
    apbcg: Apbcg,
    _reserved11: [u8; 0x04],
    pwrcr: Pwrcr,
    rstcsr: Rstcsr,
    _reserved13: [u8; 0x04],
    adcccr: Adcccr,
}
impl RegisterBlock {
    #[doc = "0x00 - Key register"]
    #[inline(always)]
    pub const fn key(&self) -> &Key {
        &self.key
    }
    #[doc = "0x04 - Crystal Oscillator Control Register"]
    #[inline(always)]
    pub const fn rccr(&self) -> &Rccr {
        &self.rccr
    }
    #[doc = "0x08 - PLL control register"]
    #[inline(always)]
    pub const fn pllcr(&self) -> &Pllcr {
        &self.pllcr
    }
    #[doc = "0x0c - Clock Divider Control Register"]
    #[inline(always)]
    pub const fn sccr(&self) -> &Sccr {
        &self.sccr
    }
    #[doc = "0x10 - Main clock control register."]
    #[inline(always)]
    pub const fn mccr(&self) -> &Mccr {
        &self.mccr
    }
    #[doc = "0x14 - Timer interrupt enable register."]
    #[inline(always)]
    pub const fn cier(&self) -> &Cier {
        &self.cier
    }
    #[doc = "0x18 - Interrupt flag control register"]
    #[inline(always)]
    pub const fn cicr(&self) -> &Cicr {
        &self.cicr
    }
    #[doc = "0x20 - AHB Peripheral Reset Register"]
    #[inline(always)]
    pub const fn ahbrst(&self) -> &Ahbrst {
        &self.ahbrst
    }
    #[doc = "0x24 - APB Peripheral Reset Register"]
    #[inline(always)]
    pub const fn apbrst(&self) -> &Apbrst {
        &self.apbrst
    }
    #[doc = "0x28 - AHB peripheral control register"]
    #[inline(always)]
    pub const fn ahbcg(&self) -> &Ahbcg {
        &self.ahbcg
    }
    #[doc = "0x30 - APB peripheral control register"]
    #[inline(always)]
    pub const fn apbcg(&self) -> &Apbcg {
        &self.apbcg
    }
    #[doc = "0x38 - PWR peripheral control register"]
    #[inline(always)]
    pub const fn pwrcr(&self) -> &Pwrcr {
        &self.pwrcr
    }
    #[doc = "0x3c - Clock reset status register"]
    #[inline(always)]
    pub const fn rstcsr(&self) -> &Rstcsr {
        &self.rstcsr
    }
    #[doc = "0x44 - Analog module control register"]
    #[inline(always)]
    pub const fn adcccr(&self) -> &Adcccr {
        &self.adcccr
    }
}
#[doc = "KEY (rw) register accessor: Key register\n\nYou can [`read`](crate::Reg::read) this register and get [`key::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`key::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@key`] module"]
#[doc(alias = "KEY")]
pub type Key = crate::Reg<key::KeySpec>;
#[doc = "Key register"]
pub mod key;
#[doc = "RCCR (rw) register accessor: Crystal Oscillator Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rccr`] module"]
#[doc(alias = "RCCR")]
pub type Rccr = crate::Reg<rccr::RccrSpec>;
#[doc = "Crystal Oscillator Control Register"]
pub mod rccr;
#[doc = "PLLCR (rw) register accessor: PLL control register\n\nYou can [`read`](crate::Reg::read) this register and get [`pllcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pllcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pllcr`] module"]
#[doc(alias = "PLLCR")]
pub type Pllcr = crate::Reg<pllcr::PllcrSpec>;
#[doc = "PLL control register"]
pub mod pllcr;
#[doc = "SCCR (rw) register accessor: Clock Divider Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sccr`] module"]
#[doc(alias = "SCCR")]
pub type Sccr = crate::Reg<sccr::SccrSpec>;
#[doc = "Clock Divider Control Register"]
pub mod sccr;
#[doc = "MCCR (rw) register accessor: Main clock control register.\n\nYou can [`read`](crate::Reg::read) this register and get [`mccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mccr`] module"]
#[doc(alias = "MCCR")]
pub type Mccr = crate::Reg<mccr::MccrSpec>;
#[doc = "Main clock control register."]
pub mod mccr;
#[doc = "CIER (rw) register accessor: Timer interrupt enable register.\n\nYou can [`read`](crate::Reg::read) this register and get [`cier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cier`] module"]
#[doc(alias = "CIER")]
pub type Cier = crate::Reg<cier::CierSpec>;
#[doc = "Timer interrupt enable register."]
pub mod cier;
#[doc = "CICR (rw) register accessor: Interrupt flag control register\n\nYou can [`read`](crate::Reg::read) this register and get [`cicr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cicr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cicr`] module"]
#[doc(alias = "CICR")]
pub type Cicr = crate::Reg<cicr::CicrSpec>;
#[doc = "Interrupt flag control register"]
pub mod cicr;
#[doc = "AHBRST (rw) register accessor: AHB Peripheral Reset Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ahbrst::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahbrst::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahbrst`] module"]
#[doc(alias = "AHBRST")]
pub type Ahbrst = crate::Reg<ahbrst::AhbrstSpec>;
#[doc = "AHB Peripheral Reset Register"]
pub mod ahbrst;
#[doc = "APBRST (rw) register accessor: APB Peripheral Reset Register\n\nYou can [`read`](crate::Reg::read) this register and get [`apbrst::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apbrst::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbrst`] module"]
#[doc(alias = "APBRST")]
pub type Apbrst = crate::Reg<apbrst::ApbrstSpec>;
#[doc = "APB Peripheral Reset Register"]
pub mod apbrst;
#[doc = "AHBCG (rw) register accessor: AHB peripheral control register\n\nYou can [`read`](crate::Reg::read) this register and get [`ahbcg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahbcg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahbcg`] module"]
#[doc(alias = "AHBCG")]
pub type Ahbcg = crate::Reg<ahbcg::AhbcgSpec>;
#[doc = "AHB peripheral control register"]
pub mod ahbcg;
#[doc = "APBCG (rw) register accessor: APB peripheral control register\n\nYou can [`read`](crate::Reg::read) this register and get [`apbcg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apbcg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbcg`] module"]
#[doc(alias = "APBCG")]
pub type Apbcg = crate::Reg<apbcg::ApbcgSpec>;
#[doc = "APB peripheral control register"]
pub mod apbcg;
#[doc = "PWRCR (rw) register accessor: PWR peripheral control register\n\nYou can [`read`](crate::Reg::read) this register and get [`pwrcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwrcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwrcr`] module"]
#[doc(alias = "PWRCR")]
pub type Pwrcr = crate::Reg<pwrcr::PwrcrSpec>;
#[doc = "PWR peripheral control register"]
pub mod pwrcr;
#[doc = "RSTCSR (rw) register accessor: Clock reset status register\n\nYou can [`read`](crate::Reg::read) this register and get [`rstcsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rstcsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rstcsr`] module"]
#[doc(alias = "RSTCSR")]
pub type Rstcsr = crate::Reg<rstcsr::RstcsrSpec>;
#[doc = "Clock reset status register"]
pub mod rstcsr;
#[doc = "ADCCCR (rw) register accessor: Analog module control register\n\nYou can [`read`](crate::Reg::read) this register and get [`adcccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcccr`] module"]
#[doc(alias = "ADCCCR")]
pub type Adcccr = crate::Reg<adcccr::AdcccrSpec>;
#[doc = "Analog module control register"]
pub mod adcccr;
