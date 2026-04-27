#[doc = "Register `RISETMAX` reader"]
pub type R = crate::R<RisetmaxSpec>;
#[doc = "Register `RISETMAX` writer"]
pub type W = crate::W<RisetmaxSpec>;
#[doc = "Field `RISETMAX` reader - Maximum rise time in Fast/Standard mode (Master mode)"]
pub type RisetmaxR = crate::FieldReader;
#[doc = "Field `RISETMAX` writer - Maximum rise time in Fast/Standard mode (Master mode)"]
pub type RisetmaxW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - Maximum rise time in Fast/Standard mode (Master mode)"]
    #[inline(always)]
    pub fn risetmax(&self) -> RisetmaxR {
        RisetmaxR::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Maximum rise time in Fast/Standard mode (Master mode)"]
    #[inline(always)]
    pub fn risetmax(&mut self) -> RisetmaxW<'_, RisetmaxSpec> {
        RisetmaxW::new(self, 0)
    }
}
#[doc = "Maximum Rise Time register\n\nYou can [`read`](crate::Reg::read) this register and get [`risetmax::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`risetmax::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RisetmaxSpec;
impl crate::RegisterSpec for RisetmaxSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`risetmax::R`](R) reader structure"]
impl crate::Readable for RisetmaxSpec {}
#[doc = "`write(|w| ..)` method takes [`risetmax::W`](W) writer structure"]
impl crate::Writable for RisetmaxSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RISETMAX to value 0x02"]
impl crate::Resettable for RisetmaxSpec {
    const RESET_VALUE: u32 = 0x02;
}
