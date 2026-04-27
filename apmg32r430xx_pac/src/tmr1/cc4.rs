#[doc = "Register `CC4` reader"]
pub type R = crate::R<Cc4Spec>;
#[doc = "Register `CC4` writer"]
pub type W = crate::W<Cc4Spec>;
#[doc = "Field `CC4` reader - Capture/Compare value"]
pub type Cc4R = crate::FieldReader<u16>;
#[doc = "Field `CC4` writer - Capture/Compare value"]
pub type Cc4W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Capture/Compare value"]
    #[inline(always)]
    pub fn cc4(&self) -> Cc4R {
        Cc4R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Capture/Compare value"]
    #[inline(always)]
    pub fn cc4(&mut self) -> Cc4W<'_, Cc4Spec> {
        Cc4W::new(self, 0)
    }
}
#[doc = "capture/compare register 4\n\nYou can [`read`](crate::Reg::read) this register and get [`cc4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cc4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cc4Spec;
impl crate::RegisterSpec for Cc4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cc4::R`](R) reader structure"]
impl crate::Readable for Cc4Spec {}
#[doc = "`write(|w| ..)` method takes [`cc4::W`](W) writer structure"]
impl crate::Writable for Cc4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CC4 to value 0"]
impl crate::Resettable for Cc4Spec {}
