#[doc = "Register `RXCRC` reader"]
pub type R = crate::R<RxcrcSpec>;
#[doc = "Field `RXCRC` reader - Rx CRC register"]
pub type RxcrcR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Rx CRC register"]
    #[inline(always)]
    pub fn rxcrc(&self) -> RxcrcR {
        RxcrcR::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "RX CRC register\n\nYou can [`read`](crate::Reg::read) this register and get [`rxcrc::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxcrcSpec;
impl crate::RegisterSpec for RxcrcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxcrc::R`](R) reader structure"]
impl crate::Readable for RxcrcSpec {}
#[doc = "`reset()` method sets RXCRC to value 0"]
impl crate::Resettable for RxcrcSpec {}
