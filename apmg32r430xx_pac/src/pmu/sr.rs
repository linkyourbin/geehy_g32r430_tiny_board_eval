#[doc = "Register `SR` reader"]
pub type R = crate::R<SrSpec>;
#[doc = "Register `SR` writer"]
pub type W = crate::W<SrSpec>;
#[doc = "Field `WKUPF0` reader - The Wake-Up Flag of PC1-WUIO"]
pub type Wkupf0R = crate::BitReader;
#[doc = "Field `WKUPF0` writer - The Wake-Up Flag of PC1-WUIO"]
pub type Wkupf0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WKUPF1` reader - The Wake-Up Flag of PC2-WUIO"]
pub type Wkupf1R = crate::BitReader;
#[doc = "Field `WKUPF1` writer - The Wake-Up Flag of PC2-WUIO"]
pub type Wkupf1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WKUPF2` reader - The Wake-Up Flag of PD0-WUIO"]
pub type Wkupf2R = crate::BitReader;
#[doc = "Field `WKUPF2` writer - The Wake-Up Flag of PD0-WUIO"]
pub type Wkupf2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WKUPF3` reader - The Wake-Up Flag of PD9-WUIO"]
pub type Wkupf3R = crate::BitReader;
#[doc = "Field `WKUPF3` writer - The Wake-Up Flag of PD9-WUIO"]
pub type Wkupf3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SBF` reader - 1: The device has entered standby mode. 0: The device has not entered standby mode."]
pub type SbfR = crate::BitReader;
#[doc = "Field `SBF` writer - 1: The device has entered standby mode. 0: The device has not entered standby mode."]
pub type SbfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EVSWKF` reader - EVS wake-up flag, the EVS function can only be used in standby mode."]
pub type EvswkfR = crate::BitReader;
#[doc = "Field `EVSWKF` writer - EVS wake-up flag, the EVS function can only be used in standby mode."]
pub type EvswkfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PVDO` reader - PVD Output 1: VDD voltage is higher than the set value of pvdthsel 0: VDD voltage is lower than the set value of pvdthsel"]
pub type PvdoR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - The Wake-Up Flag of PC1-WUIO"]
    #[inline(always)]
    pub fn wkupf0(&self) -> Wkupf0R {
        Wkupf0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The Wake-Up Flag of PC2-WUIO"]
    #[inline(always)]
    pub fn wkupf1(&self) -> Wkupf1R {
        Wkupf1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The Wake-Up Flag of PD0-WUIO"]
    #[inline(always)]
    pub fn wkupf2(&self) -> Wkupf2R {
        Wkupf2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The Wake-Up Flag of PD9-WUIO"]
    #[inline(always)]
    pub fn wkupf3(&self) -> Wkupf3R {
        Wkupf3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 1: The device has entered standby mode. 0: The device has not entered standby mode."]
    #[inline(always)]
    pub fn sbf(&self) -> SbfR {
        SbfR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - EVS wake-up flag, the EVS function can only be used in standby mode."]
    #[inline(always)]
    pub fn evswkf(&self) -> EvswkfR {
        EvswkfR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - PVD Output 1: VDD voltage is higher than the set value of pvdthsel 0: VDD voltage is lower than the set value of pvdthsel"]
    #[inline(always)]
    pub fn pvdo(&self) -> PvdoR {
        PvdoR::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - The Wake-Up Flag of PC1-WUIO"]
    #[inline(always)]
    pub fn wkupf0(&mut self) -> Wkupf0W<'_, SrSpec> {
        Wkupf0W::new(self, 0)
    }
    #[doc = "Bit 1 - The Wake-Up Flag of PC2-WUIO"]
    #[inline(always)]
    pub fn wkupf1(&mut self) -> Wkupf1W<'_, SrSpec> {
        Wkupf1W::new(self, 1)
    }
    #[doc = "Bit 2 - The Wake-Up Flag of PD0-WUIO"]
    #[inline(always)]
    pub fn wkupf2(&mut self) -> Wkupf2W<'_, SrSpec> {
        Wkupf2W::new(self, 2)
    }
    #[doc = "Bit 3 - The Wake-Up Flag of PD9-WUIO"]
    #[inline(always)]
    pub fn wkupf3(&mut self) -> Wkupf3W<'_, SrSpec> {
        Wkupf3W::new(self, 3)
    }
    #[doc = "Bit 4 - 1: The device has entered standby mode. 0: The device has not entered standby mode."]
    #[inline(always)]
    pub fn sbf(&mut self) -> SbfW<'_, SrSpec> {
        SbfW::new(self, 4)
    }
    #[doc = "Bit 5 - EVS wake-up flag, the EVS function can only be used in standby mode."]
    #[inline(always)]
    pub fn evswkf(&mut self) -> EvswkfW<'_, SrSpec> {
        EvswkfW::new(self, 5)
    }
}
#[doc = "Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SrSpec;
impl crate::RegisterSpec for SrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr::R`](R) reader structure"]
impl crate::Readable for SrSpec {}
#[doc = "`write(|w| ..)` method takes [`sr::W`](W) writer structure"]
impl crate::Writable for SrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SR to value 0"]
impl crate::Resettable for SrSpec {}
