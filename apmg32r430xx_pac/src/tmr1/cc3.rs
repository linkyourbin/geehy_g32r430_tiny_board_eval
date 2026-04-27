#[doc = "Register `CC3` reader"]
pub type R = crate::R<Cc3Spec>;
#[doc = "Register `CC3` writer"]
pub type W = crate::W<Cc3Spec>;
#[doc = "Field `CC3` reader - Capture/Compare value"]
pub type Cc3R = crate::FieldReader<u16>;
#[doc = "Field `CC3` writer - Capture/Compare value"]
pub type Cc3W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Capture/Compare value"]
    #[inline(always)]
    pub fn cc3(&self) -> Cc3R {
        Cc3R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Capture/Compare value"]
    #[inline(always)]
    pub fn cc3(&mut self) -> Cc3W<'_, Cc3Spec> {
        Cc3W::new(self, 0)
    }
}
#[doc = "capture/compare register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`cc3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cc3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cc3Spec;
impl crate::RegisterSpec for Cc3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cc3::R`](R) reader structure"]
impl crate::Readable for Cc3Spec {}
#[doc = "`write(|w| ..)` method takes [`cc3::W`](W) writer structure"]
impl crate::Writable for Cc3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CC3 to value 0"]
impl crate::Resettable for Cc3Spec {}
