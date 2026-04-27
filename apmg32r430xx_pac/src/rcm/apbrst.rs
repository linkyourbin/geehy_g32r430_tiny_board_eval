#[doc = "Register `APBRST` reader"]
pub type R = crate::R<ApbrstSpec>;
#[doc = "Register `APBRST` writer"]
pub type W = crate::W<ApbrstSpec>;
#[doc = "Field `TMR1RST` reader - TMR1 module reset"]
pub type Tmr1rstR = crate::BitReader;
#[doc = "Field `TMR1RST` writer - TMR1 module reset"]
pub type Tmr1rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR2RST` reader - TMR3 module reset"]
pub type Tmr2rstR = crate::BitReader;
#[doc = "Field `TMR2RST` writer - TMR3 module reset"]
pub type Tmr2rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR3RST` reader - TMR3 module reset"]
pub type Tmr3rstR = crate::BitReader;
#[doc = "Field `TMR3RST` writer - TMR3 module reset"]
pub type Tmr3rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR4RST` reader - TMR4 module reset"]
pub type Tmr4rstR = crate::BitReader;
#[doc = "Field `TMR4RST` writer - TMR4 module reset"]
pub type Tmr4rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPIRST` reader - SPI module reset"]
pub type SpirstR = crate::BitReader;
#[doc = "Field `SPIRST` writer - SPI module reset"]
pub type SpirstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART1RST` reader - USART1 module reset"]
pub type Usart1rstR = crate::BitReader;
#[doc = "Field `USART1RST` writer - USART1 module reset"]
pub type Usart1rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART2RST` reader - USART2 module reset"]
pub type Usart2rstR = crate::BitReader;
#[doc = "Field `USART2RST` writer - USART2 module reset"]
pub type Usart2rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2CRST` reader - I2C module reset"]
pub type I2crstR = crate::BitReader;
#[doc = "Field `I2CRST` writer - I2C module reset"]
pub type I2crstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSRST` reader - TS module reset"]
pub type TsrstR = crate::BitReader;
#[doc = "Field `TSRST` writer - TS module reset"]
pub type TsrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EINTRST` reader - EINT module reset"]
pub type EintrstR = crate::BitReader;
#[doc = "Field `EINTRST` writer - EINT module reset"]
pub type EintrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DAC1RST` reader - DAC1 module reset"]
pub type Dac1rstR = crate::BitReader;
#[doc = "Field `DAC1RST` writer - DAC1 module reset"]
pub type Dac1rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DAC2RST` reader - DAC2 module reset"]
pub type Dac2rstR = crate::BitReader;
#[doc = "Field `DAC2RST` writer - DAC2 module reset"]
pub type Dac2rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMPRST` reader - COMP module reset"]
pub type ComprstR = crate::BitReader;
#[doc = "Field `COMPRST` writer - COMP module reset"]
pub type ComprstW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - TMR1 module reset"]
    #[inline(always)]
    pub fn tmr1rst(&self) -> Tmr1rstR {
        Tmr1rstR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TMR3 module reset"]
    #[inline(always)]
    pub fn tmr2rst(&self) -> Tmr2rstR {
        Tmr2rstR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TMR3 module reset"]
    #[inline(always)]
    pub fn tmr3rst(&self) -> Tmr3rstR {
        Tmr3rstR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - TMR4 module reset"]
    #[inline(always)]
    pub fn tmr4rst(&self) -> Tmr4rstR {
        Tmr4rstR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - SPI module reset"]
    #[inline(always)]
    pub fn spirst(&self) -> SpirstR {
        SpirstR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - USART1 module reset"]
    #[inline(always)]
    pub fn usart1rst(&self) -> Usart1rstR {
        Usart1rstR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - USART2 module reset"]
    #[inline(always)]
    pub fn usart2rst(&self) -> Usart2rstR {
        Usart2rstR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - I2C module reset"]
    #[inline(always)]
    pub fn i2crst(&self) -> I2crstR {
        I2crstR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - TS module reset"]
    #[inline(always)]
    pub fn tsrst(&self) -> TsrstR {
        TsrstR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - EINT module reset"]
    #[inline(always)]
    pub fn eintrst(&self) -> EintrstR {
        EintrstR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - DAC1 module reset"]
    #[inline(always)]
    pub fn dac1rst(&self) -> Dac1rstR {
        Dac1rstR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - DAC2 module reset"]
    #[inline(always)]
    pub fn dac2rst(&self) -> Dac2rstR {
        Dac2rstR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - COMP module reset"]
    #[inline(always)]
    pub fn comprst(&self) -> ComprstR {
        ComprstR::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TMR1 module reset"]
    #[inline(always)]
    pub fn tmr1rst(&mut self) -> Tmr1rstW<'_, ApbrstSpec> {
        Tmr1rstW::new(self, 0)
    }
    #[doc = "Bit 1 - TMR3 module reset"]
    #[inline(always)]
    pub fn tmr2rst(&mut self) -> Tmr2rstW<'_, ApbrstSpec> {
        Tmr2rstW::new(self, 1)
    }
    #[doc = "Bit 2 - TMR3 module reset"]
    #[inline(always)]
    pub fn tmr3rst(&mut self) -> Tmr3rstW<'_, ApbrstSpec> {
        Tmr3rstW::new(self, 2)
    }
    #[doc = "Bit 3 - TMR4 module reset"]
    #[inline(always)]
    pub fn tmr4rst(&mut self) -> Tmr4rstW<'_, ApbrstSpec> {
        Tmr4rstW::new(self, 3)
    }
    #[doc = "Bit 4 - SPI module reset"]
    #[inline(always)]
    pub fn spirst(&mut self) -> SpirstW<'_, ApbrstSpec> {
        SpirstW::new(self, 4)
    }
    #[doc = "Bit 5 - USART1 module reset"]
    #[inline(always)]
    pub fn usart1rst(&mut self) -> Usart1rstW<'_, ApbrstSpec> {
        Usart1rstW::new(self, 5)
    }
    #[doc = "Bit 6 - USART2 module reset"]
    #[inline(always)]
    pub fn usart2rst(&mut self) -> Usart2rstW<'_, ApbrstSpec> {
        Usart2rstW::new(self, 6)
    }
    #[doc = "Bit 7 - I2C module reset"]
    #[inline(always)]
    pub fn i2crst(&mut self) -> I2crstW<'_, ApbrstSpec> {
        I2crstW::new(self, 7)
    }
    #[doc = "Bit 9 - TS module reset"]
    #[inline(always)]
    pub fn tsrst(&mut self) -> TsrstW<'_, ApbrstSpec> {
        TsrstW::new(self, 9)
    }
    #[doc = "Bit 10 - EINT module reset"]
    #[inline(always)]
    pub fn eintrst(&mut self) -> EintrstW<'_, ApbrstSpec> {
        EintrstW::new(self, 10)
    }
    #[doc = "Bit 11 - DAC1 module reset"]
    #[inline(always)]
    pub fn dac1rst(&mut self) -> Dac1rstW<'_, ApbrstSpec> {
        Dac1rstW::new(self, 11)
    }
    #[doc = "Bit 12 - DAC2 module reset"]
    #[inline(always)]
    pub fn dac2rst(&mut self) -> Dac2rstW<'_, ApbrstSpec> {
        Dac2rstW::new(self, 12)
    }
    #[doc = "Bit 13 - COMP module reset"]
    #[inline(always)]
    pub fn comprst(&mut self) -> ComprstW<'_, ApbrstSpec> {
        ComprstW::new(self, 13)
    }
}
#[doc = "APB Peripheral Reset Register\n\nYou can [`read`](crate::Reg::read) this register and get [`apbrst::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apbrst::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ApbrstSpec;
impl crate::RegisterSpec for ApbrstSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apbrst::R`](R) reader structure"]
impl crate::Readable for ApbrstSpec {}
#[doc = "`write(|w| ..)` method takes [`apbrst::W`](W) writer structure"]
impl crate::Writable for ApbrstSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets APBRST to value 0x3eff"]
impl crate::Resettable for ApbrstSpec {
    const RESET_VALUE: u32 = 0x3eff;
}
