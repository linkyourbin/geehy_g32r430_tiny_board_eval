#[doc = "Register `CNTRLD` reader"]
pub type R = crate::R<CntrldSpec>;
#[doc = "Register `CNTRLD` writer"]
pub type W = crate::W<CntrldSpec>;
#[doc = "Field `CNTRLD` reader - Watchdog counter reload value"]
pub type CntrldR = crate::FieldReader<u16>;
#[doc = "Field `CNTRLD` writer - Watchdog counter reload value"]
pub type CntrldW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - Watchdog counter reload value"]
    #[inline(always)]
    pub fn cntrld(&self) -> CntrldR {
        CntrldR::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Watchdog counter reload value"]
    #[inline(always)]
    pub fn cntrld(&mut self) -> CntrldW<'_, CntrldSpec> {
        CntrldW::new(self, 0)
    }
}
#[doc = "Reload register\n\nYou can [`read`](crate::Reg::read) this register and get [`cntrld::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cntrld::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CntrldSpec;
impl crate::RegisterSpec for CntrldSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cntrld::R`](R) reader structure"]
impl crate::Readable for CntrldSpec {}
#[doc = "`write(|w| ..)` method takes [`cntrld::W`](W) writer structure"]
impl crate::Writable for CntrldSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CNTRLD to value 0x0fff"]
impl crate::Resettable for CntrldSpec {
    const RESET_VALUE: u32 = 0x0fff;
}
