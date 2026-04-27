#[doc = "Register `CIER` reader"]
pub type R = crate::R<CierSpec>;
#[doc = "Register `CIER` writer"]
pub type W = crate::W<CierSpec>;
#[doc = "Field `LSIRDYIE` reader - LSI-ready interrupt enable"]
pub type LsirdyieR = crate::BitReader;
#[doc = "Field `LSIRDYIE` writer - LSI-ready interrupt enable"]
pub type LsirdyieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LSERDYIE` reader - LSE-ready interrupt enable"]
pub type LserdyieR = crate::BitReader;
#[doc = "Field `LSERDYIE` writer - LSE-ready interrupt enable"]
pub type LserdyieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSIRDYIE` reader - HSI-ready interrupt enable"]
pub type HsirdyieR = crate::BitReader;
#[doc = "Field `HSIRDYIE` writer - HSI-ready interrupt enable"]
pub type HsirdyieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSERDYIE` reader - HSE-ready interrupt enable"]
pub type HserdyieR = crate::BitReader;
#[doc = "Field `HSERDYIE` writer - HSE-ready interrupt enable"]
pub type HserdyieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLLRDYIE` reader - PLL-ready interrupt enable"]
pub type PllrdyieR = crate::BitReader;
#[doc = "Field `PLLRDYIE` writer - PLL-ready interrupt enable"]
pub type PllrdyieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSECSSIE` reader - HSE clock fault enable"]
pub type HsecssieR = crate::BitReader;
#[doc = "Field `HSECSSIE` writer - HSE clock fault enable"]
pub type HsecssieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LSECSSIE` reader - LSE clock fault enable"]
pub type LsecssieR = crate::BitReader;
#[doc = "Field `LSECSSIE` writer - LSE clock fault enable"]
pub type LsecssieW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - LSI-ready interrupt enable"]
    #[inline(always)]
    pub fn lsirdyie(&self) -> LsirdyieR {
        LsirdyieR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - LSE-ready interrupt enable"]
    #[inline(always)]
    pub fn lserdyie(&self) -> LserdyieR {
        LserdyieR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - HSI-ready interrupt enable"]
    #[inline(always)]
    pub fn hsirdyie(&self) -> HsirdyieR {
        HsirdyieR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - HSE-ready interrupt enable"]
    #[inline(always)]
    pub fn hserdyie(&self) -> HserdyieR {
        HserdyieR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - PLL-ready interrupt enable"]
    #[inline(always)]
    pub fn pllrdyie(&self) -> PllrdyieR {
        PllrdyieR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - HSE clock fault enable"]
    #[inline(always)]
    pub fn hsecssie(&self) -> HsecssieR {
        HsecssieR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - LSE clock fault enable"]
    #[inline(always)]
    pub fn lsecssie(&self) -> LsecssieR {
        LsecssieR::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LSI-ready interrupt enable"]
    #[inline(always)]
    pub fn lsirdyie(&mut self) -> LsirdyieW<'_, CierSpec> {
        LsirdyieW::new(self, 0)
    }
    #[doc = "Bit 1 - LSE-ready interrupt enable"]
    #[inline(always)]
    pub fn lserdyie(&mut self) -> LserdyieW<'_, CierSpec> {
        LserdyieW::new(self, 1)
    }
    #[doc = "Bit 2 - HSI-ready interrupt enable"]
    #[inline(always)]
    pub fn hsirdyie(&mut self) -> HsirdyieW<'_, CierSpec> {
        HsirdyieW::new(self, 2)
    }
    #[doc = "Bit 3 - HSE-ready interrupt enable"]
    #[inline(always)]
    pub fn hserdyie(&mut self) -> HserdyieW<'_, CierSpec> {
        HserdyieW::new(self, 3)
    }
    #[doc = "Bit 4 - PLL-ready interrupt enable"]
    #[inline(always)]
    pub fn pllrdyie(&mut self) -> PllrdyieW<'_, CierSpec> {
        PllrdyieW::new(self, 4)
    }
    #[doc = "Bit 5 - HSE clock fault enable"]
    #[inline(always)]
    pub fn hsecssie(&mut self) -> HsecssieW<'_, CierSpec> {
        HsecssieW::new(self, 5)
    }
    #[doc = "Bit 6 - LSE clock fault enable"]
    #[inline(always)]
    pub fn lsecssie(&mut self) -> LsecssieW<'_, CierSpec> {
        LsecssieW::new(self, 6)
    }
}
#[doc = "Timer interrupt enable register.\n\nYou can [`read`](crate::Reg::read) this register and get [`cier::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cier::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CierSpec;
impl crate::RegisterSpec for CierSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cier::R`](R) reader structure"]
impl crate::Readable for CierSpec {}
#[doc = "`write(|w| ..)` method takes [`cier::W`](W) writer structure"]
impl crate::Writable for CierSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CIER to value 0"]
impl crate::Resettable for CierSpec {}
