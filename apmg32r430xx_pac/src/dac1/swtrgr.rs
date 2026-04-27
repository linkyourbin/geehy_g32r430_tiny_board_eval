#[doc = "Register `SWTRGR` writer"]
pub type W = crate::W<SwtrgrSpec>;
#[doc = "Field `SWTRIG` writer - DAC channel software trigger"]
pub type SwtrigW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWTRIGB` writer - DAC channel software b trigger"]
pub type SwtrigbW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - DAC channel software trigger"]
    #[inline(always)]
    pub fn swtrig(&mut self) -> SwtrigW<'_, SwtrgrSpec> {
        SwtrigW::new(self, 0)
    }
    #[doc = "Bit 16 - DAC channel software b trigger"]
    #[inline(always)]
    pub fn swtrigb(&mut self) -> SwtrigbW<'_, SwtrgrSpec> {
        SwtrigbW::new(self, 16)
    }
}
#[doc = "DAC software trigger register(DAC_SWTRG)\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swtrgr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SwtrgrSpec;
impl crate::RegisterSpec for SwtrgrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`swtrgr::W`](W) writer structure"]
impl crate::Writable for SwtrgrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SWTRGR to value 0"]
impl crate::Resettable for SwtrgrSpec {}
