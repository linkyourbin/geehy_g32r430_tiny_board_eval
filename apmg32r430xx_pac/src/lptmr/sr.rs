#[doc = "Register `SR` reader"]
pub type R = crate::R<SrSpec>;
#[doc = "Register `SR` writer"]
pub type W = crate::W<SrSpec>;
#[doc = "Field `WK_STS` reader - Status Register, read SR register clear."]
pub type WkStsR = crate::BitReader;
#[doc = "Field `WK_STS` writer - Status Register, read SR register clear."]
pub type WkStsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PU` reader - PS value update indicator, hardware automatically resets after update."]
pub type PuR = crate::BitReader;
#[doc = "Field `RU` reader - Reload value update indication, automatically reset by hardware after update."]
pub type RuR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Status Register, read SR register clear."]
    #[inline(always)]
    pub fn wk_sts(&self) -> WkStsR {
        WkStsR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PS value update indicator, hardware automatically resets after update."]
    #[inline(always)]
    pub fn pu(&self) -> PuR {
        PuR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Reload value update indication, automatically reset by hardware after update."]
    #[inline(always)]
    pub fn ru(&self) -> RuR {
        RuR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Status Register, read SR register clear."]
    #[inline(always)]
    pub fn wk_sts(&mut self) -> WkStsW<'_, SrSpec> {
        WkStsW::new(self, 0)
    }
}
#[doc = "Status register\n\nYou can [`read`](crate::Reg::read) this register and get [`sr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
