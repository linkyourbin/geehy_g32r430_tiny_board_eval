#[doc = "Register `BRR1` reader"]
pub type R = crate::R<Brr1Spec>;
#[doc = "Register `BRR1` writer"]
pub type W = crate::W<Brr1Spec>;
#[doc = "Field `PA0_BR` reader - PAD reset pin"]
pub type Pa0BrR = crate::BitReader;
#[doc = "Field `PA0_BR` writer - PAD reset pin"]
pub type Pa0BrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PA1_BR` reader - PAD reset pin"]
pub type Pa1BrR = crate::BitReader;
#[doc = "Field `PA1_BR` writer - PAD reset pin"]
pub type Pa1BrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PA2_BR` reader - PAD reset pin"]
pub type Pa2BrR = crate::BitReader;
#[doc = "Field `PA2_BR` writer - PAD reset pin"]
pub type Pa2BrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PA3_BR` reader - PAD reset pin"]
pub type Pa3BrR = crate::BitReader;
#[doc = "Field `PA3_BR` writer - PAD reset pin"]
pub type Pa3BrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PA4_BR` reader - PAD reset pin"]
pub type Pa4BrR = crate::BitReader;
#[doc = "Field `PA4_BR` writer - PAD reset pin"]
pub type Pa4BrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PA5_BR` reader - PAD reset pin"]
pub type Pa5BrR = crate::BitReader;
#[doc = "Field `PA5_BR` writer - PAD reset pin"]
pub type Pa5BrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PA6_BR` reader - PAD reset pin"]
pub type Pa6BrR = crate::BitReader;
#[doc = "Field `PA6_BR` writer - PAD reset pin"]
pub type Pa6BrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PA7_BR` reader - PAD reset pin"]
pub type Pa7BrR = crate::BitReader;
#[doc = "Field `PA7_BR` writer - PAD reset pin"]
pub type Pa7BrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PA8_BR` reader - PAD reset pin"]
pub type Pa8BrR = crate::BitReader;
#[doc = "Field `PA8_BR` writer - PAD reset pin"]
pub type Pa8BrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PA9_BR` reader - PAD reset pin"]
pub type Pa9BrR = crate::BitReader;
#[doc = "Field `PA9_BR` writer - PAD reset pin"]
pub type Pa9BrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - PAD reset pin"]
    #[inline(always)]
    pub fn pa0_br(&self) -> Pa0BrR {
        Pa0BrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PAD reset pin"]
    #[inline(always)]
    pub fn pa1_br(&self) -> Pa1BrR {
        Pa1BrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PAD reset pin"]
    #[inline(always)]
    pub fn pa2_br(&self) -> Pa2BrR {
        Pa2BrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - PAD reset pin"]
    #[inline(always)]
    pub fn pa3_br(&self) -> Pa3BrR {
        Pa3BrR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - PAD reset pin"]
    #[inline(always)]
    pub fn pa4_br(&self) -> Pa4BrR {
        Pa4BrR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - PAD reset pin"]
    #[inline(always)]
    pub fn pa5_br(&self) -> Pa5BrR {
        Pa5BrR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - PAD reset pin"]
    #[inline(always)]
    pub fn pa6_br(&self) -> Pa6BrR {
        Pa6BrR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - PAD reset pin"]
    #[inline(always)]
    pub fn pa7_br(&self) -> Pa7BrR {
        Pa7BrR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - PAD reset pin"]
    #[inline(always)]
    pub fn pa8_br(&self) -> Pa8BrR {
        Pa8BrR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - PAD reset pin"]
    #[inline(always)]
    pub fn pa9_br(&self) -> Pa9BrR {
        Pa9BrR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PAD reset pin"]
    #[inline(always)]
    pub fn pa0_br(&mut self) -> Pa0BrW<'_, Brr1Spec> {
        Pa0BrW::new(self, 0)
    }
    #[doc = "Bit 1 - PAD reset pin"]
    #[inline(always)]
    pub fn pa1_br(&mut self) -> Pa1BrW<'_, Brr1Spec> {
        Pa1BrW::new(self, 1)
    }
    #[doc = "Bit 2 - PAD reset pin"]
    #[inline(always)]
    pub fn pa2_br(&mut self) -> Pa2BrW<'_, Brr1Spec> {
        Pa2BrW::new(self, 2)
    }
    #[doc = "Bit 3 - PAD reset pin"]
    #[inline(always)]
    pub fn pa3_br(&mut self) -> Pa3BrW<'_, Brr1Spec> {
        Pa3BrW::new(self, 3)
    }
    #[doc = "Bit 4 - PAD reset pin"]
    #[inline(always)]
    pub fn pa4_br(&mut self) -> Pa4BrW<'_, Brr1Spec> {
        Pa4BrW::new(self, 4)
    }
    #[doc = "Bit 5 - PAD reset pin"]
    #[inline(always)]
    pub fn pa5_br(&mut self) -> Pa5BrW<'_, Brr1Spec> {
        Pa5BrW::new(self, 5)
    }
    #[doc = "Bit 6 - PAD reset pin"]
    #[inline(always)]
    pub fn pa6_br(&mut self) -> Pa6BrW<'_, Brr1Spec> {
        Pa6BrW::new(self, 6)
    }
    #[doc = "Bit 7 - PAD reset pin"]
    #[inline(always)]
    pub fn pa7_br(&mut self) -> Pa7BrW<'_, Brr1Spec> {
        Pa7BrW::new(self, 7)
    }
    #[doc = "Bit 8 - PAD reset pin"]
    #[inline(always)]
    pub fn pa8_br(&mut self) -> Pa8BrW<'_, Brr1Spec> {
        Pa8BrW::new(self, 8)
    }
    #[doc = "Bit 9 - PAD reset pin"]
    #[inline(always)]
    pub fn pa9_br(&mut self) -> Pa9BrW<'_, Brr1Spec> {
        Pa9BrW::new(self, 9)
    }
}
#[doc = "Bit Reset Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`brr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`brr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Brr1Spec;
impl crate::RegisterSpec for Brr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`brr1::R`](R) reader structure"]
impl crate::Readable for Brr1Spec {}
#[doc = "`write(|w| ..)` method takes [`brr1::W`](W) writer structure"]
impl crate::Writable for Brr1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BRR1 to value 0"]
impl crate::Resettable for Brr1Spec {}
