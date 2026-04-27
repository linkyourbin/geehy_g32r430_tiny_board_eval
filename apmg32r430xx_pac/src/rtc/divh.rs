#[doc = "Register `DIVH` reader"]
pub type R = crate::R<DivhSpec>;
#[doc = "Field `PSCH` reader - RTC prescaler divider register high"]
pub type PschR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - RTC prescaler divider register high"]
    #[inline(always)]
    pub fn psch(&self) -> PschR {
        PschR::new((self.bits & 0x0f) as u8)
    }
}
#[doc = "RTC predivider remainder register High Bit\n\nYou can [`read`](crate::Reg::read) this register and get [`divh::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DivhSpec;
impl crate::RegisterSpec for DivhSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`divh::R`](R) reader structure"]
impl crate::Readable for DivhSpec {}
#[doc = "`reset()` method sets DIVH to value 0"]
impl crate::Resettable for DivhSpec {}
