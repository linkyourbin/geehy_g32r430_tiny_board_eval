#[doc = "Register `REPCNT` reader"]
pub type R = crate::R<RepcntSpec>;
#[doc = "Register `REPCNT` writer"]
pub type W = crate::W<RepcntSpec>;
#[doc = "Field `REPCNT` reader - Repetition counter value"]
pub type RepcntR = crate::FieldReader;
#[doc = "Field `REPCNT` writer - Repetition counter value"]
pub type RepcntW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Repetition counter value"]
    #[inline(always)]
    pub fn repcnt(&self) -> RepcntR {
        RepcntR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Repetition counter value"]
    #[inline(always)]
    pub fn repcnt(&mut self) -> RepcntW<'_, RepcntSpec> {
        RepcntW::new(self, 0)
    }
}
#[doc = "repetition counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`repcnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`repcnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RepcntSpec;
impl crate::RegisterSpec for RepcntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`repcnt::R`](R) reader structure"]
impl crate::Readable for RepcntSpec {}
#[doc = "`write(|w| ..)` method takes [`repcnt::W`](W) writer structure"]
impl crate::Writable for RepcntSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REPCNT to value 0"]
impl crate::Resettable for RepcntSpec {}
