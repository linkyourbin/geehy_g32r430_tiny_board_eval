#[doc = "Register `GTPSC` reader"]
pub type R = crate::R<GtpscSpec>;
#[doc = "Register `GTPSC` writer"]
pub type W = crate::W<GtpscSpec>;
#[doc = "Field `PSC` reader - Prescaler value"]
pub type PscR = crate::FieldReader;
#[doc = "Field `PSC` writer - Prescaler value"]
pub type PscW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GRDT` reader - Guard time value"]
pub type GrdtR = crate::FieldReader;
#[doc = "Field `GRDT` writer - Guard time value"]
pub type GrdtW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Prescaler value"]
    #[inline(always)]
    pub fn psc(&self) -> PscR {
        PscR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Guard time value"]
    #[inline(always)]
    pub fn grdt(&self) -> GrdtR {
        GrdtR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Prescaler value"]
    #[inline(always)]
    pub fn psc(&mut self) -> PscW<'_, GtpscSpec> {
        PscW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Guard time value"]
    #[inline(always)]
    pub fn grdt(&mut self) -> GrdtW<'_, GtpscSpec> {
        GrdtW::new(self, 8)
    }
}
#[doc = "Guard time and prescaler register\n\nYou can [`read`](crate::Reg::read) this register and get [`gtpsc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gtpsc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GtpscSpec;
impl crate::RegisterSpec for GtpscSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gtpsc::R`](R) reader structure"]
impl crate::Readable for GtpscSpec {}
#[doc = "`write(|w| ..)` method takes [`gtpsc::W`](W) writer structure"]
impl crate::Writable for GtpscSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GTPSC to value 0"]
impl crate::Resettable for GtpscSpec {}
