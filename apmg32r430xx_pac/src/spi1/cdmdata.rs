#[doc = "Register `CDMDATA` reader"]
pub type R = crate::R<CdmdataSpec>;
#[doc = "Field `DATA` reader - DATA"]
pub type DataR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - DATA"]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new((self.bits & 1) != 0)
    }
}
#[doc = "CDM Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cdmdata::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CdmdataSpec;
impl crate::RegisterSpec for CdmdataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cdmdata::R`](R) reader structure"]
impl crate::Readable for CdmdataSpec {}
#[doc = "`reset()` method sets CDMDATA to value 0"]
impl crate::Resettable for CdmdataSpec {}
