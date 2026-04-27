#[doc = "Register `CCEN` reader"]
pub type R = crate::R<CcenSpec>;
#[doc = "Register `CCEN` writer"]
pub type W = crate::W<CcenSpec>;
#[doc = "Field `CC1EN` reader - Capture/Compare 1 output enable"]
pub type Cc1enR = crate::BitReader;
#[doc = "Field `CC1EN` writer - Capture/Compare 1 output enable"]
pub type Cc1enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC1POL` reader - Capture/Compare 1 output Polarity"]
pub type Cc1polR = crate::BitReader;
#[doc = "Field `CC1POL` writer - Capture/Compare 1 output Polarity"]
pub type Cc1polW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC1NEN` reader - Capture/Compare 1 complementary output enable"]
pub type Cc1nenR = crate::BitReader;
#[doc = "Field `CC1NEN` writer - Capture/Compare 1 complementary output enable"]
pub type Cc1nenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC1NPOL` reader - Capture/Compare 1 output Polarity"]
pub type Cc1npolR = crate::BitReader;
#[doc = "Field `CC1NPOL` writer - Capture/Compare 1 output Polarity"]
pub type Cc1npolW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC2EN` reader - Capture/Compare 2 output enable"]
pub type Cc2enR = crate::BitReader;
#[doc = "Field `CC2EN` writer - Capture/Compare 2 output enable"]
pub type Cc2enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC2POL` reader - Capture/Compare 2 output Polarity"]
pub type Cc2polR = crate::BitReader;
#[doc = "Field `CC2POL` writer - Capture/Compare 2 output Polarity"]
pub type Cc2polW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC2NEN` reader - Capture/Compare 2 complementary output enable"]
pub type Cc2nenR = crate::BitReader;
#[doc = "Field `CC2NEN` writer - Capture/Compare 2 complementary output enable"]
pub type Cc2nenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC2NPOL` reader - Capture/Compare 2 output Polarity"]
pub type Cc2npolR = crate::BitReader;
#[doc = "Field `CC2NPOL` writer - Capture/Compare 2 output Polarity"]
pub type Cc2npolW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC3EN` reader - Capture/Compare 3 output enable"]
pub type Cc3enR = crate::BitReader;
#[doc = "Field `CC3EN` writer - Capture/Compare 3 output enable"]
pub type Cc3enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC3POL` reader - Capture/Compare 3 output Polarity"]
pub type Cc3polR = crate::BitReader;
#[doc = "Field `CC3POL` writer - Capture/Compare 3 output Polarity"]
pub type Cc3polW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC3NEN` reader - Capture/Compare 3 complementary output enable"]
pub type Cc3nenR = crate::BitReader;
#[doc = "Field `CC3NEN` writer - Capture/Compare 3 complementary output enable"]
pub type Cc3nenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC3NPOL` reader - Capture/Compare 3 output Polarity"]
pub type Cc3npolR = crate::BitReader;
#[doc = "Field `CC3NPOL` writer - Capture/Compare 3 output Polarity"]
pub type Cc3npolW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC4EN` reader - Capture/Compare 4 output enable"]
pub type Cc4enR = crate::BitReader;
#[doc = "Field `CC4EN` writer - Capture/Compare 4 output enable"]
pub type Cc4enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC4POL` reader - Capture/Compare 3 output Polarity"]
pub type Cc4polR = crate::BitReader;
#[doc = "Field `CC4POL` writer - Capture/Compare 3 output Polarity"]
pub type Cc4polW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Capture/Compare 1 output enable"]
    #[inline(always)]
    pub fn cc1en(&self) -> Cc1enR {
        Cc1enR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Capture/Compare 1 output Polarity"]
    #[inline(always)]
    pub fn cc1pol(&self) -> Cc1polR {
        Cc1polR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Capture/Compare 1 complementary output enable"]
    #[inline(always)]
    pub fn cc1nen(&self) -> Cc1nenR {
        Cc1nenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Capture/Compare 1 output Polarity"]
    #[inline(always)]
    pub fn cc1npol(&self) -> Cc1npolR {
        Cc1npolR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Capture/Compare 2 output enable"]
    #[inline(always)]
    pub fn cc2en(&self) -> Cc2enR {
        Cc2enR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Capture/Compare 2 output Polarity"]
    #[inline(always)]
    pub fn cc2pol(&self) -> Cc2polR {
        Cc2polR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Capture/Compare 2 complementary output enable"]
    #[inline(always)]
    pub fn cc2nen(&self) -> Cc2nenR {
        Cc2nenR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Capture/Compare 2 output Polarity"]
    #[inline(always)]
    pub fn cc2npol(&self) -> Cc2npolR {
        Cc2npolR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Capture/Compare 3 output enable"]
    #[inline(always)]
    pub fn cc3en(&self) -> Cc3enR {
        Cc3enR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Capture/Compare 3 output Polarity"]
    #[inline(always)]
    pub fn cc3pol(&self) -> Cc3polR {
        Cc3polR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Capture/Compare 3 complementary output enable"]
    #[inline(always)]
    pub fn cc3nen(&self) -> Cc3nenR {
        Cc3nenR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Capture/Compare 3 output Polarity"]
    #[inline(always)]
    pub fn cc3npol(&self) -> Cc3npolR {
        Cc3npolR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Capture/Compare 4 output enable"]
    #[inline(always)]
    pub fn cc4en(&self) -> Cc4enR {
        Cc4enR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Capture/Compare 3 output Polarity"]
    #[inline(always)]
    pub fn cc4pol(&self) -> Cc4polR {
        Cc4polR::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Capture/Compare 1 output enable"]
    #[inline(always)]
    pub fn cc1en(&mut self) -> Cc1enW<'_, CcenSpec> {
        Cc1enW::new(self, 0)
    }
    #[doc = "Bit 1 - Capture/Compare 1 output Polarity"]
    #[inline(always)]
    pub fn cc1pol(&mut self) -> Cc1polW<'_, CcenSpec> {
        Cc1polW::new(self, 1)
    }
    #[doc = "Bit 2 - Capture/Compare 1 complementary output enable"]
    #[inline(always)]
    pub fn cc1nen(&mut self) -> Cc1nenW<'_, CcenSpec> {
        Cc1nenW::new(self, 2)
    }
    #[doc = "Bit 3 - Capture/Compare 1 output Polarity"]
    #[inline(always)]
    pub fn cc1npol(&mut self) -> Cc1npolW<'_, CcenSpec> {
        Cc1npolW::new(self, 3)
    }
    #[doc = "Bit 4 - Capture/Compare 2 output enable"]
    #[inline(always)]
    pub fn cc2en(&mut self) -> Cc2enW<'_, CcenSpec> {
        Cc2enW::new(self, 4)
    }
    #[doc = "Bit 5 - Capture/Compare 2 output Polarity"]
    #[inline(always)]
    pub fn cc2pol(&mut self) -> Cc2polW<'_, CcenSpec> {
        Cc2polW::new(self, 5)
    }
    #[doc = "Bit 6 - Capture/Compare 2 complementary output enable"]
    #[inline(always)]
    pub fn cc2nen(&mut self) -> Cc2nenW<'_, CcenSpec> {
        Cc2nenW::new(self, 6)
    }
    #[doc = "Bit 7 - Capture/Compare 2 output Polarity"]
    #[inline(always)]
    pub fn cc2npol(&mut self) -> Cc2npolW<'_, CcenSpec> {
        Cc2npolW::new(self, 7)
    }
    #[doc = "Bit 8 - Capture/Compare 3 output enable"]
    #[inline(always)]
    pub fn cc3en(&mut self) -> Cc3enW<'_, CcenSpec> {
        Cc3enW::new(self, 8)
    }
    #[doc = "Bit 9 - Capture/Compare 3 output Polarity"]
    #[inline(always)]
    pub fn cc3pol(&mut self) -> Cc3polW<'_, CcenSpec> {
        Cc3polW::new(self, 9)
    }
    #[doc = "Bit 10 - Capture/Compare 3 complementary output enable"]
    #[inline(always)]
    pub fn cc3nen(&mut self) -> Cc3nenW<'_, CcenSpec> {
        Cc3nenW::new(self, 10)
    }
    #[doc = "Bit 11 - Capture/Compare 3 output Polarity"]
    #[inline(always)]
    pub fn cc3npol(&mut self) -> Cc3npolW<'_, CcenSpec> {
        Cc3npolW::new(self, 11)
    }
    #[doc = "Bit 12 - Capture/Compare 4 output enable"]
    #[inline(always)]
    pub fn cc4en(&mut self) -> Cc4enW<'_, CcenSpec> {
        Cc4enW::new(self, 12)
    }
    #[doc = "Bit 13 - Capture/Compare 3 output Polarity"]
    #[inline(always)]
    pub fn cc4pol(&mut self) -> Cc4polW<'_, CcenSpec> {
        Cc4polW::new(self, 13)
    }
}
#[doc = "capture/compare enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`ccen::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccen::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CcenSpec;
impl crate::RegisterSpec for CcenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccen::R`](R) reader structure"]
impl crate::Readable for CcenSpec {}
#[doc = "`write(|w| ..)` method takes [`ccen::W`](W) writer structure"]
impl crate::Writable for CcenSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CCEN to value 0"]
impl crate::Resettable for CcenSpec {}
