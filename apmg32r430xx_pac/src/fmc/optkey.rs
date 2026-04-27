#[doc = "Register `OPTKEY` writer"]
pub type W = crate::W<OptkeySpec>;
#[doc = "Field `OPTKEY` writer - Option byte key"]
pub type OptkeyW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - Option byte key"]
    #[inline(always)]
    pub fn optkey(&mut self) -> OptkeyW<'_, OptkeySpec> {
        OptkeyW::new(self, 0)
    }
}
#[doc = "option byte key register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`optkey::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OptkeySpec;
impl crate::RegisterSpec for OptkeySpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`optkey::W`](W) writer structure"]
impl crate::Writable for OptkeySpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OPTKEY to value 0"]
impl crate::Resettable for OptkeySpec {}
