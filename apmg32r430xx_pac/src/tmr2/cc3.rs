#[doc = "Register `CC3` reader"]
pub type R = crate::R<Cc3Spec>;
#[doc = "Register `CC3` writer"]
pub type W = crate::W<Cc3Spec>;
#[doc = "Field `CC3_L` reader - Low Capture/Compare value"]
pub type Cc3LR = crate::FieldReader<u16>;
#[doc = "Field `CC3_L` writer - Low Capture/Compare value"]
pub type Cc3LW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `CC3_H` reader - High Capture/Compare value"]
pub type Cc3HR = crate::FieldReader<u16>;
#[doc = "Field `CC3_H` writer - High Capture/Compare value"]
pub type Cc3HW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Low Capture/Compare value"]
    #[inline(always)]
    pub fn cc3_l(&self) -> Cc3LR {
        Cc3LR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - High Capture/Compare value"]
    #[inline(always)]
    pub fn cc3_h(&self) -> Cc3HR {
        Cc3HR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Low Capture/Compare value"]
    #[inline(always)]
    pub fn cc3_l(&mut self) -> Cc3LW<'_, Cc3Spec> {
        Cc3LW::new(self, 0)
    }
    #[doc = "Bits 16:31 - High Capture/Compare value"]
    #[inline(always)]
    pub fn cc3_h(&mut self) -> Cc3HW<'_, Cc3Spec> {
        Cc3HW::new(self, 16)
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
