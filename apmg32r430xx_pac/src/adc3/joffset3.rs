#[doc = "Register `JOFFSET3` reader"]
pub type R = crate::R<Joffset3Spec>;
#[doc = "Register `JOFFSET3` writer"]
pub type W = crate::W<Joffset3Spec>;
#[doc = "Field `JOFFSET3` reader - ADC offset register 3"]
pub type Joffset3R = crate::FieldReader<u16>;
#[doc = "Field `JOFFSET3` writer - ADC offset register 3"]
pub type Joffset3W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - ADC offset register 3"]
    #[inline(always)]
    pub fn joffset3(&self) -> Joffset3R {
        Joffset3R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - ADC offset register 3"]
    #[inline(always)]
    pub fn joffset3(&mut self) -> Joffset3W<'_, Joffset3Spec> {
        Joffset3W::new(self, 0)
    }
}
#[doc = "ADC offset register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`joffset3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`joffset3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Joffset3Spec;
impl crate::RegisterSpec for Joffset3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`joffset3::R`](R) reader structure"]
impl crate::Readable for Joffset3Spec {}
#[doc = "`write(|w| ..)` method takes [`joffset3::W`](W) writer structure"]
impl crate::Writable for Joffset3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets JOFFSET3 to value 0"]
impl crate::Resettable for Joffset3Spec {}
