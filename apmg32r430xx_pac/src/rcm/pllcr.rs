#[doc = "Register `PLLCR` reader"]
pub type R = crate::R<PllcrSpec>;
#[doc = "Register `PLLCR` writer"]
pub type W = crate::W<PllcrSpec>;
#[doc = "Field `PLLSRC` reader - PLL Input Clock Selection"]
pub type PllsrcR = crate::FieldReader;
#[doc = "Field `PLLSRC` writer - PLL Input Clock Selection"]
pub type PllsrcW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PLLP` reader - PLL Input Clock Division Control"]
pub type PllpR = crate::FieldReader;
#[doc = "Field `PLLP` writer - PLL Input Clock Division Control"]
pub type PllpW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PLLMUL` reader - PLL Frequency Multiplication Coefficient"]
pub type PllmulR = crate::FieldReader;
#[doc = "Field `PLLMUL` writer - PLL Frequency Multiplication Coefficient"]
pub type PllmulW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:1 - PLL Input Clock Selection"]
    #[inline(always)]
    pub fn pllsrc(&self) -> PllsrcR {
        PllsrcR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 8:11 - PLL Input Clock Division Control"]
    #[inline(always)]
    pub fn pllp(&self) -> PllpR {
        PllpR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:21 - PLL Frequency Multiplication Coefficient"]
    #[inline(always)]
    pub fn pllmul(&self) -> PllmulR {
        PllmulR::new(((self.bits >> 16) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - PLL Input Clock Selection"]
    #[inline(always)]
    pub fn pllsrc(&mut self) -> PllsrcW<'_, PllcrSpec> {
        PllsrcW::new(self, 0)
    }
    #[doc = "Bits 8:11 - PLL Input Clock Division Control"]
    #[inline(always)]
    pub fn pllp(&mut self) -> PllpW<'_, PllcrSpec> {
        PllpW::new(self, 8)
    }
    #[doc = "Bits 16:21 - PLL Frequency Multiplication Coefficient"]
    #[inline(always)]
    pub fn pllmul(&mut self) -> PllmulW<'_, PllcrSpec> {
        PllmulW::new(self, 16)
    }
}
#[doc = "PLL control register\n\nYou can [`read`](crate::Reg::read) this register and get [`pllcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pllcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PllcrSpec;
impl crate::RegisterSpec for PllcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pllcr::R`](R) reader structure"]
impl crate::Readable for PllcrSpec {}
#[doc = "`write(|w| ..)` method takes [`pllcr::W`](W) writer structure"]
impl crate::Writable for PllcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PLLCR to value 0"]
impl crate::Resettable for PllcrSpec {}
