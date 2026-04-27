#[doc = "Register `IDR1` reader"]
pub type R = crate::R<Idr1Spec>;
#[doc = "Field `PA0_DIN` reader - PAD input data"]
pub type Pa0DinR = crate::BitReader;
#[doc = "Field `PA1_DIN` reader - PAD input data"]
pub type Pa1DinR = crate::BitReader;
#[doc = "Field `PA2_DIN` reader - PAD input data"]
pub type Pa2DinR = crate::BitReader;
#[doc = "Field `PA3_DIN` reader - PAD input data"]
pub type Pa3DinR = crate::BitReader;
#[doc = "Field `PA4_DIN` reader - PAD input data"]
pub type Pa4DinR = crate::BitReader;
#[doc = "Field `PA5_DIN` reader - PAD input data"]
pub type Pa5DinR = crate::BitReader;
#[doc = "Field `PA6_DIN` reader - PAD input data"]
pub type Pa6DinR = crate::BitReader;
#[doc = "Field `PA7_DIN` reader - PAD input data"]
pub type Pa7DinR = crate::BitReader;
#[doc = "Field `PA8_DIN` reader - PAD input data"]
pub type Pa8DinR = crate::BitReader;
#[doc = "Field `PA9_DIN` reader - PAD input data"]
pub type Pa9DinR = crate::BitReader;
#[doc = "Field `PB0_DIN` reader - PAD input data"]
pub type Pb0DinR = crate::BitReader;
#[doc = "Field `PB1_DIN` reader - PAD input data"]
pub type Pb1DinR = crate::BitReader;
#[doc = "Field `PB2_DIN` reader - PAD input data"]
pub type Pb2DinR = crate::BitReader;
#[doc = "Field `PB3_DIN` reader - PAD input data"]
pub type Pb3DinR = crate::BitReader;
#[doc = "Field `PB4_DIN` reader - PAD input data"]
pub type Pb4DinR = crate::BitReader;
#[doc = "Field `PB5_DIN` reader - PAD input data"]
pub type Pb5DinR = crate::BitReader;
#[doc = "Field `PB6_DIN` reader - PAD input data"]
pub type Pb6DinR = crate::BitReader;
#[doc = "Field `PB7_DIN` reader - PAD input data"]
pub type Pb7DinR = crate::BitReader;
#[doc = "Field `PB8_DIN` reader - PAD input data"]
pub type Pb8DinR = crate::BitReader;
#[doc = "Field `PB9_DIN` reader - PAD input data"]
pub type Pb9DinR = crate::BitReader;
#[doc = "Field `PB10_DIN` reader - PAD input data"]
pub type Pb10DinR = crate::BitReader;
#[doc = "Field `PB11_DIN` reader - PAD input data"]
pub type Pb11DinR = crate::BitReader;
#[doc = "Field `PB12_DIN` reader - PAD input data"]
pub type Pb12DinR = crate::BitReader;
#[doc = "Field `PB13_DIN` reader - PAD input data"]
pub type Pb13DinR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - PAD input data"]
    #[inline(always)]
    pub fn pa0_din(&self) -> Pa0DinR {
        Pa0DinR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PAD input data"]
    #[inline(always)]
    pub fn pa1_din(&self) -> Pa1DinR {
        Pa1DinR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PAD input data"]
    #[inline(always)]
    pub fn pa2_din(&self) -> Pa2DinR {
        Pa2DinR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - PAD input data"]
    #[inline(always)]
    pub fn pa3_din(&self) -> Pa3DinR {
        Pa3DinR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - PAD input data"]
    #[inline(always)]
    pub fn pa4_din(&self) -> Pa4DinR {
        Pa4DinR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - PAD input data"]
    #[inline(always)]
    pub fn pa5_din(&self) -> Pa5DinR {
        Pa5DinR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - PAD input data"]
    #[inline(always)]
    pub fn pa6_din(&self) -> Pa6DinR {
        Pa6DinR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - PAD input data"]
    #[inline(always)]
    pub fn pa7_din(&self) -> Pa7DinR {
        Pa7DinR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - PAD input data"]
    #[inline(always)]
    pub fn pa8_din(&self) -> Pa8DinR {
        Pa8DinR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - PAD input data"]
    #[inline(always)]
    pub fn pa9_din(&self) -> Pa9DinR {
        Pa9DinR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 16 - PAD input data"]
    #[inline(always)]
    pub fn pb0_din(&self) -> Pb0DinR {
        Pb0DinR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - PAD input data"]
    #[inline(always)]
    pub fn pb1_din(&self) -> Pb1DinR {
        Pb1DinR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - PAD input data"]
    #[inline(always)]
    pub fn pb2_din(&self) -> Pb2DinR {
        Pb2DinR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - PAD input data"]
    #[inline(always)]
    pub fn pb3_din(&self) -> Pb3DinR {
        Pb3DinR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - PAD input data"]
    #[inline(always)]
    pub fn pb4_din(&self) -> Pb4DinR {
        Pb4DinR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - PAD input data"]
    #[inline(always)]
    pub fn pb5_din(&self) -> Pb5DinR {
        Pb5DinR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - PAD input data"]
    #[inline(always)]
    pub fn pb6_din(&self) -> Pb6DinR {
        Pb6DinR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - PAD input data"]
    #[inline(always)]
    pub fn pb7_din(&self) -> Pb7DinR {
        Pb7DinR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - PAD input data"]
    #[inline(always)]
    pub fn pb8_din(&self) -> Pb8DinR {
        Pb8DinR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - PAD input data"]
    #[inline(always)]
    pub fn pb9_din(&self) -> Pb9DinR {
        Pb9DinR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - PAD input data"]
    #[inline(always)]
    pub fn pb10_din(&self) -> Pb10DinR {
        Pb10DinR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - PAD input data"]
    #[inline(always)]
    pub fn pb11_din(&self) -> Pb11DinR {
        Pb11DinR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - PAD input data"]
    #[inline(always)]
    pub fn pb12_din(&self) -> Pb12DinR {
        Pb12DinR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - PAD input data"]
    #[inline(always)]
    pub fn pb13_din(&self) -> Pb13DinR {
        Pb13DinR::new(((self.bits >> 29) & 1) != 0)
    }
}
#[doc = "GPIO input data register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`idr1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Idr1Spec;
impl crate::RegisterSpec for Idr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`idr1::R`](R) reader structure"]
impl crate::Readable for Idr1Spec {}
#[doc = "`reset()` method sets IDR1 to value 0"]
impl crate::Resettable for Idr1Spec {}
