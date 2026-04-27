#[doc = "Register `OP` reader"]
pub type R = crate::R<OpSpec>;
#[doc = "Register `OP` writer"]
pub type W = crate::W<OpSpec>;
#[doc = "Field `INTRIGRMP` reader - Timer Input 4 remap"]
pub type IntrigrmpR = crate::FieldReader;
#[doc = "Field `INTRIGRMP` writer - Timer Input 4 remap"]
pub type IntrigrmpW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 10:11 - Timer Input 4 remap"]
    #[inline(always)]
    pub fn intrigrmp(&self) -> IntrigrmpR {
        IntrigrmpR::new(((self.bits >> 10) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 10:11 - Timer Input 4 remap"]
    #[inline(always)]
    pub fn intrigrmp(&mut self) -> IntrigrmpW<'_, OpSpec> {
        IntrigrmpW::new(self, 10)
    }
}
#[doc = "TMR4 option register\n\nYou can [`read`](crate::Reg::read) this register and get [`op::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`op::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OpSpec;
impl crate::RegisterSpec for OpSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`op::R`](R) reader structure"]
impl crate::Readable for OpSpec {}
#[doc = "`write(|w| ..)` method takes [`op::W`](W) writer structure"]
impl crate::Writable for OpSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OP to value 0"]
impl crate::Resettable for OpSpec {}
