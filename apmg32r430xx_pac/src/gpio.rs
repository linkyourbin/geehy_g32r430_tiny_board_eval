#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    moder1: Moder1,
    moder2: Moder2,
    moder3: Moder3,
    moder4: Moder4,
    pupdr1: Pupdr1,
    pupdr2: Pupdr2,
    pupdr3: Pupdr3,
    pupdr4: Pupdr4,
    otyper1: Otyper1,
    otyper2: Otyper2,
    ospeedr1: Ospeedr1,
    ospeedr2: Ospeedr2,
    ospeedr3: Ospeedr3,
    ospeedr4: Ospeedr4,
    idr1: Idr1,
    idr2: Idr2,
    odr1: Odr1,
    odr2: Odr2,
    afr1: Afr1,
    afr2: Afr2,
    afr3: Afr3,
    afr4: Afr4,
    filterr1: Filterr1,
    filterr2: Filterr2,
    bsrr1: Bsrr1,
    bsrr2: Bsrr2,
    bsrr3: Bsrr3,
    bsrr4: Bsrr4,
    brr1: Brr1,
    brr2: Brr2,
    brr3: Brr3,
    brr4: Brr4,
    swr1: Swr1,
    swr2: Swr2,
}
impl RegisterBlock {
    #[doc = "0x00 - GPIO mode register 1"]
    #[inline(always)]
    pub const fn moder1(&self) -> &Moder1 {
        &self.moder1
    }
    #[doc = "0x04 - GPIO mode register 2"]
    #[inline(always)]
    pub const fn moder2(&self) -> &Moder2 {
        &self.moder2
    }
    #[doc = "0x08 - GPIO mode register 3"]
    #[inline(always)]
    pub const fn moder3(&self) -> &Moder3 {
        &self.moder3
    }
    #[doc = "0x0c - GPIO mode register 4"]
    #[inline(always)]
    pub const fn moder4(&self) -> &Moder4 {
        &self.moder4
    }
    #[doc = "0x10 - GPIO pull-up/pull-down register 1"]
    #[inline(always)]
    pub const fn pupdr1(&self) -> &Pupdr1 {
        &self.pupdr1
    }
    #[doc = "0x14 - GPIO pull-up/pull-down register 2"]
    #[inline(always)]
    pub const fn pupdr2(&self) -> &Pupdr2 {
        &self.pupdr2
    }
    #[doc = "0x18 - GPIO pull-up/pull-down register 3"]
    #[inline(always)]
    pub const fn pupdr3(&self) -> &Pupdr3 {
        &self.pupdr3
    }
    #[doc = "0x1c - GPIO pull-up/pull-down register 4"]
    #[inline(always)]
    pub const fn pupdr4(&self) -> &Pupdr4 {
        &self.pupdr4
    }
    #[doc = "0x20 - GPIO output type register 1"]
    #[inline(always)]
    pub const fn otyper1(&self) -> &Otyper1 {
        &self.otyper1
    }
    #[doc = "0x24 - GPIO output type register 2"]
    #[inline(always)]
    pub const fn otyper2(&self) -> &Otyper2 {
        &self.otyper2
    }
    #[doc = "0x28 - GPIO output speed register 1"]
    #[inline(always)]
    pub const fn ospeedr1(&self) -> &Ospeedr1 {
        &self.ospeedr1
    }
    #[doc = "0x2c - GPIO output speed register 2"]
    #[inline(always)]
    pub const fn ospeedr2(&self) -> &Ospeedr2 {
        &self.ospeedr2
    }
    #[doc = "0x30 - GPIO output speed register 3"]
    #[inline(always)]
    pub const fn ospeedr3(&self) -> &Ospeedr3 {
        &self.ospeedr3
    }
    #[doc = "0x34 - GPIO output speed register 4"]
    #[inline(always)]
    pub const fn ospeedr4(&self) -> &Ospeedr4 {
        &self.ospeedr4
    }
    #[doc = "0x38 - GPIO input data register 1"]
    #[inline(always)]
    pub const fn idr1(&self) -> &Idr1 {
        &self.idr1
    }
    #[doc = "0x3c - GPIO input data register 2"]
    #[inline(always)]
    pub const fn idr2(&self) -> &Idr2 {
        &self.idr2
    }
    #[doc = "0x40 - GPIO output data register 1"]
    #[inline(always)]
    pub const fn odr1(&self) -> &Odr1 {
        &self.odr1
    }
    #[doc = "0x44 - GPIO output data register 2"]
    #[inline(always)]
    pub const fn odr2(&self) -> &Odr2 {
        &self.odr2
    }
    #[doc = "0x48 - GPIO alternate function register 1"]
    #[inline(always)]
    pub const fn afr1(&self) -> &Afr1 {
        &self.afr1
    }
    #[doc = "0x4c - GPIO alternate function register 2"]
    #[inline(always)]
    pub const fn afr2(&self) -> &Afr2 {
        &self.afr2
    }
    #[doc = "0x50 - GPIO alternate function register 3"]
    #[inline(always)]
    pub const fn afr3(&self) -> &Afr3 {
        &self.afr3
    }
    #[doc = "0x54 - GPIO alternate function register 4"]
    #[inline(always)]
    pub const fn afr4(&self) -> &Afr4 {
        &self.afr4
    }
    #[doc = "0x58 - Wave Filter Enable Register 1"]
    #[inline(always)]
    pub const fn filterr1(&self) -> &Filterr1 {
        &self.filterr1
    }
    #[doc = "0x5c - Wave Filter Enable Register 2"]
    #[inline(always)]
    pub const fn filterr2(&self) -> &Filterr2 {
        &self.filterr2
    }
    #[doc = "0x60 - Bit Set/Reset Register 1"]
    #[inline(always)]
    pub const fn bsrr1(&self) -> &Bsrr1 {
        &self.bsrr1
    }
    #[doc = "0x64 - Bit Set/Reset Register 2"]
    #[inline(always)]
    pub const fn bsrr2(&self) -> &Bsrr2 {
        &self.bsrr2
    }
    #[doc = "0x68 - Bit Set/Reset Register 3"]
    #[inline(always)]
    pub const fn bsrr3(&self) -> &Bsrr3 {
        &self.bsrr3
    }
    #[doc = "0x6c - Bit Set/Reset Register 4"]
    #[inline(always)]
    pub const fn bsrr4(&self) -> &Bsrr4 {
        &self.bsrr4
    }
    #[doc = "0x70 - Bit Reset Register 1"]
    #[inline(always)]
    pub const fn brr1(&self) -> &Brr1 {
        &self.brr1
    }
    #[doc = "0x74 - Bit Reset Register 2"]
    #[inline(always)]
    pub const fn brr2(&self) -> &Brr2 {
        &self.brr2
    }
    #[doc = "0x78 - Bit Reset Register 3"]
    #[inline(always)]
    pub const fn brr3(&self) -> &Brr3 {
        &self.brr3
    }
    #[doc = "0x7c - Bit Reset Register 4"]
    #[inline(always)]
    pub const fn brr4(&self) -> &Brr4 {
        &self.brr4
    }
    #[doc = "0x80 - Switch Register 1"]
    #[inline(always)]
    pub const fn swr1(&self) -> &Swr1 {
        &self.swr1
    }
    #[doc = "0x84 - Switch Register 2"]
    #[inline(always)]
    pub const fn swr2(&self) -> &Swr2 {
        &self.swr2
    }
}
#[doc = "MODER1 (rw) register accessor: GPIO mode register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`moder1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`moder1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@moder1`] module"]
#[doc(alias = "MODER1")]
pub type Moder1 = crate::Reg<moder1::Moder1Spec>;
#[doc = "GPIO mode register 1"]
pub mod moder1;
#[doc = "MODER2 (rw) register accessor: GPIO mode register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`moder2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`moder2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@moder2`] module"]
#[doc(alias = "MODER2")]
pub type Moder2 = crate::Reg<moder2::Moder2Spec>;
#[doc = "GPIO mode register 2"]
pub mod moder2;
#[doc = "MODER3 (rw) register accessor: GPIO mode register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`moder3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`moder3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@moder3`] module"]
#[doc(alias = "MODER3")]
pub type Moder3 = crate::Reg<moder3::Moder3Spec>;
#[doc = "GPIO mode register 3"]
pub mod moder3;
#[doc = "MODER4 (rw) register accessor: GPIO mode register 4\n\nYou can [`read`](crate::Reg::read) this register and get [`moder4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`moder4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@moder4`] module"]
#[doc(alias = "MODER4")]
pub type Moder4 = crate::Reg<moder4::Moder4Spec>;
#[doc = "GPIO mode register 4"]
pub mod moder4;
#[doc = "PUPDR1 (rw) register accessor: GPIO pull-up/pull-down register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`pupdr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pupdr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pupdr1`] module"]
#[doc(alias = "PUPDR1")]
pub type Pupdr1 = crate::Reg<pupdr1::Pupdr1Spec>;
#[doc = "GPIO pull-up/pull-down register 1"]
pub mod pupdr1;
#[doc = "PUPDR2 (rw) register accessor: GPIO pull-up/pull-down register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`pupdr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pupdr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pupdr2`] module"]
#[doc(alias = "PUPDR2")]
pub type Pupdr2 = crate::Reg<pupdr2::Pupdr2Spec>;
#[doc = "GPIO pull-up/pull-down register 2"]
pub mod pupdr2;
#[doc = "PUPDR3 (rw) register accessor: GPIO pull-up/pull-down register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`pupdr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pupdr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pupdr3`] module"]
#[doc(alias = "PUPDR3")]
pub type Pupdr3 = crate::Reg<pupdr3::Pupdr3Spec>;
#[doc = "GPIO pull-up/pull-down register 3"]
pub mod pupdr3;
#[doc = "PUPDR4 (rw) register accessor: GPIO pull-up/pull-down register 4\n\nYou can [`read`](crate::Reg::read) this register and get [`pupdr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pupdr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pupdr4`] module"]
#[doc(alias = "PUPDR4")]
pub type Pupdr4 = crate::Reg<pupdr4::Pupdr4Spec>;
#[doc = "GPIO pull-up/pull-down register 4"]
pub mod pupdr4;
#[doc = "OTYPER1 (rw) register accessor: GPIO output type register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`otyper1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otyper1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otyper1`] module"]
#[doc(alias = "OTYPER1")]
pub type Otyper1 = crate::Reg<otyper1::Otyper1Spec>;
#[doc = "GPIO output type register 1"]
pub mod otyper1;
#[doc = "OTYPER2 (rw) register accessor: GPIO output type register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`otyper2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otyper2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otyper2`] module"]
#[doc(alias = "OTYPER2")]
pub type Otyper2 = crate::Reg<otyper2::Otyper2Spec>;
#[doc = "GPIO output type register 2"]
pub mod otyper2;
#[doc = "OSPEEDR1 (rw) register accessor: GPIO output speed register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`ospeedr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ospeedr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ospeedr1`] module"]
#[doc(alias = "OSPEEDR1")]
pub type Ospeedr1 = crate::Reg<ospeedr1::Ospeedr1Spec>;
#[doc = "GPIO output speed register 1"]
pub mod ospeedr1;
#[doc = "OSPEEDR2 (rw) register accessor: GPIO output speed register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`ospeedr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ospeedr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ospeedr2`] module"]
#[doc(alias = "OSPEEDR2")]
pub type Ospeedr2 = crate::Reg<ospeedr2::Ospeedr2Spec>;
#[doc = "GPIO output speed register 2"]
pub mod ospeedr2;
#[doc = "OSPEEDR3 (rw) register accessor: GPIO output speed register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`ospeedr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ospeedr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ospeedr3`] module"]
#[doc(alias = "OSPEEDR3")]
pub type Ospeedr3 = crate::Reg<ospeedr3::Ospeedr3Spec>;
#[doc = "GPIO output speed register 3"]
pub mod ospeedr3;
#[doc = "OSPEEDR4 (rw) register accessor: GPIO output speed register 4\n\nYou can [`read`](crate::Reg::read) this register and get [`ospeedr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ospeedr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ospeedr4`] module"]
#[doc(alias = "OSPEEDR4")]
pub type Ospeedr4 = crate::Reg<ospeedr4::Ospeedr4Spec>;
#[doc = "GPIO output speed register 4"]
pub mod ospeedr4;
#[doc = "IDR1 (r) register accessor: GPIO input data register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`idr1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@idr1`] module"]
#[doc(alias = "IDR1")]
pub type Idr1 = crate::Reg<idr1::Idr1Spec>;
#[doc = "GPIO input data register 1"]
pub mod idr1;
#[doc = "IDR2 (r) register accessor: GPIO input data register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`idr2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@idr2`] module"]
#[doc(alias = "IDR2")]
pub type Idr2 = crate::Reg<idr2::Idr2Spec>;
#[doc = "GPIO input data register 2"]
pub mod idr2;
#[doc = "ODR1 (rw) register accessor: GPIO output data register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`odr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`odr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@odr1`] module"]
#[doc(alias = "ODR1")]
pub type Odr1 = crate::Reg<odr1::Odr1Spec>;
#[doc = "GPIO output data register 1"]
pub mod odr1;
#[doc = "ODR2 (rw) register accessor: GPIO output data register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`odr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`odr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@odr2`] module"]
#[doc(alias = "ODR2")]
pub type Odr2 = crate::Reg<odr2::Odr2Spec>;
#[doc = "GPIO output data register 2"]
pub mod odr2;
#[doc = "AFR1 (rw) register accessor: GPIO alternate function register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`afr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`afr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@afr1`] module"]
#[doc(alias = "AFR1")]
pub type Afr1 = crate::Reg<afr1::Afr1Spec>;
#[doc = "GPIO alternate function register 1"]
pub mod afr1;
#[doc = "AFR2 (rw) register accessor: GPIO alternate function register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`afr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`afr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@afr2`] module"]
#[doc(alias = "AFR2")]
pub type Afr2 = crate::Reg<afr2::Afr2Spec>;
#[doc = "GPIO alternate function register 2"]
pub mod afr2;
#[doc = "AFR3 (rw) register accessor: GPIO alternate function register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`afr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`afr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@afr3`] module"]
#[doc(alias = "AFR3")]
pub type Afr3 = crate::Reg<afr3::Afr3Spec>;
#[doc = "GPIO alternate function register 3"]
pub mod afr3;
#[doc = "AFR4 (rw) register accessor: GPIO alternate function register 4\n\nYou can [`read`](crate::Reg::read) this register and get [`afr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`afr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@afr4`] module"]
#[doc(alias = "AFR4")]
pub type Afr4 = crate::Reg<afr4::Afr4Spec>;
#[doc = "GPIO alternate function register 4"]
pub mod afr4;
#[doc = "FILTERR1 (rw) register accessor: Wave Filter Enable Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`filterr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`filterr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@filterr1`] module"]
#[doc(alias = "FILTERR1")]
pub type Filterr1 = crate::Reg<filterr1::Filterr1Spec>;
#[doc = "Wave Filter Enable Register 1"]
pub mod filterr1;
#[doc = "FILTERR2 (rw) register accessor: Wave Filter Enable Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`filterr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`filterr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@filterr2`] module"]
#[doc(alias = "FILTERR2")]
pub type Filterr2 = crate::Reg<filterr2::Filterr2Spec>;
#[doc = "Wave Filter Enable Register 2"]
pub mod filterr2;
#[doc = "BSRR1 (rw) register accessor: Bit Set/Reset Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`bsrr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bsrr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bsrr1`] module"]
#[doc(alias = "BSRR1")]
pub type Bsrr1 = crate::Reg<bsrr1::Bsrr1Spec>;
#[doc = "Bit Set/Reset Register 1"]
pub mod bsrr1;
#[doc = "BSRR2 (rw) register accessor: Bit Set/Reset Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`bsrr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bsrr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bsrr2`] module"]
#[doc(alias = "BSRR2")]
pub type Bsrr2 = crate::Reg<bsrr2::Bsrr2Spec>;
#[doc = "Bit Set/Reset Register 2"]
pub mod bsrr2;
#[doc = "BSRR3 (rw) register accessor: Bit Set/Reset Register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`bsrr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bsrr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bsrr3`] module"]
#[doc(alias = "BSRR3")]
pub type Bsrr3 = crate::Reg<bsrr3::Bsrr3Spec>;
#[doc = "Bit Set/Reset Register 3"]
pub mod bsrr3;
#[doc = "BSRR4 (rw) register accessor: Bit Set/Reset Register 4\n\nYou can [`read`](crate::Reg::read) this register and get [`bsrr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bsrr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bsrr4`] module"]
#[doc(alias = "BSRR4")]
pub type Bsrr4 = crate::Reg<bsrr4::Bsrr4Spec>;
#[doc = "Bit Set/Reset Register 4"]
pub mod bsrr4;
#[doc = "BRR1 (rw) register accessor: Bit Reset Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`brr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`brr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@brr1`] module"]
#[doc(alias = "BRR1")]
pub type Brr1 = crate::Reg<brr1::Brr1Spec>;
#[doc = "Bit Reset Register 1"]
pub mod brr1;
#[doc = "BRR2 (rw) register accessor: Bit Reset Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`brr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`brr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@brr2`] module"]
#[doc(alias = "BRR2")]
pub type Brr2 = crate::Reg<brr2::Brr2Spec>;
#[doc = "Bit Reset Register 2"]
pub mod brr2;
#[doc = "BRR3 (rw) register accessor: Bit Reset Register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`brr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`brr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@brr3`] module"]
#[doc(alias = "BRR3")]
pub type Brr3 = crate::Reg<brr3::Brr3Spec>;
#[doc = "Bit Reset Register 3"]
pub mod brr3;
#[doc = "BRR4 (rw) register accessor: Bit Reset Register 4\n\nYou can [`read`](crate::Reg::read) this register and get [`brr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`brr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@brr4`] module"]
#[doc(alias = "BRR4")]
pub type Brr4 = crate::Reg<brr4::Brr4Spec>;
#[doc = "Bit Reset Register 4"]
pub mod brr4;
#[doc = "SWR1 (rw) register accessor: Switch Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`swr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swr1`] module"]
#[doc(alias = "SWR1")]
pub type Swr1 = crate::Reg<swr1::Swr1Spec>;
#[doc = "Switch Register 1"]
pub mod swr1;
#[doc = "SWR2 (rw) register accessor: Switch Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`swr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swr2`] module"]
#[doc(alias = "SWR2")]
pub type Swr2 = crate::Reg<swr2::Swr2Spec>;
#[doc = "Switch Register 2"]
pub mod swr2;
