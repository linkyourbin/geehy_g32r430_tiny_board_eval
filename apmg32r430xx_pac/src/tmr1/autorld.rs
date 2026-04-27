#[doc = "Register `AUTORLD` reader"]
pub type R = crate::R<AutorldSpec>;
#[doc = "Register `AUTORLD` writer"]
pub type W = crate::W<AutorldSpec>;
#[doc = "Field `AUTORLD` reader - Auto-reload value"]
pub type AutorldR = crate::FieldReader<u16>;
#[doc = "Field `AUTORLD` writer - Auto-reload value"]
pub type AutorldW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Auto-reload value"]
    #[inline(always)]
    pub fn autorld(&self) -> AutorldR {
        AutorldR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Auto-reload value"]
    #[inline(always)]
    pub fn autorld(&mut self) -> AutorldW<'_, AutorldSpec> {
        AutorldW::new(self, 0)
    }
}
#[doc = "auto-reload register\n\nYou can [`read`](crate::Reg::read) this register and get [`autorld::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`autorld::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AutorldSpec;
impl crate::RegisterSpec for AutorldSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`autorld::R`](R) reader structure"]
impl crate::Readable for AutorldSpec {}
#[doc = "`write(|w| ..)` method takes [`autorld::W`](W) writer structure"]
impl crate::Writable for AutorldSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AUTORLD to value 0"]
impl crate::Resettable for AutorldSpec {}
