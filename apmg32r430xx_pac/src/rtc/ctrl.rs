#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CtrlSpec>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CtrlSpec>;
#[doc = "Field `SECIEN` reader - Second interrupt Enable"]
pub type SecienR = crate::BitReader;
#[doc = "Field `SECIEN` writer - Second interrupt Enable"]
pub type SecienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ALRIEN` reader - Alarm interrupt Enable"]
pub type AlrienR = crate::BitReader;
#[doc = "Field `ALRIEN` writer - Alarm interrupt Enable"]
pub type AlrienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVRIEN` reader - Overflow interrupt Enable"]
pub type OvrienR = crate::BitReader;
#[doc = "Field `OVRIEN` writer - Overflow interrupt Enable"]
pub type OvrienW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Second interrupt Enable"]
    #[inline(always)]
    pub fn secien(&self) -> SecienR {
        SecienR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Alarm interrupt Enable"]
    #[inline(always)]
    pub fn alrien(&self) -> AlrienR {
        AlrienR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Overflow interrupt Enable"]
    #[inline(always)]
    pub fn ovrien(&self) -> OvrienR {
        OvrienR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Second interrupt Enable"]
    #[inline(always)]
    pub fn secien(&mut self) -> SecienW<'_, CtrlSpec> {
        SecienW::new(self, 0)
    }
    #[doc = "Bit 1 - Alarm interrupt Enable"]
    #[inline(always)]
    pub fn alrien(&mut self) -> AlrienW<'_, CtrlSpec> {
        AlrienW::new(self, 1)
    }
    #[doc = "Bit 2 - Overflow interrupt Enable"]
    #[inline(always)]
    pub fn ovrien(&mut self) -> OvrienW<'_, CtrlSpec> {
        OvrienW::new(self, 2)
    }
}
#[doc = "Control register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
