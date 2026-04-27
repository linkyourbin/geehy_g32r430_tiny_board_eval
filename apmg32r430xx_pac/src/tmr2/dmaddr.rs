#[doc = "Register `DMADDR` reader"]
pub type R = crate::R<DmaddrSpec>;
#[doc = "Register `DMADDR` writer"]
pub type W = crate::W<DmaddrSpec>;
#[doc = "Field `DMADDR` reader - DMA register for burst accesses"]
pub type DmaddrR = crate::FieldReader<u16>;
#[doc = "Field `DMADDR` writer - DMA register for burst accesses"]
pub type DmaddrW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - DMA register for burst accesses"]
    #[inline(always)]
    pub fn dmaddr(&self) -> DmaddrR {
        DmaddrR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - DMA register for burst accesses"]
    #[inline(always)]
    pub fn dmaddr(&mut self) -> DmaddrW<'_, DmaddrSpec> {
        DmaddrW::new(self, 0)
    }
}
#[doc = "DMA address for full transfer\n\nYou can [`read`](crate::Reg::read) this register and get [`dmaddr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmaddr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaddrSpec;
impl crate::RegisterSpec for DmaddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmaddr::R`](R) reader structure"]
impl crate::Readable for DmaddrSpec {}
#[doc = "`write(|w| ..)` method takes [`dmaddr::W`](W) writer structure"]
impl crate::Writable for DmaddrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMADDR to value 0"]
impl crate::Resettable for DmaddrSpec {}
