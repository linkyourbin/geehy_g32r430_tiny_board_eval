#[doc = "Register `PS` reader"]
pub type R = crate::R<PsSpec>;
#[doc = "Register `PS` writer"]
pub type W = crate::W<PsSpec>;
#[doc = "Field `PS` reader - Prescaler divider"]
pub type PsR = crate::FieldReader;
#[doc = "Field `PS` writer - Prescaler divider"]
pub type PsW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Prescaler divider"]
    #[inline(always)]
    pub fn ps(&self) -> PsR {
        PsR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Prescaler divider"]
    #[inline(always)]
    pub fn ps(&mut self) -> PsW<'_, PsSpec> {
        PsW::new(self, 0)
    }
}
#[doc = "Prescaler register\n\nYou can [`read`](crate::Reg::read) this register and get [`ps::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ps::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PsSpec;
impl crate::RegisterSpec for PsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ps::R`](R) reader structure"]
impl crate::Readable for PsSpec {}
#[doc = "`write(|w| ..)` method takes [`ps::W`](W) writer structure"]
impl crate::Writable for PsSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PS to value 0"]
impl crate::Resettable for PsSpec {}
