#[doc = "Register `BRR3` reader"]
pub type R = crate::R<Brr3Spec>;
#[doc = "Register `BRR3` writer"]
pub type W = crate::W<Brr3Spec>;
#[doc = "Field `PC0_BR` reader - PAD reset pin"]
pub type Pc0BrR = crate::BitReader;
#[doc = "Field `PC0_BR` writer - PAD reset pin"]
pub type Pc0BrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PC1_BR` reader - PAD reset pin"]
pub type Pc1BrR = crate::BitReader;
#[doc = "Field `PC1_BR` writer - PAD reset pin"]
pub type Pc1BrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PC2_BR` reader - PAD reset pin"]
pub type Pc2BrR = crate::BitReader;
#[doc = "Field `PC2_BR` writer - PAD reset pin"]
pub type Pc2BrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PC3_BR` reader - PAD reset pin"]
pub type Pc3BrR = crate::BitReader;
#[doc = "Field `PC3_BR` writer - PAD reset pin"]
pub type Pc3BrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PC4_BR` reader - PAD reset pin"]
pub type Pc4BrR = crate::BitReader;
#[doc = "Field `PC4_BR` writer - PAD reset pin"]
pub type Pc4BrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PC5_BR` reader - PAD reset pin"]
pub type Pc5BrR = crate::BitReader;
#[doc = "Field `PC5_BR` writer - PAD reset pin"]
pub type Pc5BrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PC6_BR` reader - PAD reset pin"]
pub type Pc6BrR = crate::BitReader;
#[doc = "Field `PC6_BR` writer - PAD reset pin"]
pub type Pc6BrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PC7_BR` reader - PAD reset pin"]
pub type Pc7BrR = crate::BitReader;
#[doc = "Field `PC7_BR` writer - PAD reset pin"]
pub type Pc7BrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PC8_BR` reader - PAD reset pin"]
pub type Pc8BrR = crate::BitReader;
#[doc = "Field `PC8_BR` writer - PAD reset pin"]
pub type Pc8BrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PC9_BR` reader - PAD reset pin"]
pub type Pc9BrR = crate::BitReader;
#[doc = "Field `PC9_BR` writer - PAD reset pin"]
pub type Pc9BrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PC10_BR` reader - PAD reset pin"]
pub type Pc10BrR = crate::BitReader;
#[doc = "Field `PC10_BR` writer - PAD reset pin"]
pub type Pc10BrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PC11_BR` reader - PAD reset pin"]
pub type Pc11BrR = crate::BitReader;
#[doc = "Field `PC11_BR` writer - PAD reset pin"]
pub type Pc11BrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PC12_BR` reader - PAD reset pin"]
pub type Pc12BrR = crate::BitReader;
#[doc = "Field `PC12_BR` writer - PAD reset pin"]
pub type Pc12BrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - PAD reset pin"]
    #[inline(always)]
    pub fn pc0_br(&self) -> Pc0BrR {
        Pc0BrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PAD reset pin"]
    #[inline(always)]
    pub fn pc1_br(&self) -> Pc1BrR {
        Pc1BrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PAD reset pin"]
    #[inline(always)]
    pub fn pc2_br(&self) -> Pc2BrR {
        Pc2BrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - PAD reset pin"]
    #[inline(always)]
    pub fn pc3_br(&self) -> Pc3BrR {
        Pc3BrR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - PAD reset pin"]
    #[inline(always)]
    pub fn pc4_br(&self) -> Pc4BrR {
        Pc4BrR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - PAD reset pin"]
    #[inline(always)]
    pub fn pc5_br(&self) -> Pc5BrR {
        Pc5BrR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - PAD reset pin"]
    #[inline(always)]
    pub fn pc6_br(&self) -> Pc6BrR {
        Pc6BrR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - PAD reset pin"]
    #[inline(always)]
    pub fn pc7_br(&self) -> Pc7BrR {
        Pc7BrR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - PAD reset pin"]
    #[inline(always)]
    pub fn pc8_br(&self) -> Pc8BrR {
        Pc8BrR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - PAD reset pin"]
    #[inline(always)]
    pub fn pc9_br(&self) -> Pc9BrR {
        Pc9BrR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - PAD reset pin"]
    #[inline(always)]
    pub fn pc10_br(&self) -> Pc10BrR {
        Pc10BrR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - PAD reset pin"]
    #[inline(always)]
    pub fn pc11_br(&self) -> Pc11BrR {
        Pc11BrR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PAD reset pin"]
    #[inline(always)]
    pub fn pc12_br(&self) -> Pc12BrR {
        Pc12BrR::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PAD reset pin"]
    #[inline(always)]
    pub fn pc0_br(&mut self) -> Pc0BrW<'_, Brr3Spec> {
        Pc0BrW::new(self, 0)
    }
    #[doc = "Bit 1 - PAD reset pin"]
    #[inline(always)]
    pub fn pc1_br(&mut self) -> Pc1BrW<'_, Brr3Spec> {
        Pc1BrW::new(self, 1)
    }
    #[doc = "Bit 2 - PAD reset pin"]
    #[inline(always)]
    pub fn pc2_br(&mut self) -> Pc2BrW<'_, Brr3Spec> {
        Pc2BrW::new(self, 2)
    }
    #[doc = "Bit 3 - PAD reset pin"]
    #[inline(always)]
    pub fn pc3_br(&mut self) -> Pc3BrW<'_, Brr3Spec> {
        Pc3BrW::new(self, 3)
    }
    #[doc = "Bit 4 - PAD reset pin"]
    #[inline(always)]
    pub fn pc4_br(&mut self) -> Pc4BrW<'_, Brr3Spec> {
        Pc4BrW::new(self, 4)
    }
    #[doc = "Bit 5 - PAD reset pin"]
    #[inline(always)]
    pub fn pc5_br(&mut self) -> Pc5BrW<'_, Brr3Spec> {
        Pc5BrW::new(self, 5)
    }
    #[doc = "Bit 6 - PAD reset pin"]
    #[inline(always)]
    pub fn pc6_br(&mut self) -> Pc6BrW<'_, Brr3Spec> {
        Pc6BrW::new(self, 6)
    }
    #[doc = "Bit 7 - PAD reset pin"]
    #[inline(always)]
    pub fn pc7_br(&mut self) -> Pc7BrW<'_, Brr3Spec> {
        Pc7BrW::new(self, 7)
    }
    #[doc = "Bit 8 - PAD reset pin"]
    #[inline(always)]
    pub fn pc8_br(&mut self) -> Pc8BrW<'_, Brr3Spec> {
        Pc8BrW::new(self, 8)
    }
    #[doc = "Bit 9 - PAD reset pin"]
    #[inline(always)]
    pub fn pc9_br(&mut self) -> Pc9BrW<'_, Brr3Spec> {
        Pc9BrW::new(self, 9)
    }
    #[doc = "Bit 10 - PAD reset pin"]
    #[inline(always)]
    pub fn pc10_br(&mut self) -> Pc10BrW<'_, Brr3Spec> {
        Pc10BrW::new(self, 10)
    }
    #[doc = "Bit 11 - PAD reset pin"]
    #[inline(always)]
    pub fn pc11_br(&mut self) -> Pc11BrW<'_, Brr3Spec> {
        Pc11BrW::new(self, 11)
    }
    #[doc = "Bit 12 - PAD reset pin"]
    #[inline(always)]
    pub fn pc12_br(&mut self) -> Pc12BrW<'_, Brr3Spec> {
        Pc12BrW::new(self, 12)
    }
}
#[doc = "Bit Reset Register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`brr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`brr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Brr3Spec;
impl crate::RegisterSpec for Brr3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`brr3::R`](R) reader structure"]
impl crate::Readable for Brr3Spec {}
#[doc = "`write(|w| ..)` method takes [`brr3::W`](W) writer structure"]
impl crate::Writable for Brr3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BRR3 to value 0"]
impl crate::Resettable for Brr3Spec {}
