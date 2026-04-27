#[doc = "Register `OBSR2` reader"]
pub type R = crate::R<Obsr2Spec>;
#[doc = "Field `BOOTADDR` reader - BOOTADDR"]
pub type BootaddrR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - BOOTADDR"]
    #[inline(always)]
    pub fn bootaddr(&self) -> BootaddrR {
        BootaddrR::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Option Byte Status Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`obsr2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Obsr2Spec;
impl crate::RegisterSpec for Obsr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`obsr2::R`](R) reader structure"]
impl crate::Readable for Obsr2Spec {}
#[doc = "`reset()` method sets OBSR2 to value 0"]
impl crate::Resettable for Obsr2Spec {}
