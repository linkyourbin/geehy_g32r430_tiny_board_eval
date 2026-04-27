#[doc = "Register `AFR2` reader"]
pub type R = crate::R<Afr2Spec>;
#[doc = "Register `AFR2` writer"]
pub type W = crate::W<Afr2Spec>;
#[doc = "Field `PB0_AFSEL` reader - PAD reuse selection"]
pub type Pb0AfselR = crate::FieldReader;
#[doc = "Field `PB0_AFSEL` writer - PAD reuse selection"]
pub type Pb0AfselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PB1_AFSEL` reader - PAD reuse selection"]
pub type Pb1AfselR = crate::FieldReader;
#[doc = "Field `PB1_AFSEL` writer - PAD reuse selection"]
pub type Pb1AfselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PB2_AFSEL` reader - PAD reuse selection"]
pub type Pb2AfselR = crate::FieldReader;
#[doc = "Field `PB2_AFSEL` writer - PAD reuse selection"]
pub type Pb2AfselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PB3_AFSEL` reader - PAD reuse selection"]
pub type Pb3AfselR = crate::FieldReader;
#[doc = "Field `PB3_AFSEL` writer - PAD reuse selection"]
pub type Pb3AfselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PB4_AFSEL` reader - PAD reuse selection"]
pub type Pb4AfselR = crate::FieldReader;
#[doc = "Field `PB4_AFSEL` writer - PAD reuse selection"]
pub type Pb4AfselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PB5_AFSEL` reader - PAD reuse selection"]
pub type Pb5AfselR = crate::FieldReader;
#[doc = "Field `PB5_AFSEL` writer - PAD reuse selection"]
pub type Pb5AfselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PB6_AFSEL` reader - PAD reuse selection"]
pub type Pb6AfselR = crate::FieldReader;
#[doc = "Field `PB6_AFSEL` writer - PAD reuse selection"]
pub type Pb6AfselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PB7_AFSEL` reader - PAD reuse selection"]
pub type Pb7AfselR = crate::FieldReader;
#[doc = "Field `PB7_AFSEL` writer - PAD reuse selection"]
pub type Pb7AfselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PB8_AFSEL` reader - PAD reuse selection"]
pub type Pb8AfselR = crate::FieldReader;
#[doc = "Field `PB8_AFSEL` writer - PAD reuse selection"]
pub type Pb8AfselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PB9_AFSEL` reader - PAD reuse selection"]
pub type Pb9AfselR = crate::FieldReader;
#[doc = "Field `PB9_AFSEL` writer - PAD reuse selection"]
pub type Pb9AfselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PB10_AFSEL` reader - PAD reuse selection"]
pub type Pb10AfselR = crate::FieldReader;
#[doc = "Field `PB10_AFSEL` writer - PAD reuse selection"]
pub type Pb10AfselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PB11_AFSEL` reader - PAD reuse selection"]
pub type Pb11AfselR = crate::FieldReader;
#[doc = "Field `PB11_AFSEL` writer - PAD reuse selection"]
pub type Pb11AfselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PB12_AFSEL` reader - PAD reuse selection"]
pub type Pb12AfselR = crate::FieldReader;
#[doc = "Field `PB12_AFSEL` writer - PAD reuse selection"]
pub type Pb12AfselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PB13_AFSEL` reader - PAD reuse selection"]
pub type Pb13AfselR = crate::FieldReader;
#[doc = "Field `PB13_AFSEL` writer - PAD reuse selection"]
pub type Pb13AfselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - PAD reuse selection"]
    #[inline(always)]
    pub fn pb0_afsel(&self) -> Pb0AfselR {
        Pb0AfselR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - PAD reuse selection"]
    #[inline(always)]
    pub fn pb1_afsel(&self) -> Pb1AfselR {
        Pb1AfselR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - PAD reuse selection"]
    #[inline(always)]
    pub fn pb2_afsel(&self) -> Pb2AfselR {
        Pb2AfselR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - PAD reuse selection"]
    #[inline(always)]
    pub fn pb3_afsel(&self) -> Pb3AfselR {
        Pb3AfselR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - PAD reuse selection"]
    #[inline(always)]
    pub fn pb4_afsel(&self) -> Pb4AfselR {
        Pb4AfselR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - PAD reuse selection"]
    #[inline(always)]
    pub fn pb5_afsel(&self) -> Pb5AfselR {
        Pb5AfselR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - PAD reuse selection"]
    #[inline(always)]
    pub fn pb6_afsel(&self) -> Pb6AfselR {
        Pb6AfselR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - PAD reuse selection"]
    #[inline(always)]
    pub fn pb7_afsel(&self) -> Pb7AfselR {
        Pb7AfselR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - PAD reuse selection"]
    #[inline(always)]
    pub fn pb8_afsel(&self) -> Pb8AfselR {
        Pb8AfselR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - PAD reuse selection"]
    #[inline(always)]
    pub fn pb9_afsel(&self) -> Pb9AfselR {
        Pb9AfselR::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - PAD reuse selection"]
    #[inline(always)]
    pub fn pb10_afsel(&self) -> Pb10AfselR {
        Pb10AfselR::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - PAD reuse selection"]
    #[inline(always)]
    pub fn pb11_afsel(&self) -> Pb11AfselR {
        Pb11AfselR::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - PAD reuse selection"]
    #[inline(always)]
    pub fn pb12_afsel(&self) -> Pb12AfselR {
        Pb12AfselR::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - PAD reuse selection"]
    #[inline(always)]
    pub fn pb13_afsel(&self) -> Pb13AfselR {
        Pb13AfselR::new(((self.bits >> 26) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - PAD reuse selection"]
    #[inline(always)]
    pub fn pb0_afsel(&mut self) -> Pb0AfselW<'_, Afr2Spec> {
        Pb0AfselW::new(self, 0)
    }
    #[doc = "Bits 2:3 - PAD reuse selection"]
    #[inline(always)]
    pub fn pb1_afsel(&mut self) -> Pb1AfselW<'_, Afr2Spec> {
        Pb1AfselW::new(self, 2)
    }
    #[doc = "Bits 4:5 - PAD reuse selection"]
    #[inline(always)]
    pub fn pb2_afsel(&mut self) -> Pb2AfselW<'_, Afr2Spec> {
        Pb2AfselW::new(self, 4)
    }
    #[doc = "Bits 6:7 - PAD reuse selection"]
    #[inline(always)]
    pub fn pb3_afsel(&mut self) -> Pb3AfselW<'_, Afr2Spec> {
        Pb3AfselW::new(self, 6)
    }
    #[doc = "Bits 8:9 - PAD reuse selection"]
    #[inline(always)]
    pub fn pb4_afsel(&mut self) -> Pb4AfselW<'_, Afr2Spec> {
        Pb4AfselW::new(self, 8)
    }
    #[doc = "Bits 10:11 - PAD reuse selection"]
    #[inline(always)]
    pub fn pb5_afsel(&mut self) -> Pb5AfselW<'_, Afr2Spec> {
        Pb5AfselW::new(self, 10)
    }
    #[doc = "Bits 12:13 - PAD reuse selection"]
    #[inline(always)]
    pub fn pb6_afsel(&mut self) -> Pb6AfselW<'_, Afr2Spec> {
        Pb6AfselW::new(self, 12)
    }
    #[doc = "Bits 14:15 - PAD reuse selection"]
    #[inline(always)]
    pub fn pb7_afsel(&mut self) -> Pb7AfselW<'_, Afr2Spec> {
        Pb7AfselW::new(self, 14)
    }
    #[doc = "Bits 16:17 - PAD reuse selection"]
    #[inline(always)]
    pub fn pb8_afsel(&mut self) -> Pb8AfselW<'_, Afr2Spec> {
        Pb8AfselW::new(self, 16)
    }
    #[doc = "Bits 18:19 - PAD reuse selection"]
    #[inline(always)]
    pub fn pb9_afsel(&mut self) -> Pb9AfselW<'_, Afr2Spec> {
        Pb9AfselW::new(self, 18)
    }
    #[doc = "Bits 20:21 - PAD reuse selection"]
    #[inline(always)]
    pub fn pb10_afsel(&mut self) -> Pb10AfselW<'_, Afr2Spec> {
        Pb10AfselW::new(self, 20)
    }
    #[doc = "Bits 22:23 - PAD reuse selection"]
    #[inline(always)]
    pub fn pb11_afsel(&mut self) -> Pb11AfselW<'_, Afr2Spec> {
        Pb11AfselW::new(self, 22)
    }
    #[doc = "Bits 24:25 - PAD reuse selection"]
    #[inline(always)]
    pub fn pb12_afsel(&mut self) -> Pb12AfselW<'_, Afr2Spec> {
        Pb12AfselW::new(self, 24)
    }
    #[doc = "Bits 26:27 - PAD reuse selection"]
    #[inline(always)]
    pub fn pb13_afsel(&mut self) -> Pb13AfselW<'_, Afr2Spec> {
        Pb13AfselW::new(self, 26)
    }
}
#[doc = "GPIO alternate function register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`afr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`afr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Afr2Spec;
impl crate::RegisterSpec for Afr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`afr2::R`](R) reader structure"]
impl crate::Readable for Afr2Spec {}
#[doc = "`write(|w| ..)` method takes [`afr2::W`](W) writer structure"]
impl crate::Writable for Afr2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AFR2 to value 0"]
impl crate::Resettable for Afr2Spec {}
