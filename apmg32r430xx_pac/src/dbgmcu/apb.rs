#[doc = "Register `APB` reader"]
pub type R = crate::R<ApbSpec>;
#[doc = "Register `APB` writer"]
pub type W = crate::W<ApbSpec>;
#[doc = "Field `TMR1_STOP` reader - Debug TMR1 counter stopped when core is halted"]
pub type Tmr1StopR = crate::BitReader;
#[doc = "Field `TMR1_STOP` writer - Debug TMR1 counter stopped when core is halted"]
pub type Tmr1StopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR2_STOP` reader - Debug TMR2 counter stopped when core is halted"]
pub type Tmr2StopR = crate::BitReader;
#[doc = "Field `TMR2_STOP` writer - Debug TMR2 counter stopped when core is halted"]
pub type Tmr2StopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR3_STOP` reader - Debug TMR3 counter stopped when core is halted"]
pub type Tmr3StopR = crate::BitReader;
#[doc = "Field `TMR3_STOP` writer - Debug TMR3 counter stopped when core is halted"]
pub type Tmr3StopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR4_STOP` reader - Debug TMR4 counter stopped when core is halted"]
pub type Tmr4StopR = crate::BitReader;
#[doc = "Field `TMR4_STOP` writer - Debug TMR4 counter stopped when core is halted"]
pub type Tmr4StopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPTMR_STOP` reader - Debug LPTMR counter stopped when core is halted"]
pub type LptmrStopR = crate::FieldReader;
#[doc = "Field `LPTMR_STOP` writer - Debug LPTMR counter stopped when core is halted"]
pub type LptmrStopW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RTC_STOP` reader - Debug RTC counter stopped when core is halted"]
pub type RtcStopR = crate::BitReader;
#[doc = "Field `RTC_STOP` writer - Debug RTC counter stopped when core is halted"]
pub type RtcStopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WWDT_STOP` reader - Debug WWDT counter stopped when core is halted"]
pub type WwdtStopR = crate::BitReader;
#[doc = "Field `WWDT_STOP` writer - Debug WWDT counter stopped when core is halted"]
pub type WwdtStopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IWDT_STOP` reader - Debug IWDT counter stopped when core is halted"]
pub type IwdtStopR = crate::BitReader;
#[doc = "Field `IWDT_STOP` writer - Debug IWDT counter stopped when core is halted"]
pub type IwdtStopW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Debug TMR1 counter stopped when core is halted"]
    #[inline(always)]
    pub fn tmr1_stop(&self) -> Tmr1StopR {
        Tmr1StopR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - Debug TMR2 counter stopped when core is halted"]
    #[inline(always)]
    pub fn tmr2_stop(&self) -> Tmr2StopR {
        Tmr2StopR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Debug TMR3 counter stopped when core is halted"]
    #[inline(always)]
    pub fn tmr3_stop(&self) -> Tmr3StopR {
        Tmr3StopR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Debug TMR4 counter stopped when core is halted"]
    #[inline(always)]
    pub fn tmr4_stop(&self) -> Tmr4StopR {
        Tmr4StopR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 12:13 - Debug LPTMR counter stopped when core is halted"]
    #[inline(always)]
    pub fn lptmr_stop(&self) -> LptmrStopR {
        LptmrStopR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 15 - Debug RTC counter stopped when core is halted"]
    #[inline(always)]
    pub fn rtc_stop(&self) -> RtcStopR {
        RtcStopR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Debug WWDT counter stopped when core is halted"]
    #[inline(always)]
    pub fn wwdt_stop(&self) -> WwdtStopR {
        WwdtStopR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Debug IWDT counter stopped when core is halted"]
    #[inline(always)]
    pub fn iwdt_stop(&self) -> IwdtStopR {
        IwdtStopR::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Debug TMR1 counter stopped when core is halted"]
    #[inline(always)]
    pub fn tmr1_stop(&mut self) -> Tmr1StopW<'_, ApbSpec> {
        Tmr1StopW::new(self, 0)
    }
    #[doc = "Bit 4 - Debug TMR2 counter stopped when core is halted"]
    #[inline(always)]
    pub fn tmr2_stop(&mut self) -> Tmr2StopW<'_, ApbSpec> {
        Tmr2StopW::new(self, 4)
    }
    #[doc = "Bit 5 - Debug TMR3 counter stopped when core is halted"]
    #[inline(always)]
    pub fn tmr3_stop(&mut self) -> Tmr3StopW<'_, ApbSpec> {
        Tmr3StopW::new(self, 5)
    }
    #[doc = "Bit 6 - Debug TMR4 counter stopped when core is halted"]
    #[inline(always)]
    pub fn tmr4_stop(&mut self) -> Tmr4StopW<'_, ApbSpec> {
        Tmr4StopW::new(self, 6)
    }
    #[doc = "Bits 12:13 - Debug LPTMR counter stopped when core is halted"]
    #[inline(always)]
    pub fn lptmr_stop(&mut self) -> LptmrStopW<'_, ApbSpec> {
        LptmrStopW::new(self, 12)
    }
    #[doc = "Bit 15 - Debug RTC counter stopped when core is halted"]
    #[inline(always)]
    pub fn rtc_stop(&mut self) -> RtcStopW<'_, ApbSpec> {
        RtcStopW::new(self, 15)
    }
    #[doc = "Bit 16 - Debug WWDT counter stopped when core is halted"]
    #[inline(always)]
    pub fn wwdt_stop(&mut self) -> WwdtStopW<'_, ApbSpec> {
        WwdtStopW::new(self, 16)
    }
    #[doc = "Bit 17 - Debug IWDT counter stopped when core is halted"]
    #[inline(always)]
    pub fn iwdt_stop(&mut self) -> IwdtStopW<'_, ApbSpec> {
        IwdtStopW::new(self, 17)
    }
}
#[doc = "APB Freeze Register\n\nYou can [`read`](crate::Reg::read) this register and get [`apb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ApbSpec;
impl crate::RegisterSpec for ApbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb::R`](R) reader structure"]
impl crate::Readable for ApbSpec {}
#[doc = "`write(|w| ..)` method takes [`apb::W`](W) writer structure"]
impl crate::Writable for ApbSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets APB to value 0"]
impl crate::Resettable for ApbSpec {}
