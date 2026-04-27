#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    lintsts: Lintsts,
    hintsts: Hintsts,
    lifclr: Lifclr,
    hifclr: Hifclr,
    scfg0: Scfg0,
    ndata0: Ndata0,
    paddr0: Paddr0,
    m0addr0: M0addr0,
    m1addr0: M1addr0,
    fctrl0: Fctrl0,
    scfg1: Scfg1,
    ndata1: Ndata1,
    paddr1: Paddr1,
    m0addr1: M0addr1,
    m1addr1: M1addr1,
    fctrl1: Fctrl1,
    scfg2: Scfg2,
    ndata2: Ndata2,
    paddr2: Paddr2,
    m0addr2: M0addr2,
    m1addr2: M1addr2,
    fctrl2: Fctrl2,
    scfg3: Scfg3,
    ndata3: Ndata3,
    paddr3: Paddr3,
    m0addr3: M0addr3,
    m1addr3: M1addr3,
    fctrl3: Fctrl3,
    scfg4: Scfg4,
    ndata4: Ndata4,
    paddr4: Paddr4,
    m0addr4: M0addr4,
    m1addr4: M1addr4,
    fctrl4: Fctrl4,
    scfg5: Scfg5,
    ndata5: Ndata5,
    paddr5: Paddr5,
    m1addr5: M1addr5,
    s5m1ar: S5m1ar,
    fctrl5: Fctrl5,
    scfg6: Scfg6,
    ndata6: Ndata6,
    paddr6: Paddr6,
    m0addr6: M0addr6,
    m1addr6: M1addr6,
    fctrl6: Fctrl6,
    scfg7: Scfg7,
    ndata7: Ndata7,
    paddr7: Paddr7,
    m0addr7: M0addr7,
    m1addr7: M1addr7,
    fctrl7: Fctrl7,
}
impl RegisterBlock {
    #[doc = "0x00 - low interrupt status register"]
    #[inline(always)]
    pub const fn lintsts(&self) -> &Lintsts {
        &self.lintsts
    }
    #[doc = "0x04 - high interrupt status register"]
    #[inline(always)]
    pub const fn hintsts(&self) -> &Hintsts {
        &self.hintsts
    }
    #[doc = "0x08 - low interrupt flag clear register"]
    #[inline(always)]
    pub const fn lifclr(&self) -> &Lifclr {
        &self.lifclr
    }
    #[doc = "0x0c - high interrupt flag clear register"]
    #[inline(always)]
    pub const fn hifclr(&self) -> &Hifclr {
        &self.hifclr
    }
    #[doc = "0x10 - stream x configuration register"]
    #[inline(always)]
    pub const fn scfg0(&self) -> &Scfg0 {
        &self.scfg0
    }
    #[doc = "0x14 - stream x number of data register"]
    #[inline(always)]
    pub const fn ndata0(&self) -> &Ndata0 {
        &self.ndata0
    }
    #[doc = "0x18 - stream x peripheral address register"]
    #[inline(always)]
    pub const fn paddr0(&self) -> &Paddr0 {
        &self.paddr0
    }
    #[doc = "0x1c - stream x memory 0 address register"]
    #[inline(always)]
    pub const fn m0addr0(&self) -> &M0addr0 {
        &self.m0addr0
    }
    #[doc = "0x20 - stream x memory 1 address register"]
    #[inline(always)]
    pub const fn m1addr0(&self) -> &M1addr0 {
        &self.m1addr0
    }
    #[doc = "0x24 - stream x FIFO control register"]
    #[inline(always)]
    pub const fn fctrl0(&self) -> &Fctrl0 {
        &self.fctrl0
    }
    #[doc = "0x28 - stream x configuration register"]
    #[inline(always)]
    pub const fn scfg1(&self) -> &Scfg1 {
        &self.scfg1
    }
    #[doc = "0x2c - stream x number of data register"]
    #[inline(always)]
    pub const fn ndata1(&self) -> &Ndata1 {
        &self.ndata1
    }
    #[doc = "0x30 - stream x peripheral address register"]
    #[inline(always)]
    pub const fn paddr1(&self) -> &Paddr1 {
        &self.paddr1
    }
    #[doc = "0x34 - stream x memory 0 address register"]
    #[inline(always)]
    pub const fn m0addr1(&self) -> &M0addr1 {
        &self.m0addr1
    }
    #[doc = "0x38 - stream x memory 1 address register"]
    #[inline(always)]
    pub const fn m1addr1(&self) -> &M1addr1 {
        &self.m1addr1
    }
    #[doc = "0x3c - stream x FIFO control register"]
    #[inline(always)]
    pub const fn fctrl1(&self) -> &Fctrl1 {
        &self.fctrl1
    }
    #[doc = "0x40 - stream x configuration register"]
    #[inline(always)]
    pub const fn scfg2(&self) -> &Scfg2 {
        &self.scfg2
    }
    #[doc = "0x44 - stream x number of data register"]
    #[inline(always)]
    pub const fn ndata2(&self) -> &Ndata2 {
        &self.ndata2
    }
    #[doc = "0x48 - stream x peripheral address register"]
    #[inline(always)]
    pub const fn paddr2(&self) -> &Paddr2 {
        &self.paddr2
    }
    #[doc = "0x4c - stream x memory 0 address register"]
    #[inline(always)]
    pub const fn m0addr2(&self) -> &M0addr2 {
        &self.m0addr2
    }
    #[doc = "0x50 - stream x memory 1 address register"]
    #[inline(always)]
    pub const fn m1addr2(&self) -> &M1addr2 {
        &self.m1addr2
    }
    #[doc = "0x54 - stream x FIFO control register"]
    #[inline(always)]
    pub const fn fctrl2(&self) -> &Fctrl2 {
        &self.fctrl2
    }
    #[doc = "0x58 - stream x configuration register"]
    #[inline(always)]
    pub const fn scfg3(&self) -> &Scfg3 {
        &self.scfg3
    }
    #[doc = "0x5c - stream x number of data register"]
    #[inline(always)]
    pub const fn ndata3(&self) -> &Ndata3 {
        &self.ndata3
    }
    #[doc = "0x60 - stream x peripheral address register"]
    #[inline(always)]
    pub const fn paddr3(&self) -> &Paddr3 {
        &self.paddr3
    }
    #[doc = "0x64 - stream x memory 0 address register"]
    #[inline(always)]
    pub const fn m0addr3(&self) -> &M0addr3 {
        &self.m0addr3
    }
    #[doc = "0x68 - stream x memory 1 address register"]
    #[inline(always)]
    pub const fn m1addr3(&self) -> &M1addr3 {
        &self.m1addr3
    }
    #[doc = "0x6c - stream x FIFO control register"]
    #[inline(always)]
    pub const fn fctrl3(&self) -> &Fctrl3 {
        &self.fctrl3
    }
    #[doc = "0x70 - stream x configuration register"]
    #[inline(always)]
    pub const fn scfg4(&self) -> &Scfg4 {
        &self.scfg4
    }
    #[doc = "0x74 - stream x number of data register"]
    #[inline(always)]
    pub const fn ndata4(&self) -> &Ndata4 {
        &self.ndata4
    }
    #[doc = "0x78 - stream x peripheral address register"]
    #[inline(always)]
    pub const fn paddr4(&self) -> &Paddr4 {
        &self.paddr4
    }
    #[doc = "0x7c - stream x memory 0 address register"]
    #[inline(always)]
    pub const fn m0addr4(&self) -> &M0addr4 {
        &self.m0addr4
    }
    #[doc = "0x80 - stream x memory 1 address register"]
    #[inline(always)]
    pub const fn m1addr4(&self) -> &M1addr4 {
        &self.m1addr4
    }
    #[doc = "0x84 - stream x FIFO control register"]
    #[inline(always)]
    pub const fn fctrl4(&self) -> &Fctrl4 {
        &self.fctrl4
    }
    #[doc = "0x88 - stream x configuration register"]
    #[inline(always)]
    pub const fn scfg5(&self) -> &Scfg5 {
        &self.scfg5
    }
    #[doc = "0x8c - stream x number of data register"]
    #[inline(always)]
    pub const fn ndata5(&self) -> &Ndata5 {
        &self.ndata5
    }
    #[doc = "0x90 - stream x peripheral address register"]
    #[inline(always)]
    pub const fn paddr5(&self) -> &Paddr5 {
        &self.paddr5
    }
    #[doc = "0x94 - stream x memory 0 address register"]
    #[inline(always)]
    pub const fn m1addr5(&self) -> &M1addr5 {
        &self.m1addr5
    }
    #[doc = "0x98 - stream x memory 1 address register"]
    #[inline(always)]
    pub const fn s5m1ar(&self) -> &S5m1ar {
        &self.s5m1ar
    }
    #[doc = "0x9c - stream x FIFO control register"]
    #[inline(always)]
    pub const fn fctrl5(&self) -> &Fctrl5 {
        &self.fctrl5
    }
    #[doc = "0xa0 - stream x configuration register"]
    #[inline(always)]
    pub const fn scfg6(&self) -> &Scfg6 {
        &self.scfg6
    }
    #[doc = "0xa4 - stream x number of data register"]
    #[inline(always)]
    pub const fn ndata6(&self) -> &Ndata6 {
        &self.ndata6
    }
    #[doc = "0xa8 - stream x peripheral address register"]
    #[inline(always)]
    pub const fn paddr6(&self) -> &Paddr6 {
        &self.paddr6
    }
    #[doc = "0xac - stream x memory 0 address register"]
    #[inline(always)]
    pub const fn m0addr6(&self) -> &M0addr6 {
        &self.m0addr6
    }
    #[doc = "0xb0 - stream x memory 1 address register"]
    #[inline(always)]
    pub const fn m1addr6(&self) -> &M1addr6 {
        &self.m1addr6
    }
    #[doc = "0xb4 - stream x FIFO control register"]
    #[inline(always)]
    pub const fn fctrl6(&self) -> &Fctrl6 {
        &self.fctrl6
    }
    #[doc = "0xb8 - stream x configuration register"]
    #[inline(always)]
    pub const fn scfg7(&self) -> &Scfg7 {
        &self.scfg7
    }
    #[doc = "0xbc - stream x number of data register"]
    #[inline(always)]
    pub const fn ndata7(&self) -> &Ndata7 {
        &self.ndata7
    }
    #[doc = "0xc0 - stream x peripheral address register"]
    #[inline(always)]
    pub const fn paddr7(&self) -> &Paddr7 {
        &self.paddr7
    }
    #[doc = "0xc4 - stream x memory 0 address register"]
    #[inline(always)]
    pub const fn m0addr7(&self) -> &M0addr7 {
        &self.m0addr7
    }
    #[doc = "0xc8 - stream x memory 1 address register"]
    #[inline(always)]
    pub const fn m1addr7(&self) -> &M1addr7 {
        &self.m1addr7
    }
    #[doc = "0xcc - stream x FIFO control register"]
    #[inline(always)]
    pub const fn fctrl7(&self) -> &Fctrl7 {
        &self.fctrl7
    }
}
#[doc = "LINTSTS (r) register accessor: low interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`lintsts::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lintsts`] module"]
#[doc(alias = "LINTSTS")]
pub type Lintsts = crate::Reg<lintsts::LintstsSpec>;
#[doc = "low interrupt status register"]
pub mod lintsts;
#[doc = "HINTSTS (r) register accessor: high interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`hintsts::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hintsts`] module"]
#[doc(alias = "HINTSTS")]
pub type Hintsts = crate::Reg<hintsts::HintstsSpec>;
#[doc = "high interrupt status register"]
pub mod hintsts;
#[doc = "LIFCLR (rw) register accessor: low interrupt flag clear register\n\nYou can [`read`](crate::Reg::read) this register and get [`lifclr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lifclr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lifclr`] module"]
#[doc(alias = "LIFCLR")]
pub type Lifclr = crate::Reg<lifclr::LifclrSpec>;
#[doc = "low interrupt flag clear register"]
pub mod lifclr;
#[doc = "HIFCLR (rw) register accessor: high interrupt flag clear register\n\nYou can [`read`](crate::Reg::read) this register and get [`hifclr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hifclr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hifclr`] module"]
#[doc(alias = "HIFCLR")]
pub type Hifclr = crate::Reg<hifclr::HifclrSpec>;
#[doc = "high interrupt flag clear register"]
pub mod hifclr;
#[doc = "SCFG0 (rw) register accessor: stream x configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`scfg0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scfg0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scfg0`] module"]
#[doc(alias = "SCFG0")]
pub type Scfg0 = crate::Reg<scfg0::Scfg0Spec>;
#[doc = "stream x configuration register"]
pub mod scfg0;
#[doc = "NDATA0 (rw) register accessor: stream x number of data register\n\nYou can [`read`](crate::Reg::read) this register and get [`ndata0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ndata0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ndata0`] module"]
#[doc(alias = "NDATA0")]
pub type Ndata0 = crate::Reg<ndata0::Ndata0Spec>;
#[doc = "stream x number of data register"]
pub mod ndata0;
#[doc = "PADDR0 (rw) register accessor: stream x peripheral address register\n\nYou can [`read`](crate::Reg::read) this register and get [`paddr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`paddr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@paddr0`] module"]
#[doc(alias = "PADDR0")]
pub type Paddr0 = crate::Reg<paddr0::Paddr0Spec>;
#[doc = "stream x peripheral address register"]
pub mod paddr0;
#[doc = "M0ADDR0 (rw) register accessor: stream x memory 0 address register\n\nYou can [`read`](crate::Reg::read) this register and get [`m0addr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m0addr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m0addr0`] module"]
#[doc(alias = "M0ADDR0")]
pub type M0addr0 = crate::Reg<m0addr0::M0addr0Spec>;
#[doc = "stream x memory 0 address register"]
pub mod m0addr0;
#[doc = "M1ADDR0 (rw) register accessor: stream x memory 1 address register\n\nYou can [`read`](crate::Reg::read) this register and get [`m1addr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m1addr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m1addr0`] module"]
#[doc(alias = "M1ADDR0")]
pub type M1addr0 = crate::Reg<m1addr0::M1addr0Spec>;
#[doc = "stream x memory 1 address register"]
pub mod m1addr0;
#[doc = "FCTRL0 (rw) register accessor: stream x FIFO control register\n\nYou can [`read`](crate::Reg::read) this register and get [`fctrl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fctrl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fctrl0`] module"]
#[doc(alias = "FCTRL0")]
pub type Fctrl0 = crate::Reg<fctrl0::Fctrl0Spec>;
#[doc = "stream x FIFO control register"]
pub mod fctrl0;
#[doc = "SCFG1 (rw) register accessor: stream x configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`scfg1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scfg1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scfg1`] module"]
#[doc(alias = "SCFG1")]
pub type Scfg1 = crate::Reg<scfg1::Scfg1Spec>;
#[doc = "stream x configuration register"]
pub mod scfg1;
#[doc = "NDATA1 (rw) register accessor: stream x number of data register\n\nYou can [`read`](crate::Reg::read) this register and get [`ndata1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ndata1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ndata1`] module"]
#[doc(alias = "NDATA1")]
pub type Ndata1 = crate::Reg<ndata1::Ndata1Spec>;
#[doc = "stream x number of data register"]
pub mod ndata1;
#[doc = "PADDR1 (rw) register accessor: stream x peripheral address register\n\nYou can [`read`](crate::Reg::read) this register and get [`paddr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`paddr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@paddr1`] module"]
#[doc(alias = "PADDR1")]
pub type Paddr1 = crate::Reg<paddr1::Paddr1Spec>;
#[doc = "stream x peripheral address register"]
pub mod paddr1;
#[doc = "M0ADDR1 (rw) register accessor: stream x memory 0 address register\n\nYou can [`read`](crate::Reg::read) this register and get [`m0addr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m0addr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m0addr1`] module"]
#[doc(alias = "M0ADDR1")]
pub type M0addr1 = crate::Reg<m0addr1::M0addr1Spec>;
#[doc = "stream x memory 0 address register"]
pub mod m0addr1;
#[doc = "M1ADDR1 (rw) register accessor: stream x memory 1 address register\n\nYou can [`read`](crate::Reg::read) this register and get [`m1addr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m1addr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m1addr1`] module"]
#[doc(alias = "M1ADDR1")]
pub type M1addr1 = crate::Reg<m1addr1::M1addr1Spec>;
#[doc = "stream x memory 1 address register"]
pub mod m1addr1;
#[doc = "FCTRL1 (rw) register accessor: stream x FIFO control register\n\nYou can [`read`](crate::Reg::read) this register and get [`fctrl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fctrl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fctrl1`] module"]
#[doc(alias = "FCTRL1")]
pub type Fctrl1 = crate::Reg<fctrl1::Fctrl1Spec>;
#[doc = "stream x FIFO control register"]
pub mod fctrl1;
#[doc = "SCFG2 (rw) register accessor: stream x configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`scfg2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scfg2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scfg2`] module"]
#[doc(alias = "SCFG2")]
pub type Scfg2 = crate::Reg<scfg2::Scfg2Spec>;
#[doc = "stream x configuration register"]
pub mod scfg2;
#[doc = "NDATA2 (rw) register accessor: stream x number of data register\n\nYou can [`read`](crate::Reg::read) this register and get [`ndata2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ndata2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ndata2`] module"]
#[doc(alias = "NDATA2")]
pub type Ndata2 = crate::Reg<ndata2::Ndata2Spec>;
#[doc = "stream x number of data register"]
pub mod ndata2;
#[doc = "PADDR2 (rw) register accessor: stream x peripheral address register\n\nYou can [`read`](crate::Reg::read) this register and get [`paddr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`paddr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@paddr2`] module"]
#[doc(alias = "PADDR2")]
pub type Paddr2 = crate::Reg<paddr2::Paddr2Spec>;
#[doc = "stream x peripheral address register"]
pub mod paddr2;
#[doc = "M0ADDR2 (rw) register accessor: stream x memory 0 address register\n\nYou can [`read`](crate::Reg::read) this register and get [`m0addr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m0addr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m0addr2`] module"]
#[doc(alias = "M0ADDR2")]
pub type M0addr2 = crate::Reg<m0addr2::M0addr2Spec>;
#[doc = "stream x memory 0 address register"]
pub mod m0addr2;
#[doc = "M1ADDR2 (rw) register accessor: stream x memory 1 address register\n\nYou can [`read`](crate::Reg::read) this register and get [`m1addr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m1addr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m1addr2`] module"]
#[doc(alias = "M1ADDR2")]
pub type M1addr2 = crate::Reg<m1addr2::M1addr2Spec>;
#[doc = "stream x memory 1 address register"]
pub mod m1addr2;
#[doc = "FCTRL2 (rw) register accessor: stream x FIFO control register\n\nYou can [`read`](crate::Reg::read) this register and get [`fctrl2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fctrl2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fctrl2`] module"]
#[doc(alias = "FCTRL2")]
pub type Fctrl2 = crate::Reg<fctrl2::Fctrl2Spec>;
#[doc = "stream x FIFO control register"]
pub mod fctrl2;
#[doc = "SCFG3 (rw) register accessor: stream x configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`scfg3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scfg3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scfg3`] module"]
#[doc(alias = "SCFG3")]
pub type Scfg3 = crate::Reg<scfg3::Scfg3Spec>;
#[doc = "stream x configuration register"]
pub mod scfg3;
#[doc = "NDATA3 (rw) register accessor: stream x number of data register\n\nYou can [`read`](crate::Reg::read) this register and get [`ndata3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ndata3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ndata3`] module"]
#[doc(alias = "NDATA3")]
pub type Ndata3 = crate::Reg<ndata3::Ndata3Spec>;
#[doc = "stream x number of data register"]
pub mod ndata3;
#[doc = "PADDR3 (rw) register accessor: stream x peripheral address register\n\nYou can [`read`](crate::Reg::read) this register and get [`paddr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`paddr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@paddr3`] module"]
#[doc(alias = "PADDR3")]
pub type Paddr3 = crate::Reg<paddr3::Paddr3Spec>;
#[doc = "stream x peripheral address register"]
pub mod paddr3;
#[doc = "M0ADDR3 (rw) register accessor: stream x memory 0 address register\n\nYou can [`read`](crate::Reg::read) this register and get [`m0addr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m0addr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m0addr3`] module"]
#[doc(alias = "M0ADDR3")]
pub type M0addr3 = crate::Reg<m0addr3::M0addr3Spec>;
#[doc = "stream x memory 0 address register"]
pub mod m0addr3;
#[doc = "M1ADDR3 (rw) register accessor: stream x memory 1 address register\n\nYou can [`read`](crate::Reg::read) this register and get [`m1addr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m1addr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m1addr3`] module"]
#[doc(alias = "M1ADDR3")]
pub type M1addr3 = crate::Reg<m1addr3::M1addr3Spec>;
#[doc = "stream x memory 1 address register"]
pub mod m1addr3;
#[doc = "FCTRL3 (rw) register accessor: stream x FIFO control register\n\nYou can [`read`](crate::Reg::read) this register and get [`fctrl3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fctrl3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fctrl3`] module"]
#[doc(alias = "FCTRL3")]
pub type Fctrl3 = crate::Reg<fctrl3::Fctrl3Spec>;
#[doc = "stream x FIFO control register"]
pub mod fctrl3;
#[doc = "SCFG4 (rw) register accessor: stream x configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`scfg4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scfg4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scfg4`] module"]
#[doc(alias = "SCFG4")]
pub type Scfg4 = crate::Reg<scfg4::Scfg4Spec>;
#[doc = "stream x configuration register"]
pub mod scfg4;
#[doc = "NDATA4 (rw) register accessor: stream x number of data register\n\nYou can [`read`](crate::Reg::read) this register and get [`ndata4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ndata4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ndata4`] module"]
#[doc(alias = "NDATA4")]
pub type Ndata4 = crate::Reg<ndata4::Ndata4Spec>;
#[doc = "stream x number of data register"]
pub mod ndata4;
#[doc = "PADDR4 (rw) register accessor: stream x peripheral address register\n\nYou can [`read`](crate::Reg::read) this register and get [`paddr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`paddr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@paddr4`] module"]
#[doc(alias = "PADDR4")]
pub type Paddr4 = crate::Reg<paddr4::Paddr4Spec>;
#[doc = "stream x peripheral address register"]
pub mod paddr4;
#[doc = "M0ADDR4 (rw) register accessor: stream x memory 0 address register\n\nYou can [`read`](crate::Reg::read) this register and get [`m0addr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m0addr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m0addr4`] module"]
#[doc(alias = "M0ADDR4")]
pub type M0addr4 = crate::Reg<m0addr4::M0addr4Spec>;
#[doc = "stream x memory 0 address register"]
pub mod m0addr4;
#[doc = "M1ADDR4 (rw) register accessor: stream x memory 1 address register\n\nYou can [`read`](crate::Reg::read) this register and get [`m1addr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m1addr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m1addr4`] module"]
#[doc(alias = "M1ADDR4")]
pub type M1addr4 = crate::Reg<m1addr4::M1addr4Spec>;
#[doc = "stream x memory 1 address register"]
pub mod m1addr4;
#[doc = "FCTRL4 (rw) register accessor: stream x FIFO control register\n\nYou can [`read`](crate::Reg::read) this register and get [`fctrl4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fctrl4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fctrl4`] module"]
#[doc(alias = "FCTRL4")]
pub type Fctrl4 = crate::Reg<fctrl4::Fctrl4Spec>;
#[doc = "stream x FIFO control register"]
pub mod fctrl4;
#[doc = "SCFG5 (rw) register accessor: stream x configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`scfg5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scfg5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scfg5`] module"]
#[doc(alias = "SCFG5")]
pub type Scfg5 = crate::Reg<scfg5::Scfg5Spec>;
#[doc = "stream x configuration register"]
pub mod scfg5;
#[doc = "NDATA5 (rw) register accessor: stream x number of data register\n\nYou can [`read`](crate::Reg::read) this register and get [`ndata5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ndata5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ndata5`] module"]
#[doc(alias = "NDATA5")]
pub type Ndata5 = crate::Reg<ndata5::Ndata5Spec>;
#[doc = "stream x number of data register"]
pub mod ndata5;
#[doc = "PADDR5 (rw) register accessor: stream x peripheral address register\n\nYou can [`read`](crate::Reg::read) this register and get [`paddr5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`paddr5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@paddr5`] module"]
#[doc(alias = "PADDR5")]
pub type Paddr5 = crate::Reg<paddr5::Paddr5Spec>;
#[doc = "stream x peripheral address register"]
pub mod paddr5;
#[doc = "M1ADDR5 (rw) register accessor: stream x memory 0 address register\n\nYou can [`read`](crate::Reg::read) this register and get [`m1addr5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m1addr5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m1addr5`] module"]
#[doc(alias = "M1ADDR5")]
pub type M1addr5 = crate::Reg<m1addr5::M1addr5Spec>;
#[doc = "stream x memory 0 address register"]
pub mod m1addr5;
#[doc = "S5M1AR (rw) register accessor: stream x memory 1 address register\n\nYou can [`read`](crate::Reg::read) this register and get [`s5m1ar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`s5m1ar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s5m1ar`] module"]
#[doc(alias = "S5M1AR")]
pub type S5m1ar = crate::Reg<s5m1ar::S5m1arSpec>;
#[doc = "stream x memory 1 address register"]
pub mod s5m1ar;
#[doc = "FCTRL5 (rw) register accessor: stream x FIFO control register\n\nYou can [`read`](crate::Reg::read) this register and get [`fctrl5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fctrl5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fctrl5`] module"]
#[doc(alias = "FCTRL5")]
pub type Fctrl5 = crate::Reg<fctrl5::Fctrl5Spec>;
#[doc = "stream x FIFO control register"]
pub mod fctrl5;
#[doc = "SCFG6 (rw) register accessor: stream x configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`scfg6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scfg6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scfg6`] module"]
#[doc(alias = "SCFG6")]
pub type Scfg6 = crate::Reg<scfg6::Scfg6Spec>;
#[doc = "stream x configuration register"]
pub mod scfg6;
#[doc = "NDATA6 (rw) register accessor: stream x number of data register\n\nYou can [`read`](crate::Reg::read) this register and get [`ndata6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ndata6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ndata6`] module"]
#[doc(alias = "NDATA6")]
pub type Ndata6 = crate::Reg<ndata6::Ndata6Spec>;
#[doc = "stream x number of data register"]
pub mod ndata6;
#[doc = "PADDR6 (rw) register accessor: stream x peripheral address register\n\nYou can [`read`](crate::Reg::read) this register and get [`paddr6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`paddr6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@paddr6`] module"]
#[doc(alias = "PADDR6")]
pub type Paddr6 = crate::Reg<paddr6::Paddr6Spec>;
#[doc = "stream x peripheral address register"]
pub mod paddr6;
#[doc = "M0ADDR6 (rw) register accessor: stream x memory 0 address register\n\nYou can [`read`](crate::Reg::read) this register and get [`m0addr6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m0addr6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m0addr6`] module"]
#[doc(alias = "M0ADDR6")]
pub type M0addr6 = crate::Reg<m0addr6::M0addr6Spec>;
#[doc = "stream x memory 0 address register"]
pub mod m0addr6;
#[doc = "M1ADDR6 (rw) register accessor: stream x memory 1 address register\n\nYou can [`read`](crate::Reg::read) this register and get [`m1addr6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m1addr6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m1addr6`] module"]
#[doc(alias = "M1ADDR6")]
pub type M1addr6 = crate::Reg<m1addr6::M1addr6Spec>;
#[doc = "stream x memory 1 address register"]
pub mod m1addr6;
#[doc = "FCTRL6 (rw) register accessor: stream x FIFO control register\n\nYou can [`read`](crate::Reg::read) this register and get [`fctrl6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fctrl6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fctrl6`] module"]
#[doc(alias = "FCTRL6")]
pub type Fctrl6 = crate::Reg<fctrl6::Fctrl6Spec>;
#[doc = "stream x FIFO control register"]
pub mod fctrl6;
#[doc = "SCFG7 (rw) register accessor: stream x configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`scfg7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scfg7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scfg7`] module"]
#[doc(alias = "SCFG7")]
pub type Scfg7 = crate::Reg<scfg7::Scfg7Spec>;
#[doc = "stream x configuration register"]
pub mod scfg7;
#[doc = "NDATA7 (rw) register accessor: stream x number of data register\n\nYou can [`read`](crate::Reg::read) this register and get [`ndata7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ndata7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ndata7`] module"]
#[doc(alias = "NDATA7")]
pub type Ndata7 = crate::Reg<ndata7::Ndata7Spec>;
#[doc = "stream x number of data register"]
pub mod ndata7;
#[doc = "PADDR7 (rw) register accessor: stream x peripheral address register\n\nYou can [`read`](crate::Reg::read) this register and get [`paddr7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`paddr7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@paddr7`] module"]
#[doc(alias = "PADDR7")]
pub type Paddr7 = crate::Reg<paddr7::Paddr7Spec>;
#[doc = "stream x peripheral address register"]
pub mod paddr7;
#[doc = "M0ADDR7 (rw) register accessor: stream x memory 0 address register\n\nYou can [`read`](crate::Reg::read) this register and get [`m0addr7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m0addr7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m0addr7`] module"]
#[doc(alias = "M0ADDR7")]
pub type M0addr7 = crate::Reg<m0addr7::M0addr7Spec>;
#[doc = "stream x memory 0 address register"]
pub mod m0addr7;
#[doc = "M1ADDR7 (rw) register accessor: stream x memory 1 address register\n\nYou can [`read`](crate::Reg::read) this register and get [`m1addr7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m1addr7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m1addr7`] module"]
#[doc(alias = "M1ADDR7")]
pub type M1addr7 = crate::Reg<m1addr7::M1addr7Spec>;
#[doc = "stream x memory 1 address register"]
pub mod m1addr7;
#[doc = "FCTRL7 (rw) register accessor: stream x FIFO control register\n\nYou can [`read`](crate::Reg::read) this register and get [`fctrl7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fctrl7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fctrl7`] module"]
#[doc(alias = "FCTRL7")]
pub type Fctrl7 = crate::Reg<fctrl7::Fctrl7Spec>;
#[doc = "stream x FIFO control register"]
pub mod fctrl7;
