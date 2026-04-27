#[doc = "Register `JOFFSET2` reader"]
pub type R = crate::R<Joffset2Spec>;
#[doc = "Register `JOFFSET2` writer"]
pub type W = crate::W<Joffset2Spec>;
#[doc = "Field `JOFFSET2` reader - ADC offset register 2"]
pub type Joffset2R = crate::FieldReader<u16>;
#[doc = "Field `JOFFSET2` writer - ADC offset register 2"]
pub type Joffset2W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - ADC offset register 2"]
    #[inline(always)]
    pub fn joffset2(&self) -> Joffset2R {
        Joffset2R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - ADC offset register 2"]
    #[inline(always)]
    pub fn joffset2(&mut self) -> Joffset2W<'_, Joffset2Spec> {
        Joffset2W::new(self, 0)
    }
}
#[doc = "ADC offset register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`joffset2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`joffset2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Joffset2Spec;
impl crate::RegisterSpec for Joffset2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`joffset2::R`](R) reader structure"]
impl crate::Readable for Joffset2Spec {}
#[doc = "`write(|w| ..)` method takes [`joffset2::W`](W) writer structure"]
impl crate::Writable for Joffset2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets JOFFSET2 to value 0"]
impl crate::Resettable for Joffset2Spec {}
