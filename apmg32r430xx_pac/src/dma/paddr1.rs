#[doc = "Register `PADDR1` reader"]
pub type R = crate::R<Paddr1Spec>;
#[doc = "Register `PADDR1` writer"]
pub type W = crate::W<Paddr1Spec>;
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
    pub fn paddr(&mut self) -> PaddrW<'_, Paddr1Spec> {
        PaddrW::new(self, 0)
    }
}
#[doc = "stream x peripheral address register\n\nYou can [`read`](crate::Reg::read) this register and get [`paddr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`paddr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Paddr1Spec;
impl crate::RegisterSpec for Paddr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`paddr1::R`](R) reader structure"]
impl crate::Readable for Paddr1Spec {}
#[doc = "`write(|w| ..)` method takes [`paddr1::W`](W) writer structure"]
impl crate::Writable for Paddr1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PADDR1 to value 0"]
impl crate::Resettable for Paddr1Spec {}
