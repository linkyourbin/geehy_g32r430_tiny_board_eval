#[doc = "Register `AFR4` reader"]
pub type R = crate::R<Afr4Spec>;
#[doc = "Register `AFR4` writer"]
pub type W = crate::W<Afr4Spec>;
#[doc = "Field `PD0_AFSEL` reader - PAD reuse selection"]
pub type Pd0AfselR = crate::FieldReader;
#[doc = "Field `PD0_AFSEL` writer - PAD reuse selection"]
pub type Pd0AfselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PD1_AFSEL` reader - PAD reuse selection"]
pub type Pd1AfselR = crate::FieldReader;
#[doc = "Field `PD1_AFSEL` writer - PAD reuse selection"]
pub type Pd1AfselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PD2_AFSEL` reader - PAD reuse selection"]
pub type Pd2AfselR = crate::FieldReader;
#[doc = "Field `PD2_AFSEL` writer - PAD reuse selection"]
pub type Pd2AfselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PD3_AFSEL` reader - PAD reuse selection"]
pub type Pd3AfselR = crate::FieldReader;
#[doc = "Field `PD3_AFSEL` writer - PAD reuse selection"]
pub type Pd3AfselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PD4_AFSEL` reader - PAD reuse selection"]
pub type Pd4AfselR = crate::FieldReader;
#[doc = "Field `PD4_AFSEL` writer - PAD reuse selection"]
pub type Pd4AfselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PD5_AFSEL` reader - PAD reuse selection"]
pub type Pd5AfselR = crate::FieldReader;
#[doc = "Field `PD5_AFSEL` writer - PAD reuse selection"]
pub type Pd5AfselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PD6_AFSEL` reader - PAD reuse selection"]
pub type Pd6AfselR = crate::FieldReader;
#[doc = "Field `PD6_AFSEL` writer - PAD reuse selection"]
pub type Pd6AfselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PD7_AFSEL` reader - PAD reuse selection"]
pub type Pd7AfselR = crate::FieldReader;
#[doc = "Field `PD7_AFSEL` writer - PAD reuse selection"]
pub type Pd7AfselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PD8_AFSEL` reader - PAD reuse selection"]
pub type Pd8AfselR = crate::FieldReader;
#[doc = "Field `PD8_AFSEL` writer - PAD reuse selection"]
pub type Pd8AfselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PD9_AFSEL` reader - PAD reuse selection"]
pub type Pd9AfselR = crate::FieldReader;
#[doc = "Field `PD9_AFSEL` writer - PAD reuse selection"]
pub type Pd9AfselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PD10_AFSEL` reader - PAD reuse selection"]
pub type Pd10AfselR = crate::FieldReader;
#[doc = "Field `PD10_AFSEL` writer - PAD reuse selection"]
pub type Pd10AfselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PD11_AFSEL` reader - PAD reuse selection"]
pub type Pd11AfselR = crate::FieldReader;
#[doc = "Field `PD11_AFSEL` writer - PAD reuse selection"]
pub type Pd11AfselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PD12_AFSEL` reader - PAD reuse selection"]
pub type Pd12AfselR = crate::FieldReader;
#[doc = "Field `PD12_AFSEL` writer - PAD reuse selection"]
pub type Pd12AfselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PD13_AFSEL` reader - PAD reuse selection"]
pub type Pd13AfselR = crate::FieldReader;
#[doc = "Field `PD13_AFSEL` writer - PAD reuse selection"]
pub type Pd13AfselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PD14_AFSEL` reader - PAD reuse selection"]
pub type Pd14AfselR = crate::FieldReader;
#[doc = "Field `PD14_AFSEL` writer - PAD reuse selection"]
pub type Pd14AfselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PD15_AFSEL` reader - PAD reuse selection"]
pub type Pd15AfselR = crate::FieldReader;
#[doc = "Field `PD15_AFSEL` writer - PAD reuse selection"]
pub type Pd15AfselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - PAD reuse selection"]
    #[inline(always)]
    pub fn pd0_afsel(&self) -> Pd0AfselR {
        Pd0AfselR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - PAD reuse selection"]
    #[inline(always)]
    pub fn pd1_afsel(&self) -> Pd1AfselR {
        Pd1AfselR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - PAD reuse selection"]
    #[inline(always)]
    pub fn pd2_afsel(&self) -> Pd2AfselR {
        Pd2AfselR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - PAD reuse selection"]
    #[inline(always)]
    pub fn pd3_afsel(&self) -> Pd3AfselR {
        Pd3AfselR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - PAD reuse selection"]
    #[inline(always)]
    pub fn pd4_afsel(&self) -> Pd4AfselR {
        Pd4AfselR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - PAD reuse selection"]
    #[inline(always)]
    pub fn pd5_afsel(&self) -> Pd5AfselR {
        Pd5AfselR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - PAD reuse selection"]
    #[inline(always)]
    pub fn pd6_afsel(&self) -> Pd6AfselR {
        Pd6AfselR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - PAD reuse selection"]
    #[inline(always)]
    pub fn pd7_afsel(&self) -> Pd7AfselR {
        Pd7AfselR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - PAD reuse selection"]
    #[inline(always)]
    pub fn pd8_afsel(&self) -> Pd8AfselR {
        Pd8AfselR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - PAD reuse selection"]
    #[inline(always)]
    pub fn pd9_afsel(&self) -> Pd9AfselR {
        Pd9AfselR::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - PAD reuse selection"]
    #[inline(always)]
    pub fn pd10_afsel(&self) -> Pd10AfselR {
        Pd10AfselR::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - PAD reuse selection"]
    #[inline(always)]
    pub fn pd11_afsel(&self) -> Pd11AfselR {
        Pd11AfselR::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - PAD reuse selection"]
    #[inline(always)]
    pub fn pd12_afsel(&self) -> Pd12AfselR {
        Pd12AfselR::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - PAD reuse selection"]
    #[inline(always)]
    pub fn pd13_afsel(&self) -> Pd13AfselR {
        Pd13AfselR::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - PAD reuse selection"]
    #[inline(always)]
    pub fn pd14_afsel(&self) -> Pd14AfselR {
        Pd14AfselR::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - PAD reuse selection"]
    #[inline(always)]
    pub fn pd15_afsel(&self) -> Pd15AfselR {
        Pd15AfselR::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - PAD reuse selection"]
    #[inline(always)]
    pub fn pd0_afsel(&mut self) -> Pd0AfselW<'_, Afr4Spec> {
        Pd0AfselW::new(self, 0)
    }
    #[doc = "Bits 2:3 - PAD reuse selection"]
    #[inline(always)]
    pub fn pd1_afsel(&mut self) -> Pd1AfselW<'_, Afr4Spec> {
        Pd1AfselW::new(self, 2)
    }
    #[doc = "Bits 4:5 - PAD reuse selection"]
    #[inline(always)]
    pub fn pd2_afsel(&mut self) -> Pd2AfselW<'_, Afr4Spec> {
        Pd2AfselW::new(self, 4)
    }
    #[doc = "Bits 6:7 - PAD reuse selection"]
    #[inline(always)]
    pub fn pd3_afsel(&mut self) -> Pd3AfselW<'_, Afr4Spec> {
        Pd3AfselW::new(self, 6)
    }
    #[doc = "Bits 8:9 - PAD reuse selection"]
    #[inline(always)]
    pub fn pd4_afsel(&mut self) -> Pd4AfselW<'_, Afr4Spec> {
        Pd4AfselW::new(self, 8)
    }
    #[doc = "Bits 10:11 - PAD reuse selection"]
    #[inline(always)]
    pub fn pd5_afsel(&mut self) -> Pd5AfselW<'_, Afr4Spec> {
        Pd5AfselW::new(self, 10)
    }
    #[doc = "Bits 12:13 - PAD reuse selection"]
    #[inline(always)]
    pub fn pd6_afsel(&mut self) -> Pd6AfselW<'_, Afr4Spec> {
        Pd6AfselW::new(self, 12)
    }
    #[doc = "Bits 14:15 - PAD reuse selection"]
    #[inline(always)]
    pub fn pd7_afsel(&mut self) -> Pd7AfselW<'_, Afr4Spec> {
        Pd7AfselW::new(self, 14)
    }
    #[doc = "Bits 16:17 - PAD reuse selection"]
    #[inline(always)]
    pub fn pd8_afsel(&mut self) -> Pd8AfselW<'_, Afr4Spec> {
        Pd8AfselW::new(self, 16)
    }
    #[doc = "Bits 18:19 - PAD reuse selection"]
    #[inline(always)]
    pub fn pd9_afsel(&mut self) -> Pd9AfselW<'_, Afr4Spec> {
        Pd9AfselW::new(self, 18)
    }
    #[doc = "Bits 20:21 - PAD reuse selection"]
    #[inline(always)]
    pub fn pd10_afsel(&mut self) -> Pd10AfselW<'_, Afr4Spec> {
        Pd10AfselW::new(self, 20)
    }
    #[doc = "Bits 22:23 - PAD reuse selection"]
    #[inline(always)]
    pub fn pd11_afsel(&mut self) -> Pd11AfselW<'_, Afr4Spec> {
        Pd11AfselW::new(self, 22)
    }
    #[doc = "Bits 24:25 - PAD reuse selection"]
    #[inline(always)]
    pub fn pd12_afsel(&mut self) -> Pd12AfselW<'_, Afr4Spec> {
        Pd12AfselW::new(self, 24)
    }
    #[doc = "Bits 26:27 - PAD reuse selection"]
    #[inline(always)]
    pub fn pd13_afsel(&mut self) -> Pd13AfselW<'_, Afr4Spec> {
        Pd13AfselW::new(self, 26)
    }
    #[doc = "Bits 28:29 - PAD reuse selection"]
    #[inline(always)]
    pub fn pd14_afsel(&mut self) -> Pd14AfselW<'_, Afr4Spec> {
        Pd14AfselW::new(self, 28)
    }
    #[doc = "Bits 30:31 - PAD reuse selection"]
    #[inline(always)]
    pub fn pd15_afsel(&mut self) -> Pd15AfselW<'_, Afr4Spec> {
        Pd15AfselW::new(self, 30)
    }
}
#[doc = "GPIO alternate function register 4\n\nYou can [`read`](crate::Reg::read) this register and get [`afr4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`afr4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Afr4Spec;
impl crate::RegisterSpec for Afr4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`afr4::R`](R) reader structure"]
impl crate::Readable for Afr4Spec {}
#[doc = "`write(|w| ..)` method takes [`afr4::W`](W) writer structure"]
impl crate::Writable for Afr4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AFR4 to value 0"]
impl crate::Resettable for Afr4Spec {}
