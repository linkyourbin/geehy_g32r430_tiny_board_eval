#[doc = "Register `CC1` reader"]
pub type R = crate::R<Cc1Spec>;
#[doc = "Register `CC1` writer"]
pub type W = crate::W<Cc1Spec>;
#[doc = "Field `CC1_L` reader - Low Capture/Compare 1 value"]
pub type Cc1LR = crate::FieldReader<u16>;
#[doc = "Field `CC1_L` writer - Low Capture/Compare 1 value"]
pub type Cc1LW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `CC1_H` reader - High Capture/Compare 1 value"]
pub type Cc1HR = crate::FieldReader<u16>;
#[doc = "Field `CC1_H` writer - High Capture/Compare 1 value"]
pub type Cc1HW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Low Capture/Compare 1 value"]
    #[inline(always)]
    pub fn cc1_l(&self) -> Cc1LR {
        Cc1LR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - High Capture/Compare 1 value"]
    #[inline(always)]
    pub fn cc1_h(&self) -> Cc1HR {
        Cc1HR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Low Capture/Compare 1 value"]
    #[inline(always)]
    pub fn cc1_l(&mut self) -> Cc1LW<'_, Cc1Spec> {
        Cc1LW::new(self, 0)
    }
    #[doc = "Bits 16:31 - High Capture/Compare 1 value"]
    #[inline(always)]
    pub fn cc1_h(&mut self) -> Cc1HW<'_, Cc1Spec> {
        Cc1HW::new(self, 16)
    }
}
#[doc = "capture/compare register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`cc1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cc1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cc1Spec;
impl crate::RegisterSpec for Cc1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cc1::R`](R) reader structure"]
impl crate::Readable for Cc1Spec {}
#[doc = "`write(|w| ..)` method takes [`cc1::W`](W) writer structure"]
impl crate::Writable for Cc1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CC1 to value 0"]
impl crate::Resettable for Cc1Spec {}
