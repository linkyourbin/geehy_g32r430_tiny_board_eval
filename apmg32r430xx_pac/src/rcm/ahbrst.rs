#[doc = "Register `AHBRST` reader"]
pub type R = crate::R<AhbrstSpec>;
#[doc = "Register `AHBRST` writer"]
pub type W = crate::W<AhbrstSpec>;
#[doc = "Field `DMARST` reader - DMA module reset"]
pub type DmarstR = crate::BitReader;
#[doc = "Field `DMARST` writer - DMA module reset"]
pub type DmarstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIORST` reader - GPIO module reset"]
pub type GpiorstR = crate::BitReader;
#[doc = "Field `GPIORST` writer - GPIO module reset"]
pub type GpiorstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC1RST` reader - ADC1 module reset"]
pub type Adc1rstR = crate::BitReader;
#[doc = "Field `ADC1RST` writer - ADC1 module reset"]
pub type Adc1rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC2RST` reader - ADC2 module reset"]
pub type Adc2rstR = crate::BitReader;
#[doc = "Field `ADC2RST` writer - ADC2 module reset"]
pub type Adc2rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC3RST` reader - ADC3 module reset"]
pub type Adc3rstR = crate::BitReader;
#[doc = "Field `ADC3RST` writer - ADC3 module reset"]
pub type Adc3rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLASHRST` reader - FLASH module reset"]
pub type FlashrstR = crate::BitReader;
#[doc = "Field `FLASHRST` writer - FLASH module reset"]
pub type FlashrstW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - DMA module reset"]
    #[inline(always)]
    pub fn dmarst(&self) -> DmarstR {
        DmarstR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - GPIO module reset"]
    #[inline(always)]
    pub fn gpiorst(&self) -> GpiorstR {
        GpiorstR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 5 - ADC1 module reset"]
    #[inline(always)]
    pub fn adc1rst(&self) -> Adc1rstR {
        Adc1rstR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - ADC2 module reset"]
    #[inline(always)]
    pub fn adc2rst(&self) -> Adc2rstR {
        Adc2rstR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - ADC3 module reset"]
    #[inline(always)]
    pub fn adc3rst(&self) -> Adc3rstR {
        Adc3rstR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - FLASH module reset"]
    #[inline(always)]
    pub fn flashrst(&self) -> FlashrstR {
        FlashrstR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DMA module reset"]
    #[inline(always)]
    pub fn dmarst(&mut self) -> DmarstW<'_, AhbrstSpec> {
        DmarstW::new(self, 0)
    }
    #[doc = "Bit 1 - GPIO module reset"]
    #[inline(always)]
    pub fn gpiorst(&mut self) -> GpiorstW<'_, AhbrstSpec> {
        GpiorstW::new(self, 1)
    }
    #[doc = "Bit 5 - ADC1 module reset"]
    #[inline(always)]
    pub fn adc1rst(&mut self) -> Adc1rstW<'_, AhbrstSpec> {
        Adc1rstW::new(self, 5)
    }
    #[doc = "Bit 6 - ADC2 module reset"]
    #[inline(always)]
    pub fn adc2rst(&mut self) -> Adc2rstW<'_, AhbrstSpec> {
        Adc2rstW::new(self, 6)
    }
    #[doc = "Bit 7 - ADC3 module reset"]
    #[inline(always)]
    pub fn adc3rst(&mut self) -> Adc3rstW<'_, AhbrstSpec> {
        Adc3rstW::new(self, 7)
    }
    #[doc = "Bit 8 - FLASH module reset"]
    #[inline(always)]
    pub fn flashrst(&mut self) -> FlashrstW<'_, AhbrstSpec> {
        FlashrstW::new(self, 8)
    }
}
#[doc = "AHB Peripheral Reset Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ahbrst::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahbrst::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AhbrstSpec;
impl crate::RegisterSpec for AhbrstSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahbrst::R`](R) reader structure"]
impl crate::Readable for AhbrstSpec {}
#[doc = "`write(|w| ..)` method takes [`ahbrst::W`](W) writer structure"]
impl crate::Writable for AhbrstSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AHBRST to value 0x01e3"]
impl crate::Resettable for AhbrstSpec {
    const RESET_VALUE: u32 = 0x01e3;
}
