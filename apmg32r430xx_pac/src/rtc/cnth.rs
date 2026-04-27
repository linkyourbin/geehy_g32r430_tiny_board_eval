#[doc = "Register `CNTH` reader"]
pub type R = crate::R<CnthSpec>;
#[doc = "Register `CNTH` writer"]
pub type W = crate::W<CnthSpec>;
#[doc = "Field `CNTH` reader - RTC counter register high"]
pub type CnthR = crate::FieldReader<u16>;
#[doc = "Field `CNTH` writer - RTC counter register high"]
pub type CnthW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - RTC counter register high"]
    #[inline(always)]
    pub fn cnth(&self) -> CnthR {
        CnthR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - RTC counter register high"]
    #[inline(always)]
    pub fn cnth(&mut self) -> CnthW<'_, CnthSpec> {
        CnthW::new(self, 0)
    }
}
#[doc = "RTC count register High Bit\n\nYou can [`read`](crate::Reg::read) this register and get [`cnth::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cnth::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CnthSpec;
impl crate::RegisterSpec for CnthSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cnth::R`](R) reader structure"]
impl crate::Readable for CnthSpec {}
#[doc = "`write(|w| ..)` method takes [`cnth::W`](W) writer structure"]
impl crate::Writable for CnthSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CNTH to value 0"]
impl crate::Resettable for CnthSpec {}
