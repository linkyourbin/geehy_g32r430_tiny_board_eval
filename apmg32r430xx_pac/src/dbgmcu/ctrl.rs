#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CtrlSpec>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CtrlSpec>;
#[doc = "Field `STOP` reader - Debug Stop mode"]
pub type StopR = crate::BitReader;
#[doc = "Field `STOP` writer - Debug Stop mode"]
pub type StopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STANDBY` reader - Debug Standby mode"]
pub type StandbyR = crate::BitReader;
#[doc = "Field `STANDBY` writer - Debug Standby mode"]
pub type StandbyW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - Debug Stop mode"]
    #[inline(always)]
    pub fn stop(&self) -> StopR {
        StopR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Debug Standby mode"]
    #[inline(always)]
    pub fn standby(&self) -> StandbyR {
        StandbyR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Debug Stop mode"]
    #[inline(always)]
    pub fn stop(&mut self) -> StopW<'_, CtrlSpec> {
        StopW::new(self, 1)
    }
    #[doc = "Bit 2 - Debug Standby mode"]
    #[inline(always)]
    pub fn standby(&mut self) -> StandbyW<'_, CtrlSpec> {
        StandbyW::new(self, 2)
    }
}
#[doc = "Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlSpec;
impl crate::RegisterSpec for CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl::R`](R) reader structure"]
impl crate::Readable for CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CtrlSpec {}
