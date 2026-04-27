#[doc = "Register `JSQR` reader"]
pub type R = crate::R<JsqrSpec>;
#[doc = "Register `JSQR` writer"]
pub type W = crate::W<JsqrSpec>;
#[doc = "Field `JL` reader - ADC group injected sequencer scan length"]
pub type JlR = crate::FieldReader;
#[doc = "Field `JL` writer - ADC group injected sequencer scan length"]
pub type JlW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `JEXTSEL` reader - ADC group injected external trigger source"]
pub type JextselR = crate::FieldReader;
#[doc = "Field `JEXTSEL` writer - ADC group injected external trigger source"]
pub type JextselW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `JEXTEN` reader - ADC group injected external trigger polarity"]
pub type JextenR = crate::FieldReader;
#[doc = "Field `JEXTEN` writer - ADC group injected external trigger polarity"]
pub type JextenW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `JSQ1` reader - ADC group injected sequencer rank 1"]
pub type Jsq1R = crate::FieldReader;
#[doc = "Field `JSQ1` writer - ADC group injected sequencer rank 1"]
pub type Jsq1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `JSQ2` reader - ADC group injected sequencer rank 2"]
pub type Jsq2R = crate::FieldReader;
#[doc = "Field `JSQ2` writer - ADC group injected sequencer rank 2"]
pub type Jsq2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `JSQ3` reader - ADC group injected sequencer rank 3"]
pub type Jsq3R = crate::FieldReader;
#[doc = "Field `JSQ3` writer - ADC group injected sequencer rank 3"]
pub type Jsq3W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `JSQ4` reader - ADC group injected sequencer rank 4"]
pub type Jsq4R = crate::FieldReader;
#[doc = "Field `JSQ4` writer - ADC group injected sequencer rank 4"]
pub type Jsq4W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:1 - ADC group injected sequencer scan length"]
    #[inline(always)]
    pub fn jl(&self) -> JlR {
        JlR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:5 - ADC group injected external trigger source"]
    #[inline(always)]
    pub fn jextsel(&self) -> JextselR {
        JextselR::new(((self.bits >> 2) & 0x0f) as u8)
    }
    #[doc = "Bits 6:7 - ADC group injected external trigger polarity"]
    #[inline(always)]
    pub fn jexten(&self) -> JextenR {
        JextenR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:11 - ADC group injected sequencer rank 1"]
    #[inline(always)]
    pub fn jsq1(&self) -> Jsq1R {
        Jsq1R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - ADC group injected sequencer rank 2"]
    #[inline(always)]
    pub fn jsq2(&self) -> Jsq2R {
        Jsq2R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - ADC group injected sequencer rank 3"]
    #[inline(always)]
    pub fn jsq3(&self) -> Jsq3R {
        Jsq3R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - ADC group injected sequencer rank 4"]
    #[inline(always)]
    pub fn jsq4(&self) -> Jsq4R {
        Jsq4R::new(((self.bits >> 20) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - ADC group injected sequencer scan length"]
    #[inline(always)]
    pub fn jl(&mut self) -> JlW<'_, JsqrSpec> {
        JlW::new(self, 0)
    }
    #[doc = "Bits 2:5 - ADC group injected external trigger source"]
    #[inline(always)]
    pub fn jextsel(&mut self) -> JextselW<'_, JsqrSpec> {
        JextselW::new(self, 2)
    }
    #[doc = "Bits 6:7 - ADC group injected external trigger polarity"]
    #[inline(always)]
    pub fn jexten(&mut self) -> JextenW<'_, JsqrSpec> {
        JextenW::new(self, 6)
    }
    #[doc = "Bits 8:11 - ADC group injected sequencer rank 1"]
    #[inline(always)]
    pub fn jsq1(&mut self) -> Jsq1W<'_, JsqrSpec> {
        Jsq1W::new(self, 8)
    }
    #[doc = "Bits 12:15 - ADC group injected sequencer rank 2"]
    #[inline(always)]
    pub fn jsq2(&mut self) -> Jsq2W<'_, JsqrSpec> {
        Jsq2W::new(self, 12)
    }
    #[doc = "Bits 16:19 - ADC group injected sequencer rank 3"]
    #[inline(always)]
    pub fn jsq3(&mut self) -> Jsq3W<'_, JsqrSpec> {
        Jsq3W::new(self, 16)
    }
    #[doc = "Bits 20:23 - ADC group injected sequencer rank 4"]
    #[inline(always)]
    pub fn jsq4(&mut self) -> Jsq4W<'_, JsqrSpec> {
        Jsq4W::new(self, 20)
    }
}
#[doc = "ADC group injected sequencer register\n\nYou can [`read`](crate::Reg::read) this register and get [`jsqr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`jsqr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct JsqrSpec;
impl crate::RegisterSpec for JsqrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`jsqr::R`](R) reader structure"]
impl crate::Readable for JsqrSpec {}
#[doc = "`write(|w| ..)` method takes [`jsqr::W`](W) writer structure"]
impl crate::Writable for JsqrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets JSQR to value 0"]
impl crate::Resettable for JsqrSpec {}
