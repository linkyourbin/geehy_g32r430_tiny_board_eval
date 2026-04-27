#[doc = "Register `RCCR` reader"]
pub type R = crate::R<RccrSpec>;
#[doc = "Register `RCCR` writer"]
pub type W = crate::W<RccrSpec>;
#[doc = "Field `HSION` reader - HSI enable"]
pub type HsionR = crate::BitReader;
#[doc = "Field `HSION` writer - HSI enable"]
pub type HsionW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSIRDY` reader - HSI READY FLAG"]
pub type HsirdyR = crate::BitReader;
#[doc = "Field `HSEON` reader - HSE enable"]
pub type HseonR = crate::BitReader;
#[doc = "Field `HSEON` writer - HSE enable"]
pub type HseonW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSERDY` reader - HSE READY FLAG"]
pub type HserdyR = crate::BitReader;
#[doc = "Field `HSEBYPASS` reader - HSE BYPASS enable"]
pub type HsebypassR = crate::BitReader;
#[doc = "Field `HSEBYPASS` writer - HSE BYPASS enable"]
pub type HsebypassW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSSON` reader - CSS enable"]
pub type CssonR = crate::BitReader;
#[doc = "Field `CSSON` writer - CSS enable"]
pub type CssonW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLLON` reader - PLL enable"]
pub type PllonR = crate::BitReader;
#[doc = "Field `PLLON` writer - PLL enable"]
pub type PllonW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLLRDY` reader - PLL READY FLAG"]
pub type PllrdyR = crate::BitReader;
#[doc = "Field `LSEON` reader - LSE enable"]
pub type LseonR = crate::BitReader;
#[doc = "Field `LSEON` writer - LSE enable"]
pub type LseonW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LSERDY` reader - LSE READY FLAG"]
pub type LserdyR = crate::BitReader;
#[doc = "Field `LSECSSON` reader - LSE clock fault detection enable"]
pub type LsecssonR = crate::BitReader;
#[doc = "Field `LSECSSON` writer - LSE clock fault detection enable"]
pub type LsecssonW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LSEBYPASS` reader - LSE BYPASS enable"]
pub type LsebypassR = crate::BitReader;
#[doc = "Field `LSEBYPASS` writer - LSE BYPASS enable"]
pub type LsebypassW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - HSI enable"]
    #[inline(always)]
    pub fn hsion(&self) -> HsionR {
        HsionR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - HSI READY FLAG"]
    #[inline(always)]
    pub fn hsirdy(&self) -> HsirdyR {
        HsirdyR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - HSE enable"]
    #[inline(always)]
    pub fn hseon(&self) -> HseonR {
        HseonR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - HSE READY FLAG"]
    #[inline(always)]
    pub fn hserdy(&self) -> HserdyR {
        HserdyR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - HSE BYPASS enable"]
    #[inline(always)]
    pub fn hsebypass(&self) -> HsebypassR {
        HsebypassR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - CSS enable"]
    #[inline(always)]
    pub fn csson(&self) -> CssonR {
        CssonR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 16 - PLL enable"]
    #[inline(always)]
    pub fn pllon(&self) -> PllonR {
        PllonR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - PLL READY FLAG"]
    #[inline(always)]
    pub fn pllrdy(&self) -> PllrdyR {
        PllrdyR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 24 - LSE enable"]
    #[inline(always)]
    pub fn lseon(&self) -> LseonR {
        LseonR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - LSE READY FLAG"]
    #[inline(always)]
    pub fn lserdy(&self) -> LserdyR {
        LserdyR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - LSE clock fault detection enable"]
    #[inline(always)]
    pub fn lsecsson(&self) -> LsecssonR {
        LsecssonR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - LSE BYPASS enable"]
    #[inline(always)]
    pub fn lsebypass(&self) -> LsebypassR {
        LsebypassR::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - HSI enable"]
    #[inline(always)]
    pub fn hsion(&mut self) -> HsionW<'_, RccrSpec> {
        HsionW::new(self, 0)
    }
    #[doc = "Bit 8 - HSE enable"]
    #[inline(always)]
    pub fn hseon(&mut self) -> HseonW<'_, RccrSpec> {
        HseonW::new(self, 8)
    }
    #[doc = "Bit 10 - HSE BYPASS enable"]
    #[inline(always)]
    pub fn hsebypass(&mut self) -> HsebypassW<'_, RccrSpec> {
        HsebypassW::new(self, 10)
    }
    #[doc = "Bit 11 - CSS enable"]
    #[inline(always)]
    pub fn csson(&mut self) -> CssonW<'_, RccrSpec> {
        CssonW::new(self, 11)
    }
    #[doc = "Bit 16 - PLL enable"]
    #[inline(always)]
    pub fn pllon(&mut self) -> PllonW<'_, RccrSpec> {
        PllonW::new(self, 16)
    }
    #[doc = "Bit 24 - LSE enable"]
    #[inline(always)]
    pub fn lseon(&mut self) -> LseonW<'_, RccrSpec> {
        LseonW::new(self, 24)
    }
    #[doc = "Bit 26 - LSE clock fault detection enable"]
    #[inline(always)]
    pub fn lsecsson(&mut self) -> LsecssonW<'_, RccrSpec> {
        LsecssonW::new(self, 26)
    }
    #[doc = "Bit 27 - LSE BYPASS enable"]
    #[inline(always)]
    pub fn lsebypass(&mut self) -> LsebypassW<'_, RccrSpec> {
        LsebypassW::new(self, 27)
    }
}
#[doc = "Crystal Oscillator Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rccr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rccr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RccrSpec;
impl crate::RegisterSpec for RccrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rccr::R`](R) reader structure"]
impl crate::Readable for RccrSpec {}
#[doc = "`write(|w| ..)` method takes [`rccr::W`](W) writer structure"]
impl crate::Writable for RccrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RCCR to value 0x03"]
impl crate::Resettable for RccrSpec {
    const RESET_VALUE: u32 = 0x03;
}
