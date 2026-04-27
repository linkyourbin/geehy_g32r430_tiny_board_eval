#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    c1csr: C1csr,
    c2csr: C2csr,
    c3csr: C3csr,
    c4csr: C4csr,
}
impl RegisterBlock {
    #[doc = "0x00 - Comparator 1 control register."]
    #[inline(always)]
    pub const fn c1csr(&self) -> &C1csr {
        &self.c1csr
    }
    #[doc = "0x04 - Comparator 2 control register."]
    #[inline(always)]
    pub const fn c2csr(&self) -> &C2csr {
        &self.c2csr
    }
    #[doc = "0x08 - Comparator 3 control register."]
    #[inline(always)]
    pub const fn c3csr(&self) -> &C3csr {
        &self.c3csr
    }
    #[doc = "0x0c - Comparator 4 control register."]
    #[inline(always)]
    pub const fn c4csr(&self) -> &C4csr {
        &self.c4csr
    }
}
#[doc = "C1CSR (rw) register accessor: Comparator 1 control register.\n\nYou can [`read`](crate::Reg::read) this register and get [`c1csr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c1csr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c1csr`] module"]
#[doc(alias = "C1CSR")]
pub type C1csr = crate::Reg<c1csr::C1csrSpec>;
#[doc = "Comparator 1 control register."]
pub mod c1csr;
#[doc = "C2CSR (rw) register accessor: Comparator 2 control register.\n\nYou can [`read`](crate::Reg::read) this register and get [`c2csr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c2csr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c2csr`] module"]
#[doc(alias = "C2CSR")]
pub type C2csr = crate::Reg<c2csr::C2csrSpec>;
#[doc = "Comparator 2 control register."]
pub mod c2csr;
#[doc = "C3CSR (rw) register accessor: Comparator 3 control register.\n\nYou can [`read`](crate::Reg::read) this register and get [`c3csr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c3csr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c3csr`] module"]
#[doc(alias = "C3CSR")]
pub type C3csr = crate::Reg<c3csr::C3csrSpec>;
#[doc = "Comparator 3 control register."]
pub mod c3csr;
#[doc = "C4CSR (rw) register accessor: Comparator 4 control register.\n\nYou can [`read`](crate::Reg::read) this register and get [`c4csr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c4csr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c4csr`] module"]
#[doc(alias = "C4CSR")]
pub type C4csr = crate::Reg<c4csr::C4csrSpec>;
#[doc = "Comparator 4 control register."]
pub mod c4csr;
