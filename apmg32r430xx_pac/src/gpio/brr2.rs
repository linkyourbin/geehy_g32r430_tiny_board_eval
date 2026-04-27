#[doc = "Register `BRR2` reader"]
pub type R = crate::R<Brr2Spec>;
#[doc = "Register `BRR2` writer"]
pub type W = crate::W<Brr2Spec>;
#[doc = "Field `PB0_BR` reader - PAD reset pin"]
pub type Pb0BrR = crate::BitReader;
#[doc = "Field `PB0_BR` writer - PAD reset pin"]
pub type Pb0BrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PB1_BR` reader - PAD reset pin"]
pub type Pb1BrR = crate::BitReader;
#[doc = "Field `PB1_BR` writer - PAD reset pin"]
pub type Pb1BrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PB2_BR` reader - PAD reset pin"]
pub type Pb2BrR = crate::BitReader;
#[doc = "Field `PB2_BR` writer - PAD reset pin"]
pub type Pb2BrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PB3_BR` reader - PAD reset pin"]
pub type Pb3BrR = crate::BitReader;
#[doc = "Field `PB3_BR` writer - PAD reset pin"]
pub type Pb3BrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PB4_BR` reader - PAD reset pin"]
pub type Pb4BrR = crate::BitReader;
#[doc = "Field `PB4_BR` writer - PAD reset pin"]
pub type Pb4BrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PB5_BR` reader - PAD reset pin"]
pub type Pb5BrR = crate::BitReader;
#[doc = "Field `PB5_BR` writer - PAD reset pin"]
pub type Pb5BrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PB6_BR` reader - PAD reset pin"]
pub type Pb6BrR = crate::BitReader;
#[doc = "Field `PB6_BR` writer - PAD reset pin"]
pub type Pb6BrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PB7_BR` reader - PAD reset pin"]
pub type Pb7BrR = crate::BitReader;
#[doc = "Field `PB7_BR` writer - PAD reset pin"]
pub type Pb7BrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PB8_BR` reader - PAD reset pin"]
pub type Pb8BrR = crate::BitReader;
#[doc = "Field `PB8_BR` writer - PAD reset pin"]
pub type Pb8BrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PB9_BR` reader - PAD reset pin"]
pub type Pb9BrR = crate::BitReader;
#[doc = "Field `PB9_BR` writer - PAD reset pin"]
pub type Pb9BrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PB10_BR` reader - PAD reset pin"]
pub type Pb10BrR = crate::BitReader;
#[doc = "Field `PB10_BR` writer - PAD reset pin"]
pub type Pb10BrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PB11_BR` reader - PAD reset pin"]
pub type Pb11BrR = crate::BitReader;
#[doc = "Field `PB11_BR` writer - PAD reset pin"]
pub type Pb11BrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PB12_BR` reader - PAD reset pin"]
pub type Pb12BrR = crate::BitReader;
#[doc = "Field `PB12_BR` writer - PAD reset pin"]
pub type Pb12BrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PB13_BR` reader - PAD reset pin"]
pub type Pb13BrR = crate::BitReader;
#[doc = "Field `PB13_BR` writer - PAD reset pin"]
pub type Pb13BrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - PAD reset pin"]
    #[inline(always)]
    pub fn pb0_br(&self) -> Pb0BrR {
        Pb0BrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PAD reset pin"]
    #[inline(always)]
    pub fn pb1_br(&self) -> Pb1BrR {
        Pb1BrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PAD reset pin"]
    #[inline(always)]
    pub fn pb2_br(&self) -> Pb2BrR {
        Pb2BrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - PAD reset pin"]
    #[inline(always)]
    pub fn pb3_br(&self) -> Pb3BrR {
        Pb3BrR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - PAD reset pin"]
    #[inline(always)]
    pub fn pb4_br(&self) -> Pb4BrR {
        Pb4BrR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - PAD reset pin"]
    #[inline(always)]
    pub fn pb5_br(&self) -> Pb5BrR {
        Pb5BrR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - PAD reset pin"]
    #[inline(always)]
    pub fn pb6_br(&self) -> Pb6BrR {
        Pb6BrR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - PAD reset pin"]
    #[inline(always)]
    pub fn pb7_br(&self) -> Pb7BrR {
        Pb7BrR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - PAD reset pin"]
    #[inline(always)]
    pub fn pb8_br(&self) -> Pb8BrR {
        Pb8BrR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - PAD reset pin"]
    #[inline(always)]
    pub fn pb9_br(&self) -> Pb9BrR {
        Pb9BrR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - PAD reset pin"]
    #[inline(always)]
    pub fn pb10_br(&self) -> Pb10BrR {
        Pb10BrR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - PAD reset pin"]
    #[inline(always)]
    pub fn pb11_br(&self) -> Pb11BrR {
        Pb11BrR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PAD reset pin"]
    #[inline(always)]
    pub fn pb12_br(&self) -> Pb12BrR {
        Pb12BrR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - PAD reset pin"]
    #[inline(always)]
    pub fn pb13_br(&self) -> Pb13BrR {
        Pb13BrR::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PAD reset pin"]
    #[inline(always)]
    pub fn pb0_br(&mut self) -> Pb0BrW<'_, Brr2Spec> {
        Pb0BrW::new(self, 0)
    }
    #[doc = "Bit 1 - PAD reset pin"]
    #[inline(always)]
    pub fn pb1_br(&mut self) -> Pb1BrW<'_, Brr2Spec> {
        Pb1BrW::new(self, 1)
    }
    #[doc = "Bit 2 - PAD reset pin"]
    #[inline(always)]
    pub fn pb2_br(&mut self) -> Pb2BrW<'_, Brr2Spec> {
        Pb2BrW::new(self, 2)
    }
    #[doc = "Bit 3 - PAD reset pin"]
    #[inline(always)]
    pub fn pb3_br(&mut self) -> Pb3BrW<'_, Brr2Spec> {
        Pb3BrW::new(self, 3)
    }
    #[doc = "Bit 4 - PAD reset pin"]
    #[inline(always)]
    pub fn pb4_br(&mut self) -> Pb4BrW<'_, Brr2Spec> {
        Pb4BrW::new(self, 4)
    }
    #[doc = "Bit 5 - PAD reset pin"]
    #[inline(always)]
    pub fn pb5_br(&mut self) -> Pb5BrW<'_, Brr2Spec> {
        Pb5BrW::new(self, 5)
    }
    #[doc = "Bit 6 - PAD reset pin"]
    #[inline(always)]
    pub fn pb6_br(&mut self) -> Pb6BrW<'_, Brr2Spec> {
        Pb6BrW::new(self, 6)
    }
    #[doc = "Bit 7 - PAD reset pin"]
    #[inline(always)]
    pub fn pb7_br(&mut self) -> Pb7BrW<'_, Brr2Spec> {
        Pb7BrW::new(self, 7)
    }
    #[doc = "Bit 8 - PAD reset pin"]
    #[inline(always)]
    pub fn pb8_br(&mut self) -> Pb8BrW<'_, Brr2Spec> {
        Pb8BrW::new(self, 8)
    }
    #[doc = "Bit 9 - PAD reset pin"]
    #[inline(always)]
    pub fn pb9_br(&mut self) -> Pb9BrW<'_, Brr2Spec> {
        Pb9BrW::new(self, 9)
    }
    #[doc = "Bit 10 - PAD reset pin"]
    #[inline(always)]
    pub fn pb10_br(&mut self) -> Pb10BrW<'_, Brr2Spec> {
        Pb10BrW::new(self, 10)
    }
    #[doc = "Bit 11 - PAD reset pin"]
    #[inline(always)]
    pub fn pb11_br(&mut self) -> Pb11BrW<'_, Brr2Spec> {
        Pb11BrW::new(self, 11)
    }
    #[doc = "Bit 12 - PAD reset pin"]
    #[inline(always)]
    pub fn pb12_br(&mut self) -> Pb12BrW<'_, Brr2Spec> {
        Pb12BrW::new(self, 12)
    }
    #[doc = "Bit 13 - PAD reset pin"]
    #[inline(always)]
    pub fn pb13_br(&mut self) -> Pb13BrW<'_, Brr2Spec> {
        Pb13BrW::new(self, 13)
    }
}
#[doc = "Bit Reset Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`brr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`brr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Brr2Spec;
impl crate::RegisterSpec for Brr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`brr2::R`](R) reader structure"]
impl crate::Readable for Brr2Spec {}
#[doc = "`write(|w| ..)` method takes [`brr2::W`](W) writer structure"]
impl crate::Writable for Brr2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BRR2 to value 0"]
impl crate::Resettable for Brr2Spec {}
