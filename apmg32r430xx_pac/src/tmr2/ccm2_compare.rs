#[doc = "Register `CCM2_COMPARE` reader"]
pub type R = crate::R<Ccm2CompareSpec>;
#[doc = "Register `CCM2_COMPARE` writer"]
pub type W = crate::W<Ccm2CompareSpec>;
#[doc = "Field `CC3SEL` reader - Capture/Compare 3 selection"]
pub type Cc3selR = crate::FieldReader;
#[doc = "Field `CC3SEL` writer - Capture/Compare 3 selection"]
pub type Cc3selW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `OC3FEN` reader - Output compare 3 fast enable"]
pub type Oc3fenR = crate::BitReader;
#[doc = "Field `OC3FEN` writer - Output compare 3 fast enable"]
pub type Oc3fenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OC3PEN` reader - Output compare 3 preload enable"]
pub type Oc3penR = crate::BitReader;
#[doc = "Field `OC3PEN` writer - Output compare 3 preload enable"]
pub type Oc3penW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OC3MOD` reader - Output compare 3 mode"]
pub type Oc3modR = crate::FieldReader;
#[doc = "Field `OC3MOD` writer - Output compare 3 mode"]
pub type Oc3modW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `OC3CEN` reader - Output compare 3 clear enable"]
pub type Oc3cenR = crate::BitReader;
#[doc = "Field `OC3CEN` writer - Output compare 3 clear enable"]
pub type Oc3cenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC4SEL` reader - Capture/Compare 4 selection"]
pub type Cc4selR = crate::FieldReader;
#[doc = "Field `CC4SEL` writer - Capture/Compare 4 selection"]
pub type Cc4selW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `OC4FEN` reader - Output compare 4 fast enable"]
pub type Oc4fenR = crate::BitReader;
#[doc = "Field `OC4FEN` writer - Output compare 4 fast enable"]
pub type Oc4fenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OC4PEN` reader - Output compare 4 preload enable"]
pub type Oc4penR = crate::BitReader;
#[doc = "Field `OC4PEN` writer - Output compare 4 preload enable"]
pub type Oc4penW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OC4MOD` reader - Output compare 4 mode"]
pub type Oc4modR = crate::FieldReader;
#[doc = "Field `OC4MOD` writer - Output compare 4 mode"]
pub type Oc4modW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `OC4CEN` reader - Output compare 4 clear enable"]
pub type Oc4cenR = crate::BitReader;
#[doc = "Field `OC4CEN` writer - Output compare 4 clear enable"]
pub type Oc4cenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - Capture/Compare 3 selection"]
    #[inline(always)]
    pub fn cc3sel(&self) -> Cc3selR {
        Cc3selR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Output compare 3 fast enable"]
    #[inline(always)]
    pub fn oc3fen(&self) -> Oc3fenR {
        Oc3fenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Output compare 3 preload enable"]
    #[inline(always)]
    pub fn oc3pen(&self) -> Oc3penR {
        Oc3penR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - Output compare 3 mode"]
    #[inline(always)]
    pub fn oc3mod(&self) -> Oc3modR {
        Oc3modR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - Output compare 3 clear enable"]
    #[inline(always)]
    pub fn oc3cen(&self) -> Oc3cenR {
        Oc3cenR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Capture/Compare 4 selection"]
    #[inline(always)]
    pub fn cc4sel(&self) -> Cc4selR {
        Cc4selR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - Output compare 4 fast enable"]
    #[inline(always)]
    pub fn oc4fen(&self) -> Oc4fenR {
        Oc4fenR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Output compare 4 preload enable"]
    #[inline(always)]
    pub fn oc4pen(&self) -> Oc4penR {
        Oc4penR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:14 - Output compare 4 mode"]
    #[inline(always)]
    pub fn oc4mod(&self) -> Oc4modR {
        Oc4modR::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - Output compare 4 clear enable"]
    #[inline(always)]
    pub fn oc4cen(&self) -> Oc4cenR {
        Oc4cenR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Capture/Compare 3 selection"]
    #[inline(always)]
    pub fn cc3sel(&mut self) -> Cc3selW<'_, Ccm2CompareSpec> {
        Cc3selW::new(self, 0)
    }
    #[doc = "Bit 2 - Output compare 3 fast enable"]
    #[inline(always)]
    pub fn oc3fen(&mut self) -> Oc3fenW<'_, Ccm2CompareSpec> {
        Oc3fenW::new(self, 2)
    }
    #[doc = "Bit 3 - Output compare 3 preload enable"]
    #[inline(always)]
    pub fn oc3pen(&mut self) -> Oc3penW<'_, Ccm2CompareSpec> {
        Oc3penW::new(self, 3)
    }
    #[doc = "Bits 4:6 - Output compare 3 mode"]
    #[inline(always)]
    pub fn oc3mod(&mut self) -> Oc3modW<'_, Ccm2CompareSpec> {
        Oc3modW::new(self, 4)
    }
    #[doc = "Bit 7 - Output compare 3 clear enable"]
    #[inline(always)]
    pub fn oc3cen(&mut self) -> Oc3cenW<'_, Ccm2CompareSpec> {
        Oc3cenW::new(self, 7)
    }
    #[doc = "Bits 8:9 - Capture/Compare 4 selection"]
    #[inline(always)]
    pub fn cc4sel(&mut self) -> Cc4selW<'_, Ccm2CompareSpec> {
        Cc4selW::new(self, 8)
    }
    #[doc = "Bit 10 - Output compare 4 fast enable"]
    #[inline(always)]
    pub fn oc4fen(&mut self) -> Oc4fenW<'_, Ccm2CompareSpec> {
        Oc4fenW::new(self, 10)
    }
    #[doc = "Bit 11 - Output compare 4 preload enable"]
    #[inline(always)]
    pub fn oc4pen(&mut self) -> Oc4penW<'_, Ccm2CompareSpec> {
        Oc4penW::new(self, 11)
    }
    #[doc = "Bits 12:14 - Output compare 4 mode"]
    #[inline(always)]
    pub fn oc4mod(&mut self) -> Oc4modW<'_, Ccm2CompareSpec> {
        Oc4modW::new(self, 12)
    }
    #[doc = "Bit 15 - Output compare 4 clear enable"]
    #[inline(always)]
    pub fn oc4cen(&mut self) -> Oc4cenW<'_, Ccm2CompareSpec> {
        Oc4cenW::new(self, 15)
    }
}
#[doc = "capture/compare mode register 2 (output mode)\n\nYou can [`read`](crate::Reg::read) this register and get [`ccm2_compare::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccm2_compare::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ccm2CompareSpec;
impl crate::RegisterSpec for Ccm2CompareSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccm2_compare::R`](R) reader structure"]
impl crate::Readable for Ccm2CompareSpec {}
#[doc = "`write(|w| ..)` method takes [`ccm2_compare::W`](W) writer structure"]
impl crate::Writable for Ccm2CompareSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CCM2_COMPARE to value 0"]
impl crate::Resettable for Ccm2CompareSpec {}
