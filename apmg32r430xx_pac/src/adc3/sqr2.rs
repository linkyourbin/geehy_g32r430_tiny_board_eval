#[doc = "Register `SQR2` reader"]
pub type R = crate::R<Sqr2Spec>;
#[doc = "Register `SQR2` writer"]
pub type W = crate::W<Sqr2Spec>;
#[doc = "Field `SQ8` reader - ADC group regular sequencer rank 8"]
pub type Sq8R = crate::FieldReader;
#[doc = "Field `SQ8` writer - ADC group regular sequencer rank 8"]
pub type Sq8W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SQ9` reader - ADC group regular sequencer rank 9"]
pub type Sq9R = crate::FieldReader;
#[doc = "Field `SQ9` writer - ADC group regular sequencer rank 9"]
pub type Sq9W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SQ10` reader - ADC group regular sequencer rank 10"]
pub type Sq10R = crate::FieldReader;
#[doc = "Field `SQ10` writer - ADC group regular sequencer rank 10"]
pub type Sq10W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SQ11` reader - ADC group regular sequencer rank 11"]
pub type Sq11R = crate::FieldReader;
#[doc = "Field `SQ11` writer - ADC group regular sequencer rank 11"]
pub type Sq11W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SQ12` reader - ADC group regular sequencer rank 12"]
pub type Sq12R = crate::FieldReader;
#[doc = "Field `SQ12` writer - ADC group regular sequencer rank 12"]
pub type Sq12W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SQ13` reader - ADC group regular sequencer rank 13"]
pub type Sq13R = crate::FieldReader;
#[doc = "Field `SQ13` writer - ADC group regular sequencer rank 13"]
pub type Sq13W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SQ14` reader - ADC group regular sequencer rank 14"]
pub type Sq14R = crate::FieldReader;
#[doc = "Field `SQ14` writer - ADC group regular sequencer rank 14"]
pub type Sq14W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - ADC group regular sequencer rank 8"]
    #[inline(always)]
    pub fn sq8(&self) -> Sq8R {
        Sq8R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - ADC group regular sequencer rank 9"]
    #[inline(always)]
    pub fn sq9(&self) -> Sq9R {
        Sq9R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - ADC group regular sequencer rank 10"]
    #[inline(always)]
    pub fn sq10(&self) -> Sq10R {
        Sq10R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - ADC group regular sequencer rank 11"]
    #[inline(always)]
    pub fn sq11(&self) -> Sq11R {
        Sq11R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - ADC group regular sequencer rank 12"]
    #[inline(always)]
    pub fn sq12(&self) -> Sq12R {
        Sq12R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - ADC group regular sequencer rank 13"]
    #[inline(always)]
    pub fn sq13(&self) -> Sq13R {
        Sq13R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - ADC group regular sequencer rank 14"]
    #[inline(always)]
    pub fn sq14(&self) -> Sq14R {
        Sq14R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - ADC group regular sequencer rank 8"]
    #[inline(always)]
    pub fn sq8(&mut self) -> Sq8W<'_, Sqr2Spec> {
        Sq8W::new(self, 0)
    }
    #[doc = "Bits 4:7 - ADC group regular sequencer rank 9"]
    #[inline(always)]
    pub fn sq9(&mut self) -> Sq9W<'_, Sqr2Spec> {
        Sq9W::new(self, 4)
    }
    #[doc = "Bits 8:11 - ADC group regular sequencer rank 10"]
    #[inline(always)]
    pub fn sq10(&mut self) -> Sq10W<'_, Sqr2Spec> {
        Sq10W::new(self, 8)
    }
    #[doc = "Bits 12:15 - ADC group regular sequencer rank 11"]
    #[inline(always)]
    pub fn sq11(&mut self) -> Sq11W<'_, Sqr2Spec> {
        Sq11W::new(self, 12)
    }
    #[doc = "Bits 16:19 - ADC group regular sequencer rank 12"]
    #[inline(always)]
    pub fn sq12(&mut self) -> Sq12W<'_, Sqr2Spec> {
        Sq12W::new(self, 16)
    }
    #[doc = "Bits 20:23 - ADC group regular sequencer rank 13"]
    #[inline(always)]
    pub fn sq13(&mut self) -> Sq13W<'_, Sqr2Spec> {
        Sq13W::new(self, 20)
    }
    #[doc = "Bits 24:27 - ADC group regular sequencer rank 14"]
    #[inline(always)]
    pub fn sq14(&mut self) -> Sq14W<'_, Sqr2Spec> {
        Sq14W::new(self, 24)
    }
}
#[doc = "ADC group regular sequencer register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`sqr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sqr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sqr2Spec;
impl crate::RegisterSpec for Sqr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sqr2::R`](R) reader structure"]
impl crate::Readable for Sqr2Spec {}
#[doc = "`write(|w| ..)` method takes [`sqr2::W`](W) writer structure"]
impl crate::Writable for Sqr2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SQR2 to value 0"]
impl crate::Resettable for Sqr2Spec {}
