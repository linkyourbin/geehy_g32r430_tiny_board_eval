#[doc = "Register `BRR4` reader"]
pub type R = crate::R<Brr4Spec>;
#[doc = "Register `BRR4` writer"]
pub type W = crate::W<Brr4Spec>;
#[doc = "Field `PD0_BR` reader - PAD reset pin"]
pub type Pd0BrR = crate::BitReader;
#[doc = "Field `PD0_BR` writer - PAD reset pin"]
pub type Pd0BrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PD1_BR` reader - PAD reset pin"]
pub type Pd1BrR = crate::BitReader;
#[doc = "Field `PD1_BR` writer - PAD reset pin"]
pub type Pd1BrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PD2_BR` reader - PAD reset pin"]
pub type Pd2BrR = crate::BitReader;
#[doc = "Field `PD2_BR` writer - PAD reset pin"]
pub type Pd2BrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PD3_BR` reader - PAD reset pin"]
pub type Pd3BrR = crate::BitReader;
#[doc = "Field `PD3_BR` writer - PAD reset pin"]
pub type Pd3BrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PD4_BR` reader - PAD reset pin"]
pub type Pd4BrR = crate::BitReader;
#[doc = "Field `PD4_BR` writer - PAD reset pin"]
pub type Pd4BrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PD5_BR` reader - PAD reset pin"]
pub type Pd5BrR = crate::BitReader;
#[doc = "Field `PD5_BR` writer - PAD reset pin"]
pub type Pd5BrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PD6_BR` reader - PAD reset pin"]
pub type Pd6BrR = crate::BitReader;
#[doc = "Field `PD6_BR` writer - PAD reset pin"]
pub type Pd6BrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PD7_BR` reader - PAD reset pin"]
pub type Pd7BrR = crate::BitReader;
#[doc = "Field `PD7_BR` writer - PAD reset pin"]
pub type Pd7BrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PD8_BR` reader - PAD reset pin"]
pub type Pd8BrR = crate::BitReader;
#[doc = "Field `PD8_BR` writer - PAD reset pin"]
pub type Pd8BrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PD9_BR` reader - PAD reset pin"]
pub type Pd9BrR = crate::BitReader;
#[doc = "Field `PD9_BR` writer - PAD reset pin"]
pub type Pd9BrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PD10_BR` reader - PAD reset pin"]
pub type Pd10BrR = crate::BitReader;
#[doc = "Field `PD10_BR` writer - PAD reset pin"]
pub type Pd10BrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PD11_BR` reader - PAD reset pin"]
pub type Pd11BrR = crate::BitReader;
#[doc = "Field `PD11_BR` writer - PAD reset pin"]
pub type Pd11BrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PD12_BR` reader - PAD reset pin"]
pub type Pd12BrR = crate::BitReader;
#[doc = "Field `PD12_BR` writer - PAD reset pin"]
pub type Pd12BrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PD13_BR` reader - PAD reset pin"]
pub type Pd13BrR = crate::BitReader;
#[doc = "Field `PD13_BR` writer - PAD reset pin"]
pub type Pd13BrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PD14_BR` reader - PAD reset pin"]
pub type Pd14BrR = crate::BitReader;
#[doc = "Field `PD14_BR` writer - PAD reset pin"]
pub type Pd14BrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PD15_BR` reader - PAD reset pin"]
pub type Pd15BrR = crate::BitReader;
#[doc = "Field `PD15_BR` writer - PAD reset pin"]
pub type Pd15BrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - PAD reset pin"]
    #[inline(always)]
    pub fn pd0_br(&self) -> Pd0BrR {
        Pd0BrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PAD reset pin"]
    #[inline(always)]
    pub fn pd1_br(&self) -> Pd1BrR {
        Pd1BrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PAD reset pin"]
    #[inline(always)]
    pub fn pd2_br(&self) -> Pd2BrR {
        Pd2BrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - PAD reset pin"]
    #[inline(always)]
    pub fn pd3_br(&self) -> Pd3BrR {
        Pd3BrR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - PAD reset pin"]
    #[inline(always)]
    pub fn pd4_br(&self) -> Pd4BrR {
        Pd4BrR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - PAD reset pin"]
    #[inline(always)]
    pub fn pd5_br(&self) -> Pd5BrR {
        Pd5BrR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - PAD reset pin"]
    #[inline(always)]
    pub fn pd6_br(&self) -> Pd6BrR {
        Pd6BrR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - PAD reset pin"]
    #[inline(always)]
    pub fn pd7_br(&self) -> Pd7BrR {
        Pd7BrR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - PAD reset pin"]
    #[inline(always)]
    pub fn pd8_br(&self) -> Pd8BrR {
        Pd8BrR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - PAD reset pin"]
    #[inline(always)]
    pub fn pd9_br(&self) -> Pd9BrR {
        Pd9BrR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - PAD reset pin"]
    #[inline(always)]
    pub fn pd10_br(&self) -> Pd10BrR {
        Pd10BrR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - PAD reset pin"]
    #[inline(always)]
    pub fn pd11_br(&self) -> Pd11BrR {
        Pd11BrR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PAD reset pin"]
    #[inline(always)]
    pub fn pd12_br(&self) -> Pd12BrR {
        Pd12BrR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - PAD reset pin"]
    #[inline(always)]
    pub fn pd13_br(&self) -> Pd13BrR {
        Pd13BrR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - PAD reset pin"]
    #[inline(always)]
    pub fn pd14_br(&self) -> Pd14BrR {
        Pd14BrR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - PAD reset pin"]
    #[inline(always)]
    pub fn pd15_br(&self) -> Pd15BrR {
        Pd15BrR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PAD reset pin"]
    #[inline(always)]
    pub fn pd0_br(&mut self) -> Pd0BrW<'_, Brr4Spec> {
        Pd0BrW::new(self, 0)
    }
    #[doc = "Bit 1 - PAD reset pin"]
    #[inline(always)]
    pub fn pd1_br(&mut self) -> Pd1BrW<'_, Brr4Spec> {
        Pd1BrW::new(self, 1)
    }
    #[doc = "Bit 2 - PAD reset pin"]
    #[inline(always)]
    pub fn pd2_br(&mut self) -> Pd2BrW<'_, Brr4Spec> {
        Pd2BrW::new(self, 2)
    }
    #[doc = "Bit 3 - PAD reset pin"]
    #[inline(always)]
    pub fn pd3_br(&mut self) -> Pd3BrW<'_, Brr4Spec> {
        Pd3BrW::new(self, 3)
    }
    #[doc = "Bit 4 - PAD reset pin"]
    #[inline(always)]
    pub fn pd4_br(&mut self) -> Pd4BrW<'_, Brr4Spec> {
        Pd4BrW::new(self, 4)
    }
    #[doc = "Bit 5 - PAD reset pin"]
    #[inline(always)]
    pub fn pd5_br(&mut self) -> Pd5BrW<'_, Brr4Spec> {
        Pd5BrW::new(self, 5)
    }
    #[doc = "Bit 6 - PAD reset pin"]
    #[inline(always)]
    pub fn pd6_br(&mut self) -> Pd6BrW<'_, Brr4Spec> {
        Pd6BrW::new(self, 6)
    }
    #[doc = "Bit 7 - PAD reset pin"]
    #[inline(always)]
    pub fn pd7_br(&mut self) -> Pd7BrW<'_, Brr4Spec> {
        Pd7BrW::new(self, 7)
    }
    #[doc = "Bit 8 - PAD reset pin"]
    #[inline(always)]
    pub fn pd8_br(&mut self) -> Pd8BrW<'_, Brr4Spec> {
        Pd8BrW::new(self, 8)
    }
    #[doc = "Bit 9 - PAD reset pin"]
    #[inline(always)]
    pub fn pd9_br(&mut self) -> Pd9BrW<'_, Brr4Spec> {
        Pd9BrW::new(self, 9)
    }
    #[doc = "Bit 10 - PAD reset pin"]
    #[inline(always)]
    pub fn pd10_br(&mut self) -> Pd10BrW<'_, Brr4Spec> {
        Pd10BrW::new(self, 10)
    }
    #[doc = "Bit 11 - PAD reset pin"]
    #[inline(always)]
    pub fn pd11_br(&mut self) -> Pd11BrW<'_, Brr4Spec> {
        Pd11BrW::new(self, 11)
    }
    #[doc = "Bit 12 - PAD reset pin"]
    #[inline(always)]
    pub fn pd12_br(&mut self) -> Pd12BrW<'_, Brr4Spec> {
        Pd12BrW::new(self, 12)
    }
    #[doc = "Bit 13 - PAD reset pin"]
    #[inline(always)]
    pub fn pd13_br(&mut self) -> Pd13BrW<'_, Brr4Spec> {
        Pd13BrW::new(self, 13)
    }
    #[doc = "Bit 14 - PAD reset pin"]
    #[inline(always)]
    pub fn pd14_br(&mut self) -> Pd14BrW<'_, Brr4Spec> {
        Pd14BrW::new(self, 14)
    }
    #[doc = "Bit 15 - PAD reset pin"]
    #[inline(always)]
    pub fn pd15_br(&mut self) -> Pd15BrW<'_, Brr4Spec> {
        Pd15BrW::new(self, 15)
    }
}
#[doc = "Bit Reset Register 4\n\nYou can [`read`](crate::Reg::read) this register and get [`brr4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`brr4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Brr4Spec;
impl crate::RegisterSpec for Brr4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`brr4::R`](R) reader structure"]
impl crate::Readable for Brr4Spec {}
#[doc = "`write(|w| ..)` method takes [`brr4::W`](W) writer structure"]
impl crate::Writable for Brr4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BRR4 to value 0"]
impl crate::Resettable for Brr4Spec {}
