#[doc = "Register `PSCRLDL` writer"]
pub type W = crate::W<PscrldlSpec>;
#[doc = "Field `PSCRLDL` writer - RTC Prescaler Divider Register Low"]
pub type PscrldlW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl W {
    #[doc = "Bits 0:15 - RTC Prescaler Divider Register Low"]
    #[inline(always)]
    pub fn pscrldl(&mut self) -> PscrldlW<'_, PscrldlSpec> {
        PscrldlW::new(self, 0)
    }
}
#[doc = "RTC predivision loading register Low Bit\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pscrldl::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PscrldlSpec;
impl crate::RegisterSpec for PscrldlSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`pscrldl::W`](W) writer structure"]
impl crate::Writable for PscrldlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PSCRLDL to value 0x8000"]
impl crate::Resettable for PscrldlSpec {
    const RESET_VALUE: u32 = 0x8000;
}
