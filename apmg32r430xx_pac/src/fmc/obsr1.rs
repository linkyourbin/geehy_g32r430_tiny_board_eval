#[doc = "Register `OBSR1` reader"]
pub type R = crate::R<Obsr1Spec>;
#[doc = "Field `RDP` reader - Read protection switch"]
pub type RdpR = crate::FieldReader;
#[doc = "Field `IWDTSW` reader - Hardware IWDT switch"]
pub type IwdtswR = crate::BitReader;
#[doc = "Field `WWDTSW` reader - Hardware WWDT switch"]
pub type WwdtswR = crate::BitReader;
#[doc = "Field `WLOCKEN` reader - Write protection enabled."]
pub type WlockenR = crate::BitReader;
#[doc = "Field `NMIDIS` reader - NMI Pin Enable Configuration"]
pub type NmidisR = crate::BitReader;
#[doc = "Field `ADTSLOAD` reader - ADC TRIM Value Load Configuration"]
pub type AdtsloadR = crate::BitReader;
#[doc = "Field `WLOCK` reader - Write protect sector"]
pub type WlockR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Read protection switch"]
    #[inline(always)]
    pub fn rdp(&self) -> RdpR {
        RdpR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - Hardware IWDT switch"]
    #[inline(always)]
    pub fn iwdtsw(&self) -> IwdtswR {
        IwdtswR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Hardware WWDT switch"]
    #[inline(always)]
    pub fn wwdtsw(&self) -> WwdtswR {
        WwdtswR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Write protection enabled."]
    #[inline(always)]
    pub fn wlocken(&self) -> WlockenR {
        WlockenR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - NMI Pin Enable Configuration"]
    #[inline(always)]
    pub fn nmidis(&self) -> NmidisR {
        NmidisR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 15 - ADC TRIM Value Load Configuration"]
    #[inline(always)]
    pub fn adtsload(&self) -> AdtsloadR {
        AdtsloadR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:23 - Write protect sector"]
    #[inline(always)]
    pub fn wlock(&self) -> WlockR {
        WlockR::new(((self.bits >> 16) & 0xff) as u8)
    }
}
#[doc = "Option Byte Status Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`obsr1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Obsr1Spec;
impl crate::RegisterSpec for Obsr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`obsr1::R`](R) reader structure"]
impl crate::Readable for Obsr1Spec {}
#[doc = "`reset()` method sets OBSR1 to value 0"]
impl crate::Resettable for Obsr1Spec {}
