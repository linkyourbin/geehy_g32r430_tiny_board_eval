#[doc = "Register `FILTER` reader"]
pub type R = crate::R<FilterSpec>;
#[doc = "Register `FILTER` writer"]
pub type W = crate::W<FilterSpec>;
#[doc = "Field `DNFCFG` reader - Digitial Noise Filter Filtering Capabitity Configure"]
pub type DnfcfgR = crate::FieldReader;
#[doc = "Field `DNFCFG` writer - Digitial Noise Filter Filtering Capabitity Configure"]
pub type DnfcfgW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ANFDIS` reader - Analog Noise Filter Disable"]
pub type AnfdisR = crate::BitReader;
#[doc = "Field `ANFDIS` writer - Analog Noise Filter Disable"]
pub type AnfdisW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - Digitial Noise Filter Filtering Capabitity Configure"]
    #[inline(always)]
    pub fn dnfcfg(&self) -> DnfcfgR {
        DnfcfgR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - Analog Noise Filter Disable"]
    #[inline(always)]
    pub fn anfdis(&self) -> AnfdisR {
        AnfdisR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Digitial Noise Filter Filtering Capabitity Configure"]
    #[inline(always)]
    pub fn dnfcfg(&mut self) -> DnfcfgW<'_, FilterSpec> {
        DnfcfgW::new(self, 0)
    }
    #[doc = "Bit 4 - Analog Noise Filter Disable"]
    #[inline(always)]
    pub fn anfdis(&mut self) -> AnfdisW<'_, FilterSpec> {
        AnfdisW::new(self, 4)
    }
}
#[doc = "Filter control register\n\nYou can [`read`](crate::Reg::read) this register and get [`filter::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`filter::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FilterSpec;
impl crate::RegisterSpec for FilterSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`filter::R`](R) reader structure"]
impl crate::Readable for FilterSpec {}
#[doc = "`write(|w| ..)` method takes [`filter::W`](W) writer structure"]
impl crate::Writable for FilterSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FILTER to value 0"]
impl crate::Resettable for FilterSpec {}
