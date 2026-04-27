#[doc = "Register `IER` reader"]
pub type R = crate::R<IerSpec>;
#[doc = "Register `IER` writer"]
pub type W = crate::W<IerSpec>;
#[doc = "Field `EOSIMPIE` reader - End of sampling flag interrupt enable for regular conversions"]
pub type EosimpieR = crate::BitReader;
#[doc = "Field `EOSIMPIE` writer - End of sampling flag interrupt enable for regular conversions"]
pub type EosimpieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOCIE` reader - End of regular conversion interrupt enable"]
pub type EocieR = crate::BitReader;
#[doc = "Field `EOCIE` writer - End of regular conversion interrupt enable"]
pub type EocieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOSIE` reader - End of regular sequence of conversions interrupt enable"]
pub type EosieR = crate::BitReader;
#[doc = "Field `EOSIE` writer - End of regular sequence of conversions interrupt enable"]
pub type EosieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVRIE` reader - Overrun interrupt enable"]
pub type OvrieR = crate::BitReader;
#[doc = "Field `OVRIE` writer - Overrun interrupt enable"]
pub type OvrieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `JEOCIE` reader - End of injected conversion interrupt enable"]
pub type JeocieR = crate::BitReader;
#[doc = "Field `JEOCIE` writer - End of injected conversion interrupt enable"]
pub type JeocieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `JEOSIE` reader - End of injected sequence of conversions interrupt enable"]
pub type JeosieR = crate::BitReader;
#[doc = "Field `JEOSIE` writer - End of injected sequence of conversions interrupt enable"]
pub type JeosieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AWD1IE` reader - Analog watchdog 1 interrupt enable"]
pub type Awd1ieR = crate::BitReader;
#[doc = "Field `AWD1IE` writer - Analog watchdog 1 interrupt enable"]
pub type Awd1ieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AWD2IE` reader - Analog watchdog 2 interrupt enable"]
pub type Awd2ieR = crate::BitReader;
#[doc = "Field `AWD2IE` writer - Analog watchdog 2 interrupt enable"]
pub type Awd2ieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AWD3IE` reader - Analog watchdog 3 interrupt enable"]
pub type Awd3ieR = crate::BitReader;
#[doc = "Field `AWD3IE` writer - Analog watchdog 3 interrupt enable"]
pub type Awd3ieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `JQOVFIE` reader - Injected context queue overflow interrupt enable"]
pub type JqovfieR = crate::BitReader;
#[doc = "Field `JQOVFIE` writer - Injected context queue overflow interrupt enable"]
pub type JqovfieW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - End of sampling flag interrupt enable for regular conversions"]
    #[inline(always)]
    pub fn eosimpie(&self) -> EosimpieR {
        EosimpieR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - End of regular conversion interrupt enable"]
    #[inline(always)]
    pub fn eocie(&self) -> EocieR {
        EocieR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - End of regular sequence of conversions interrupt enable"]
    #[inline(always)]
    pub fn eosie(&self) -> EosieR {
        EosieR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Overrun interrupt enable"]
    #[inline(always)]
    pub fn ovrie(&self) -> OvrieR {
        OvrieR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - End of injected conversion interrupt enable"]
    #[inline(always)]
    pub fn jeocie(&self) -> JeocieR {
        JeocieR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - End of injected sequence of conversions interrupt enable"]
    #[inline(always)]
    pub fn jeosie(&self) -> JeosieR {
        JeosieR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Analog watchdog 1 interrupt enable"]
    #[inline(always)]
    pub fn awd1ie(&self) -> Awd1ieR {
        Awd1ieR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Analog watchdog 2 interrupt enable"]
    #[inline(always)]
    pub fn awd2ie(&self) -> Awd2ieR {
        Awd2ieR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Analog watchdog 3 interrupt enable"]
    #[inline(always)]
    pub fn awd3ie(&self) -> Awd3ieR {
        Awd3ieR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Injected context queue overflow interrupt enable"]
    #[inline(always)]
    pub fn jqovfie(&self) -> JqovfieR {
        JqovfieR::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - End of sampling flag interrupt enable for regular conversions"]
    #[inline(always)]
    pub fn eosimpie(&mut self) -> EosimpieW<'_, IerSpec> {
        EosimpieW::new(self, 1)
    }
    #[doc = "Bit 2 - End of regular conversion interrupt enable"]
    #[inline(always)]
    pub fn eocie(&mut self) -> EocieW<'_, IerSpec> {
        EocieW::new(self, 2)
    }
    #[doc = "Bit 3 - End of regular sequence of conversions interrupt enable"]
    #[inline(always)]
    pub fn eosie(&mut self) -> EosieW<'_, IerSpec> {
        EosieW::new(self, 3)
    }
    #[doc = "Bit 4 - Overrun interrupt enable"]
    #[inline(always)]
    pub fn ovrie(&mut self) -> OvrieW<'_, IerSpec> {
        OvrieW::new(self, 4)
    }
    #[doc = "Bit 5 - End of injected conversion interrupt enable"]
    #[inline(always)]
    pub fn jeocie(&mut self) -> JeocieW<'_, IerSpec> {
        JeocieW::new(self, 5)
    }
    #[doc = "Bit 6 - End of injected sequence of conversions interrupt enable"]
    #[inline(always)]
    pub fn jeosie(&mut self) -> JeosieW<'_, IerSpec> {
        JeosieW::new(self, 6)
    }
    #[doc = "Bit 7 - Analog watchdog 1 interrupt enable"]
    #[inline(always)]
    pub fn awd1ie(&mut self) -> Awd1ieW<'_, IerSpec> {
        Awd1ieW::new(self, 7)
    }
    #[doc = "Bit 8 - Analog watchdog 2 interrupt enable"]
    #[inline(always)]
    pub fn awd2ie(&mut self) -> Awd2ieW<'_, IerSpec> {
        Awd2ieW::new(self, 8)
    }
    #[doc = "Bit 9 - Analog watchdog 3 interrupt enable"]
    #[inline(always)]
    pub fn awd3ie(&mut self) -> Awd3ieW<'_, IerSpec> {
        Awd3ieW::new(self, 9)
    }
    #[doc = "Bit 10 - Injected context queue overflow interrupt enable"]
    #[inline(always)]
    pub fn jqovfie(&mut self) -> JqovfieW<'_, IerSpec> {
        JqovfieW::new(self, 10)
    }
}
#[doc = "ADC interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`ier::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IerSpec;
impl crate::RegisterSpec for IerSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ier::R`](R) reader structure"]
impl crate::Readable for IerSpec {}
#[doc = "`write(|w| ..)` method takes [`ier::W`](W) writer structure"]
impl crate::Writable for IerSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IER to value 0"]
impl crate::Resettable for IerSpec {}
