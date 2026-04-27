#[doc = "Register `PSCRLDH` writer"]
pub type W = crate::W<PscrldhSpec>;
#[doc = "Field `PSCRLDH` writer - RTC Prescaler Load Register High"]
pub type PscrldhW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl W {
    #[doc = "Bits 0:3 - RTC Prescaler Load Register High"]
    #[inline(always)]
    pub fn pscrldh(&mut self) -> PscrldhW<'_, PscrldhSpec> {
        PscrldhW::new(self, 0)
    }
}
#[doc = "RTC predivision loading register High Bit\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pscrldh::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PscrldhSpec;
impl crate::RegisterSpec for PscrldhSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`pscrldh::W`](W) writer structure"]
impl crate::Writable for PscrldhSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PSCRLDH to value 0"]
impl crate::Resettable for PscrldhSpec {}
