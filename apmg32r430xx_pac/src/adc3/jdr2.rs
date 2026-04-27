#[doc = "Register `JDR2` reader"]
pub type R = crate::R<Jdr2Spec>;
#[doc = "Field `JDATA` reader - ADC group injected sequencer rank 2 conversion data"]
pub type JdataR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - ADC group injected sequencer rank 2 conversion data"]
    #[inline(always)]
    pub fn jdata(&self) -> JdataR {
        JdataR::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "ADC group injected rank 2 data register\n\nYou can [`read`](crate::Reg::read) this register and get [`jdr2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Jdr2Spec;
impl crate::RegisterSpec for Jdr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`jdr2::R`](R) reader structure"]
impl crate::Readable for Jdr2Spec {}
#[doc = "`reset()` method sets JDR2 to value 0"]
impl crate::Resettable for Jdr2Spec {}
