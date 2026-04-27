#[doc = "Register `STMODR` reader"]
pub type R = crate::R<StmodrSpec>;
#[doc = "Register `STMODR` writer"]
pub type W = crate::W<StmodrSpec>;
#[doc = "Field `STRSTTRIGSEL` reader - DAC channel sawtooth wave reset trigger selection."]
pub type StrsttrigselR = crate::FieldReader;
#[doc = "Field `STRSTTRIGSEL` writer - DAC channel sawtooth wave reset trigger selection."]
pub type StrsttrigselW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `STINCTRIGSEL` reader - DAC Channel Sawtooth Wave Incremental Trigger Selection"]
pub type StinctrigselR = crate::FieldReader;
#[doc = "Field `STINCTRIGSEL` writer - DAC Channel Sawtooth Wave Incremental Trigger Selection"]
pub type StinctrigselW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - DAC channel sawtooth wave reset trigger selection."]
    #[inline(always)]
    pub fn strsttrigsel(&self) -> StrsttrigselR {
        StrsttrigselR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 8:10 - DAC Channel Sawtooth Wave Incremental Trigger Selection"]
    #[inline(always)]
    pub fn stinctrigsel(&self) -> StinctrigselR {
        StinctrigselR::new(((self.bits >> 8) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - DAC channel sawtooth wave reset trigger selection."]
    #[inline(always)]
    pub fn strsttrigsel(&mut self) -> StrsttrigselW<'_, StmodrSpec> {
        StrsttrigselW::new(self, 0)
    }
    #[doc = "Bits 8:10 - DAC Channel Sawtooth Wave Incremental Trigger Selection"]
    #[inline(always)]
    pub fn stinctrigsel(&mut self) -> StinctrigselW<'_, StmodrSpec> {
        StinctrigselW::new(self, 8)
    }
}
#[doc = "DAC channel sawtooth wave trigger register\n\nYou can [`read`](crate::Reg::read) this register and get [`stmodr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stmodr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StmodrSpec;
impl crate::RegisterSpec for StmodrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stmodr::R`](R) reader structure"]
impl crate::Readable for StmodrSpec {}
#[doc = "`write(|w| ..)` method takes [`stmodr::W`](W) writer structure"]
impl crate::Writable for StmodrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets STMODR to value 0"]
impl crate::Resettable for StmodrSpec {}
