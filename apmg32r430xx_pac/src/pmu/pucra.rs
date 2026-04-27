#[doc = "Register `PUCRA` reader"]
pub type R = crate::R<PucraSpec>;
#[doc = "Register `PUCRA` writer"]
pub type W = crate::W<PucraSpec>;
#[doc = "Field `WKPU0` reader - In standby mode, when APC is enabled, this position is 1, corresponding to the IO being in a pull-up state."]
pub type Wkpu0R = crate::BitReader;
#[doc = "Field `WKPU0` writer - In standby mode, when APC is enabled, this position is 1, corresponding to the IO being in a pull-up state."]
pub type Wkpu0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WKPU1` reader - In standby mode, when APC is enabled, this position is 1, corresponding to the IO being in a pull-up state."]
pub type Wkpu1R = crate::BitReader;
#[doc = "Field `WKPU1` writer - In standby mode, when APC is enabled, this position is 1, corresponding to the IO being in a pull-up state."]
pub type Wkpu1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WKPU2` reader - In standby mode, when APC is enabled, this position is 1, corresponding to the IO being in a pull-up state."]
pub type Wkpu2R = crate::BitReader;
#[doc = "Field `WKPU2` writer - In standby mode, when APC is enabled, this position is 1, corresponding to the IO being in a pull-up state."]
pub type Wkpu2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WKPU3` reader - In standby mode, when APC is enabled, this position is 1, corresponding to the IO being in a pull-up state."]
pub type Wkpu3R = crate::BitReader;
#[doc = "Field `WKPU3` writer - In standby mode, when APC is enabled, this position is 1, corresponding to the IO being in a pull-up state."]
pub type Wkpu3W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - In standby mode, when APC is enabled, this position is 1, corresponding to the IO being in a pull-up state."]
    #[inline(always)]
    pub fn wkpu0(&self) -> Wkpu0R {
        Wkpu0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - In standby mode, when APC is enabled, this position is 1, corresponding to the IO being in a pull-up state."]
    #[inline(always)]
    pub fn wkpu1(&self) -> Wkpu1R {
        Wkpu1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - In standby mode, when APC is enabled, this position is 1, corresponding to the IO being in a pull-up state."]
    #[inline(always)]
    pub fn wkpu2(&self) -> Wkpu2R {
        Wkpu2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - In standby mode, when APC is enabled, this position is 1, corresponding to the IO being in a pull-up state."]
    #[inline(always)]
    pub fn wkpu3(&self) -> Wkpu3R {
        Wkpu3R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - In standby mode, when APC is enabled, this position is 1, corresponding to the IO being in a pull-up state."]
    #[inline(always)]
    pub fn wkpu0(&mut self) -> Wkpu0W<'_, PucraSpec> {
        Wkpu0W::new(self, 0)
    }
    #[doc = "Bit 1 - In standby mode, when APC is enabled, this position is 1, corresponding to the IO being in a pull-up state."]
    #[inline(always)]
    pub fn wkpu1(&mut self) -> Wkpu1W<'_, PucraSpec> {
        Wkpu1W::new(self, 1)
    }
    #[doc = "Bit 2 - In standby mode, when APC is enabled, this position is 1, corresponding to the IO being in a pull-up state."]
    #[inline(always)]
    pub fn wkpu2(&mut self) -> Wkpu2W<'_, PucraSpec> {
        Wkpu2W::new(self, 2)
    }
    #[doc = "Bit 3 - In standby mode, when APC is enabled, this position is 1, corresponding to the IO being in a pull-up state."]
    #[inline(always)]
    pub fn wkpu3(&mut self) -> Wkpu3W<'_, PucraSpec> {
        Wkpu3W::new(self, 3)
    }
}
#[doc = "Wake up IO pull-up control register\n\nYou can [`read`](crate::Reg::read) this register and get [`pucra::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pucra::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PucraSpec;
impl crate::RegisterSpec for PucraSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pucra::R`](R) reader structure"]
impl crate::Readable for PucraSpec {}
#[doc = "`write(|w| ..)` method takes [`pucra::W`](W) writer structure"]
impl crate::Writable for PucraSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PUCRA to value 0"]
impl crate::Resettable for PucraSpec {}
