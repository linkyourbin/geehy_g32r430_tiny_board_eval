#[doc = "Register `PDCRA` reader"]
pub type R = crate::R<PdcraSpec>;
#[doc = "Register `PDCRA` writer"]
pub type W = crate::W<PdcraSpec>;
#[doc = "Field `WKPU0` reader - In standby mode, when APC is enabled, this position is 1, corresponding to the IO being in a pull-down state."]
pub type Wkpu0R = crate::BitReader;
#[doc = "Field `WKPU0` writer - In standby mode, when APC is enabled, this position is 1, corresponding to the IO being in a pull-down state."]
pub type Wkpu0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WKPU1` reader - In standby mode, when APC is enabled, this position is 1, corresponding to the IO being in a pull-down state."]
pub type Wkpu1R = crate::BitReader;
#[doc = "Field `WKPU1` writer - In standby mode, when APC is enabled, this position is 1, corresponding to the IO being in a pull-down state."]
pub type Wkpu1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WKPU2` reader - In standby mode, when APC is enabled, this position is 1, corresponding to the IO being in a pull-down state."]
pub type Wkpu2R = crate::BitReader;
#[doc = "Field `WKPU2` writer - In standby mode, when APC is enabled, this position is 1, corresponding to the IO being in a pull-down state."]
pub type Wkpu2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WKPU3` reader - In standby mode, when APC is enabled, this position is 1, corresponding to the IO being in a pull-down state."]
pub type Wkpu3R = crate::BitReader;
#[doc = "Field `WKPU3` writer - In standby mode, when APC is enabled, this position is 1, corresponding to the IO being in a pull-down state."]
pub type Wkpu3W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - In standby mode, when APC is enabled, this position is 1, corresponding to the IO being in a pull-down state."]
    #[inline(always)]
    pub fn wkpu0(&self) -> Wkpu0R {
        Wkpu0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - In standby mode, when APC is enabled, this position is 1, corresponding to the IO being in a pull-down state."]
    #[inline(always)]
    pub fn wkpu1(&self) -> Wkpu1R {
        Wkpu1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - In standby mode, when APC is enabled, this position is 1, corresponding to the IO being in a pull-down state."]
    #[inline(always)]
    pub fn wkpu2(&self) -> Wkpu2R {
        Wkpu2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - In standby mode, when APC is enabled, this position is 1, corresponding to the IO being in a pull-down state."]
    #[inline(always)]
    pub fn wkpu3(&self) -> Wkpu3R {
        Wkpu3R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - In standby mode, when APC is enabled, this position is 1, corresponding to the IO being in a pull-down state."]
    #[inline(always)]
    pub fn wkpu0(&mut self) -> Wkpu0W<'_, PdcraSpec> {
        Wkpu0W::new(self, 0)
    }
    #[doc = "Bit 1 - In standby mode, when APC is enabled, this position is 1, corresponding to the IO being in a pull-down state."]
    #[inline(always)]
    pub fn wkpu1(&mut self) -> Wkpu1W<'_, PdcraSpec> {
        Wkpu1W::new(self, 1)
    }
    #[doc = "Bit 2 - In standby mode, when APC is enabled, this position is 1, corresponding to the IO being in a pull-down state."]
    #[inline(always)]
    pub fn wkpu2(&mut self) -> Wkpu2W<'_, PdcraSpec> {
        Wkpu2W::new(self, 2)
    }
    #[doc = "Bit 3 - In standby mode, when APC is enabled, this position is 1, corresponding to the IO being in a pull-down state."]
    #[inline(always)]
    pub fn wkpu3(&mut self) -> Wkpu3W<'_, PdcraSpec> {
        Wkpu3W::new(self, 3)
    }
}
#[doc = "Wake-up IO Pull-down Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pdcra::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdcra::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PdcraSpec;
impl crate::RegisterSpec for PdcraSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pdcra::R`](R) reader structure"]
impl crate::Readable for PdcraSpec {}
#[doc = "`write(|w| ..)` method takes [`pdcra::W`](W) writer structure"]
impl crate::Writable for PdcraSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PDCRA to value 0"]
impl crate::Resettable for PdcraSpec {}
