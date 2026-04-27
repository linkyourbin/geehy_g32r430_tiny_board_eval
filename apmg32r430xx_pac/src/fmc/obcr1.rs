#[doc = "Register `OBCR1` reader"]
pub type R = crate::R<Obcr1Spec>;
#[doc = "Register `OBCR1` writer"]
pub type W = crate::W<Obcr1Spec>;
#[doc = "Field `RDPWBF` reader - Read protection switch"]
pub type RdpwbfR = crate::FieldReader;
#[doc = "Field `RDPWBF` writer - Read protection switch"]
pub type RdpwbfW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `IWDTSWWBF` reader - Hardware IWDT switch"]
pub type IwdtswwbfR = crate::BitReader;
#[doc = "Field `IWDTSWWBF` writer - Hardware IWDT switch"]
pub type IwdtswwbfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WWDTSWWBF` reader - Hardware WWDT switch"]
pub type WwdtswwbfR = crate::BitReader;
#[doc = "Field `WWDTSWWBF` writer - Hardware WWDT switch"]
pub type WwdtswwbfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WLOCKENWBF` reader - Write protection enabled."]
pub type WlockenwbfR = crate::BitReader;
#[doc = "Field `WLOCKENWBF` writer - Write protection enabled."]
pub type WlockenwbfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NMIDISWBF` reader - NMI Pin Enable Configuration"]
pub type NmidiswbfR = crate::BitReader;
#[doc = "Field `NMIDISWBF` writer - NMI Pin Enable Configuration"]
pub type NmidiswbfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADTSLOADWBF` reader - ADC TRIM Value Load Configuration"]
pub type AdtsloadwbfR = crate::BitReader;
#[doc = "Field `ADTSLOADWBF` writer - ADC TRIM Value Load Configuration"]
pub type AdtsloadwbfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WLOCKWBF` reader - Write protect sector"]
pub type WlockwbfR = crate::FieldReader;
#[doc = "Field `WLOCKWBF` writer - Write protect sector"]
pub type WlockwbfW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `OPBWREQ` reader - option_byte write request"]
pub type OpbwreqR = crate::BitReader;
#[doc = "Field `OPBWREQ` writer - option_byte write request"]
pub type OpbwreqW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - Read protection switch"]
    #[inline(always)]
    pub fn rdpwbf(&self) -> RdpwbfR {
        RdpwbfR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - Hardware IWDT switch"]
    #[inline(always)]
    pub fn iwdtswwbf(&self) -> IwdtswwbfR {
        IwdtswwbfR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Hardware WWDT switch"]
    #[inline(always)]
    pub fn wwdtswwbf(&self) -> WwdtswwbfR {
        WwdtswwbfR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Write protection enabled."]
    #[inline(always)]
    pub fn wlockenwbf(&self) -> WlockenwbfR {
        WlockenwbfR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - NMI Pin Enable Configuration"]
    #[inline(always)]
    pub fn nmidiswbf(&self) -> NmidiswbfR {
        NmidiswbfR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 15 - ADC TRIM Value Load Configuration"]
    #[inline(always)]
    pub fn adtsloadwbf(&self) -> AdtsloadwbfR {
        AdtsloadwbfR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:23 - Write protect sector"]
    #[inline(always)]
    pub fn wlockwbf(&self) -> WlockwbfR {
        WlockwbfR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - option_byte write request"]
    #[inline(always)]
    pub fn opbwreq(&self) -> OpbwreqR {
        OpbwreqR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Read protection switch"]
    #[inline(always)]
    pub fn rdpwbf(&mut self) -> RdpwbfW<'_, Obcr1Spec> {
        RdpwbfW::new(self, 0)
    }
    #[doc = "Bit 8 - Hardware IWDT switch"]
    #[inline(always)]
    pub fn iwdtswwbf(&mut self) -> IwdtswwbfW<'_, Obcr1Spec> {
        IwdtswwbfW::new(self, 8)
    }
    #[doc = "Bit 9 - Hardware WWDT switch"]
    #[inline(always)]
    pub fn wwdtswwbf(&mut self) -> WwdtswwbfW<'_, Obcr1Spec> {
        WwdtswwbfW::new(self, 9)
    }
    #[doc = "Bit 10 - Write protection enabled."]
    #[inline(always)]
    pub fn wlockenwbf(&mut self) -> WlockenwbfW<'_, Obcr1Spec> {
        WlockenwbfW::new(self, 10)
    }
    #[doc = "Bit 11 - NMI Pin Enable Configuration"]
    #[inline(always)]
    pub fn nmidiswbf(&mut self) -> NmidiswbfW<'_, Obcr1Spec> {
        NmidiswbfW::new(self, 11)
    }
    #[doc = "Bit 15 - ADC TRIM Value Load Configuration"]
    #[inline(always)]
    pub fn adtsloadwbf(&mut self) -> AdtsloadwbfW<'_, Obcr1Spec> {
        AdtsloadwbfW::new(self, 15)
    }
    #[doc = "Bits 16:23 - Write protect sector"]
    #[inline(always)]
    pub fn wlockwbf(&mut self) -> WlockwbfW<'_, Obcr1Spec> {
        WlockwbfW::new(self, 16)
    }
    #[doc = "Bit 24 - option_byte write request"]
    #[inline(always)]
    pub fn opbwreq(&mut self) -> OpbwreqW<'_, Obcr1Spec> {
        OpbwreqW::new(self, 24)
    }
}
#[doc = "Option Byte Control Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`obcr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`obcr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Obcr1Spec;
impl crate::RegisterSpec for Obcr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`obcr1::R`](R) reader structure"]
impl crate::Readable for Obcr1Spec {}
#[doc = "`write(|w| ..)` method takes [`obcr1::W`](W) writer structure"]
impl crate::Writable for Obcr1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OBCR1 to value 0"]
impl crate::Resettable for Obcr1Spec {}
