#[doc = "Register `CFGR2` reader"]
pub type R = crate::R<Cfgr2Spec>;
#[doc = "Register `CFGR2` writer"]
pub type W = crate::W<Cfgr2Spec>;
#[doc = "Field `ROVSE` reader - REGULAR OVERSAMPLING ENABLE"]
pub type RovseR = crate::BitReader;
#[doc = "Field `ROVSE` writer - REGULAR OVERSAMPLING ENABLE"]
pub type RovseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `JOVSE` reader - INJECTED OVERSAMPLING ENABLE"]
pub type JovseR = crate::BitReader;
#[doc = "Field `JOVSE` writer - INJECTED OVERSAMPLING ENABLE"]
pub type JovseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVSR` reader - OVERSAMPLING RATIO"]
pub type OvsrR = crate::FieldReader;
#[doc = "Field `OVSR` writer - OVERSAMPLING RATIO"]
pub type OvsrW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `OVSS` reader - OVERSAMPLING SHIFT"]
pub type OvssR = crate::FieldReader;
#[doc = "Field `OVSS` writer - OVERSAMPLING SHIFT"]
pub type OvssW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `TROVS` reader - TRIGGERED REGULAR OVERSAMPLING"]
pub type TrovsR = crate::BitReader;
#[doc = "Field `TROVS` writer - TRIGGERED REGULAR OVERSAMPLING"]
pub type TrovsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ROVSM` reader - REGULAR OVERSAMPLING MODE"]
pub type RovsmR = crate::BitReader;
#[doc = "Field `ROVSM` writer - REGULAR OVERSAMPLING MODE"]
pub type RovsmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AWDFILT` reader - ANALOG WATCHDOG FILTERING PARAMETER"]
pub type AwdfiltR = crate::FieldReader;
#[doc = "Field `AWDFILT` writer - ANALOG WATCHDOG FILTERING PARAMETER"]
pub type AwdfiltW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `DIFSEL` reader - ADC Single-End/Differential Mode Control Signal"]
pub type DifselR = crate::BitReader;
#[doc = "Field `DIFSEL` writer - ADC Single-End/Differential Mode Control Signal"]
pub type DifselW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - REGULAR OVERSAMPLING ENABLE"]
    #[inline(always)]
    pub fn rovse(&self) -> RovseR {
        RovseR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - INJECTED OVERSAMPLING ENABLE"]
    #[inline(always)]
    pub fn jovse(&self) -> JovseR {
        JovseR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - OVERSAMPLING RATIO"]
    #[inline(always)]
    pub fn ovsr(&self) -> OvsrR {
        OvsrR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 6:8 - OVERSAMPLING SHIFT"]
    #[inline(always)]
    pub fn ovss(&self) -> OvssR {
        OvssR::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bit 9 - TRIGGERED REGULAR OVERSAMPLING"]
    #[inline(always)]
    pub fn trovs(&self) -> TrovsR {
        TrovsR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - REGULAR OVERSAMPLING MODE"]
    #[inline(always)]
    pub fn rovsm(&self) -> RovsmR {
        RovsmR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:13 - ANALOG WATCHDOG FILTERING PARAMETER"]
    #[inline(always)]
    pub fn awdfilt(&self) -> AwdfiltR {
        AwdfiltR::new(((self.bits >> 11) & 7) as u8)
    }
    #[doc = "Bit 14 - ADC Single-End/Differential Mode Control Signal"]
    #[inline(always)]
    pub fn difsel(&self) -> DifselR {
        DifselR::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - REGULAR OVERSAMPLING ENABLE"]
    #[inline(always)]
    pub fn rovse(&mut self) -> RovseW<'_, Cfgr2Spec> {
        RovseW::new(self, 0)
    }
    #[doc = "Bit 1 - INJECTED OVERSAMPLING ENABLE"]
    #[inline(always)]
    pub fn jovse(&mut self) -> JovseW<'_, Cfgr2Spec> {
        JovseW::new(self, 1)
    }
    #[doc = "Bits 2:3 - OVERSAMPLING RATIO"]
    #[inline(always)]
    pub fn ovsr(&mut self) -> OvsrW<'_, Cfgr2Spec> {
        OvsrW::new(self, 2)
    }
    #[doc = "Bits 6:8 - OVERSAMPLING SHIFT"]
    #[inline(always)]
    pub fn ovss(&mut self) -> OvssW<'_, Cfgr2Spec> {
        OvssW::new(self, 6)
    }
    #[doc = "Bit 9 - TRIGGERED REGULAR OVERSAMPLING"]
    #[inline(always)]
    pub fn trovs(&mut self) -> TrovsW<'_, Cfgr2Spec> {
        TrovsW::new(self, 9)
    }
    #[doc = "Bit 10 - REGULAR OVERSAMPLING MODE"]
    #[inline(always)]
    pub fn rovsm(&mut self) -> RovsmW<'_, Cfgr2Spec> {
        RovsmW::new(self, 10)
    }
    #[doc = "Bits 11:13 - ANALOG WATCHDOG FILTERING PARAMETER"]
    #[inline(always)]
    pub fn awdfilt(&mut self) -> AwdfiltW<'_, Cfgr2Spec> {
        AwdfiltW::new(self, 11)
    }
    #[doc = "Bit 14 - ADC Single-End/Differential Mode Control Signal"]
    #[inline(always)]
    pub fn difsel(&mut self) -> DifselW<'_, Cfgr2Spec> {
        DifselW::new(self, 14)
    }
}
#[doc = "ADC configuration register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`cfgr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfgr2Spec;
impl crate::RegisterSpec for Cfgr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfgr2::R`](R) reader structure"]
impl crate::Readable for Cfgr2Spec {}
#[doc = "`write(|w| ..)` method takes [`cfgr2::W`](W) writer structure"]
impl crate::Writable for Cfgr2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CFGR2 to value 0"]
impl crate::Resettable for Cfgr2Spec {}
