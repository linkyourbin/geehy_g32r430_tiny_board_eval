#[doc = "Register `CCM2_CAPTURE` reader"]
pub type R = crate::R<Ccm2CaptureSpec>;
#[doc = "Register `CCM2_CAPTURE` writer"]
pub type W = crate::W<Ccm2CaptureSpec>;
#[doc = "Field `CC3SEL` reader - Capture/compare 3 selection"]
pub type Cc3selR = crate::FieldReader;
#[doc = "Field `CC3SEL` writer - Capture/compare 3 selection"]
pub type Cc3selW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IC3PSC` reader - Input capture 3 prescaler"]
pub type Ic3pscR = crate::FieldReader;
#[doc = "Field `IC3PSC` writer - Input capture 3 prescaler"]
pub type Ic3pscW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IC3F` reader - Input capture 3 filter"]
pub type Ic3fR = crate::FieldReader;
#[doc = "Field `IC3F` writer - Input capture 3 filter"]
pub type Ic3fW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CC4SEL` reader - Capture/Compare 4 selection"]
pub type Cc4selR = crate::FieldReader;
#[doc = "Field `CC4SEL` writer - Capture/Compare 4 selection"]
pub type Cc4selW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IC4PSC` reader - Input capture 4 prescaler"]
pub type Ic4pscR = crate::FieldReader;
#[doc = "Field `IC4PSC` writer - Input capture 4 prescaler"]
pub type Ic4pscW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IC4F` reader - Input capture 4 filter"]
pub type Ic4fR = crate::FieldReader;
#[doc = "Field `IC4F` writer - Input capture 4 filter"]
pub type Ic4fW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:1 - Capture/compare 3 selection"]
    #[inline(always)]
    pub fn cc3sel(&self) -> Cc3selR {
        Cc3selR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Input capture 3 prescaler"]
    #[inline(always)]
    pub fn ic3psc(&self) -> Ic3pscR {
        Ic3pscR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:7 - Input capture 3 filter"]
    #[inline(always)]
    pub fn ic3f(&self) -> Ic3fR {
        Ic3fR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:9 - Capture/Compare 4 selection"]
    #[inline(always)]
    pub fn cc4sel(&self) -> Cc4selR {
        Cc4selR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Input capture 4 prescaler"]
    #[inline(always)]
    pub fn ic4psc(&self) -> Ic4pscR {
        Ic4pscR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:15 - Input capture 4 filter"]
    #[inline(always)]
    pub fn ic4f(&self) -> Ic4fR {
        Ic4fR::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Capture/compare 3 selection"]
    #[inline(always)]
    pub fn cc3sel(&mut self) -> Cc3selW<'_, Ccm2CaptureSpec> {
        Cc3selW::new(self, 0)
    }
    #[doc = "Bits 2:3 - Input capture 3 prescaler"]
    #[inline(always)]
    pub fn ic3psc(&mut self) -> Ic3pscW<'_, Ccm2CaptureSpec> {
        Ic3pscW::new(self, 2)
    }
    #[doc = "Bits 4:7 - Input capture 3 filter"]
    #[inline(always)]
    pub fn ic3f(&mut self) -> Ic3fW<'_, Ccm2CaptureSpec> {
        Ic3fW::new(self, 4)
    }
    #[doc = "Bits 8:9 - Capture/Compare 4 selection"]
    #[inline(always)]
    pub fn cc4sel(&mut self) -> Cc4selW<'_, Ccm2CaptureSpec> {
        Cc4selW::new(self, 8)
    }
    #[doc = "Bits 10:11 - Input capture 4 prescaler"]
    #[inline(always)]
    pub fn ic4psc(&mut self) -> Ic4pscW<'_, Ccm2CaptureSpec> {
        Ic4pscW::new(self, 10)
    }
    #[doc = "Bits 12:15 - Input capture 4 filter"]
    #[inline(always)]
    pub fn ic4f(&mut self) -> Ic4fW<'_, Ccm2CaptureSpec> {
        Ic4fW::new(self, 12)
    }
}
#[doc = "capture/compare mode register 2 (input mode)\n\nYou can [`read`](crate::Reg::read) this register and get [`ccm2_capture::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccm2_capture::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ccm2CaptureSpec;
impl crate::RegisterSpec for Ccm2CaptureSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccm2_capture::R`](R) reader structure"]
impl crate::Readable for Ccm2CaptureSpec {}
#[doc = "`write(|w| ..)` method takes [`ccm2_capture::W`](W) writer structure"]
impl crate::Writable for Ccm2CaptureSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CCM2_CAPTURE to value 0"]
impl crate::Resettable for Ccm2CaptureSpec {}
