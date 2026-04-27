#[doc = "Register `OBCR2` reader"]
pub type R = crate::R<Obcr2Spec>;
#[doc = "Register `OBCR2` writer"]
pub type W = crate::W<Obcr2Spec>;
#[doc = "Field `BOOTADDRWBF` reader - BOOTADDRWBF"]
pub type BootaddrwbfR = crate::FieldReader<u16>;
#[doc = "Field `BOOTADDRWBF` writer - BOOTADDRWBF"]
pub type BootaddrwbfW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - BOOTADDRWBF"]
    #[inline(always)]
    pub fn bootaddrwbf(&self) -> BootaddrwbfR {
        BootaddrwbfR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - BOOTADDRWBF"]
    #[inline(always)]
    pub fn bootaddrwbf(&mut self) -> BootaddrwbfW<'_, Obcr2Spec> {
        BootaddrwbfW::new(self, 0)
    }
}
#[doc = "Option Byte Control Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`obcr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`obcr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Obcr2Spec;
impl crate::RegisterSpec for Obcr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`obcr2::R`](R) reader structure"]
impl crate::Readable for Obcr2Spec {}
#[doc = "`write(|w| ..)` method takes [`obcr2::W`](W) writer structure"]
impl crate::Writable for Obcr2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OBCR2 to value 0"]
impl crate::Resettable for Obcr2Spec {}
