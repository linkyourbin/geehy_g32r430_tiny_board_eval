#[doc = "Register `PADDR0` reader"]
pub type R = crate::R<Paddr0Spec>;
#[doc = "Register `PADDR0` writer"]
pub type W = crate::W<Paddr0Spec>;
#[doc = "Field `PADDR` reader - Peripheral address"]
pub type PaddrR = crate::FieldReader<u32>;
#[doc = "Field `PADDR` writer - Peripheral address"]
pub type PaddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Peripheral address"]
    #[inline(always)]
    pub fn paddr(&self) -> PaddrR {
        PaddrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Peripheral address"]
    #[inline(always)]
    pub fn paddr(&mut self) -> PaddrW<'_, Paddr0Spec> {
        PaddrW::new(self, 0)
    }
}
#[doc = "stream x peripheral address register\n\nYou can [`read`](crate::Reg::read) this register and get [`paddr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`paddr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Paddr0Spec;
impl crate::RegisterSpec for Paddr0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`paddr0::R`](R) reader structure"]
impl crate::Readable for Paddr0Spec {}
#[doc = "`write(|w| ..)` method takes [`paddr0::W`](W) writer structure"]
impl crate::Writable for Paddr0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PADDR0 to value 0"]
impl crate::Resettable for Paddr0Spec {}
