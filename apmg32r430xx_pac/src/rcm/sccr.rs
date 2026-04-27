#[doc = "Register `SCCR` reader"]
pub type R = crate::R<SccrSpec>;
#[doc = "Register `SCCR` writer"]
pub type W = crate::W<SccrSpec>;
#[doc = "Field `SW` reader - System Clock Selection"]
pub type SwR = crate::FieldReader;
#[doc = "Field `SW` writer - System Clock Selection"]
pub type SwW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `HDIV` reader - AHB clock division."]
pub type HdivR = crate::FieldReader;
#[doc = "Field `HDIV` writer - AHB clock division."]
pub type HdivW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PDIV` reader - APB clock division."]
pub type PdivR = crate::FieldReader;
#[doc = "Field `PDIV` writer - APB clock division."]
pub type PdivW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CLKOUTSEL` reader - CLKOUT clock output selection"]
pub type ClkoutselR = crate::FieldReader;
#[doc = "Field `CLKOUTSEL` writer - CLKOUT clock output selection"]
pub type ClkoutselW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CLKOUTEN` reader - Clock output enable"]
pub type ClkoutenR = crate::BitReader;
#[doc = "Field `CLKOUTEN` writer - Clock output enable"]
pub type ClkoutenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLKOUTDIV` reader - CLKOUT clock frequency output"]
pub type ClkoutdivR = crate::FieldReader;
#[doc = "Field `CLKOUTDIV` writer - CLKOUT clock frequency output"]
pub type ClkoutdivW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:1 - System Clock Selection"]
    #[inline(always)]
    pub fn sw(&self) -> SwR {
        SwR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:7 - AHB clock division."]
    #[inline(always)]
    pub fn hdiv(&self) -> HdivR {
        HdivR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:10 - APB clock division."]
    #[inline(always)]
    pub fn pdiv(&self) -> PdivR {
        PdivR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 16:18 - CLKOUT clock output selection"]
    #[inline(always)]
    pub fn clkoutsel(&self) -> ClkoutselR {
        ClkoutselR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 19 - Clock output enable"]
    #[inline(always)]
    pub fn clkouten(&self) -> ClkoutenR {
        ClkoutenR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:22 - CLKOUT clock frequency output"]
    #[inline(always)]
    pub fn clkoutdiv(&self) -> ClkoutdivR {
        ClkoutdivR::new(((self.bits >> 20) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - System Clock Selection"]
    #[inline(always)]
    pub fn sw(&mut self) -> SwW<'_, SccrSpec> {
        SwW::new(self, 0)
    }
    #[doc = "Bits 4:7 - AHB clock division."]
    #[inline(always)]
    pub fn hdiv(&mut self) -> HdivW<'_, SccrSpec> {
        HdivW::new(self, 4)
    }
    #[doc = "Bits 8:10 - APB clock division."]
    #[inline(always)]
    pub fn pdiv(&mut self) -> PdivW<'_, SccrSpec> {
        PdivW::new(self, 8)
    }
    #[doc = "Bits 16:18 - CLKOUT clock output selection"]
    #[inline(always)]
    pub fn clkoutsel(&mut self) -> ClkoutselW<'_, SccrSpec> {
        ClkoutselW::new(self, 16)
    }
    #[doc = "Bit 19 - Clock output enable"]
    #[inline(always)]
    pub fn clkouten(&mut self) -> ClkoutenW<'_, SccrSpec> {
        ClkoutenW::new(self, 19)
    }
    #[doc = "Bits 20:22 - CLKOUT clock frequency output"]
    #[inline(always)]
    pub fn clkoutdiv(&mut self) -> ClkoutdivW<'_, SccrSpec> {
        ClkoutdivW::new(self, 20)
    }
}
#[doc = "Clock Divider Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sccr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sccr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SccrSpec;
impl crate::RegisterSpec for SccrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sccr::R`](R) reader structure"]
impl crate::Readable for SccrSpec {}
#[doc = "`write(|w| ..)` method takes [`sccr::W`](W) writer structure"]
impl crate::Writable for SccrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCCR to value 0"]
impl crate::Resettable for SccrSpec {}
