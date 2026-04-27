#[doc = "Register `IDR2` reader"]
pub type R = crate::R<Idr2Spec>;
#[doc = "Field `PC0_DIN` reader - PAD input data"]
pub type Pc0DinR = crate::BitReader;
#[doc = "Field `PC1_DIN` reader - PAD input data"]
pub type Pc1DinR = crate::BitReader;
#[doc = "Field `PC2_DIN` reader - PAD input data"]
pub type Pc2DinR = crate::BitReader;
#[doc = "Field `PC3_DIN` reader - PAD input data"]
pub type Pc3DinR = crate::BitReader;
#[doc = "Field `PC4_DIN` reader - PAD input data"]
pub type Pc4DinR = crate::BitReader;
#[doc = "Field `PC5_DIN` reader - PAD input data"]
pub type Pc5DinR = crate::BitReader;
#[doc = "Field `PC6_DIN` reader - PAD input data"]
pub type Pc6DinR = crate::BitReader;
#[doc = "Field `PC7_DIN` reader - PAD input data"]
pub type Pc7DinR = crate::BitReader;
#[doc = "Field `PC8_DIN` reader - PAD input data"]
pub type Pc8DinR = crate::BitReader;
#[doc = "Field `PC9_DIN` reader - PAD input data"]
pub type Pc9DinR = crate::BitReader;
#[doc = "Field `PC10_DIN` reader - PAD input data"]
pub type Pc10DinR = crate::BitReader;
#[doc = "Field `PC11_DIN` reader - PAD input data"]
pub type Pc11DinR = crate::BitReader;
#[doc = "Field `PC12_DIN` reader - PAD input data"]
pub type Pc12DinR = crate::BitReader;
#[doc = "Field `PD0_DIN` reader - PAD input data"]
pub type Pd0DinR = crate::BitReader;
#[doc = "Field `PD1_DIN` reader - PAD input data"]
pub type Pd1DinR = crate::BitReader;
#[doc = "Field `PD2_DIN` reader - PAD input data"]
pub type Pd2DinR = crate::BitReader;
#[doc = "Field `PD3_DIN` reader - PAD input data"]
pub type Pd3DinR = crate::BitReader;
#[doc = "Field `PD4_DIN` reader - PAD input data"]
pub type Pd4DinR = crate::BitReader;
#[doc = "Field `PD5_DIN` reader - PAD input data"]
pub type Pd5DinR = crate::BitReader;
#[doc = "Field `PD6_DIN` reader - PAD input data"]
pub type Pd6DinR = crate::BitReader;
#[doc = "Field `PD7_DIN` reader - PAD input data"]
pub type Pd7DinR = crate::BitReader;
#[doc = "Field `PD8_DIN` reader - PAD input data"]
pub type Pd8DinR = crate::BitReader;
#[doc = "Field `PD9_DIN` reader - PAD input data"]
pub type Pd9DinR = crate::BitReader;
#[doc = "Field `PD10_DIN` reader - PAD input data"]
pub type Pd10DinR = crate::BitReader;
#[doc = "Field `PD11_DIN` reader - PAD input data"]
pub type Pd11DinR = crate::BitReader;
#[doc = "Field `PD12_DIN` reader - PAD input data"]
pub type Pd12DinR = crate::BitReader;
#[doc = "Field `PD13_DIN` reader - PAD input data"]
pub type Pd13DinR = crate::BitReader;
#[doc = "Field `PD14_DIN` reader - PAD input data"]
pub type Pd14DinR = crate::BitReader;
#[doc = "Field `PD15_DIN` reader - PAD input data"]
pub type Pd15DinR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - PAD input data"]
    #[inline(always)]
    pub fn pc0_din(&self) -> Pc0DinR {
        Pc0DinR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PAD input data"]
    #[inline(always)]
    pub fn pc1_din(&self) -> Pc1DinR {
        Pc1DinR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PAD input data"]
    #[inline(always)]
    pub fn pc2_din(&self) -> Pc2DinR {
        Pc2DinR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - PAD input data"]
    #[inline(always)]
    pub fn pc3_din(&self) -> Pc3DinR {
        Pc3DinR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - PAD input data"]
    #[inline(always)]
    pub fn pc4_din(&self) -> Pc4DinR {
        Pc4DinR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - PAD input data"]
    #[inline(always)]
    pub fn pc5_din(&self) -> Pc5DinR {
        Pc5DinR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - PAD input data"]
    #[inline(always)]
    pub fn pc6_din(&self) -> Pc6DinR {
        Pc6DinR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - PAD input data"]
    #[inline(always)]
    pub fn pc7_din(&self) -> Pc7DinR {
        Pc7DinR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - PAD input data"]
    #[inline(always)]
    pub fn pc8_din(&self) -> Pc8DinR {
        Pc8DinR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - PAD input data"]
    #[inline(always)]
    pub fn pc9_din(&self) -> Pc9DinR {
        Pc9DinR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - PAD input data"]
    #[inline(always)]
    pub fn pc10_din(&self) -> Pc10DinR {
        Pc10DinR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - PAD input data"]
    #[inline(always)]
    pub fn pc11_din(&self) -> Pc11DinR {
        Pc11DinR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PAD input data"]
    #[inline(always)]
    pub fn pc12_din(&self) -> Pc12DinR {
        Pc12DinR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16 - PAD input data"]
    #[inline(always)]
    pub fn pd0_din(&self) -> Pd0DinR {
        Pd0DinR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - PAD input data"]
    #[inline(always)]
    pub fn pd1_din(&self) -> Pd1DinR {
        Pd1DinR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - PAD input data"]
    #[inline(always)]
    pub fn pd2_din(&self) -> Pd2DinR {
        Pd2DinR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - PAD input data"]
    #[inline(always)]
    pub fn pd3_din(&self) -> Pd3DinR {
        Pd3DinR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - PAD input data"]
    #[inline(always)]
    pub fn pd4_din(&self) -> Pd4DinR {
        Pd4DinR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - PAD input data"]
    #[inline(always)]
    pub fn pd5_din(&self) -> Pd5DinR {
        Pd5DinR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - PAD input data"]
    #[inline(always)]
    pub fn pd6_din(&self) -> Pd6DinR {
        Pd6DinR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - PAD input data"]
    #[inline(always)]
    pub fn pd7_din(&self) -> Pd7DinR {
        Pd7DinR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - PAD input data"]
    #[inline(always)]
    pub fn pd8_din(&self) -> Pd8DinR {
        Pd8DinR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - PAD input data"]
    #[inline(always)]
    pub fn pd9_din(&self) -> Pd9DinR {
        Pd9DinR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - PAD input data"]
    #[inline(always)]
    pub fn pd10_din(&self) -> Pd10DinR {
        Pd10DinR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - PAD input data"]
    #[inline(always)]
    pub fn pd11_din(&self) -> Pd11DinR {
        Pd11DinR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - PAD input data"]
    #[inline(always)]
    pub fn pd12_din(&self) -> Pd12DinR {
        Pd12DinR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - PAD input data"]
    #[inline(always)]
    pub fn pd13_din(&self) -> Pd13DinR {
        Pd13DinR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - PAD input data"]
    #[inline(always)]
    pub fn pd14_din(&self) -> Pd14DinR {
        Pd14DinR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - PAD input data"]
    #[inline(always)]
    pub fn pd15_din(&self) -> Pd15DinR {
        Pd15DinR::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "GPIO input data register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`idr2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Idr2Spec;
impl crate::RegisterSpec for Idr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`idr2::R`](R) reader structure"]
impl crate::Readable for Idr2Spec {}
#[doc = "`reset()` method sets IDR2 to value 0"]
impl crate::Resettable for Idr2Spec {}
