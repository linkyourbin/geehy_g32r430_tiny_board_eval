#[doc = "Register `DR` reader"]
pub type R = crate::R<DrSpec>;
#[doc = "Field `RDATA` reader - ADC group regular conversion data"]
pub type RdataR = crate::FieldReader<u16>;
#[doc = "Field `ADC2RDATA` reader - The data converted by ADC2 (ADC2 DATA) In dual mode, these bits contain the regular channel data converted by ADC2."]
pub type Adc2rdataR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - ADC group regular conversion data"]
    #[inline(always)]
    pub fn rdata(&self) -> RdataR {
        RdataR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - The data converted by ADC2 (ADC2 DATA) In dual mode, these bits contain the regular channel data converted by ADC2."]
    #[inline(always)]
    pub fn adc2rdata(&self) -> Adc2rdataR {
        Adc2rdataR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "ADC group regular data register\n\nYou can [`read`](crate::Reg::read) this register and get [`dr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DrSpec;
impl crate::RegisterSpec for DrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dr::R`](R) reader structure"]
impl crate::Readable for DrSpec {}
#[doc = "`reset()` method sets DR to value 0"]
impl crate::Resettable for DrSpec {}
