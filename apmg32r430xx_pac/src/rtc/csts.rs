#[doc = "Register `CSTS` reader"]
pub type R = crate::R<CstsSpec>;
#[doc = "Register `CSTS` writer"]
pub type W = crate::W<CstsSpec>;
#[doc = "Field `SECFLG` reader - Second Flag"]
pub type SecflgR = crate::BitReader;
#[doc = "Field `SECFLG` writer - Second Flag"]
pub type SecflgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ALRFLG` reader - Alarm Flag"]
pub type AlrflgR = crate::BitReader;
#[doc = "Field `ALRFLG` writer - Alarm Flag"]
pub type AlrflgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVRFLG` reader - Overflow Flag"]
pub type OvrflgR = crate::BitReader;
#[doc = "Field `OVRFLG` writer - Overflow Flag"]
pub type OvrflgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSYNCFLG` reader - Registers Synchronized Flag"]
pub type RsyncflgR = crate::BitReader;
#[doc = "Field `RSYNCFLG` writer - Registers Synchronized Flag"]
pub type RsyncflgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CFGMFLG` reader - Configuration Flag"]
pub type CfgmflgR = crate::BitReader;
#[doc = "Field `CFGMFLG` writer - Configuration Flag"]
pub type CfgmflgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OCFLG` reader - RTC operation OFF"]
pub type OcflgR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Second Flag"]
    #[inline(always)]
    pub fn secflg(&self) -> SecflgR {
        SecflgR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Alarm Flag"]
    #[inline(always)]
    pub fn alrflg(&self) -> AlrflgR {
        AlrflgR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Overflow Flag"]
    #[inline(always)]
    pub fn ovrflg(&self) -> OvrflgR {
        OvrflgR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Registers Synchronized Flag"]
    #[inline(always)]
    pub fn rsyncflg(&self) -> RsyncflgR {
        RsyncflgR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Configuration Flag"]
    #[inline(always)]
    pub fn cfgmflg(&self) -> CfgmflgR {
        CfgmflgR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - RTC operation OFF"]
    #[inline(always)]
    pub fn ocflg(&self) -> OcflgR {
        OcflgR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Second Flag"]
    #[inline(always)]
    pub fn secflg(&mut self) -> SecflgW<'_, CstsSpec> {
        SecflgW::new(self, 0)
    }
    #[doc = "Bit 1 - Alarm Flag"]
    #[inline(always)]
    pub fn alrflg(&mut self) -> AlrflgW<'_, CstsSpec> {
        AlrflgW::new(self, 1)
    }
    #[doc = "Bit 2 - Overflow Flag"]
    #[inline(always)]
    pub fn ovrflg(&mut self) -> OvrflgW<'_, CstsSpec> {
        OvrflgW::new(self, 2)
    }
    #[doc = "Bit 3 - Registers Synchronized Flag"]
    #[inline(always)]
    pub fn rsyncflg(&mut self) -> RsyncflgW<'_, CstsSpec> {
        RsyncflgW::new(self, 3)
    }
    #[doc = "Bit 4 - Configuration Flag"]
    #[inline(always)]
    pub fn cfgmflg(&mut self) -> CfgmflgW<'_, CstsSpec> {
        CfgmflgW::new(self, 4)
    }
}
#[doc = "Control and State register\n\nYou can [`read`](crate::Reg::read) this register and get [`csts::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csts::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CstsSpec;
impl crate::RegisterSpec for CstsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csts::R`](R) reader structure"]
impl crate::Readable for CstsSpec {}
#[doc = "`write(|w| ..)` method takes [`csts::W`](W) writer structure"]
impl crate::Writable for CstsSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CSTS to value 0x20"]
impl crate::Resettable for CstsSpec {
    const RESET_VALUE: u32 = 0x20;
}
