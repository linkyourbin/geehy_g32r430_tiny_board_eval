#[doc = "Register `CCM1_CAPTURE` reader"]
pub type R = crate::R<Ccm1CaptureSpec>;
#[doc = "Register `CCM1_CAPTURE` writer"]
pub type W = crate::W<Ccm1CaptureSpec>;
#[doc = "Field `CC1SEL` reader - Capture/Compare 1 selection"]
pub type Cc1selR = crate::FieldReader;
#[doc = "Field `CC1SEL` writer - Capture/Compare 1 selection"]
pub type Cc1selW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IC1PSC` reader - Input capture 1 prescaler"]
pub type Ic1pscR = crate::FieldReader;
#[doc = "Field `IC1PSC` writer - Input capture 1 prescaler"]
pub type Ic1pscW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IC1F` reader - Input capture 1 filter"]
pub type Ic1fR = crate::FieldReader;
#[doc = "Field `IC1F` writer - Input capture 1 filter"]
pub type Ic1fW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CC2SEL` reader - Capture/Compare 2 selection"]
pub type Cc2selR = crate::FieldReader;
#[doc = "Field `CC2SEL` writer - Capture/Compare 2 selection"]
pub type Cc2selW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IC2PSC` reader - Input capture 2 prescaler"]
pub type Ic2pscR = crate::FieldReader;
#[doc = "Field `IC2PSC` writer - Input capture 2 prescaler"]
pub type Ic2pscW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IC2F` reader - Input capture 2 filter"]
pub type Ic2fR = crate::FieldReader;
#[doc = "Field `IC2F` writer - Input capture 2 filter"]
pub type Ic2fW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:1 - Capture/Compare 1 selection"]
    #[inline(always)]
    pub fn cc1sel(&self) -> Cc1selR {
        Cc1selR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Input capture 1 prescaler"]
    #[inline(always)]
    pub fn ic1psc(&self) -> Ic1pscR {
        Ic1pscR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:7 - Input capture 1 filter"]
    #[inline(always)]
    pub fn ic1f(&self) -> Ic1fR {
        Ic1fR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:9 - Capture/Compare 2 selection"]
    #[inline(always)]
    pub fn cc2sel(&self) -> Cc2selR {
        Cc2selR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Input capture 2 prescaler"]
    #[inline(always)]
    pub fn ic2psc(&self) -> Ic2pscR {
        Ic2pscR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:15 - Input capture 2 filter"]
    #[inline(always)]
    pub fn ic2f(&self) -> Ic2fR {
        Ic2fR::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Capture/Compare 1 selection"]
    #[inline(always)]
    pub fn cc1sel(&mut self) -> Cc1selW<'_, Ccm1CaptureSpec> {
        Cc1selW::new(self, 0)
    }
    #[doc = "Bits 2:3 - Input capture 1 prescaler"]
    #[inline(always)]
    pub fn ic1psc(&mut self) -> Ic1pscW<'_, Ccm1CaptureSpec> {
        Ic1pscW::new(self, 2)
    }
    #[doc = "Bits 4:7 - Input capture 1 filter"]
    #[inline(always)]
    pub fn ic1f(&mut self) -> Ic1fW<'_, Ccm1CaptureSpec> {
        Ic1fW::new(self, 4)
    }
    #[doc = "Bits 8:9 - Capture/Compare 2 selection"]
    #[inline(always)]
    pub fn cc2sel(&mut self) -> Cc2selW<'_, Ccm1CaptureSpec> {
        Cc2selW::new(self, 8)
    }
    #[doc = "Bits 10:11 - Input capture 2 prescaler"]
    #[inline(always)]
    pub fn ic2psc(&mut self) -> Ic2pscW<'_, Ccm1CaptureSpec> {
        Ic2pscW::new(self, 10)
    }
    #[doc = "Bits 12:15 - Input capture 2 filter"]
    #[inline(always)]
    pub fn ic2f(&mut self) -> Ic2fW<'_, Ccm1CaptureSpec> {
        Ic2fW::new(self, 12)
    }
}
#[doc = "capture/compare mode register 1 (input mode)\n\nYou can [`read`](crate::Reg::read) this register and get [`ccm1_capture::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccm1_capture::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ccm1CaptureSpec;
impl crate::RegisterSpec for Ccm1CaptureSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccm1_capture::R`](R) reader structure"]
impl crate::Readable for Ccm1CaptureSpec {}
#[doc = "`write(|w| ..)` method takes [`ccm1_capture::W`](W) writer structure"]
impl crate::Writable for Ccm1CaptureSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CCM1_CAPTURE to value 0"]
impl crate::Resettable for Ccm1CaptureSpec {}
