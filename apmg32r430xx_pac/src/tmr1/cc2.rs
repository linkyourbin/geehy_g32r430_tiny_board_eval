#[doc = "Register `CC2` reader"]
pub type R = crate::R<Cc2Spec>;
#[doc = "Register `CC2` writer"]
pub type W = crate::W<Cc2Spec>;
#[doc = "Field `CC2` reader - Capture/Compare 2 value"]
pub type Cc2R = crate::FieldReader<u16>;
#[doc = "Field `CC2` writer - Capture/Compare 2 value"]
pub type Cc2W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Capture/Compare 2 value"]
    #[inline(always)]
    pub fn cc2(&self) -> Cc2R {
        Cc2R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Capture/Compare 2 value"]
    #[inline(always)]
    pub fn cc2(&mut self) -> Cc2W<'_, Cc2Spec> {
        Cc2W::new(self, 0)
    }
}
#[doc = "capture/compare register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`cc2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cc2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cc2Spec;
impl crate::RegisterSpec for Cc2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cc2::R`](R) reader structure"]
impl crate::Readable for Cc2Spec {}
#[doc = "`write(|w| ..)` method takes [`cc2::W`](W) writer structure"]
impl crate::Writable for Cc2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CC2 to value 0"]
impl crate::Resettable for Cc2Spec {}
