#[doc = "Register `CR` reader"]
pub type R = crate::R<CrSpec>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CrSpec>;
#[doc = "Field `TS_EN` reader - TS Module Enable Bit"]
pub type TsEnR = crate::BitReader;
#[doc = "Field `TS_EN` writer - TS Module Enable Bit"]
pub type TsEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OSR_SEL` reader - Sampling Rate Configuration"]
pub type OsrSelR = crate::FieldReader;
#[doc = "Field `OSR_SEL` writer - Sampling Rate Configuration"]
pub type OsrSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `OVRMOD` reader - Data Overflow Control Bit"]
pub type OvrmodR = crate::BitReader;
#[doc = "Field `OVRMOD` writer - Data Overflow Control Bit"]
pub type OvrmodW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TS_IE` reader - Interrupt enable bit in TS module"]
pub type TsIeR = crate::BitReader;
#[doc = "Field `TS_IE` writer - Interrupt enable bit in TS module"]
pub type TsIeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVRIE` reader - TS Module Data Overflow Interrupt Enable Bit"]
pub type OvrieR = crate::BitReader;
#[doc = "Field `OVRIE` writer - TS Module Data Overflow Interrupt Enable Bit"]
pub type OvrieW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - TS Module Enable Bit"]
    #[inline(always)]
    pub fn ts_en(&self) -> TsEnR {
        TsEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Sampling Rate Configuration"]
    #[inline(always)]
    pub fn osr_sel(&self) -> OsrSelR {
        OsrSelR::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 4 - Data Overflow Control Bit"]
    #[inline(always)]
    pub fn ovrmod(&self) -> OvrmodR {
        OvrmodR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 16 - Interrupt enable bit in TS module"]
    #[inline(always)]
    pub fn ts_ie(&self) -> TsIeR {
        TsIeR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - TS Module Data Overflow Interrupt Enable Bit"]
    #[inline(always)]
    pub fn ovrie(&self) -> OvrieR {
        OvrieR::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TS Module Enable Bit"]
    #[inline(always)]
    pub fn ts_en(&mut self) -> TsEnW<'_, CrSpec> {
        TsEnW::new(self, 0)
    }
    #[doc = "Bits 1:2 - Sampling Rate Configuration"]
    #[inline(always)]
    pub fn osr_sel(&mut self) -> OsrSelW<'_, CrSpec> {
        OsrSelW::new(self, 1)
    }
    #[doc = "Bit 4 - Data Overflow Control Bit"]
    #[inline(always)]
    pub fn ovrmod(&mut self) -> OvrmodW<'_, CrSpec> {
        OvrmodW::new(self, 4)
    }
    #[doc = "Bit 16 - Interrupt enable bit in TS module"]
    #[inline(always)]
    pub fn ts_ie(&mut self) -> TsIeW<'_, CrSpec> {
        TsIeW::new(self, 16)
    }
    #[doc = "Bit 17 - TS Module Data Overflow Interrupt Enable Bit"]
    #[inline(always)]
    pub fn ovrie(&mut self) -> OvrieW<'_, CrSpec> {
        OvrieW::new(self, 17)
    }
}
#[doc = "Control register\n\nYou can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CrSpec {}
