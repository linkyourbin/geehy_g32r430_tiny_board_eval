#[doc = "Register `DHR10R` reader"]
pub type R = crate::R<Dhr10rSpec>;
#[doc = "Register `DHR10R` writer"]
pub type W = crate::W<Dhr10rSpec>;
#[doc = "Field `DHR10R` reader - DAC channel1 10-bit right-aligned data"]
pub type Dhr10rR = crate::FieldReader<u16>;
#[doc = "Field `DHR10R` writer - DAC channel1 10-bit right-aligned data"]
pub type Dhr10rW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - DAC channel1 10-bit right-aligned data"]
    #[inline(always)]
    pub fn dhr10r(&self) -> Dhr10rR {
        Dhr10rR::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - DAC channel1 10-bit right-aligned data"]
    #[inline(always)]
    pub fn dhr10r(&mut self) -> Dhr10rW<'_, Dhr10rSpec> {
        Dhr10rW::new(self, 0)
    }
}
#[doc = "DAC channel1 10-bit right-aligned data holding register(DAC_DH12R1)\n\nYou can [`read`](crate::Reg::read) this register and get [`dhr10r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhr10r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dhr10rSpec;
impl crate::RegisterSpec for Dhr10rSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dhr10r::R`](R) reader structure"]
impl crate::Readable for Dhr10rSpec {}
#[doc = "`write(|w| ..)` method takes [`dhr10r::W`](W) writer structure"]
impl crate::Writable for Dhr10rSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DHR10R to value 0"]
impl crate::Resettable for Dhr10rSpec {}
