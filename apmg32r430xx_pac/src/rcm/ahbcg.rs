#[doc = "Register `AHBCG` reader"]
pub type R = crate::R<AhbcgSpec>;
#[doc = "Register `AHBCG` writer"]
pub type W = crate::W<AhbcgSpec>;
#[doc = "Field `DMACEN` reader - DMA clock enable"]
pub type DmacenR = crate::BitReader;
#[doc = "Field `DMACEN` writer - DMA clock enable"]
pub type DmacenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOCEN` reader - GPIO clock enable"]
pub type GpiocenR = crate::BitReader;
#[doc = "Field `GPIOCEN` writer - GPIO clock enable"]
pub type GpiocenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC1EN` reader - ADC1 clock enable"]
pub type Adc1enR = crate::BitReader;
#[doc = "Field `ADC1EN` writer - ADC1 clock enable"]
pub type Adc1enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC2EN` reader - ADC2 clock enable"]
pub type Adc2enR = crate::BitReader;
#[doc = "Field `ADC2EN` writer - ADC2 clock enable"]
pub type Adc2enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC3EN` reader - ADC3 clock enable"]
pub type Adc3enR = crate::BitReader;
#[doc = "Field `ADC3EN` writer - ADC3 clock enable"]
pub type Adc3enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLASHCEN` reader - FLASH clock enable"]
pub type FlashcenR = crate::BitReader;
#[doc = "Field `FLASHCEN` writer - FLASH clock enable"]
pub type FlashcenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ROMCEN` reader - ROM clock enable"]
pub type RomcenR = crate::BitReader;
#[doc = "Field `ROMCEN` writer - ROM clock enable"]
pub type RomcenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - DMA clock enable"]
    #[inline(always)]
    pub fn dmacen(&self) -> DmacenR {
        DmacenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - GPIO clock enable"]
    #[inline(always)]
    pub fn gpiocen(&self) -> GpiocenR {
        GpiocenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 5 - ADC1 clock enable"]
    #[inline(always)]
    pub fn adc1en(&self) -> Adc1enR {
        Adc1enR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - ADC2 clock enable"]
    #[inline(always)]
    pub fn adc2en(&self) -> Adc2enR {
        Adc2enR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - ADC3 clock enable"]
    #[inline(always)]
    pub fn adc3en(&self) -> Adc3enR {
        Adc3enR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - FLASH clock enable"]
    #[inline(always)]
    pub fn flashcen(&self) -> FlashcenR {
        FlashcenR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - ROM clock enable"]
    #[inline(always)]
    pub fn romcen(&self) -> RomcenR {
        RomcenR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DMA clock enable"]
    #[inline(always)]
    pub fn dmacen(&mut self) -> DmacenW<'_, AhbcgSpec> {
        DmacenW::new(self, 0)
    }
    #[doc = "Bit 1 - GPIO clock enable"]
    #[inline(always)]
    pub fn gpiocen(&mut self) -> GpiocenW<'_, AhbcgSpec> {
        GpiocenW::new(self, 1)
    }
    #[doc = "Bit 5 - ADC1 clock enable"]
    #[inline(always)]
    pub fn adc1en(&mut self) -> Adc1enW<'_, AhbcgSpec> {
        Adc1enW::new(self, 5)
    }
    #[doc = "Bit 6 - ADC2 clock enable"]
    #[inline(always)]
    pub fn adc2en(&mut self) -> Adc2enW<'_, AhbcgSpec> {
        Adc2enW::new(self, 6)
    }
    #[doc = "Bit 7 - ADC3 clock enable"]
    #[inline(always)]
    pub fn adc3en(&mut self) -> Adc3enW<'_, AhbcgSpec> {
        Adc3enW::new(self, 7)
    }
    #[doc = "Bit 8 - FLASH clock enable"]
    #[inline(always)]
    pub fn flashcen(&mut self) -> FlashcenW<'_, AhbcgSpec> {
        FlashcenW::new(self, 8)
    }
    #[doc = "Bit 9 - ROM clock enable"]
    #[inline(always)]
    pub fn romcen(&mut self) -> RomcenW<'_, AhbcgSpec> {
        RomcenW::new(self, 9)
    }
}
#[doc = "AHB peripheral control register\n\nYou can [`read`](crate::Reg::read) this register and get [`ahbcg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahbcg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AhbcgSpec;
impl crate::RegisterSpec for AhbcgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahbcg::R`](R) reader structure"]
impl crate::Readable for AhbcgSpec {}
#[doc = "`write(|w| ..)` method takes [`ahbcg::W`](W) writer structure"]
impl crate::Writable for AhbcgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AHBCG to value 0x0300"]
impl crate::Resettable for AhbcgSpec {
    const RESET_VALUE: u32 = 0x0300;
}
