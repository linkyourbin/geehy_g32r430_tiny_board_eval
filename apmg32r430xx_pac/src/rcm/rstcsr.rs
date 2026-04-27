#[doc = "Register `RSTCSR` reader"]
pub type R = crate::R<RstcsrSpec>;
#[doc = "Register `RSTCSR` writer"]
pub type W = crate::W<RstcsrSpec>;
#[doc = "Field `OPRSTF` reader - Reset caused by opload."]
pub type OprstfR = crate::BitReader;
#[doc = "Field `OPRSTF` writer - Reset caused by opload."]
pub type OprstfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PINRSTF` reader - Reset caused by external RST pin."]
pub type PinrstfR = crate::BitReader;
#[doc = "Field `PINRSTF` writer - Reset caused by external RST pin."]
pub type PinrstfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PORRSTF` reader - Power reset caused by."]
pub type PorrstfR = crate::BitReader;
#[doc = "Field `PORRSTF` writer - Power reset caused by."]
pub type PorrstfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SFTRSTF` reader - Software-induced reset"]
pub type SftrstfR = crate::BitReader;
#[doc = "Field `SFTRSTF` writer - Software-induced reset"]
pub type SftrstfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IWDTRSTF` reader - Reset caused by independent watchdog."]
pub type IwdtrstfR = crate::BitReader;
#[doc = "Field `IWDTRSTF` writer - Reset caused by independent watchdog."]
pub type IwdtrstfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WWDTRSTF` reader - Window watchdog reset."]
pub type WwdtrstfR = crate::BitReader;
#[doc = "Field `WWDTRSTF` writer - Window watchdog reset."]
pub type WwdtrstfW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Reset caused by opload."]
    #[inline(always)]
    pub fn oprstf(&self) -> OprstfR {
        OprstfR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Reset caused by external RST pin."]
    #[inline(always)]
    pub fn pinrstf(&self) -> PinrstfR {
        PinrstfR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Power reset caused by."]
    #[inline(always)]
    pub fn porrstf(&self) -> PorrstfR {
        PorrstfR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Software-induced reset"]
    #[inline(always)]
    pub fn sftrstf(&self) -> SftrstfR {
        SftrstfR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Reset caused by independent watchdog."]
    #[inline(always)]
    pub fn iwdtrstf(&self) -> IwdtrstfR {
        IwdtrstfR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Window watchdog reset."]
    #[inline(always)]
    pub fn wwdtrstf(&self) -> WwdtrstfR {
        WwdtrstfR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Reset caused by opload."]
    #[inline(always)]
    pub fn oprstf(&mut self) -> OprstfW<'_, RstcsrSpec> {
        OprstfW::new(self, 0)
    }
    #[doc = "Bit 1 - Reset caused by external RST pin."]
    #[inline(always)]
    pub fn pinrstf(&mut self) -> PinrstfW<'_, RstcsrSpec> {
        PinrstfW::new(self, 1)
    }
    #[doc = "Bit 2 - Power reset caused by."]
    #[inline(always)]
    pub fn porrstf(&mut self) -> PorrstfW<'_, RstcsrSpec> {
        PorrstfW::new(self, 2)
    }
    #[doc = "Bit 3 - Software-induced reset"]
    #[inline(always)]
    pub fn sftrstf(&mut self) -> SftrstfW<'_, RstcsrSpec> {
        SftrstfW::new(self, 3)
    }
    #[doc = "Bit 4 - Reset caused by independent watchdog."]
    #[inline(always)]
    pub fn iwdtrstf(&mut self) -> IwdtrstfW<'_, RstcsrSpec> {
        IwdtrstfW::new(self, 4)
    }
    #[doc = "Bit 5 - Window watchdog reset."]
    #[inline(always)]
    pub fn wwdtrstf(&mut self) -> WwdtrstfW<'_, RstcsrSpec> {
        WwdtrstfW::new(self, 5)
    }
}
#[doc = "Clock reset status register\n\nYou can [`read`](crate::Reg::read) this register and get [`rstcsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rstcsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RstcsrSpec;
impl crate::RegisterSpec for RstcsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rstcsr::R`](R) reader structure"]
impl crate::Readable for RstcsrSpec {}
#[doc = "`write(|w| ..)` method takes [`rstcsr::W`](W) writer structure"]
impl crate::Writable for RstcsrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RSTCSR to value 0"]
impl crate::Resettable for RstcsrSpec {}
