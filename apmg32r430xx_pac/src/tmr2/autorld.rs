#[doc = "Register `AUTORLD` reader"]
pub type R = crate::R<AutorldSpec>;
#[doc = "Register `AUTORLD` writer"]
pub type W = crate::W<AutorldSpec>;
#[doc = "Field `AUTORLD` reader - Auto Reload Value"]
pub type AutorldR = crate::FieldReader<u32>;
#[doc = "Field `AUTORLD` writer - Auto Reload Value"]
pub type AutorldW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Auto Reload Value"]
    #[inline(always)]
    pub fn autorld(&self) -> AutorldR {
        AutorldR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Auto Reload Value"]
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
#[doc = "`reset()` method sets AUTORLD to value 0xffff_ffff"]
impl crate::Resettable for AutorldSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
