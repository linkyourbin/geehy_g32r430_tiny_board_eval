#[doc = "Register `JOFFSET4` reader"]
pub type R = crate::R<Joffset4Spec>;
#[doc = "Register `JOFFSET4` writer"]
pub type W = crate::W<Joffset4Spec>;
#[doc = "Field `JOFFSET4` reader - ADC offset register 4"]
pub type Joffset4R = crate::FieldReader<u16>;
#[doc = "Field `JOFFSET4` writer - ADC offset register 4"]
pub type Joffset4W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - ADC offset register 4"]
    #[inline(always)]
    pub fn joffset4(&self) -> Joffset4R {
        Joffset4R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - ADC offset register 4"]
    #[inline(always)]
    pub fn joffset4(&mut self) -> Joffset4W<'_, Joffset4Spec> {
        Joffset4W::new(self, 0)
    }
}
#[doc = "ADC offset register 4\n\nYou can [`read`](crate::Reg::read) this register and get [`joffset4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`joffset4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Joffset4Spec;
impl crate::RegisterSpec for Joffset4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`joffset4::R`](R) reader structure"]
impl crate::Readable for Joffset4Spec {}
#[doc = "`write(|w| ..)` method takes [`joffset4::W`](W) writer structure"]
impl crate::Writable for Joffset4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets JOFFSET4 to value 0"]
impl crate::Resettable for Joffset4Spec {}
