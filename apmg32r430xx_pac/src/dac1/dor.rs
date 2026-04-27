#[doc = "Register `DOR` reader"]
pub type R = crate::R<DorSpec>;
#[doc = "Field `DOR` reader - DAC channel data output"]
pub type DorR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:9 - DAC channel data output"]
    #[inline(always)]
    pub fn dor(&self) -> DorR {
        DorR::new((self.bits & 0x03ff) as u16)
    }
}
#[doc = "DAC channel data output register\n\nYou can [`read`](crate::Reg::read) this register and get [`dor::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DorSpec;
impl crate::RegisterSpec for DorSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dor::R`](R) reader structure"]
impl crate::Readable for DorSpec {}
#[doc = "`reset()` method sets DOR to value 0"]
impl crate::Resettable for DorSpec {}
