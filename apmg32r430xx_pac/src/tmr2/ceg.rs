#[doc = "Register `CEG` writer"]
pub type W = crate::W<CegSpec>;
#[doc = "Field `UEG` writer - Update generation"]
pub type UegW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC1EG` writer - Capture/compare 1 generation"]
pub type Cc1egW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC2EG` writer - Capture/compare 2 generation"]
pub type Cc2egW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC3EG` writer - Capture/compare 3 generation"]
pub type Cc3egW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC4EG` writer - Capture/compare 4 generation"]
pub type Cc4egW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TEG` writer - Trigger generation"]
pub type TegW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Update generation"]
    #[inline(always)]
    pub fn ueg(&mut self) -> UegW<'_, CegSpec> {
        UegW::new(self, 0)
    }
    #[doc = "Bit 1 - Capture/compare 1 generation"]
    #[inline(always)]
    pub fn cc1eg(&mut self) -> Cc1egW<'_, CegSpec> {
        Cc1egW::new(self, 1)
    }
    #[doc = "Bit 2 - Capture/compare 2 generation"]
    #[inline(always)]
    pub fn cc2eg(&mut self) -> Cc2egW<'_, CegSpec> {
        Cc2egW::new(self, 2)
    }
    #[doc = "Bit 3 - Capture/compare 3 generation"]
    #[inline(always)]
    pub fn cc3eg(&mut self) -> Cc3egW<'_, CegSpec> {
        Cc3egW::new(self, 3)
    }
    #[doc = "Bit 4 - Capture/compare 4 generation"]
    #[inline(always)]
    pub fn cc4eg(&mut self) -> Cc4egW<'_, CegSpec> {
        Cc4egW::new(self, 4)
    }
    #[doc = "Bit 6 - Trigger generation"]
    #[inline(always)]
    pub fn teg(&mut self) -> TegW<'_, CegSpec> {
        TegW::new(self, 6)
    }
}
#[doc = "event generation register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ceg::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CegSpec;
impl crate::RegisterSpec for CegSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ceg::W`](W) writer structure"]
impl crate::Writable for CegSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CEG to value 0"]
impl crate::Resettable for CegSpec {}
