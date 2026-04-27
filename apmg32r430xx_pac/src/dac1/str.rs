#[doc = "Register `STR` reader"]
pub type R = crate::R<StrSpec>;
#[doc = "Register `STR` writer"]
pub type W = crate::W<StrSpec>;
#[doc = "Field `STRSTDATA` reader - DAC channel sawtooth wave reset value"]
pub type StrstdataR = crate::FieldReader<u16>;
#[doc = "Field `STRSTDATA` writer - DAC channel sawtooth wave reset value"]
pub type StrstdataW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `STDIR` reader - DAC channel sawtooth wave direction setting"]
pub type StdirR = crate::BitReader;
#[doc = "Field `STDIR` writer - DAC channel sawtooth wave direction setting"]
pub type StdirW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STINCDATA` reader - DAC channel sawtooth wave increment value"]
pub type StincdataR = crate::FieldReader<u16>;
#[doc = "Field `STINCDATA` writer - DAC channel sawtooth wave increment value"]
pub type StincdataW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - DAC channel sawtooth wave reset value"]
    #[inline(always)]
    pub fn strstdata(&self) -> StrstdataR {
        StrstdataR::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 12 - DAC channel sawtooth wave direction setting"]
    #[inline(always)]
    pub fn stdir(&self) -> StdirR {
        StdirR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 22:31 - DAC channel sawtooth wave increment value"]
    #[inline(always)]
    pub fn stincdata(&self) -> StincdataR {
        StincdataR::new(((self.bits >> 22) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - DAC channel sawtooth wave reset value"]
    #[inline(always)]
    pub fn strstdata(&mut self) -> StrstdataW<'_, StrSpec> {
        StrstdataW::new(self, 0)
    }
    #[doc = "Bit 12 - DAC channel sawtooth wave direction setting"]
    #[inline(always)]
    pub fn stdir(&mut self) -> StdirW<'_, StrSpec> {
        StdirW::new(self, 12)
    }
    #[doc = "Bits 22:31 - DAC channel sawtooth wave increment value"]
    #[inline(always)]
    pub fn stincdata(&mut self) -> StincdataW<'_, StrSpec> {
        StincdataW::new(self, 22)
    }
}
#[doc = "DAC channel sawtooth wave register.\n\nYou can [`read`](crate::Reg::read) this register and get [`str::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`str::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StrSpec;
impl crate::RegisterSpec for StrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`str::R`](R) reader structure"]
impl crate::Readable for StrSpec {}
#[doc = "`write(|w| ..)` method takes [`str::W`](W) writer structure"]
impl crate::Writable for StrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets STR to value 0"]
impl crate::Resettable for StrSpec {}
