#[doc = "Register `MODER3` reader"]
pub type R = crate::R<Moder3Spec>;
#[doc = "Register `MODER3` writer"]
pub type W = crate::W<Moder3Spec>;
#[doc = "Field `PC0_MD` reader - Work Mode Configuration"]
pub type Pc0MdR = crate::FieldReader;
#[doc = "Field `PC0_MD` writer - Work Mode Configuration"]
pub type Pc0MdW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PC1_MD` reader - Work Mode Configuration"]
pub type Pc1MdR = crate::FieldReader;
#[doc = "Field `PC1_MD` writer - Work Mode Configuration"]
pub type Pc1MdW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PC2_MD` reader - Work Mode Configuration"]
pub type Pc2MdR = crate::FieldReader;
#[doc = "Field `PC2_MD` writer - Work Mode Configuration"]
pub type Pc2MdW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PC3_MD` reader - Work Mode Configuration"]
pub type Pc3MdR = crate::FieldReader;
#[doc = "Field `PC3_MD` writer - Work Mode Configuration"]
pub type Pc3MdW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PC4_MD` reader - Work Mode Configuration"]
pub type Pc4MdR = crate::FieldReader;
#[doc = "Field `PC4_MD` writer - Work Mode Configuration"]
pub type Pc4MdW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PC5_MD` reader - Work Mode Configuration"]
pub type Pc5MdR = crate::FieldReader;
#[doc = "Field `PC5_MD` writer - Work Mode Configuration"]
pub type Pc5MdW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PC6_MD` reader - Work Mode Configuration"]
pub type Pc6MdR = crate::FieldReader;
#[doc = "Field `PC6_MD` writer - Work Mode Configuration"]
pub type Pc6MdW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PC7_MD` reader - Work Mode Configuration"]
pub type Pc7MdR = crate::FieldReader;
#[doc = "Field `PC7_MD` writer - Work Mode Configuration"]
pub type Pc7MdW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PC8_MD` reader - Work Mode Configuration"]
pub type Pc8MdR = crate::FieldReader;
#[doc = "Field `PC8_MD` writer - Work Mode Configuration"]
pub type Pc8MdW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PC9_MD` reader - Work Mode Configuration"]
pub type Pc9MdR = crate::FieldReader;
#[doc = "Field `PC9_MD` writer - Work Mode Configuration"]
pub type Pc9MdW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PC10_MD` reader - Work Mode Configuration"]
pub type Pc10MdR = crate::FieldReader;
#[doc = "Field `PC10_MD` writer - Work Mode Configuration"]
pub type Pc10MdW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PC11_MD` reader - Work Mode Configuration"]
pub type Pc11MdR = crate::FieldReader;
#[doc = "Field `PC11_MD` writer - Work Mode Configuration"]
pub type Pc11MdW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PC12_MD` reader - Work Mode Configuration"]
pub type Pc12MdR = crate::FieldReader;
#[doc = "Field `PC12_MD` writer - Work Mode Configuration"]
pub type Pc12MdW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - Work Mode Configuration"]
    #[inline(always)]
    pub fn pc0_md(&self) -> Pc0MdR {
        Pc0MdR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Work Mode Configuration"]
    #[inline(always)]
    pub fn pc1_md(&self) -> Pc1MdR {
        Pc1MdR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Work Mode Configuration"]
    #[inline(always)]
    pub fn pc2_md(&self) -> Pc2MdR {
        Pc2MdR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Work Mode Configuration"]
    #[inline(always)]
    pub fn pc3_md(&self) -> Pc3MdR {
        Pc3MdR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Work Mode Configuration"]
    #[inline(always)]
    pub fn pc4_md(&self) -> Pc4MdR {
        Pc4MdR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Work Mode Configuration"]
    #[inline(always)]
    pub fn pc5_md(&self) -> Pc5MdR {
        Pc5MdR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Work Mode Configuration"]
    #[inline(always)]
    pub fn pc6_md(&self) -> Pc6MdR {
        Pc6MdR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Work Mode Configuration"]
    #[inline(always)]
    pub fn pc7_md(&self) -> Pc7MdR {
        Pc7MdR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Work Mode Configuration"]
    #[inline(always)]
    pub fn pc8_md(&self) -> Pc8MdR {
        Pc8MdR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - Work Mode Configuration"]
    #[inline(always)]
    pub fn pc9_md(&self) -> Pc9MdR {
        Pc9MdR::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - Work Mode Configuration"]
    #[inline(always)]
    pub fn pc10_md(&self) -> Pc10MdR {
        Pc10MdR::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - Work Mode Configuration"]
    #[inline(always)]
    pub fn pc11_md(&self) -> Pc11MdR {
        Pc11MdR::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - Work Mode Configuration"]
    #[inline(always)]
    pub fn pc12_md(&self) -> Pc12MdR {
        Pc12MdR::new(((self.bits >> 24) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Work Mode Configuration"]
    #[inline(always)]
    pub fn pc0_md(&mut self) -> Pc0MdW<'_, Moder3Spec> {
        Pc0MdW::new(self, 0)
    }
    #[doc = "Bits 2:3 - Work Mode Configuration"]
    #[inline(always)]
    pub fn pc1_md(&mut self) -> Pc1MdW<'_, Moder3Spec> {
        Pc1MdW::new(self, 2)
    }
    #[doc = "Bits 4:5 - Work Mode Configuration"]
    #[inline(always)]
    pub fn pc2_md(&mut self) -> Pc2MdW<'_, Moder3Spec> {
        Pc2MdW::new(self, 4)
    }
    #[doc = "Bits 6:7 - Work Mode Configuration"]
    #[inline(always)]
    pub fn pc3_md(&mut self) -> Pc3MdW<'_, Moder3Spec> {
        Pc3MdW::new(self, 6)
    }
    #[doc = "Bits 8:9 - Work Mode Configuration"]
    #[inline(always)]
    pub fn pc4_md(&mut self) -> Pc4MdW<'_, Moder3Spec> {
        Pc4MdW::new(self, 8)
    }
    #[doc = "Bits 10:11 - Work Mode Configuration"]
    #[inline(always)]
    pub fn pc5_md(&mut self) -> Pc5MdW<'_, Moder3Spec> {
        Pc5MdW::new(self, 10)
    }
    #[doc = "Bits 12:13 - Work Mode Configuration"]
    #[inline(always)]
    pub fn pc6_md(&mut self) -> Pc6MdW<'_, Moder3Spec> {
        Pc6MdW::new(self, 12)
    }
    #[doc = "Bits 14:15 - Work Mode Configuration"]
    #[inline(always)]
    pub fn pc7_md(&mut self) -> Pc7MdW<'_, Moder3Spec> {
        Pc7MdW::new(self, 14)
    }
    #[doc = "Bits 16:17 - Work Mode Configuration"]
    #[inline(always)]
    pub fn pc8_md(&mut self) -> Pc8MdW<'_, Moder3Spec> {
        Pc8MdW::new(self, 16)
    }
    #[doc = "Bits 18:19 - Work Mode Configuration"]
    #[inline(always)]
    pub fn pc9_md(&mut self) -> Pc9MdW<'_, Moder3Spec> {
        Pc9MdW::new(self, 18)
    }
    #[doc = "Bits 20:21 - Work Mode Configuration"]
    #[inline(always)]
    pub fn pc10_md(&mut self) -> Pc10MdW<'_, Moder3Spec> {
        Pc10MdW::new(self, 20)
    }
    #[doc = "Bits 22:23 - Work Mode Configuration"]
    #[inline(always)]
    pub fn pc11_md(&mut self) -> Pc11MdW<'_, Moder3Spec> {
        Pc11MdW::new(self, 22)
    }
    #[doc = "Bits 24:25 - Work Mode Configuration"]
    #[inline(always)]
    pub fn pc12_md(&mut self) -> Pc12MdW<'_, Moder3Spec> {
        Pc12MdW::new(self, 24)
    }
}
#[doc = "GPIO mode register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`moder3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`moder3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Moder3Spec;
impl crate::RegisterSpec for Moder3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`moder3::R`](R) reader structure"]
impl crate::Readable for Moder3Spec {}
#[doc = "`write(|w| ..)` method takes [`moder3::W`](W) writer structure"]
impl crate::Writable for Moder3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MODER3 to value 0"]
impl crate::Resettable for Moder3Spec {}
