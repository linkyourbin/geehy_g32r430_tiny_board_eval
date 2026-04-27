#[doc = "Register `AFR1` reader"]
pub type R = crate::R<Afr1Spec>;
#[doc = "Register `AFR1` writer"]
pub type W = crate::W<Afr1Spec>;
#[doc = "Field `PA0_AFSEL` reader - PAD reuse selection"]
pub type Pa0AfselR = crate::FieldReader;
#[doc = "Field `PA0_AFSEL` writer - PAD reuse selection"]
pub type Pa0AfselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PA1_AFSEL` reader - PAD reuse selection"]
pub type Pa1AfselR = crate::FieldReader;
#[doc = "Field `PA1_AFSEL` writer - PAD reuse selection"]
pub type Pa1AfselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PA2_AFSEL` reader - PAD reuse selection"]
pub type Pa2AfselR = crate::FieldReader;
#[doc = "Field `PA2_AFSEL` writer - PAD reuse selection"]
pub type Pa2AfselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PA3_AFSEL` reader - PAD reuse selection"]
pub type Pa3AfselR = crate::FieldReader;
#[doc = "Field `PA3_AFSEL` writer - PAD reuse selection"]
pub type Pa3AfselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PA4_AFSEL` reader - PAD reuse selection"]
pub type Pa4AfselR = crate::FieldReader;
#[doc = "Field `PA4_AFSEL` writer - PAD reuse selection"]
pub type Pa4AfselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PA5_AFSEL` reader - PAD reuse selection"]
pub type Pa5AfselR = crate::FieldReader;
#[doc = "Field `PA5_AFSEL` writer - PAD reuse selection"]
pub type Pa5AfselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PA6_AFSEL` reader - PAD reuse selection"]
pub type Pa6AfselR = crate::FieldReader;
#[doc = "Field `PA6_AFSEL` writer - PAD reuse selection"]
pub type Pa6AfselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PA7_AFSEL` reader - PAD reuse selection"]
pub type Pa7AfselR = crate::FieldReader;
#[doc = "Field `PA7_AFSEL` writer - PAD reuse selection"]
pub type Pa7AfselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PA8_AFSEL` reader - PAD reuse selection"]
pub type Pa8AfselR = crate::FieldReader;
#[doc = "Field `PA8_AFSEL` writer - PAD reuse selection"]
pub type Pa8AfselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PA9_AFSEL` reader - PAD reuse selection"]
pub type Pa9AfselR = crate::FieldReader;
#[doc = "Field `PA9_AFSEL` writer - PAD reuse selection"]
pub type Pa9AfselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - PAD reuse selection"]
    #[inline(always)]
    pub fn pa0_afsel(&self) -> Pa0AfselR {
        Pa0AfselR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - PAD reuse selection"]
    #[inline(always)]
    pub fn pa1_afsel(&self) -> Pa1AfselR {
        Pa1AfselR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - PAD reuse selection"]
    #[inline(always)]
    pub fn pa2_afsel(&self) -> Pa2AfselR {
        Pa2AfselR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - PAD reuse selection"]
    #[inline(always)]
    pub fn pa3_afsel(&self) -> Pa3AfselR {
        Pa3AfselR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - PAD reuse selection"]
    #[inline(always)]
    pub fn pa4_afsel(&self) -> Pa4AfselR {
        Pa4AfselR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - PAD reuse selection"]
    #[inline(always)]
    pub fn pa5_afsel(&self) -> Pa5AfselR {
        Pa5AfselR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - PAD reuse selection"]
    #[inline(always)]
    pub fn pa6_afsel(&self) -> Pa6AfselR {
        Pa6AfselR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - PAD reuse selection"]
    #[inline(always)]
    pub fn pa7_afsel(&self) -> Pa7AfselR {
        Pa7AfselR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - PAD reuse selection"]
    #[inline(always)]
    pub fn pa8_afsel(&self) -> Pa8AfselR {
        Pa8AfselR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - PAD reuse selection"]
    #[inline(always)]
    pub fn pa9_afsel(&self) -> Pa9AfselR {
        Pa9AfselR::new(((self.bits >> 18) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - PAD reuse selection"]
    #[inline(always)]
    pub fn pa0_afsel(&mut self) -> Pa0AfselW<'_, Afr1Spec> {
        Pa0AfselW::new(self, 0)
    }
    #[doc = "Bits 2:3 - PAD reuse selection"]
    #[inline(always)]
    pub fn pa1_afsel(&mut self) -> Pa1AfselW<'_, Afr1Spec> {
        Pa1AfselW::new(self, 2)
    }
    #[doc = "Bits 4:5 - PAD reuse selection"]
    #[inline(always)]
    pub fn pa2_afsel(&mut self) -> Pa2AfselW<'_, Afr1Spec> {
        Pa2AfselW::new(self, 4)
    }
    #[doc = "Bits 6:7 - PAD reuse selection"]
    #[inline(always)]
    pub fn pa3_afsel(&mut self) -> Pa3AfselW<'_, Afr1Spec> {
        Pa3AfselW::new(self, 6)
    }
    #[doc = "Bits 8:9 - PAD reuse selection"]
    #[inline(always)]
    pub fn pa4_afsel(&mut self) -> Pa4AfselW<'_, Afr1Spec> {
        Pa4AfselW::new(self, 8)
    }
    #[doc = "Bits 10:11 - PAD reuse selection"]
    #[inline(always)]
    pub fn pa5_afsel(&mut self) -> Pa5AfselW<'_, Afr1Spec> {
        Pa5AfselW::new(self, 10)
    }
    #[doc = "Bits 12:13 - PAD reuse selection"]
    #[inline(always)]
    pub fn pa6_afsel(&mut self) -> Pa6AfselW<'_, Afr1Spec> {
        Pa6AfselW::new(self, 12)
    }
    #[doc = "Bits 14:15 - PAD reuse selection"]
    #[inline(always)]
    pub fn pa7_afsel(&mut self) -> Pa7AfselW<'_, Afr1Spec> {
        Pa7AfselW::new(self, 14)
    }
    #[doc = "Bits 16:17 - PAD reuse selection"]
    #[inline(always)]
    pub fn pa8_afsel(&mut self) -> Pa8AfselW<'_, Afr1Spec> {
        Pa8AfselW::new(self, 16)
    }
    #[doc = "Bits 18:19 - PAD reuse selection"]
    #[inline(always)]
    pub fn pa9_afsel(&mut self) -> Pa9AfselW<'_, Afr1Spec> {
        Pa9AfselW::new(self, 18)
    }
}
#[doc = "GPIO alternate function register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`afr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`afr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Afr1Spec;
impl crate::RegisterSpec for Afr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`afr1::R`](R) reader structure"]
impl crate::Readable for Afr1Spec {}
#[doc = "`write(|w| ..)` method takes [`afr1::W`](W) writer structure"]
impl crate::Writable for Afr1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AFR1 to value 0"]
impl crate::Resettable for Afr1Spec {}
