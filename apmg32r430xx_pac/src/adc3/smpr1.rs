#[doc = "Register `SMPR1` reader"]
pub type R = crate::R<Smpr1Spec>;
#[doc = "Register `SMPR1` writer"]
pub type W = crate::W<Smpr1Spec>;
#[doc = "Field `SMP0` reader - ADC channel 0 sampling time selection"]
pub type Smp0R = crate::FieldReader;
#[doc = "Field `SMP0` writer - ADC channel 0 sampling time selection"]
pub type Smp0W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SMP1` reader - ADC channel 1 sampling time selection"]
pub type Smp1R = crate::FieldReader;
#[doc = "Field `SMP1` writer - ADC channel 1 sampling time selection"]
pub type Smp1W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SMP2` reader - ADC channel 2 sampling time selection"]
pub type Smp2R = crate::FieldReader;
#[doc = "Field `SMP2` writer - ADC channel 2 sampling time selection"]
pub type Smp2W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SMP3` reader - ADC channel 3 sampling time selection"]
pub type Smp3R = crate::FieldReader;
#[doc = "Field `SMP3` writer - ADC channel 3 sampling time selection"]
pub type Smp3W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SMP4` reader - ADC channel 4 sampling time selection"]
pub type Smp4R = crate::FieldReader;
#[doc = "Field `SMP4` writer - ADC channel 4 sampling time selection"]
pub type Smp4W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SMP5` reader - ADC channel 5 sampling time selection"]
pub type Smp5R = crate::FieldReader;
#[doc = "Field `SMP5` writer - ADC channel 5 sampling time selection"]
pub type Smp5W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SMP6` reader - ADC channel 6 sampling time selection"]
pub type Smp6R = crate::FieldReader;
#[doc = "Field `SMP6` writer - ADC channel 6 sampling time selection"]
pub type Smp6W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SMP7` reader - ADC channel 7 sampling time selection"]
pub type Smp7R = crate::FieldReader;
#[doc = "Field `SMP7` writer - ADC channel 7 sampling time selection"]
pub type Smp7W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - ADC channel 0 sampling time selection"]
    #[inline(always)]
    pub fn smp0(&self) -> Smp0R {
        Smp0R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - ADC channel 1 sampling time selection"]
    #[inline(always)]
    pub fn smp1(&self) -> Smp1R {
        Smp1R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:10 - ADC channel 2 sampling time selection"]
    #[inline(always)]
    pub fn smp2(&self) -> Smp2R {
        Smp2R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:14 - ADC channel 3 sampling time selection"]
    #[inline(always)]
    pub fn smp3(&self) -> Smp3R {
        Smp3R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 16:18 - ADC channel 4 sampling time selection"]
    #[inline(always)]
    pub fn smp4(&self) -> Smp4R {
        Smp4R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 20:22 - ADC channel 5 sampling time selection"]
    #[inline(always)]
    pub fn smp5(&self) -> Smp5R {
        Smp5R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bits 24:26 - ADC channel 6 sampling time selection"]
    #[inline(always)]
    pub fn smp6(&self) -> Smp6R {
        Smp6R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 28:30 - ADC channel 7 sampling time selection"]
    #[inline(always)]
    pub fn smp7(&self) -> Smp7R {
        Smp7R::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - ADC channel 0 sampling time selection"]
    #[inline(always)]
    pub fn smp0(&mut self) -> Smp0W<'_, Smpr1Spec> {
        Smp0W::new(self, 0)
    }
    #[doc = "Bits 4:6 - ADC channel 1 sampling time selection"]
    #[inline(always)]
    pub fn smp1(&mut self) -> Smp1W<'_, Smpr1Spec> {
        Smp1W::new(self, 4)
    }
    #[doc = "Bits 8:10 - ADC channel 2 sampling time selection"]
    #[inline(always)]
    pub fn smp2(&mut self) -> Smp2W<'_, Smpr1Spec> {
        Smp2W::new(self, 8)
    }
    #[doc = "Bits 12:14 - ADC channel 3 sampling time selection"]
    #[inline(always)]
    pub fn smp3(&mut self) -> Smp3W<'_, Smpr1Spec> {
        Smp3W::new(self, 12)
    }
    #[doc = "Bits 16:18 - ADC channel 4 sampling time selection"]
    #[inline(always)]
    pub fn smp4(&mut self) -> Smp4W<'_, Smpr1Spec> {
        Smp4W::new(self, 16)
    }
    #[doc = "Bits 20:22 - ADC channel 5 sampling time selection"]
    #[inline(always)]
    pub fn smp5(&mut self) -> Smp5W<'_, Smpr1Spec> {
        Smp5W::new(self, 20)
    }
    #[doc = "Bits 24:26 - ADC channel 6 sampling time selection"]
    #[inline(always)]
    pub fn smp6(&mut self) -> Smp6W<'_, Smpr1Spec> {
        Smp6W::new(self, 24)
    }
    #[doc = "Bits 28:30 - ADC channel 7 sampling time selection"]
    #[inline(always)]
    pub fn smp7(&mut self) -> Smp7W<'_, Smpr1Spec> {
        Smp7W::new(self, 28)
    }
}
#[doc = "ADC sampling time register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`smpr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smpr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Smpr1Spec;
impl crate::RegisterSpec for Smpr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`smpr1::R`](R) reader structure"]
impl crate::Readable for Smpr1Spec {}
#[doc = "`write(|w| ..)` method takes [`smpr1::W`](W) writer structure"]
impl crate::Writable for Smpr1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SMPR1 to value 0"]
impl crate::Resettable for Smpr1Spec {}
