#[doc = "Register `SQR1` reader"]
pub type R = crate::R<Sqr1Spec>;
#[doc = "Register `SQR1` writer"]
pub type W = crate::W<Sqr1Spec>;
#[doc = "Field `RL` reader - ADC group regular sequencer scan length"]
pub type RlR = crate::FieldReader;
#[doc = "Field `RL` writer - ADC group regular sequencer scan length"]
pub type RlW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SQ1` reader - ADC group regular sequencer rank 1"]
pub type Sq1R = crate::FieldReader;
#[doc = "Field `SQ1` writer - ADC group regular sequencer rank 1"]
pub type Sq1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SQ2` reader - ADC group regular sequencer rank 2"]
pub type Sq2R = crate::FieldReader;
#[doc = "Field `SQ2` writer - ADC group regular sequencer rank 2"]
pub type Sq2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SQ3` reader - ADC group regular sequencer rank 3"]
pub type Sq3R = crate::FieldReader;
#[doc = "Field `SQ3` writer - ADC group regular sequencer rank 3"]
pub type Sq3W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SQ4` reader - ADC group regular sequencer rank 4"]
pub type Sq4R = crate::FieldReader;
#[doc = "Field `SQ4` writer - ADC group regular sequencer rank 4"]
pub type Sq4W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SQ5` reader - ADC group regular sequencer rank 5"]
pub type Sq5R = crate::FieldReader;
#[doc = "Field `SQ5` writer - ADC group regular sequencer rank 5"]
pub type Sq5W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SQ6` reader - ADC group regular sequencer rank 6"]
pub type Sq6R = crate::FieldReader;
#[doc = "Field `SQ6` writer - ADC group regular sequencer rank 6"]
pub type Sq6W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SQ7` reader - ADC group regular sequencer rank 7"]
pub type Sq7R = crate::FieldReader;
#[doc = "Field `SQ7` writer - ADC group regular sequencer rank 7"]
pub type Sq7W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - ADC group regular sequencer scan length"]
    #[inline(always)]
    pub fn rl(&self) -> RlR {
        RlR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - ADC group regular sequencer rank 1"]
    #[inline(always)]
    pub fn sq1(&self) -> Sq1R {
        Sq1R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - ADC group regular sequencer rank 2"]
    #[inline(always)]
    pub fn sq2(&self) -> Sq2R {
        Sq2R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - ADC group regular sequencer rank 3"]
    #[inline(always)]
    pub fn sq3(&self) -> Sq3R {
        Sq3R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - ADC group regular sequencer rank 4"]
    #[inline(always)]
    pub fn sq4(&self) -> Sq4R {
        Sq4R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - ADC group regular sequencer rank 5"]
    #[inline(always)]
    pub fn sq5(&self) -> Sq5R {
        Sq5R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - ADC group regular sequencer rank 6"]
    #[inline(always)]
    pub fn sq6(&self) -> Sq6R {
        Sq6R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - ADC group regular sequencer rank 7"]
    #[inline(always)]
    pub fn sq7(&self) -> Sq7R {
        Sq7R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - ADC group regular sequencer scan length"]
    #[inline(always)]
    pub fn rl(&mut self) -> RlW<'_, Sqr1Spec> {
        RlW::new(self, 0)
    }
    #[doc = "Bits 4:7 - ADC group regular sequencer rank 1"]
    #[inline(always)]
    pub fn sq1(&mut self) -> Sq1W<'_, Sqr1Spec> {
        Sq1W::new(self, 4)
    }
    #[doc = "Bits 8:11 - ADC group regular sequencer rank 2"]
    #[inline(always)]
    pub fn sq2(&mut self) -> Sq2W<'_, Sqr1Spec> {
        Sq2W::new(self, 8)
    }
    #[doc = "Bits 12:15 - ADC group regular sequencer rank 3"]
    #[inline(always)]
    pub fn sq3(&mut self) -> Sq3W<'_, Sqr1Spec> {
        Sq3W::new(self, 12)
    }
    #[doc = "Bits 16:19 - ADC group regular sequencer rank 4"]
    #[inline(always)]
    pub fn sq4(&mut self) -> Sq4W<'_, Sqr1Spec> {
        Sq4W::new(self, 16)
    }
    #[doc = "Bits 20:23 - ADC group regular sequencer rank 5"]
    #[inline(always)]
    pub fn sq5(&mut self) -> Sq5W<'_, Sqr1Spec> {
        Sq5W::new(self, 20)
    }
    #[doc = "Bits 24:27 - ADC group regular sequencer rank 6"]
    #[inline(always)]
    pub fn sq6(&mut self) -> Sq6W<'_, Sqr1Spec> {
        Sq6W::new(self, 24)
    }
    #[doc = "Bits 28:31 - ADC group regular sequencer rank 7"]
    #[inline(always)]
    pub fn sq7(&mut self) -> Sq7W<'_, Sqr1Spec> {
        Sq7W::new(self, 28)
    }
}
#[doc = "ADC group regular sequencer register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`sqr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sqr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sqr1Spec;
impl crate::RegisterSpec for Sqr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sqr1::R`](R) reader structure"]
impl crate::Readable for Sqr1Spec {}
#[doc = "`write(|w| ..)` method takes [`sqr1::W`](W) writer structure"]
impl crate::Writable for Sqr1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SQR1 to value 0"]
impl crate::Resettable for Sqr1Spec {}
