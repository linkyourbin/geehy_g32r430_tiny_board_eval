#[doc = "Register `STS` reader"]
pub type R = crate::R<StsSpec>;
#[doc = "Register `STS` writer"]
pub type W = crate::W<StsSpec>;
#[doc = "Field `UIFLG` reader - Update interrupt flag"]
pub type UiflgR = crate::BitReader;
#[doc = "Field `UIFLG` writer - Update interrupt flag"]
pub type UiflgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC1IFLG` reader - Capture/compare 1 interrupt flag"]
pub type Cc1iflgR = crate::BitReader;
#[doc = "Field `CC1IFLG` writer - Capture/compare 1 interrupt flag"]
pub type Cc1iflgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC2IFLG` reader - Capture/Compare 2 interrupt flag"]
pub type Cc2iflgR = crate::BitReader;
#[doc = "Field `CC2IFLG` writer - Capture/Compare 2 interrupt flag"]
pub type Cc2iflgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC3IFLG` reader - Capture/Compare 3 interrupt flag"]
pub type Cc3iflgR = crate::BitReader;
#[doc = "Field `CC3IFLG` writer - Capture/Compare 3 interrupt flag"]
pub type Cc3iflgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC4IFLG` reader - Capture/Compare 4 interrupt flag"]
pub type Cc4iflgR = crate::BitReader;
#[doc = "Field `CC4IFLG` writer - Capture/Compare 4 interrupt flag"]
pub type Cc4iflgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMIFLG` reader - COM interrupt flag"]
pub type ComiflgR = crate::BitReader;
#[doc = "Field `COMIFLG` writer - COM interrupt flag"]
pub type ComiflgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRGIFLG` reader - Trigger interrupt flag"]
pub type TrgiflgR = crate::BitReader;
#[doc = "Field `TRGIFLG` writer - Trigger interrupt flag"]
pub type TrgiflgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BRKIFLG` reader - Break interrupt flag"]
pub type BrkiflgR = crate::BitReader;
#[doc = "Field `BRKIFLG` writer - Break interrupt flag"]
pub type BrkiflgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC1RCFLG` reader - Capture/Compare 1 overcapture flag"]
pub type Cc1rcflgR = crate::BitReader;
#[doc = "Field `CC1RCFLG` writer - Capture/Compare 1 overcapture flag"]
pub type Cc1rcflgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC2RCFLG` reader - Capture/compare 2 overcapture flag"]
pub type Cc2rcflgR = crate::BitReader;
#[doc = "Field `CC2RCFLG` writer - Capture/compare 2 overcapture flag"]
pub type Cc2rcflgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC3RCFLG` reader - Capture/Compare 3 overcapture flag"]
pub type Cc3rcflgR = crate::BitReader;
#[doc = "Field `CC3RCFLG` writer - Capture/Compare 3 overcapture flag"]
pub type Cc3rcflgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC4RCFLG` reader - Capture/Compare 4 overcapture flag"]
pub type Cc4rcflgR = crate::BitReader;
#[doc = "Field `CC4RCFLG` writer - Capture/Compare 4 overcapture flag"]
pub type Cc4rcflgW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Update interrupt flag"]
    #[inline(always)]
    pub fn uiflg(&self) -> UiflgR {
        UiflgR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Capture/compare 1 interrupt flag"]
    #[inline(always)]
    pub fn cc1iflg(&self) -> Cc1iflgR {
        Cc1iflgR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Capture/Compare 2 interrupt flag"]
    #[inline(always)]
    pub fn cc2iflg(&self) -> Cc2iflgR {
        Cc2iflgR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Capture/Compare 3 interrupt flag"]
    #[inline(always)]
    pub fn cc3iflg(&self) -> Cc3iflgR {
        Cc3iflgR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Capture/Compare 4 interrupt flag"]
    #[inline(always)]
    pub fn cc4iflg(&self) -> Cc4iflgR {
        Cc4iflgR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - COM interrupt flag"]
    #[inline(always)]
    pub fn comiflg(&self) -> ComiflgR {
        ComiflgR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Trigger interrupt flag"]
    #[inline(always)]
    pub fn trgiflg(&self) -> TrgiflgR {
        TrgiflgR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Break interrupt flag"]
    #[inline(always)]
    pub fn brkiflg(&self) -> BrkiflgR {
        BrkiflgR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - Capture/Compare 1 overcapture flag"]
    #[inline(always)]
    pub fn cc1rcflg(&self) -> Cc1rcflgR {
        Cc1rcflgR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Capture/compare 2 overcapture flag"]
    #[inline(always)]
    pub fn cc2rcflg(&self) -> Cc2rcflgR {
        Cc2rcflgR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Capture/Compare 3 overcapture flag"]
    #[inline(always)]
    pub fn cc3rcflg(&self) -> Cc3rcflgR {
        Cc3rcflgR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Capture/Compare 4 overcapture flag"]
    #[inline(always)]
    pub fn cc4rcflg(&self) -> Cc4rcflgR {
        Cc4rcflgR::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Update interrupt flag"]
    #[inline(always)]
    pub fn uiflg(&mut self) -> UiflgW<'_, StsSpec> {
        UiflgW::new(self, 0)
    }
    #[doc = "Bit 1 - Capture/compare 1 interrupt flag"]
    #[inline(always)]
    pub fn cc1iflg(&mut self) -> Cc1iflgW<'_, StsSpec> {
        Cc1iflgW::new(self, 1)
    }
    #[doc = "Bit 2 - Capture/Compare 2 interrupt flag"]
    #[inline(always)]
    pub fn cc2iflg(&mut self) -> Cc2iflgW<'_, StsSpec> {
        Cc2iflgW::new(self, 2)
    }
    #[doc = "Bit 3 - Capture/Compare 3 interrupt flag"]
    #[inline(always)]
    pub fn cc3iflg(&mut self) -> Cc3iflgW<'_, StsSpec> {
        Cc3iflgW::new(self, 3)
    }
    #[doc = "Bit 4 - Capture/Compare 4 interrupt flag"]
    #[inline(always)]
    pub fn cc4iflg(&mut self) -> Cc4iflgW<'_, StsSpec> {
        Cc4iflgW::new(self, 4)
    }
    #[doc = "Bit 5 - COM interrupt flag"]
    #[inline(always)]
    pub fn comiflg(&mut self) -> ComiflgW<'_, StsSpec> {
        ComiflgW::new(self, 5)
    }
    #[doc = "Bit 6 - Trigger interrupt flag"]
    #[inline(always)]
    pub fn trgiflg(&mut self) -> TrgiflgW<'_, StsSpec> {
        TrgiflgW::new(self, 6)
    }
    #[doc = "Bit 7 - Break interrupt flag"]
    #[inline(always)]
    pub fn brkiflg(&mut self) -> BrkiflgW<'_, StsSpec> {
        BrkiflgW::new(self, 7)
    }
    #[doc = "Bit 9 - Capture/Compare 1 overcapture flag"]
    #[inline(always)]
    pub fn cc1rcflg(&mut self) -> Cc1rcflgW<'_, StsSpec> {
        Cc1rcflgW::new(self, 9)
    }
    #[doc = "Bit 10 - Capture/compare 2 overcapture flag"]
    #[inline(always)]
    pub fn cc2rcflg(&mut self) -> Cc2rcflgW<'_, StsSpec> {
        Cc2rcflgW::new(self, 10)
    }
    #[doc = "Bit 11 - Capture/Compare 3 overcapture flag"]
    #[inline(always)]
    pub fn cc3rcflg(&mut self) -> Cc3rcflgW<'_, StsSpec> {
        Cc3rcflgW::new(self, 11)
    }
    #[doc = "Bit 12 - Capture/Compare 4 overcapture flag"]
    #[inline(always)]
    pub fn cc4rcflg(&mut self) -> Cc4rcflgW<'_, StsSpec> {
        Cc4rcflgW::new(self, 12)
    }
}
#[doc = "status register\n\nYou can [`read`](crate::Reg::read) this register and get [`sts::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sts::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StsSpec;
impl crate::RegisterSpec for StsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sts::R`](R) reader structure"]
impl crate::Readable for StsSpec {}
#[doc = "`write(|w| ..)` method takes [`sts::W`](W) writer structure"]
impl crate::Writable for StsSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets STS to value 0"]
impl crate::Resettable for StsSpec {}
