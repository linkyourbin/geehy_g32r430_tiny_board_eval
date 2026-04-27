#[doc = "Register `AFR3` reader"]
pub type R = crate::R<Afr3Spec>;
#[doc = "Register `AFR3` writer"]
pub type W = crate::W<Afr3Spec>;
#[doc = "Field `PC0_AFSEL` reader - PAD reuse selection"]
pub type Pc0AfselR = crate::FieldReader;
#[doc = "Field `PC0_AFSEL` writer - PAD reuse selection"]
pub type Pc0AfselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PC1_AFSEL` reader - PAD reuse selection"]
pub type Pc1AfselR = crate::FieldReader;
#[doc = "Field `PC1_AFSEL` writer - PAD reuse selection"]
pub type Pc1AfselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PC2_AFSEL` reader - PAD reuse selection"]
pub type Pc2AfselR = crate::FieldReader;
#[doc = "Field `PC2_AFSEL` writer - PAD reuse selection"]
pub type Pc2AfselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PC3_AFSEL` reader - PAD reuse selection"]
pub type Pc3AfselR = crate::FieldReader;
#[doc = "Field `PC3_AFSEL` writer - PAD reuse selection"]
pub type Pc3AfselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PC4_AFSEL` reader - PAD reuse selection"]
pub type Pc4AfselR = crate::FieldReader;
#[doc = "Field `PC4_AFSEL` writer - PAD reuse selection"]
pub type Pc4AfselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PC5_AFSEL` reader - PAD reuse selection"]
pub type Pc5AfselR = crate::FieldReader;
#[doc = "Field `PC5_AFSEL` writer - PAD reuse selection"]
pub type Pc5AfselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PC6_AFSEL` reader - PAD reuse selection"]
pub type Pc6AfselR = crate::FieldReader;
#[doc = "Field `PC6_AFSEL` writer - PAD reuse selection"]
pub type Pc6AfselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PC7_AFSEL` reader - PAD reuse selection"]
pub type Pc7AfselR = crate::FieldReader;
#[doc = "Field `PC7_AFSEL` writer - PAD reuse selection"]
pub type Pc7AfselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PC8_AFSEL` reader - PAD reuse selection"]
pub type Pc8AfselR = crate::FieldReader;
#[doc = "Field `PC8_AFSEL` writer - PAD reuse selection"]
pub type Pc8AfselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PC9_AFSEL` reader - PAD reuse selection"]
pub type Pc9AfselR = crate::FieldReader;
#[doc = "Field `PC9_AFSEL` writer - PAD reuse selection"]
pub type Pc9AfselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PC10_AFSEL` reader - PAD reuse selection"]
pub type Pc10AfselR = crate::FieldReader;
#[doc = "Field `PC10_AFSEL` writer - PAD reuse selection"]
pub type Pc10AfselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PC11_AFSEL` reader - PAD reuse selection"]
pub type Pc11AfselR = crate::FieldReader;
#[doc = "Field `PC11_AFSEL` writer - PAD reuse selection"]
pub type Pc11AfselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PC12_AFSEL` reader - PAD reuse selection"]
pub type Pc12AfselR = crate::FieldReader;
#[doc = "Field `PC12_AFSEL` writer - PAD reuse selection"]
pub type Pc12AfselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - PAD reuse selection"]
    #[inline(always)]
    pub fn pc0_afsel(&self) -> Pc0AfselR {
        Pc0AfselR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - PAD reuse selection"]
    #[inline(always)]
    pub fn pc1_afsel(&self) -> Pc1AfselR {
        Pc1AfselR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - PAD reuse selection"]
    #[inline(always)]
    pub fn pc2_afsel(&self) -> Pc2AfselR {
        Pc2AfselR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - PAD reuse selection"]
    #[inline(always)]
    pub fn pc3_afsel(&self) -> Pc3AfselR {
        Pc3AfselR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - PAD reuse selection"]
    #[inline(always)]
    pub fn pc4_afsel(&self) -> Pc4AfselR {
        Pc4AfselR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - PAD reuse selection"]
    #[inline(always)]
    pub fn pc5_afsel(&self) -> Pc5AfselR {
        Pc5AfselR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - PAD reuse selection"]
    #[inline(always)]
    pub fn pc6_afsel(&self) -> Pc6AfselR {
        Pc6AfselR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - PAD reuse selection"]
    #[inline(always)]
    pub fn pc7_afsel(&self) -> Pc7AfselR {
        Pc7AfselR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - PAD reuse selection"]
    #[inline(always)]
    pub fn pc8_afsel(&self) -> Pc8AfselR {
        Pc8AfselR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - PAD reuse selection"]
    #[inline(always)]
    pub fn pc9_afsel(&self) -> Pc9AfselR {
        Pc9AfselR::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - PAD reuse selection"]
    #[inline(always)]
    pub fn pc10_afsel(&self) -> Pc10AfselR {
        Pc10AfselR::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - PAD reuse selection"]
    #[inline(always)]
    pub fn pc11_afsel(&self) -> Pc11AfselR {
        Pc11AfselR::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - PAD reuse selection"]
    #[inline(always)]
    pub fn pc12_afsel(&self) -> Pc12AfselR {
        Pc12AfselR::new(((self.bits >> 24) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - PAD reuse selection"]
    #[inline(always)]
    pub fn pc0_afsel(&mut self) -> Pc0AfselW<'_, Afr3Spec> {
        Pc0AfselW::new(self, 0)
    }
    #[doc = "Bits 2:3 - PAD reuse selection"]
    #[inline(always)]
    pub fn pc1_afsel(&mut self) -> Pc1AfselW<'_, Afr3Spec> {
        Pc1AfselW::new(self, 2)
    }
    #[doc = "Bits 4:5 - PAD reuse selection"]
    #[inline(always)]
    pub fn pc2_afsel(&mut self) -> Pc2AfselW<'_, Afr3Spec> {
        Pc2AfselW::new(self, 4)
    }
    #[doc = "Bits 6:7 - PAD reuse selection"]
    #[inline(always)]
    pub fn pc3_afsel(&mut self) -> Pc3AfselW<'_, Afr3Spec> {
        Pc3AfselW::new(self, 6)
    }
    #[doc = "Bits 8:9 - PAD reuse selection"]
    #[inline(always)]
    pub fn pc4_afsel(&mut self) -> Pc4AfselW<'_, Afr3Spec> {
        Pc4AfselW::new(self, 8)
    }
    #[doc = "Bits 10:11 - PAD reuse selection"]
    #[inline(always)]
    pub fn pc5_afsel(&mut self) -> Pc5AfselW<'_, Afr3Spec> {
        Pc5AfselW::new(self, 10)
    }
    #[doc = "Bits 12:13 - PAD reuse selection"]
    #[inline(always)]
    pub fn pc6_afsel(&mut self) -> Pc6AfselW<'_, Afr3Spec> {
        Pc6AfselW::new(self, 12)
    }
    #[doc = "Bits 14:15 - PAD reuse selection"]
    #[inline(always)]
    pub fn pc7_afsel(&mut self) -> Pc7AfselW<'_, Afr3Spec> {
        Pc7AfselW::new(self, 14)
    }
    #[doc = "Bits 16:17 - PAD reuse selection"]
    #[inline(always)]
    pub fn pc8_afsel(&mut self) -> Pc8AfselW<'_, Afr3Spec> {
        Pc8AfselW::new(self, 16)
    }
    #[doc = "Bits 18:19 - PAD reuse selection"]
    #[inline(always)]
    pub fn pc9_afsel(&mut self) -> Pc9AfselW<'_, Afr3Spec> {
        Pc9AfselW::new(self, 18)
    }
    #[doc = "Bits 20:21 - PAD reuse selection"]
    #[inline(always)]
    pub fn pc10_afsel(&mut self) -> Pc10AfselW<'_, Afr3Spec> {
        Pc10AfselW::new(self, 20)
    }
    #[doc = "Bits 22:23 - PAD reuse selection"]
    #[inline(always)]
    pub fn pc11_afsel(&mut self) -> Pc11AfselW<'_, Afr3Spec> {
        Pc11AfselW::new(self, 22)
    }
    #[doc = "Bits 24:25 - PAD reuse selection"]
    #[inline(always)]
    pub fn pc12_afsel(&mut self) -> Pc12AfselW<'_, Afr3Spec> {
        Pc12AfselW::new(self, 24)
    }
}
#[doc = "GPIO alternate function register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`afr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`afr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Afr3Spec;
impl crate::RegisterSpec for Afr3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`afr3::R`](R) reader structure"]
impl crate::Readable for Afr3Spec {}
#[doc = "`write(|w| ..)` method takes [`afr3::W`](W) writer structure"]
impl crate::Writable for Afr3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AFR3 to value 0"]
impl crate::Resettable for Afr3Spec {}
