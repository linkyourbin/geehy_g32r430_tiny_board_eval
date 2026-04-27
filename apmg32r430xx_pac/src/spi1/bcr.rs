#[doc = "Register `BCR` reader"]
pub type R = crate::R<BcrSpec>;
#[doc = "Register `BCR` writer"]
pub type W = crate::W<BcrSpec>;
#[doc = "Field `BISS_EN` reader - BISS_EN"]
pub type BissEnR = crate::BitReader;
#[doc = "Field `BISS_EN` writer - BISS_EN"]
pub type BissEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SSI_EN` reader - SSI_EN"]
pub type SsiEnR = crate::BitReader;
#[doc = "Field `SSI_EN` writer - SSI_EN"]
pub type SsiEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STOP_EN` reader - STOP_EN"]
pub type StopEnR = crate::BitReader;
#[doc = "Field `STOP_EN` writer - STOP_EN"]
pub type StopEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CDM_IE` reader - CDM_IE"]
pub type CdmIeR = crate::BitReader;
#[doc = "Field `CDM_IE` writer - CDM_IE"]
pub type CdmIeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CDM_DMAEN` reader - CDM_DMAEN"]
pub type CdmDmaenR = crate::BitReader;
#[doc = "Field `CDM_DMAEN` writer - CDM_DMAEN"]
pub type CdmDmaenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - BISS_EN"]
    #[inline(always)]
    pub fn biss_en(&self) -> BissEnR {
        BissEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SSI_EN"]
    #[inline(always)]
    pub fn ssi_en(&self) -> SsiEnR {
        SsiEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - STOP_EN"]
    #[inline(always)]
    pub fn stop_en(&self) -> StopEnR {
        StopEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - CDM_IE"]
    #[inline(always)]
    pub fn cdm_ie(&self) -> CdmIeR {
        CdmIeR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - CDM_DMAEN"]
    #[inline(always)]
    pub fn cdm_dmaen(&self) -> CdmDmaenR {
        CdmDmaenR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - BISS_EN"]
    #[inline(always)]
    pub fn biss_en(&mut self) -> BissEnW<'_, BcrSpec> {
        BissEnW::new(self, 0)
    }
    #[doc = "Bit 1 - SSI_EN"]
    #[inline(always)]
    pub fn ssi_en(&mut self) -> SsiEnW<'_, BcrSpec> {
        SsiEnW::new(self, 1)
    }
    #[doc = "Bit 2 - STOP_EN"]
    #[inline(always)]
    pub fn stop_en(&mut self) -> StopEnW<'_, BcrSpec> {
        StopEnW::new(self, 2)
    }
    #[doc = "Bit 3 - CDM_IE"]
    #[inline(always)]
    pub fn cdm_ie(&mut self) -> CdmIeW<'_, BcrSpec> {
        CdmIeW::new(self, 3)
    }
    #[doc = "Bit 4 - CDM_DMAEN"]
    #[inline(always)]
    pub fn cdm_dmaen(&mut self) -> CdmDmaenW<'_, BcrSpec> {
        CdmDmaenW::new(self, 4)
    }
}
#[doc = "BISS-C Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`bcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BcrSpec;
impl crate::RegisterSpec for BcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bcr::R`](R) reader structure"]
impl crate::Readable for BcrSpec {}
#[doc = "`write(|w| ..)` method takes [`bcr::W`](W) writer structure"]
impl crate::Writable for BcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BCR to value 0"]
impl crate::Resettable for BcrSpec {}
