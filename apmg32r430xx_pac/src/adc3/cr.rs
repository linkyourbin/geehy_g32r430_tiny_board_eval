#[doc = "Register `CR` reader"]
pub type R = crate::R<CrSpec>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CrSpec>;
#[doc = "Field `ADEN` reader - ADC enable"]
pub type AdenR = crate::BitReader;
#[doc = "Field `ADEN` writer - ADC enable"]
pub type AdenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADSTART` reader - ADC group regular conversion start"]
pub type AdstartR = crate::BitReader;
#[doc = "Field `ADSTART` writer - ADC group regular conversion start"]
pub type AdstartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `JADSTART` reader - ADC group injected conversion start"]
pub type JadstartR = crate::BitReader;
#[doc = "Field `JADSTART` writer - ADC group injected conversion start"]
pub type JadstartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADSTP` reader - ADC group regular conversion stop"]
pub type AdstpR = crate::BitReader;
#[doc = "Field `ADSTP` writer - ADC group regular conversion stop"]
pub type AdstpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `JADSTP` reader - ADC group injected conversion stop"]
pub type JadstpR = crate::BitReader;
#[doc = "Field `JADSTP` writer - ADC group injected conversion stop"]
pub type JadstpW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - ADC enable"]
    #[inline(always)]
    pub fn aden(&self) -> AdenR {
        AdenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - ADC group regular conversion start"]
    #[inline(always)]
    pub fn adstart(&self) -> AdstartR {
        AdstartR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - ADC group injected conversion start"]
    #[inline(always)]
    pub fn jadstart(&self) -> JadstartR {
        JadstartR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - ADC group regular conversion stop"]
    #[inline(always)]
    pub fn adstp(&self) -> AdstpR {
        AdstpR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - ADC group injected conversion stop"]
    #[inline(always)]
    pub fn jadstp(&self) -> JadstpR {
        JadstpR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ADC enable"]
    #[inline(always)]
    pub fn aden(&mut self) -> AdenW<'_, CrSpec> {
        AdenW::new(self, 0)
    }
    #[doc = "Bit 2 - ADC group regular conversion start"]
    #[inline(always)]
    pub fn adstart(&mut self) -> AdstartW<'_, CrSpec> {
        AdstartW::new(self, 2)
    }
    #[doc = "Bit 3 - ADC group injected conversion start"]
    #[inline(always)]
    pub fn jadstart(&mut self) -> JadstartW<'_, CrSpec> {
        JadstartW::new(self, 3)
    }
    #[doc = "Bit 4 - ADC group regular conversion stop"]
    #[inline(always)]
    pub fn adstp(&mut self) -> AdstpW<'_, CrSpec> {
        AdstpW::new(self, 4)
    }
    #[doc = "Bit 5 - ADC group injected conversion stop"]
    #[inline(always)]
    pub fn jadstp(&mut self) -> JadstpW<'_, CrSpec> {
        JadstpW::new(self, 5)
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
