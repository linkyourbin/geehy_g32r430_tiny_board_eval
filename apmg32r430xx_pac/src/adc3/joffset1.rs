#[doc = "Register `JOFFSET1` reader"]
pub type R = crate::R<Joffset1Spec>;
#[doc = "Register `JOFFSET1` writer"]
pub type W = crate::W<Joffset1Spec>;
#[doc = "Field `JOFFSET1` reader - ADC offset register 1"]
pub type Joffset1R = crate::FieldReader<u16>;
#[doc = "Field `JOFFSET1` writer - ADC offset register 1"]
pub type Joffset1W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - ADC offset register 1"]
    #[inline(always)]
    pub fn joffset1(&self) -> Joffset1R {
        Joffset1R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - ADC offset register 1"]
    #[inline(always)]
    pub fn joffset1(&mut self) -> Joffset1W<'_, Joffset1Spec> {
        Joffset1W::new(self, 0)
    }
}
#[doc = "ADC offset register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`joffset1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`joffset1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Joffset1Spec;
impl crate::RegisterSpec for Joffset1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`joffset1::R`](R) reader structure"]
impl crate::Readable for Joffset1Spec {}
#[doc = "`write(|w| ..)` method takes [`joffset1::W`](W) writer structure"]
impl crate::Writable for Joffset1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets JOFFSET1 to value 0"]
impl crate::Resettable for Joffset1Spec {}
