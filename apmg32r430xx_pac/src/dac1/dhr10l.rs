#[doc = "Register `DHR10L` reader"]
pub type R = crate::R<Dhr10lSpec>;
#[doc = "Register `DHR10L` writer"]
pub type W = crate::W<Dhr10lSpec>;
#[doc = "Field `DHR10L` reader - DAC channel1 10-bit left-aligned data"]
pub type Dhr10lR = crate::FieldReader<u16>;
#[doc = "Field `DHR10L` writer - DAC channel1 10-bit left-aligned data"]
pub type Dhr10lW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 6:15 - DAC channel1 10-bit left-aligned data"]
    #[inline(always)]
    pub fn dhr10l(&self) -> Dhr10lR {
        Dhr10lR::new(((self.bits >> 6) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 6:15 - DAC channel1 10-bit left-aligned data"]
    #[inline(always)]
    pub fn dhr10l(&mut self) -> Dhr10lW<'_, Dhr10lSpec> {
        Dhr10lW::new(self, 6)
    }
}
#[doc = "DAC channel1 10-bit left aligned data holding register (DAC_DH12L1)\n\nYou can [`read`](crate::Reg::read) this register and get [`dhr10l::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhr10l::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dhr10lSpec;
impl crate::RegisterSpec for Dhr10lSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dhr10l::R`](R) reader structure"]
impl crate::Readable for Dhr10lSpec {}
#[doc = "`write(|w| ..)` method takes [`dhr10l::W`](W) writer structure"]
impl crate::Writable for Dhr10lSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DHR10L to value 0"]
impl crate::Resettable for Dhr10lSpec {}
