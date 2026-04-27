#[doc = "Register `SQR3` reader"]
pub type R = crate::R<Sqr3Spec>;
#[doc = "Register `SQR3` writer"]
pub type W = crate::W<Sqr3Spec>;
#[doc = "Field `SQ15` reader - ADC group regular sequencer rank 15"]
pub type Sq15R = crate::FieldReader;
#[doc = "Field `SQ15` writer - ADC group regular sequencer rank 15"]
pub type Sq15W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SQ16` reader - ADC group regular sequencer rank 16"]
pub type Sq16R = crate::FieldReader;
#[doc = "Field `SQ16` writer - ADC group regular sequencer rank 16"]
pub type Sq16W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - ADC group regular sequencer rank 15"]
    #[inline(always)]
    pub fn sq15(&self) -> Sq15R {
        Sq15R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - ADC group regular sequencer rank 16"]
    #[inline(always)]
    pub fn sq16(&self) -> Sq16R {
        Sq16R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - ADC group regular sequencer rank 15"]
    #[inline(always)]
    pub fn sq15(&mut self) -> Sq15W<'_, Sqr3Spec> {
        Sq15W::new(self, 0)
    }
    #[doc = "Bits 4:7 - ADC group regular sequencer rank 16"]
    #[inline(always)]
    pub fn sq16(&mut self) -> Sq16W<'_, Sqr3Spec> {
        Sq16W::new(self, 4)
    }
}
#[doc = "ADC group regular sequencer register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`sqr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sqr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sqr3Spec;
impl crate::RegisterSpec for Sqr3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sqr3::R`](R) reader structure"]
impl crate::Readable for Sqr3Spec {}
#[doc = "`write(|w| ..)` method takes [`sqr3::W`](W) writer structure"]
impl crate::Writable for Sqr3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SQR3 to value 0"]
impl crate::Resettable for Sqr3Spec {}
