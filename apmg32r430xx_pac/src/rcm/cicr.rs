#[doc = "Register `CICR` reader"]
pub type R = crate::R<CicrSpec>;
#[doc = "Register `CICR` writer"]
pub type W = crate::W<CicrSpec>;
#[doc = "Field `LSIRDYF` reader - LSI-ready flag"]
pub type LsirdyfR = crate::BitReader;
#[doc = "Field `LSIRDYF` writer - LSI-ready flag"]
pub type LsirdyfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LSERDYF` reader - LSE-ready flag"]
pub type LserdyfR = crate::BitReader;
#[doc = "Field `LSERDYF` writer - LSE-ready flag"]
pub type LserdyfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSIRDYF` reader - HSI-ready flag"]
pub type HsirdyfR = crate::BitReader;
#[doc = "Field `HSIRDYF` writer - HSI-ready flag"]
pub type HsirdyfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSERDYF` reader - HSE-ready flag"]
pub type HserdyfR = crate::BitReader;
#[doc = "Field `HSERDYF` writer - HSE-ready flag"]
pub type HserdyfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLLRDYF` reader - PLL-ready flag"]
pub type PllrdyfR = crate::BitReader;
#[doc = "Field `PLLRDYF` writer - PLL-ready flag"]
pub type PllrdyfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSECSSF` reader - HSE - Clock Fault Flag"]
pub type HsecssfR = crate::BitReader;
#[doc = "Field `HSECSSF` writer - HSE - Clock Fault Flag"]
pub type HsecssfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LSECSSF` reader - LSE - Clock Fault Flag"]
pub type LsecssfR = crate::BitReader;
#[doc = "Field `LSECSSF` writer - LSE - Clock Fault Flag"]
pub type LsecssfW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - LSI-ready flag"]
    #[inline(always)]
    pub fn lsirdyf(&self) -> LsirdyfR {
        LsirdyfR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - LSE-ready flag"]
    #[inline(always)]
    pub fn lserdyf(&self) -> LserdyfR {
        LserdyfR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - HSI-ready flag"]
    #[inline(always)]
    pub fn hsirdyf(&self) -> HsirdyfR {
        HsirdyfR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - HSE-ready flag"]
    #[inline(always)]
    pub fn hserdyf(&self) -> HserdyfR {
        HserdyfR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - PLL-ready flag"]
    #[inline(always)]
    pub fn pllrdyf(&self) -> PllrdyfR {
        PllrdyfR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - HSE - Clock Fault Flag"]
    #[inline(always)]
    pub fn hsecssf(&self) -> HsecssfR {
        HsecssfR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - LSE - Clock Fault Flag"]
    #[inline(always)]
    pub fn lsecssf(&self) -> LsecssfR {
        LsecssfR::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LSI-ready flag"]
    #[inline(always)]
    pub fn lsirdyf(&mut self) -> LsirdyfW<'_, CicrSpec> {
        LsirdyfW::new(self, 0)
    }
    #[doc = "Bit 1 - LSE-ready flag"]
    #[inline(always)]
    pub fn lserdyf(&mut self) -> LserdyfW<'_, CicrSpec> {
        LserdyfW::new(self, 1)
    }
    #[doc = "Bit 2 - HSI-ready flag"]
    #[inline(always)]
    pub fn hsirdyf(&mut self) -> HsirdyfW<'_, CicrSpec> {
        HsirdyfW::new(self, 2)
    }
    #[doc = "Bit 3 - HSE-ready flag"]
    #[inline(always)]
    pub fn hserdyf(&mut self) -> HserdyfW<'_, CicrSpec> {
        HserdyfW::new(self, 3)
    }
    #[doc = "Bit 4 - PLL-ready flag"]
    #[inline(always)]
    pub fn pllrdyf(&mut self) -> PllrdyfW<'_, CicrSpec> {
        PllrdyfW::new(self, 4)
    }
    #[doc = "Bit 5 - HSE - Clock Fault Flag"]
    #[inline(always)]
    pub fn hsecssf(&mut self) -> HsecssfW<'_, CicrSpec> {
        HsecssfW::new(self, 5)
    }
    #[doc = "Bit 6 - LSE - Clock Fault Flag"]
    #[inline(always)]
    pub fn lsecssf(&mut self) -> LsecssfW<'_, CicrSpec> {
        LsecssfW::new(self, 6)
    }
}
#[doc = "Interrupt flag control register\n\nYou can [`read`](crate::Reg::read) this register and get [`cicr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cicr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CicrSpec;
impl crate::RegisterSpec for CicrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cicr::R`](R) reader structure"]
impl crate::Readable for CicrSpec {}
#[doc = "`write(|w| ..)` method takes [`cicr::W`](W) writer structure"]
impl crate::Writable for CicrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CICR to value 0"]
impl crate::Resettable for CicrSpec {}
