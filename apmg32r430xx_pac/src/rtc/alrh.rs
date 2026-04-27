#[doc = "Register `ALRH` writer"]
pub type W = crate::W<AlrhSpec>;
#[doc = "Field `ALRH` writer - RTC alarm register high"]
pub type AlrhW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl W {
    #[doc = "Bits 0:15 - RTC alarm register high"]
    #[inline(always)]
    pub fn alrh(&mut self) -> AlrhW<'_, AlrhSpec> {
        AlrhW::new(self, 0)
    }
}
#[doc = "RTC alarm clock register High Bit\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`alrh::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AlrhSpec;
impl crate::RegisterSpec for AlrhSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`alrh::W`](W) writer structure"]
impl crate::Writable for AlrhSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ALRH to value 0xffff"]
impl crate::Resettable for AlrhSpec {
    const RESET_VALUE: u32 = 0xffff;
}
