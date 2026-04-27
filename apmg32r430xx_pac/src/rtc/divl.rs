#[doc = "Register `DIVL` reader"]
pub type R = crate::R<DivlSpec>;
#[doc = "Field `PSCL` reader - RTC prescaler divider register Low"]
pub type PsclR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - RTC prescaler divider register Low"]
    #[inline(always)]
    pub fn pscl(&self) -> PsclR {
        PsclR::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "RTC predivider remainder register Low Bit\n\nYou can [`read`](crate::Reg::read) this register and get [`divl::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DivlSpec;
impl crate::RegisterSpec for DivlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`divl::R`](R) reader structure"]
impl crate::Readable for DivlSpec {}
#[doc = "`reset()` method sets DIVL to value 0x8000"]
impl crate::Resettable for DivlSpec {
    const RESET_VALUE: u32 = 0x8000;
}
