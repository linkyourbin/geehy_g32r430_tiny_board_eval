#[doc = "Register `APBCG` reader"]
pub type R = crate::R<ApbcgSpec>;
#[doc = "Register `APBCG` writer"]
pub type W = crate::W<ApbcgSpec>;
#[doc = "Field `TMR1CEN` reader - TMR1 clock enable"]
pub type Tmr1cenR = crate::BitReader;
#[doc = "Field `TMR1CEN` writer - TMR1 clock enable"]
pub type Tmr1cenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR2CEN` reader - TMR3 clock enable"]
pub type Tmr2cenR = crate::BitReader;
#[doc = "Field `TMR2CEN` writer - TMR3 clock enable"]
pub type Tmr2cenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR3CEN` reader - TMR3 clock enable"]
pub type Tmr3cenR = crate::BitReader;
#[doc = "Field `TMR3CEN` writer - TMR3 clock enable"]
pub type Tmr3cenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR4CEN` reader - TMR4 clock enable"]
pub type Tmr4cenR = crate::BitReader;
#[doc = "Field `TMR4CEN` writer - TMR4 clock enable"]
pub type Tmr4cenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPICEN` reader - SPI clock enable"]
pub type SpicenR = crate::BitReader;
#[doc = "Field `SPICEN` writer - SPI clock enable"]
pub type SpicenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART1CEN` reader - USART1 clock enable"]
pub type Usart1cenR = crate::BitReader;
#[doc = "Field `USART1CEN` writer - USART1 clock enable"]
pub type Usart1cenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART2CEN` reader - USART2 clock enable"]
pub type Usart2cenR = crate::BitReader;
#[doc = "Field `USART2CEN` writer - USART2 clock enable"]
pub type Usart2cenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2CCEN` reader - I2C clock enable"]
pub type I2ccenR = crate::BitReader;
#[doc = "Field `I2CCEN` writer - I2C clock enable"]
pub type I2ccenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WWDTCEN` reader - WWDT clock enable"]
pub type WwdtcenR = crate::BitReader;
#[doc = "Field `WWDTCEN` writer - WWDT clock enable"]
pub type WwdtcenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSCEN` reader - TS clock enable"]
pub type TscenR = crate::BitReader;
#[doc = "Field `TSCEN` writer - TS clock enable"]
pub type TscenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EINTCEN` reader - EINT clock enable"]
pub type EintcenR = crate::BitReader;
#[doc = "Field `EINTCEN` writer - EINT clock enable"]
pub type EintcenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DAC1CEN` reader - DAC1 clock enable"]
pub type Dac1cenR = crate::BitReader;
#[doc = "Field `DAC1CEN` writer - DAC1 clock enable"]
pub type Dac1cenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DAC2CEN` reader - DAC2 clock enable"]
pub type Dac2cenR = crate::BitReader;
#[doc = "Field `DAC2CEN` writer - DAC2 clock enable"]
pub type Dac2cenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMPCEN` reader - COMP clock enable"]
pub type CompcenR = crate::BitReader;
#[doc = "Field `COMPCEN` writer - COMP clock enable"]
pub type CompcenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BKPCEN` reader - BKP clock enable"]
pub type BkpcenR = crate::BitReader;
#[doc = "Field `BKPCEN` writer - BKP clock enable"]
pub type BkpcenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - TMR1 clock enable"]
    #[inline(always)]
    pub fn tmr1cen(&self) -> Tmr1cenR {
        Tmr1cenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TMR3 clock enable"]
    #[inline(always)]
    pub fn tmr2cen(&self) -> Tmr2cenR {
        Tmr2cenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TMR3 clock enable"]
    #[inline(always)]
    pub fn tmr3cen(&self) -> Tmr3cenR {
        Tmr3cenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - TMR4 clock enable"]
    #[inline(always)]
    pub fn tmr4cen(&self) -> Tmr4cenR {
        Tmr4cenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - SPI clock enable"]
    #[inline(always)]
    pub fn spicen(&self) -> SpicenR {
        SpicenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - USART1 clock enable"]
    #[inline(always)]
    pub fn usart1cen(&self) -> Usart1cenR {
        Usart1cenR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - USART2 clock enable"]
    #[inline(always)]
    pub fn usart2cen(&self) -> Usart2cenR {
        Usart2cenR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - I2C clock enable"]
    #[inline(always)]
    pub fn i2ccen(&self) -> I2ccenR {
        I2ccenR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - WWDT clock enable"]
    #[inline(always)]
    pub fn wwdtcen(&self) -> WwdtcenR {
        WwdtcenR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - TS clock enable"]
    #[inline(always)]
    pub fn tscen(&self) -> TscenR {
        TscenR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - EINT clock enable"]
    #[inline(always)]
    pub fn eintcen(&self) -> EintcenR {
        EintcenR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - DAC1 clock enable"]
    #[inline(always)]
    pub fn dac1cen(&self) -> Dac1cenR {
        Dac1cenR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - DAC2 clock enable"]
    #[inline(always)]
    pub fn dac2cen(&self) -> Dac2cenR {
        Dac2cenR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - COMP clock enable"]
    #[inline(always)]
    pub fn compcen(&self) -> CompcenR {
        CompcenR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - BKP clock enable"]
    #[inline(always)]
    pub fn bkpcen(&self) -> BkpcenR {
        BkpcenR::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TMR1 clock enable"]
    #[inline(always)]
    pub fn tmr1cen(&mut self) -> Tmr1cenW<'_, ApbcgSpec> {
        Tmr1cenW::new(self, 0)
    }
    #[doc = "Bit 1 - TMR3 clock enable"]
    #[inline(always)]
    pub fn tmr2cen(&mut self) -> Tmr2cenW<'_, ApbcgSpec> {
        Tmr2cenW::new(self, 1)
    }
    #[doc = "Bit 2 - TMR3 clock enable"]
    #[inline(always)]
    pub fn tmr3cen(&mut self) -> Tmr3cenW<'_, ApbcgSpec> {
        Tmr3cenW::new(self, 2)
    }
    #[doc = "Bit 3 - TMR4 clock enable"]
    #[inline(always)]
    pub fn tmr4cen(&mut self) -> Tmr4cenW<'_, ApbcgSpec> {
        Tmr4cenW::new(self, 3)
    }
    #[doc = "Bit 4 - SPI clock enable"]
    #[inline(always)]
    pub fn spicen(&mut self) -> SpicenW<'_, ApbcgSpec> {
        SpicenW::new(self, 4)
    }
    #[doc = "Bit 5 - USART1 clock enable"]
    #[inline(always)]
    pub fn usart1cen(&mut self) -> Usart1cenW<'_, ApbcgSpec> {
        Usart1cenW::new(self, 5)
    }
    #[doc = "Bit 6 - USART2 clock enable"]
    #[inline(always)]
    pub fn usart2cen(&mut self) -> Usart2cenW<'_, ApbcgSpec> {
        Usart2cenW::new(self, 6)
    }
    #[doc = "Bit 7 - I2C clock enable"]
    #[inline(always)]
    pub fn i2ccen(&mut self) -> I2ccenW<'_, ApbcgSpec> {
        I2ccenW::new(self, 7)
    }
    #[doc = "Bit 8 - WWDT clock enable"]
    #[inline(always)]
    pub fn wwdtcen(&mut self) -> WwdtcenW<'_, ApbcgSpec> {
        WwdtcenW::new(self, 8)
    }
    #[doc = "Bit 9 - TS clock enable"]
    #[inline(always)]
    pub fn tscen(&mut self) -> TscenW<'_, ApbcgSpec> {
        TscenW::new(self, 9)
    }
    #[doc = "Bit 10 - EINT clock enable"]
    #[inline(always)]
    pub fn eintcen(&mut self) -> EintcenW<'_, ApbcgSpec> {
        EintcenW::new(self, 10)
    }
    #[doc = "Bit 11 - DAC1 clock enable"]
    #[inline(always)]
    pub fn dac1cen(&mut self) -> Dac1cenW<'_, ApbcgSpec> {
        Dac1cenW::new(self, 11)
    }
    #[doc = "Bit 12 - DAC2 clock enable"]
    #[inline(always)]
    pub fn dac2cen(&mut self) -> Dac2cenW<'_, ApbcgSpec> {
        Dac2cenW::new(self, 12)
    }
    #[doc = "Bit 13 - COMP clock enable"]
    #[inline(always)]
    pub fn compcen(&mut self) -> CompcenW<'_, ApbcgSpec> {
        CompcenW::new(self, 13)
    }
    #[doc = "Bit 14 - BKP clock enable"]
    #[inline(always)]
    pub fn bkpcen(&mut self) -> BkpcenW<'_, ApbcgSpec> {
        BkpcenW::new(self, 14)
    }
}
#[doc = "APB peripheral control register\n\nYou can [`read`](crate::Reg::read) this register and get [`apbcg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apbcg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ApbcgSpec;
impl crate::RegisterSpec for ApbcgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apbcg::R`](R) reader structure"]
impl crate::Readable for ApbcgSpec {}
#[doc = "`write(|w| ..)` method takes [`apbcg::W`](W) writer structure"]
impl crate::Writable for ApbcgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets APBCG to value 0"]
impl crate::Resettable for ApbcgSpec {}
