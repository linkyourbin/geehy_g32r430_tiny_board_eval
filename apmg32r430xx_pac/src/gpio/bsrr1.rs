#[doc = "Register `BSRR1` reader"]
pub type R = crate::R<Bsrr1Spec>;
#[doc = "Register `BSRR1` writer"]
pub type W = crate::W<Bsrr1Spec>;
#[doc = "Field `PA0_BS` reader - PAD set pin"]
pub type Pa0BsR = crate::BitReader;
#[doc = "Field `PA0_BS` writer - PAD set pin"]
pub type Pa0BsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PA1_BS` reader - PAD set pin"]
pub type Pa1BsR = crate::BitReader;
#[doc = "Field `PA1_BS` writer - PAD set pin"]
pub type Pa1BsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PA2_BS` reader - PAD set pin"]
pub type Pa2BsR = crate::BitReader;
#[doc = "Field `PA2_BS` writer - PAD set pin"]
pub type Pa2BsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PA3_BS` reader - PAD set pin"]
pub type Pa3BsR = crate::BitReader;
#[doc = "Field `PA3_BS` writer - PAD set pin"]
pub type Pa3BsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PA4_BS` reader - PAD set pin"]
pub type Pa4BsR = crate::BitReader;
#[doc = "Field `PA4_BS` writer - PAD set pin"]
pub type Pa4BsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PA5_BS` reader - PAD set pin"]
pub type Pa5BsR = crate::BitReader;
#[doc = "Field `PA5_BS` writer - PAD set pin"]
pub type Pa5BsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PA6_BS` reader - PAD set pin"]
pub type Pa6BsR = crate::BitReader;
#[doc = "Field `PA6_BS` writer - PAD set pin"]
pub type Pa6BsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PA7_BS` reader - PAD set pin"]
pub type Pa7BsR = crate::BitReader;
#[doc = "Field `PA7_BS` writer - PAD set pin"]
pub type Pa7BsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PA8_BS` reader - PAD set pin"]
pub type Pa8BsR = crate::BitReader;
#[doc = "Field `PA8_BS` writer - PAD set pin"]
pub type Pa8BsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PA9_BS` reader - PAD set pin"]
pub type Pa9BsR = crate::BitReader;
#[doc = "Field `PA9_BS` writer - PAD set pin"]
pub type Pa9BsW<'a, REG> = crate::BitWriter<'a, REG>;
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
    #[doc = "Bit 0 - PAD set pin"]
    #[inline(always)]
    pub fn pa0_bs(&self) -> Pa0BsR {
        Pa0BsR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PAD set pin"]
    #[inline(always)]
    pub fn pa1_bs(&self) -> Pa1BsR {
        Pa1BsR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PAD set pin"]
    #[inline(always)]
    pub fn pa2_bs(&self) -> Pa2BsR {
        Pa2BsR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - PAD set pin"]
    #[inline(always)]
    pub fn pa3_bs(&self) -> Pa3BsR {
        Pa3BsR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - PAD set pin"]
    #[inline(always)]
    pub fn pa4_bs(&self) -> Pa4BsR {
        Pa4BsR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - PAD set pin"]
    #[inline(always)]
    pub fn pa5_bs(&self) -> Pa5BsR {
        Pa5BsR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - PAD set pin"]
    #[inline(always)]
    pub fn pa6_bs(&self) -> Pa6BsR {
        Pa6BsR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - PAD set pin"]
    #[inline(always)]
    pub fn pa7_bs(&self) -> Pa7BsR {
        Pa7BsR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - PAD set pin"]
    #[inline(always)]
    pub fn pa8_bs(&self) -> Pa8BsR {
        Pa8BsR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - PAD set pin"]
    #[inline(always)]
    pub fn pa9_bs(&self) -> Pa9BsR {
        Pa9BsR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 16 - PAD reset pin"]
    #[inline(always)]
    pub fn pa0_br(&self) -> Pa0BrR {
        Pa0BrR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - PAD reset pin"]
    #[inline(always)]
    pub fn pa1_br(&self) -> Pa1BrR {
        Pa1BrR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - PAD reset pin"]
    #[inline(always)]
    pub fn pa2_br(&self) -> Pa2BrR {
        Pa2BrR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - PAD reset pin"]
    #[inline(always)]
    pub fn pa3_br(&self) -> Pa3BrR {
        Pa3BrR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - PAD reset pin"]
    #[inline(always)]
    pub fn pa4_br(&self) -> Pa4BrR {
        Pa4BrR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - PAD reset pin"]
    #[inline(always)]
    pub fn pa5_br(&self) -> Pa5BrR {
        Pa5BrR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - PAD reset pin"]
    #[inline(always)]
    pub fn pa6_br(&self) -> Pa6BrR {
        Pa6BrR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - PAD reset pin"]
    #[inline(always)]
    pub fn pa7_br(&self) -> Pa7BrR {
        Pa7BrR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - PAD reset pin"]
    #[inline(always)]
    pub fn pa8_br(&self) -> Pa8BrR {
        Pa8BrR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - PAD reset pin"]
    #[inline(always)]
    pub fn pa9_br(&self) -> Pa9BrR {
        Pa9BrR::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PAD set pin"]
    #[inline(always)]
    pub fn pa0_bs(&mut self) -> Pa0BsW<'_, Bsrr1Spec> {
        Pa0BsW::new(self, 0)
    }
    #[doc = "Bit 1 - PAD set pin"]
    #[inline(always)]
    pub fn pa1_bs(&mut self) -> Pa1BsW<'_, Bsrr1Spec> {
        Pa1BsW::new(self, 1)
    }
    #[doc = "Bit 2 - PAD set pin"]
    #[inline(always)]
    pub fn pa2_bs(&mut self) -> Pa2BsW<'_, Bsrr1Spec> {
        Pa2BsW::new(self, 2)
    }
    #[doc = "Bit 3 - PAD set pin"]
    #[inline(always)]
    pub fn pa3_bs(&mut self) -> Pa3BsW<'_, Bsrr1Spec> {
        Pa3BsW::new(self, 3)
    }
    #[doc = "Bit 4 - PAD set pin"]
    #[inline(always)]
    pub fn pa4_bs(&mut self) -> Pa4BsW<'_, Bsrr1Spec> {
        Pa4BsW::new(self, 4)
    }
    #[doc = "Bit 5 - PAD set pin"]
    #[inline(always)]
    pub fn pa5_bs(&mut self) -> Pa5BsW<'_, Bsrr1Spec> {
        Pa5BsW::new(self, 5)
    }
    #[doc = "Bit 6 - PAD set pin"]
    #[inline(always)]
    pub fn pa6_bs(&mut self) -> Pa6BsW<'_, Bsrr1Spec> {
        Pa6BsW::new(self, 6)
    }
    #[doc = "Bit 7 - PAD set pin"]
    #[inline(always)]
    pub fn pa7_bs(&mut self) -> Pa7BsW<'_, Bsrr1Spec> {
        Pa7BsW::new(self, 7)
    }
    #[doc = "Bit 8 - PAD set pin"]
    #[inline(always)]
    pub fn pa8_bs(&mut self) -> Pa8BsW<'_, Bsrr1Spec> {
        Pa8BsW::new(self, 8)
    }
    #[doc = "Bit 9 - PAD set pin"]
    #[inline(always)]
    pub fn pa9_bs(&mut self) -> Pa9BsW<'_, Bsrr1Spec> {
        Pa9BsW::new(self, 9)
    }
    #[doc = "Bit 16 - PAD reset pin"]
    #[inline(always)]
    pub fn pa0_br(&mut self) -> Pa0BrW<'_, Bsrr1Spec> {
        Pa0BrW::new(self, 16)
    }
    #[doc = "Bit 17 - PAD reset pin"]
    #[inline(always)]
    pub fn pa1_br(&mut self) -> Pa1BrW<'_, Bsrr1Spec> {
        Pa1BrW::new(self, 17)
    }
    #[doc = "Bit 18 - PAD reset pin"]
    #[inline(always)]
    pub fn pa2_br(&mut self) -> Pa2BrW<'_, Bsrr1Spec> {
        Pa2BrW::new(self, 18)
    }
    #[doc = "Bit 19 - PAD reset pin"]
    #[inline(always)]
    pub fn pa3_br(&mut self) -> Pa3BrW<'_, Bsrr1Spec> {
        Pa3BrW::new(self, 19)
    }
    #[doc = "Bit 20 - PAD reset pin"]
    #[inline(always)]
    pub fn pa4_br(&mut self) -> Pa4BrW<'_, Bsrr1Spec> {
        Pa4BrW::new(self, 20)
    }
    #[doc = "Bit 21 - PAD reset pin"]
    #[inline(always)]
    pub fn pa5_br(&mut self) -> Pa5BrW<'_, Bsrr1Spec> {
        Pa5BrW::new(self, 21)
    }
    #[doc = "Bit 22 - PAD reset pin"]
    #[inline(always)]
    pub fn pa6_br(&mut self) -> Pa6BrW<'_, Bsrr1Spec> {
        Pa6BrW::new(self, 22)
    }
    #[doc = "Bit 23 - PAD reset pin"]
    #[inline(always)]
    pub fn pa7_br(&mut self) -> Pa7BrW<'_, Bsrr1Spec> {
        Pa7BrW::new(self, 23)
    }
    #[doc = "Bit 24 - PAD reset pin"]
    #[inline(always)]
    pub fn pa8_br(&mut self) -> Pa8BrW<'_, Bsrr1Spec> {
        Pa8BrW::new(self, 24)
    }
    #[doc = "Bit 25 - PAD reset pin"]
    #[inline(always)]
    pub fn pa9_br(&mut self) -> Pa9BrW<'_, Bsrr1Spec> {
        Pa9BrW::new(self, 25)
    }
}
#[doc = "Bit Set/Reset Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`bsrr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bsrr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Bsrr1Spec;
impl crate::RegisterSpec for Bsrr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bsrr1::R`](R) reader structure"]
impl crate::Readable for Bsrr1Spec {}
#[doc = "`write(|w| ..)` method takes [`bsrr1::W`](W) writer structure"]
impl crate::Writable for Bsrr1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BSRR1 to value 0"]
impl crate::Resettable for Bsrr1Spec {}
