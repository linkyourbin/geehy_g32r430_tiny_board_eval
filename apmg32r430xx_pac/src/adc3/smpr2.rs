#[doc = "Register `SMPR2` reader"]
pub type R = crate::R<Smpr2Spec>;
#[doc = "Register `SMPR2` writer"]
pub type W = crate::W<Smpr2Spec>;
#[doc = "Field `SMP8` reader - ADC channel 8 sampling time selection"]
pub type Smp8R = crate::FieldReader;
#[doc = "Field `SMP8` writer - ADC channel 8 sampling time selection"]
pub type Smp8W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SMP9` reader - ADC channel 9 sampling time selection"]
pub type Smp9R = crate::FieldReader;
#[doc = "Field `SMP9` writer - ADC channel 9 sampling time selection"]
pub type Smp9W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SMP10` reader - ADC channel 10 sampling time selection"]
pub type Smp10R = crate::FieldReader;
#[doc = "Field `SMP10` writer - ADC channel 10 sampling time selection"]
pub type Smp10W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SMP11` reader - ADC channel 11 sampling time selection"]
pub type Smp11R = crate::FieldReader;
#[doc = "Field `SMP11` writer - ADC channel 11 sampling time selection"]
pub type Smp11W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SMP12` reader - ADC channel 12 sampling time selection"]
pub type Smp12R = crate::FieldReader;
#[doc = "Field `SMP12` writer - ADC channel 12 sampling time selection"]
pub type Smp12W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SMP13` reader - ADC channel 13 sampling time selection"]
pub type Smp13R = crate::FieldReader;
#[doc = "Field `SMP13` writer - ADC channel 13 sampling time selection"]
pub type Smp13W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SMP14` reader - ADC channel 14 sampling time selection"]
pub type Smp14R = crate::FieldReader;
#[doc = "Field `SMP14` writer - ADC channel 14 sampling time selection"]
pub type Smp14W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SMP15` reader - ADC channel 15 sampling time selection"]
pub type Smp15R = crate::FieldReader;
#[doc = "Field `SMP15` writer - ADC channel 15 sampling time selection"]
pub type Smp15W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - ADC channel 8 sampling time selection"]
    #[inline(always)]
    pub fn smp8(&self) -> Smp8R {
        Smp8R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - ADC channel 9 sampling time selection"]
    #[inline(always)]
    pub fn smp9(&self) -> Smp9R {
        Smp9R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:10 - ADC channel 10 sampling time selection"]
    #[inline(always)]
    pub fn smp10(&self) -> Smp10R {
        Smp10R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:14 - ADC channel 11 sampling time selection"]
    #[inline(always)]
    pub fn smp11(&self) -> Smp11R {
        Smp11R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 16:18 - ADC channel 12 sampling time selection"]
    #[inline(always)]
    pub fn smp12(&self) -> Smp12R {
        Smp12R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 20:22 - ADC channel 13 sampling time selection"]
    #[inline(always)]
    pub fn smp13(&self) -> Smp13R {
        Smp13R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bits 24:26 - ADC channel 14 sampling time selection"]
    #[inline(always)]
    pub fn smp14(&self) -> Smp14R {
        Smp14R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 28:30 - ADC channel 15 sampling time selection"]
    #[inline(always)]
    pub fn smp15(&self) -> Smp15R {
        Smp15R::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - ADC channel 8 sampling time selection"]
    #[inline(always)]
    pub fn smp8(&mut self) -> Smp8W<'_, Smpr2Spec> {
        Smp8W::new(self, 0)
    }
    #[doc = "Bits 4:6 - ADC channel 9 sampling time selection"]
    #[inline(always)]
    pub fn smp9(&mut self) -> Smp9W<'_, Smpr2Spec> {
        Smp9W::new(self, 4)
    }
    #[doc = "Bits 8:10 - ADC channel 10 sampling time selection"]
    #[inline(always)]
    pub fn smp10(&mut self) -> Smp10W<'_, Smpr2Spec> {
        Smp10W::new(self, 8)
    }
    #[doc = "Bits 12:14 - ADC channel 11 sampling time selection"]
    #[inline(always)]
    pub fn smp11(&mut self) -> Smp11W<'_, Smpr2Spec> {
        Smp11W::new(self, 12)
    }
    #[doc = "Bits 16:18 - ADC channel 12 sampling time selection"]
    #[inline(always)]
    pub fn smp12(&mut self) -> Smp12W<'_, Smpr2Spec> {
        Smp12W::new(self, 16)
    }
    #[doc = "Bits 20:22 - ADC channel 13 sampling time selection"]
    #[inline(always)]
    pub fn smp13(&mut self) -> Smp13W<'_, Smpr2Spec> {
        Smp13W::new(self, 20)
    }
    #[doc = "Bits 24:26 - ADC channel 14 sampling time selection"]
    #[inline(always)]
    pub fn smp14(&mut self) -> Smp14W<'_, Smpr2Spec> {
        Smp14W::new(self, 24)
    }
    #[doc = "Bits 28:30 - ADC channel 15 sampling time selection"]
    #[inline(always)]
    pub fn smp15(&mut self) -> Smp15W<'_, Smpr2Spec> {
        Smp15W::new(self, 28)
    }
}
#[doc = "ADC sampling time register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`smpr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smpr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Smpr2Spec;
impl crate::RegisterSpec for Smpr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`smpr2::R`](R) reader structure"]
impl crate::Readable for Smpr2Spec {}
#[doc = "`write(|w| ..)` method takes [`smpr2::W`](W) writer structure"]
impl crate::Writable for Smpr2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SMPR2 to value 0"]
impl crate::Resettable for Smpr2Spec {}
