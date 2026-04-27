#[doc = "Register `CCM1` reader"]
pub type R = crate::R<Ccm1Spec>;
#[doc = "Register `CCM1` writer"]
pub type W = crate::W<Ccm1Spec>;
#[doc = "Field `CC1SEL` reader - Capture/Compare 1 selection"]
pub type Cc1selR = crate::FieldReader;
#[doc = "Field `CC1SEL` writer - Capture/Compare 1 selection"]
pub type Cc1selW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `OC1FEN` reader - Output Compare 1 fast enable"]
pub type Oc1fenR = crate::BitReader;
#[doc = "Field `OC1FEN` writer - Output Compare 1 fast enable"]
pub type Oc1fenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OC1PEN` reader - Output Compare 1 preload enable"]
pub type Oc1penR = crate::BitReader;
#[doc = "Field `OC1PEN` writer - Output Compare 1 preload enable"]
pub type Oc1penW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OC1MOD` reader - Output Compare 1 mode"]
pub type Oc1modR = crate::FieldReader;
#[doc = "Field `OC1MOD` writer - Output Compare 1 mode"]
pub type Oc1modW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `OC1CEN` reader - Output Compare 1 clear enable"]
pub type Oc1cenR = crate::BitReader;
#[doc = "Field `OC1CEN` writer - Output Compare 1 clear enable"]
pub type Oc1cenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC2SEL` reader - Capture/Compare 2 selection"]
pub type Cc2selR = crate::FieldReader;
#[doc = "Field `CC2SEL` writer - Capture/Compare 2 selection"]
pub type Cc2selW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `OC2FEN` reader - Output Compare 2 fast enable"]
pub type Oc2fenR = crate::BitReader;
#[doc = "Field `OC2FEN` writer - Output Compare 2 fast enable"]
pub type Oc2fenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OC2PEN` reader - Output Compare 2 preload enable"]
pub type Oc2penR = crate::BitReader;
#[doc = "Field `OC2PEN` writer - Output Compare 2 preload enable"]
pub type Oc2penW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OC2MOD` reader - Output Compare 2 mode"]
pub type Oc2modR = crate::FieldReader;
#[doc = "Field `OC2MOD` writer - Output Compare 2 mode"]
pub type Oc2modW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `OC2CEN` reader - Output Compare 2 clear enable"]
pub type Oc2cenR = crate::BitReader;
#[doc = "Field `OC2CEN` writer - Output Compare 2 clear enable"]
pub type Oc2cenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - Capture/Compare 1 selection"]
    #[inline(always)]
    pub fn cc1sel(&self) -> Cc1selR {
        Cc1selR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Output Compare 1 fast enable"]
    #[inline(always)]
    pub fn oc1fen(&self) -> Oc1fenR {
        Oc1fenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Output Compare 1 preload enable"]
    #[inline(always)]
    pub fn oc1pen(&self) -> Oc1penR {
        Oc1penR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - Output Compare 1 mode"]
    #[inline(always)]
    pub fn oc1mod(&self) -> Oc1modR {
        Oc1modR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - Output Compare 1 clear enable"]
    #[inline(always)]
    pub fn oc1cen(&self) -> Oc1cenR {
        Oc1cenR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Capture/Compare 2 selection"]
    #[inline(always)]
    pub fn cc2sel(&self) -> Cc2selR {
        Cc2selR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - Output Compare 2 fast enable"]
    #[inline(always)]
    pub fn oc2fen(&self) -> Oc2fenR {
        Oc2fenR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Output Compare 2 preload enable"]
    #[inline(always)]
    pub fn oc2pen(&self) -> Oc2penR {
        Oc2penR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:14 - Output Compare 2 mode"]
    #[inline(always)]
    pub fn oc2mod(&self) -> Oc2modR {
        Oc2modR::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - Output Compare 2 clear enable"]
    #[inline(always)]
    pub fn oc2cen(&self) -> Oc2cenR {
        Oc2cenR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Capture/Compare 1 selection"]
    #[inline(always)]
    pub fn cc1sel(&mut self) -> Cc1selW<'_, Ccm1Spec> {
        Cc1selW::new(self, 0)
    }
    #[doc = "Bit 2 - Output Compare 1 fast enable"]
    #[inline(always)]
    pub fn oc1fen(&mut self) -> Oc1fenW<'_, Ccm1Spec> {
        Oc1fenW::new(self, 2)
    }
    #[doc = "Bit 3 - Output Compare 1 preload enable"]
    #[inline(always)]
    pub fn oc1pen(&mut self) -> Oc1penW<'_, Ccm1Spec> {
        Oc1penW::new(self, 3)
    }
    #[doc = "Bits 4:6 - Output Compare 1 mode"]
    #[inline(always)]
    pub fn oc1mod(&mut self) -> Oc1modW<'_, Ccm1Spec> {
        Oc1modW::new(self, 4)
    }
    #[doc = "Bit 7 - Output Compare 1 clear enable"]
    #[inline(always)]
    pub fn oc1cen(&mut self) -> Oc1cenW<'_, Ccm1Spec> {
        Oc1cenW::new(self, 7)
    }
    #[doc = "Bits 8:9 - Capture/Compare 2 selection"]
    #[inline(always)]
    pub fn cc2sel(&mut self) -> Cc2selW<'_, Ccm1Spec> {
        Cc2selW::new(self, 8)
    }
    #[doc = "Bit 10 - Output Compare 2 fast enable"]
    #[inline(always)]
    pub fn oc2fen(&mut self) -> Oc2fenW<'_, Ccm1Spec> {
        Oc2fenW::new(self, 10)
    }
    #[doc = "Bit 11 - Output Compare 2 preload enable"]
    #[inline(always)]
    pub fn oc2pen(&mut self) -> Oc2penW<'_, Ccm1Spec> {
        Oc2penW::new(self, 11)
    }
    #[doc = "Bits 12:14 - Output Compare 2 mode"]
    #[inline(always)]
    pub fn oc2mod(&mut self) -> Oc2modW<'_, Ccm1Spec> {
        Oc2modW::new(self, 12)
    }
    #[doc = "Bit 15 - Output Compare 2 clear enable"]
    #[inline(always)]
    pub fn oc2cen(&mut self) -> Oc2cenW<'_, Ccm1Spec> {
        Oc2cenW::new(self, 15)
    }
}
#[doc = "capture/compare mode register 1 (output mode)\n\nYou can [`read`](crate::Reg::read) this register and get [`ccm1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccm1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ccm1Spec;
impl crate::RegisterSpec for Ccm1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccm1::R`](R) reader structure"]
impl crate::Readable for Ccm1Spec {}
#[doc = "`write(|w| ..)` method takes [`ccm1::W`](W) writer structure"]
impl crate::Writable for Ccm1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CCM1 to value 0"]
impl crate::Resettable for Ccm1Spec {}
