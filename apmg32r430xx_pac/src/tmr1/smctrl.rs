#[doc = "Register `SMCTRL` reader"]
pub type R = crate::R<SmctrlSpec>;
#[doc = "Register `SMCTRL` writer"]
pub type W = crate::W<SmctrlSpec>;
#[doc = "Field `SMFSEL` reader - Slave mode selection"]
pub type SmfselR = crate::FieldReader;
#[doc = "Field `SMFSEL` writer - Slave mode selection"]
pub type SmfselW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `TRGSEL` reader - Trigger selection"]
pub type TrgselR = crate::FieldReader;
#[doc = "Field `TRGSEL` writer - Trigger selection"]
pub type TrgselW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `MSMEN` reader - Master/Slave mode"]
pub type MsmenR = crate::BitReader;
#[doc = "Field `MSMEN` writer - Master/Slave mode"]
pub type MsmenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ETFCFG` reader - External trigger filter"]
pub type EtfcfgR = crate::FieldReader;
#[doc = "Field `ETFCFG` writer - External trigger filter"]
pub type EtfcfgW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ETPCFG` reader - External trigger prescaler"]
pub type EtpcfgR = crate::FieldReader;
#[doc = "Field `ETPCFG` writer - External trigger prescaler"]
pub type EtpcfgW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ECEN` reader - External clock enable"]
pub type EcenR = crate::BitReader;
#[doc = "Field `ECEN` writer - External clock enable"]
pub type EcenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ETPOL` reader - External trigger polarity"]
pub type EtpolR = crate::BitReader;
#[doc = "Field `ETPOL` writer - External trigger polarity"]
pub type EtpolW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - Slave mode selection"]
    #[inline(always)]
    pub fn smfsel(&self) -> SmfselR {
        SmfselR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - Trigger selection"]
    #[inline(always)]
    pub fn trgsel(&self) -> TrgselR {
        TrgselR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - Master/Slave mode"]
    #[inline(always)]
    pub fn msmen(&self) -> MsmenR {
        MsmenR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:11 - External trigger filter"]
    #[inline(always)]
    pub fn etfcfg(&self) -> EtfcfgR {
        EtfcfgR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:13 - External trigger prescaler"]
    #[inline(always)]
    pub fn etpcfg(&self) -> EtpcfgR {
        EtpcfgR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - External clock enable"]
    #[inline(always)]
    pub fn ecen(&self) -> EcenR {
        EcenR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - External trigger polarity"]
    #[inline(always)]
    pub fn etpol(&self) -> EtpolR {
        EtpolR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Slave mode selection"]
    #[inline(always)]
    pub fn smfsel(&mut self) -> SmfselW<'_, SmctrlSpec> {
        SmfselW::new(self, 0)
    }
    #[doc = "Bits 4:6 - Trigger selection"]
    #[inline(always)]
    pub fn trgsel(&mut self) -> TrgselW<'_, SmctrlSpec> {
        TrgselW::new(self, 4)
    }
    #[doc = "Bit 7 - Master/Slave mode"]
    #[inline(always)]
    pub fn msmen(&mut self) -> MsmenW<'_, SmctrlSpec> {
        MsmenW::new(self, 7)
    }
    #[doc = "Bits 8:11 - External trigger filter"]
    #[inline(always)]
    pub fn etfcfg(&mut self) -> EtfcfgW<'_, SmctrlSpec> {
        EtfcfgW::new(self, 8)
    }
    #[doc = "Bits 12:13 - External trigger prescaler"]
    #[inline(always)]
    pub fn etpcfg(&mut self) -> EtpcfgW<'_, SmctrlSpec> {
        EtpcfgW::new(self, 12)
    }
    #[doc = "Bit 14 - External clock enable"]
    #[inline(always)]
    pub fn ecen(&mut self) -> EcenW<'_, SmctrlSpec> {
        EcenW::new(self, 14)
    }
    #[doc = "Bit 15 - External trigger polarity"]
    #[inline(always)]
    pub fn etpol(&mut self) -> EtpolW<'_, SmctrlSpec> {
        EtpolW::new(self, 15)
    }
}
#[doc = "slave mode control register\n\nYou can [`read`](crate::Reg::read) this register and get [`smctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SmctrlSpec;
impl crate::RegisterSpec for SmctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`smctrl::R`](R) reader structure"]
impl crate::Readable for SmctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`smctrl::W`](W) writer structure"]
impl crate::Writable for SmctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SMCTRL to value 0"]
impl crate::Resettable for SmctrlSpec {}
