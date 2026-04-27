#[doc = "Register `ADCCCR` reader"]
pub type R = crate::R<AdcccrSpec>;
#[doc = "Register `ADCCCR` writer"]
pub type W = crate::W<AdcccrSpec>;
#[doc = "Field `ADC16ADIV` reader - ADC1/2_ANALOG clock (duty ratio 75%), this value can only be changed when ADC01ACLKRDY is 0 and neither ADC1 nor ADC2 modules are enabled,to change the value of ADC16ADIV."]
pub type Adc16adivR = crate::FieldReader;
#[doc = "Field `ADC16ADIV` writer - ADC1/2_ANALOG clock (duty ratio 75%), this value can only be changed when ADC01ACLKRDY is 0 and neither ADC1 nor ADC2 modules are enabled,to change the value of ADC16ADIV."]
pub type Adc16adivW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ADC16ACLKRDY` reader - ADC1/2_ANALOG clock status bit. When ADC1/2 is enabled, you need to wait for this BIT to be 1, then ADC can carry out the conversion."]
pub type Adc16aclkrdyR = crate::BitReader;
#[doc = "Field `ADC16ACLKRDY` writer - ADC1/2_ANALOG clock status bit. When ADC1/2 is enabled, you need to wait for this BIT to be 1, then ADC can carry out the conversion."]
pub type Adc16aclkrdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC12ADIV` reader - The ADC3_ANALOG clock (with a 50% duty cycle), this value can only be changed when ADC3ACLKRDY is 0 and module ADC3 is disabled, then the value of ADC12ADIV can be altered."]
pub type Adc12adivR = crate::FieldReader;
#[doc = "Field `ADC12ADIV` writer - The ADC3_ANALOG clock (with a 50% duty cycle), this value can only be changed when ADC3ACLKRDY is 0 and module ADC3 is disabled, then the value of ADC12ADIV can be altered."]
pub type Adc12adivW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ADC12ACLKRDY` reader - ADC3_ANALOG Clock Status Bit"]
pub type Adc12aclkrdyR = crate::BitReader;
#[doc = "Field `ADCACLKSEL` reader - Clock Source Selection for ADC1/2/3 ANALOG Clock Divider"]
pub type AdcaclkselR = crate::BitReader;
#[doc = "Field `ADCACLKSEL` writer - Clock Source Selection for ADC1/2/3 ANALOG Clock Divider"]
pub type AdcaclkselW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - ADC1/2_ANALOG clock (duty ratio 75%), this value can only be changed when ADC01ACLKRDY is 0 and neither ADC1 nor ADC2 modules are enabled,to change the value of ADC16ADIV."]
    #[inline(always)]
    pub fn adc16adiv(&self) -> Adc16adivR {
        Adc16adivR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - ADC1/2_ANALOG clock status bit. When ADC1/2 is enabled, you need to wait for this BIT to be 1, then ADC can carry out the conversion."]
    #[inline(always)]
    pub fn adc16aclkrdy(&self) -> Adc16aclkrdyR {
        Adc16aclkrdyR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 5:6 - The ADC3_ANALOG clock (with a 50% duty cycle), this value can only be changed when ADC3ACLKRDY is 0 and module ADC3 is disabled, then the value of ADC12ADIV can be altered."]
    #[inline(always)]
    pub fn adc12adiv(&self) -> Adc12adivR {
        Adc12adivR::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - ADC3_ANALOG Clock Status Bit"]
    #[inline(always)]
    pub fn adc12aclkrdy(&self) -> Adc12aclkrdyR {
        Adc12aclkrdyR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Clock Source Selection for ADC1/2/3 ANALOG Clock Divider"]
    #[inline(always)]
    pub fn adcaclksel(&self) -> AdcaclkselR {
        AdcaclkselR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - ADC1/2_ANALOG clock (duty ratio 75%), this value can only be changed when ADC01ACLKRDY is 0 and neither ADC1 nor ADC2 modules are enabled,to change the value of ADC16ADIV."]
    #[inline(always)]
    pub fn adc16adiv(&mut self) -> Adc16adivW<'_, AdcccrSpec> {
        Adc16adivW::new(self, 0)
    }
    #[doc = "Bit 2 - ADC1/2_ANALOG clock status bit. When ADC1/2 is enabled, you need to wait for this BIT to be 1, then ADC can carry out the conversion."]
    #[inline(always)]
    pub fn adc16aclkrdy(&mut self) -> Adc16aclkrdyW<'_, AdcccrSpec> {
        Adc16aclkrdyW::new(self, 2)
    }
    #[doc = "Bits 5:6 - The ADC3_ANALOG clock (with a 50% duty cycle), this value can only be changed when ADC3ACLKRDY is 0 and module ADC3 is disabled, then the value of ADC12ADIV can be altered."]
    #[inline(always)]
    pub fn adc12adiv(&mut self) -> Adc12adivW<'_, AdcccrSpec> {
        Adc12adivW::new(self, 5)
    }
    #[doc = "Bit 8 - Clock Source Selection for ADC1/2/3 ANALOG Clock Divider"]
    #[inline(always)]
    pub fn adcaclksel(&mut self) -> AdcaclkselW<'_, AdcccrSpec> {
        AdcaclkselW::new(self, 8)
    }
}
#[doc = "Analog module control register\n\nYou can [`read`](crate::Reg::read) this register and get [`adcccr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcccr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AdcccrSpec;
impl crate::RegisterSpec for AdcccrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adcccr::R`](R) reader structure"]
impl crate::Readable for AdcccrSpec {}
#[doc = "`write(|w| ..)` method takes [`adcccr::W`](W) writer structure"]
impl crate::Writable for AdcccrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ADCCCR to value 0"]
impl crate::Resettable for AdcccrSpec {}
