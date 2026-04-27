#[doc = "Register `PWRCR` reader"]
pub type R = crate::R<PwrcrSpec>;
#[doc = "Register `PWRCR` writer"]
pub type W = crate::W<PwrcrSpec>;
#[doc = "Field `LPM` reader - Low power mode"]
pub type LpmR = crate::BitReader;
#[doc = "Field `LPM` writer - Low power mode"]
pub type LpmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RAM1RET` reader - sram1 power failure."]
pub type Ram1retR = crate::BitReader;
#[doc = "Field `RAM1RET` writer - sram1 power failure."]
pub type Ram1retW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BKPWP` reader - Prohibit enabling write protection for the backup area."]
pub type BkpwpR = crate::BitReader;
#[doc = "Field `BKPWP` writer - Prohibit enabling write protection for the backup area."]
pub type BkpwpW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Low power mode"]
    #[inline(always)]
    pub fn lpm(&self) -> LpmR {
        LpmR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - sram1 power failure."]
    #[inline(always)]
    pub fn ram1ret(&self) -> Ram1retR {
        Ram1retR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - Prohibit enabling write protection for the backup area."]
    #[inline(always)]
    pub fn bkpwp(&self) -> BkpwpR {
        BkpwpR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Low power mode"]
    #[inline(always)]
    pub fn lpm(&mut self) -> LpmW<'_, PwrcrSpec> {
        LpmW::new(self, 0)
    }
    #[doc = "Bit 1 - sram1 power failure."]
    #[inline(always)]
    pub fn ram1ret(&mut self) -> Ram1retW<'_, PwrcrSpec> {
        Ram1retW::new(self, 1)
    }
    #[doc = "Bit 8 - Prohibit enabling write protection for the backup area."]
    #[inline(always)]
    pub fn bkpwp(&mut self) -> BkpwpW<'_, PwrcrSpec> {
        BkpwpW::new(self, 8)
    }
}
#[doc = "PWR peripheral control register\n\nYou can [`read`](crate::Reg::read) this register and get [`pwrcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwrcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PwrcrSpec;
impl crate::RegisterSpec for PwrcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwrcr::R`](R) reader structure"]
impl crate::Readable for PwrcrSpec {}
#[doc = "`write(|w| ..)` method takes [`pwrcr::W`](W) writer structure"]
impl crate::Writable for PwrcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PWRCR to value 0"]
impl crate::Resettable for PwrcrSpec {}
