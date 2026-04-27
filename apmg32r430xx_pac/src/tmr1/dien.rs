#[doc = "Register `DIEN` reader"]
pub type R = crate::R<DienSpec>;
#[doc = "Register `DIEN` writer"]
pub type W = crate::W<DienSpec>;
#[doc = "Field `UIEN` reader - Update interrupt enable"]
pub type UienR = crate::BitReader;
#[doc = "Field `UIEN` writer - Update interrupt enable"]
pub type UienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC1IEN` reader - Capture/Compare 1 interrupt enable"]
pub type Cc1ienR = crate::BitReader;
#[doc = "Field `CC1IEN` writer - Capture/Compare 1 interrupt enable"]
pub type Cc1ienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC2IEN` reader - Capture/Compare 2 interrupt enable"]
pub type Cc2ienR = crate::BitReader;
#[doc = "Field `CC2IEN` writer - Capture/Compare 2 interrupt enable"]
pub type Cc2ienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC3IEN` reader - Capture/Compare 3 interrupt enable"]
pub type Cc3ienR = crate::BitReader;
#[doc = "Field `CC3IEN` writer - Capture/Compare 3 interrupt enable"]
pub type Cc3ienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC4IEN` reader - Capture/Compare 4 interrupt enable"]
pub type Cc4ienR = crate::BitReader;
#[doc = "Field `CC4IEN` writer - Capture/Compare 4 interrupt enable"]
pub type Cc4ienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMIEN` reader - COM interrupt enable"]
pub type ComienR = crate::BitReader;
#[doc = "Field `COMIEN` writer - COM interrupt enable"]
pub type ComienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRGIEN` reader - Trigger interrupt enable"]
pub type TrgienR = crate::BitReader;
#[doc = "Field `TRGIEN` writer - Trigger interrupt enable"]
pub type TrgienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BRKIEN` reader - Break interrupt enable"]
pub type BrkienR = crate::BitReader;
#[doc = "Field `BRKIEN` writer - Break interrupt enable"]
pub type BrkienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UDIEN` reader - Update DMA request enable"]
pub type UdienR = crate::BitReader;
#[doc = "Field `UDIEN` writer - Update DMA request enable"]
pub type UdienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC1DEN` reader - Capture/Compare 1 DMA request enable"]
pub type Cc1denR = crate::BitReader;
#[doc = "Field `CC1DEN` writer - Capture/Compare 1 DMA request enable"]
pub type Cc1denW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC2DEN` reader - Capture/Compare 2 DMA request enable"]
pub type Cc2denR = crate::BitReader;
#[doc = "Field `CC2DEN` writer - Capture/Compare 2 DMA request enable"]
pub type Cc2denW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC3DEN` reader - Capture/Compare 3 DMA request enable"]
pub type Cc3denR = crate::BitReader;
#[doc = "Field `CC3DEN` writer - Capture/Compare 3 DMA request enable"]
pub type Cc3denW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC4DEN` reader - Capture/Compare 4 DMA request enable"]
pub type Cc4denR = crate::BitReader;
#[doc = "Field `CC4DEN` writer - Capture/Compare 4 DMA request enable"]
pub type Cc4denW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMDEN` reader - COM DMA request enable"]
pub type ComdenR = crate::BitReader;
#[doc = "Field `COMDEN` writer - COM DMA request enable"]
pub type ComdenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRGDEN` reader - Trigger DMA request enable"]
pub type TrgdenR = crate::BitReader;
#[doc = "Field `TRGDEN` writer - Trigger DMA request enable"]
pub type TrgdenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Update interrupt enable"]
    #[inline(always)]
    pub fn uien(&self) -> UienR {
        UienR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Capture/Compare 1 interrupt enable"]
    #[inline(always)]
    pub fn cc1ien(&self) -> Cc1ienR {
        Cc1ienR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Capture/Compare 2 interrupt enable"]
    #[inline(always)]
    pub fn cc2ien(&self) -> Cc2ienR {
        Cc2ienR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Capture/Compare 3 interrupt enable"]
    #[inline(always)]
    pub fn cc3ien(&self) -> Cc3ienR {
        Cc3ienR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Capture/Compare 4 interrupt enable"]
    #[inline(always)]
    pub fn cc4ien(&self) -> Cc4ienR {
        Cc4ienR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - COM interrupt enable"]
    #[inline(always)]
    pub fn comien(&self) -> ComienR {
        ComienR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Trigger interrupt enable"]
    #[inline(always)]
    pub fn trgien(&self) -> TrgienR {
        TrgienR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Break interrupt enable"]
    #[inline(always)]
    pub fn brkien(&self) -> BrkienR {
        BrkienR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Update DMA request enable"]
    #[inline(always)]
    pub fn udien(&self) -> UdienR {
        UdienR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Capture/Compare 1 DMA request enable"]
    #[inline(always)]
    pub fn cc1den(&self) -> Cc1denR {
        Cc1denR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Capture/Compare 2 DMA request enable"]
    #[inline(always)]
    pub fn cc2den(&self) -> Cc2denR {
        Cc2denR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Capture/Compare 3 DMA request enable"]
    #[inline(always)]
    pub fn cc3den(&self) -> Cc3denR {
        Cc3denR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Capture/Compare 4 DMA request enable"]
    #[inline(always)]
    pub fn cc4den(&self) -> Cc4denR {
        Cc4denR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - COM DMA request enable"]
    #[inline(always)]
    pub fn comden(&self) -> ComdenR {
        ComdenR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Trigger DMA request enable"]
    #[inline(always)]
    pub fn trgden(&self) -> TrgdenR {
        TrgdenR::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Update interrupt enable"]
    #[inline(always)]
    pub fn uien(&mut self) -> UienW<'_, DienSpec> {
        UienW::new(self, 0)
    }
    #[doc = "Bit 1 - Capture/Compare 1 interrupt enable"]
    #[inline(always)]
    pub fn cc1ien(&mut self) -> Cc1ienW<'_, DienSpec> {
        Cc1ienW::new(self, 1)
    }
    #[doc = "Bit 2 - Capture/Compare 2 interrupt enable"]
    #[inline(always)]
    pub fn cc2ien(&mut self) -> Cc2ienW<'_, DienSpec> {
        Cc2ienW::new(self, 2)
    }
    #[doc = "Bit 3 - Capture/Compare 3 interrupt enable"]
    #[inline(always)]
    pub fn cc3ien(&mut self) -> Cc3ienW<'_, DienSpec> {
        Cc3ienW::new(self, 3)
    }
    #[doc = "Bit 4 - Capture/Compare 4 interrupt enable"]
    #[inline(always)]
    pub fn cc4ien(&mut self) -> Cc4ienW<'_, DienSpec> {
        Cc4ienW::new(self, 4)
    }
    #[doc = "Bit 5 - COM interrupt enable"]
    #[inline(always)]
    pub fn comien(&mut self) -> ComienW<'_, DienSpec> {
        ComienW::new(self, 5)
    }
    #[doc = "Bit 6 - Trigger interrupt enable"]
    #[inline(always)]
    pub fn trgien(&mut self) -> TrgienW<'_, DienSpec> {
        TrgienW::new(self, 6)
    }
    #[doc = "Bit 7 - Break interrupt enable"]
    #[inline(always)]
    pub fn brkien(&mut self) -> BrkienW<'_, DienSpec> {
        BrkienW::new(self, 7)
    }
    #[doc = "Bit 8 - Update DMA request enable"]
    #[inline(always)]
    pub fn udien(&mut self) -> UdienW<'_, DienSpec> {
        UdienW::new(self, 8)
    }
    #[doc = "Bit 9 - Capture/Compare 1 DMA request enable"]
    #[inline(always)]
    pub fn cc1den(&mut self) -> Cc1denW<'_, DienSpec> {
        Cc1denW::new(self, 9)
    }
    #[doc = "Bit 10 - Capture/Compare 2 DMA request enable"]
    #[inline(always)]
    pub fn cc2den(&mut self) -> Cc2denW<'_, DienSpec> {
        Cc2denW::new(self, 10)
    }
    #[doc = "Bit 11 - Capture/Compare 3 DMA request enable"]
    #[inline(always)]
    pub fn cc3den(&mut self) -> Cc3denW<'_, DienSpec> {
        Cc3denW::new(self, 11)
    }
    #[doc = "Bit 12 - Capture/Compare 4 DMA request enable"]
    #[inline(always)]
    pub fn cc4den(&mut self) -> Cc4denW<'_, DienSpec> {
        Cc4denW::new(self, 12)
    }
    #[doc = "Bit 13 - COM DMA request enable"]
    #[inline(always)]
    pub fn comden(&mut self) -> ComdenW<'_, DienSpec> {
        ComdenW::new(self, 13)
    }
    #[doc = "Bit 14 - Trigger DMA request enable"]
    #[inline(always)]
    pub fn trgden(&mut self) -> TrgdenW<'_, DienSpec> {
        TrgdenW::new(self, 14)
    }
}
#[doc = "DMA/Interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`dien::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dien::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DienSpec;
impl crate::RegisterSpec for DienSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dien::R`](R) reader structure"]
impl crate::Readable for DienSpec {}
#[doc = "`write(|w| ..)` method takes [`dien::W`](W) writer structure"]
impl crate::Writable for DienSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DIEN to value 0"]
impl crate::Resettable for DienSpec {}
