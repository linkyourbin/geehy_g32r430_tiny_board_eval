#[doc = "Register `TXCRC` reader"]
pub type R = crate::R<TxcrcSpec>;
#[doc = "Field `TXCRC` reader - Tx CRC register"]
pub type TxcrcR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Tx CRC register"]
    #[inline(always)]
    pub fn txcrc(&self) -> TxcrcR {
        TxcrcR::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "TX CRC register\n\nYou can [`read`](crate::Reg::read) this register and get [`txcrc::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxcrcSpec;
impl crate::RegisterSpec for TxcrcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txcrc::R`](R) reader structure"]
impl crate::Readable for TxcrcSpec {}
#[doc = "`reset()` method sets TXCRC to value 0"]
impl crate::Resettable for TxcrcSpec {}
