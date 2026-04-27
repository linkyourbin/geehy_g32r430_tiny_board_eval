#[doc = "Register `RV` reader"]
pub type R = crate::R<RvSpec>;
#[doc = "Register `RV` writer"]
pub type W = crate::W<RvSpec>;
#[doc = "Field `RV` reader - Awakening Value"]
pub type RvR = crate::FieldReader<u16>;
#[doc = "Field `RV` writer - Awakening Value"]
pub type RvW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Awakening Value"]
    #[inline(always)]
    pub fn rv(&self) -> RvR {
        RvR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Awakening Value"]
    #[inline(always)]
    pub fn rv(&mut self) -> RvW<'_, RvSpec> {
        RvW::new(self, 0)
    }
}
#[doc = "Reload register\n\nYou can [`read`](crate::Reg::read) this register and get [`rv::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rv::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RvSpec;
impl crate::RegisterSpec for RvSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rv::R`](R) reader structure"]
impl crate::Readable for RvSpec {}
#[doc = "`write(|w| ..)` method takes [`rv::W`](W) writer structure"]
impl crate::Writable for RvSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RV to value 0xffff"]
impl crate::Resettable for RvSpec {
    const RESET_VALUE: u32 = 0xffff;
}
