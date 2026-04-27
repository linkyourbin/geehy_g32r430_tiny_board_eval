#[doc = "Register `CR` reader"]
pub type R = crate::R<CrSpec>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CrSpec>;
#[doc = "Field `LSION` reader - LSI Enable"]
pub type LsionR = crate::BitReader;
#[doc = "Field `LSION` writer - LSI Enable"]
pub type LsionW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LSIRDY` reader - LSI Ready Flag Bit"]
pub type LsirdyR = crate::BitReader;
#[doc = "Field `LSIRDY` writer - LSI Ready Flag Bit"]
pub type LsirdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTCSEL` reader - RTC Clock Selection"]
pub type RtcselR = crate::FieldReader;
#[doc = "Field `RTCSEL` writer - RTC Clock Selection"]
pub type RtcselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RTCEN` reader - RTC clock enable"]
pub type RtcenR = crate::BitReader;
#[doc = "Field `RTCEN` writer - RTC clock enable"]
pub type RtcenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPTMRCEN` reader - LPTMR clock enable"]
pub type LptmrcenR = crate::BitReader;
#[doc = "Field `LPTMRCEN` writer - LPTMR clock enable"]
pub type LptmrcenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NRSTOUTDIS` reader - NRST output pulse to NRST pin enable"]
pub type NrstoutdisR = crate::BitReader;
#[doc = "Field `NRSTOUTDIS` writer - NRST output pulse to NRST pin enable"]
pub type NrstoutdisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BDRST` reader - Reset the bkup area"]
pub type BdrstR = crate::BitReader;
#[doc = "Field `BDRST` writer - Reset the bkup area"]
pub type BdrstW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 5 - LSI Enable"]
    #[inline(always)]
    pub fn lsion(&self) -> LsionR {
        LsionR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - LSI Ready Flag Bit"]
    #[inline(always)]
    pub fn lsirdy(&self) -> LsirdyR {
        LsirdyR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 8:9 - RTC Clock Selection"]
    #[inline(always)]
    pub fn rtcsel(&self) -> RtcselR {
        RtcselR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - RTC clock enable"]
    #[inline(always)]
    pub fn rtcen(&self) -> RtcenR {
        RtcenR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - LPTMR clock enable"]
    #[inline(always)]
    pub fn lptmrcen(&self) -> LptmrcenR {
        LptmrcenR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 13 - NRST output pulse to NRST pin enable"]
    #[inline(always)]
    pub fn nrstoutdis(&self) -> NrstoutdisR {
        NrstoutdisR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 15 - Reset the bkup area"]
    #[inline(always)]
    pub fn bdrst(&self) -> BdrstR {
        BdrstR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - LSI Enable"]
    #[inline(always)]
    pub fn lsion(&mut self) -> LsionW<'_, CrSpec> {
        LsionW::new(self, 5)
    }
    #[doc = "Bit 6 - LSI Ready Flag Bit"]
    #[inline(always)]
    pub fn lsirdy(&mut self) -> LsirdyW<'_, CrSpec> {
        LsirdyW::new(self, 6)
    }
    #[doc = "Bits 8:9 - RTC Clock Selection"]
    #[inline(always)]
    pub fn rtcsel(&mut self) -> RtcselW<'_, CrSpec> {
        RtcselW::new(self, 8)
    }
    #[doc = "Bit 10 - RTC clock enable"]
    #[inline(always)]
    pub fn rtcen(&mut self) -> RtcenW<'_, CrSpec> {
        RtcenW::new(self, 10)
    }
    #[doc = "Bit 11 - LPTMR clock enable"]
    #[inline(always)]
    pub fn lptmrcen(&mut self) -> LptmrcenW<'_, CrSpec> {
        LptmrcenW::new(self, 11)
    }
    #[doc = "Bit 13 - NRST output pulse to NRST pin enable"]
    #[inline(always)]
    pub fn nrstoutdis(&mut self) -> NrstoutdisW<'_, CrSpec> {
        NrstoutdisW::new(self, 13)
    }
    #[doc = "Bit 15 - Reset the bkup area"]
    #[inline(always)]
    pub fn bdrst(&mut self) -> BdrstW<'_, CrSpec> {
        BdrstW::new(self, 15)
    }
}
#[doc = "Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrSpec;
impl crate::RegisterSpec for CrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr::R`](R) reader structure"]
impl crate::Readable for CrSpec {}
#[doc = "`write(|w| ..)` method takes [`cr::W`](W) writer structure"]
impl crate::Writable for CrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CR to value 0x20"]
impl crate::Resettable for CrSpec {
    const RESET_VALUE: u32 = 0x20;
}
