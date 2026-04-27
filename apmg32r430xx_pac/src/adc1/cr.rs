#[doc = "Register `CR` reader"]
pub type R = crate::R<CrSpec>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CrSpec>;
#[doc = "Field `ADEN` reader - ADC ENABLE CONTROL"]
pub type AdenR = crate::BitReader;
#[doc = "Field `ADEN` writer - ADC ENABLE CONTROL"]
pub type AdenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDIS` reader - ADC DISABLE COMMAND"]
pub type AddisR = crate::BitReader;
#[doc = "Field `ADDIS` writer - ADC DISABLE COMMAND"]
pub type AddisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADSTART` reader - ADC START OF REGULAR CONVERSION"]
pub type AdstartR = crate::BitReader;
#[doc = "Field `ADSTART` writer - ADC START OF REGULAR CONVERSION"]
pub type AdstartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `JADSTART` reader - ADC START OF INJECTED CONVERSION"]
pub type JadstartR = crate::BitReader;
#[doc = "Field `JADSTART` writer - ADC START OF INJECTED CONVERSION"]
pub type JadstartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADSTP` reader - ADC STOP OF REGULAR CONVERSION COMMAND"]
pub type AdstpR = crate::BitReader;
#[doc = "Field `ADSTP` writer - ADC STOP OF REGULAR CONVERSION COMMAND"]
pub type AdstpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `JADSTP` reader - ADC STOP OF INJECTED CONVERSION COMMAND"]
pub type JadstpR = crate::BitReader;
#[doc = "Field `JADSTP` writer - ADC STOP OF INJECTED CONVERSION COMMAND"]
pub type JadstpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DUALMOD` reader - DUAL MODE SELECTION"]
pub type DualmodR = crate::FieldReader;
#[doc = "Field `DUALMOD` writer - DUAL MODE SELECTION"]
pub type DualmodW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0 - ADC ENABLE CONTROL"]
    #[inline(always)]
    pub fn aden(&self) -> AdenR {
        AdenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ADC DISABLE COMMAND"]
    #[inline(always)]
    pub fn addis(&self) -> AddisR {
        AddisR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ADC START OF REGULAR CONVERSION"]
    #[inline(always)]
    pub fn adstart(&self) -> AdstartR {
        AdstartR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - ADC START OF INJECTED CONVERSION"]
    #[inline(always)]
    pub fn jadstart(&self) -> JadstartR {
        JadstartR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - ADC STOP OF REGULAR CONVERSION COMMAND"]
    #[inline(always)]
    pub fn adstp(&self) -> AdstpR {
        AdstpR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - ADC STOP OF INJECTED CONVERSION COMMAND"]
    #[inline(always)]
    pub fn jadstp(&self) -> JadstpR {
        JadstpR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:9 - DUAL MODE SELECTION"]
    #[inline(always)]
    pub fn dualmod(&self) -> DualmodR {
        DualmodR::new(((self.bits >> 6) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - ADC ENABLE CONTROL"]
    #[inline(always)]
    pub fn aden(&mut self) -> AdenW<'_, CrSpec> {
        AdenW::new(self, 0)
    }
    #[doc = "Bit 1 - ADC DISABLE COMMAND"]
    #[inline(always)]
    pub fn addis(&mut self) -> AddisW<'_, CrSpec> {
        AddisW::new(self, 1)
    }
    #[doc = "Bit 2 - ADC START OF REGULAR CONVERSION"]
    #[inline(always)]
    pub fn adstart(&mut self) -> AdstartW<'_, CrSpec> {
        AdstartW::new(self, 2)
    }
    #[doc = "Bit 3 - ADC START OF INJECTED CONVERSION"]
    #[inline(always)]
    pub fn jadstart(&mut self) -> JadstartW<'_, CrSpec> {
        JadstartW::new(self, 3)
    }
    #[doc = "Bit 4 - ADC STOP OF REGULAR CONVERSION COMMAND"]
    #[inline(always)]
    pub fn adstp(&mut self) -> AdstpW<'_, CrSpec> {
        AdstpW::new(self, 4)
    }
    #[doc = "Bit 5 - ADC STOP OF INJECTED CONVERSION COMMAND"]
    #[inline(always)]
    pub fn jadstp(&mut self) -> JadstpW<'_, CrSpec> {
        JadstpW::new(self, 5)
    }
    #[doc = "Bits 6:9 - DUAL MODE SELECTION"]
    #[inline(always)]
    pub fn dualmod(&mut self) -> DualmodW<'_, CrSpec> {
        DualmodW::new(self, 6)
    }
}
#[doc = "ADC control register\n\nYou can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrSpec;
impl crate::RegisterSpec for CrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr::R`](R) reader structure"]
impl crate::Readable for CrSpec {}
#[doc = "`write(|w| ..)` method takes [`cr::W`](W) writer structure"]
impl crate::Writable for CrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CrSpec {}
