#[doc = "Register `MODER1` reader"]
pub type R = crate::R<Moder1Spec>;
#[doc = "Register `MODER1` writer"]
pub type W = crate::W<Moder1Spec>;
#[doc = "Field `PA0_MD` reader - Work Mode Configuration"]
pub type Pa0MdR = crate::FieldReader;
#[doc = "Field `PA0_MD` writer - Work Mode Configuration"]
pub type Pa0MdW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PA1_MD` reader - Work Mode Configuration"]
pub type Pa1MdR = crate::FieldReader;
#[doc = "Field `PA1_MD` writer - Work Mode Configuration"]
pub type Pa1MdW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PA2_MD` reader - Work Mode Configuration"]
pub type Pa2MdR = crate::FieldReader;
#[doc = "Field `PA2_MD` writer - Work Mode Configuration"]
pub type Pa2MdW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PA3_MD` reader - Work Mode Configuration"]
pub type Pa3MdR = crate::FieldReader;
#[doc = "Field `PA3_MD` writer - Work Mode Configuration"]
pub type Pa3MdW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PA4_MD` reader - Work Mode Configuration"]
pub type Pa4MdR = crate::FieldReader;
#[doc = "Field `PA4_MD` writer - Work Mode Configuration"]
pub type Pa4MdW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PA5_MD` reader - Work Mode Configuration"]
pub type Pa5MdR = crate::FieldReader;
#[doc = "Field `PA5_MD` writer - Work Mode Configuration"]
pub type Pa5MdW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PA6_MD` reader - Work Mode Configuration"]
pub type Pa6MdR = crate::FieldReader;
#[doc = "Field `PA6_MD` writer - Work Mode Configuration"]
pub type Pa6MdW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PA7_MD` reader - Work Mode Configuration"]
pub type Pa7MdR = crate::FieldReader;
#[doc = "Field `PA7_MD` writer - Work Mode Configuration"]
pub type Pa7MdW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PA8_MD` reader - Work Mode Configuration"]
pub type Pa8MdR = crate::FieldReader;
#[doc = "Field `PA8_MD` writer - Work Mode Configuration"]
pub type Pa8MdW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PA9_MD` reader - Work Mode Configuration"]
pub type Pa9MdR = crate::FieldReader;
#[doc = "Field `PA9_MD` writer - Work Mode Configuration"]
pub type Pa9MdW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - Work Mode Configuration"]
    #[inline(always)]
    pub fn pa0_md(&self) -> Pa0MdR {
        Pa0MdR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Work Mode Configuration"]
    #[inline(always)]
    pub fn pa1_md(&self) -> Pa1MdR {
        Pa1MdR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Work Mode Configuration"]
    #[inline(always)]
    pub fn pa2_md(&self) -> Pa2MdR {
        Pa2MdR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Work Mode Configuration"]
    #[inline(always)]
    pub fn pa3_md(&self) -> Pa3MdR {
        Pa3MdR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Work Mode Configuration"]
    #[inline(always)]
    pub fn pa4_md(&self) -> Pa4MdR {
        Pa4MdR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Work Mode Configuration"]
    #[inline(always)]
    pub fn pa5_md(&self) -> Pa5MdR {
        Pa5MdR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Work Mode Configuration"]
    #[inline(always)]
    pub fn pa6_md(&self) -> Pa6MdR {
        Pa6MdR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Work Mode Configuration"]
    #[inline(always)]
    pub fn pa7_md(&self) -> Pa7MdR {
        Pa7MdR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Work Mode Configuration"]
    #[inline(always)]
    pub fn pa8_md(&self) -> Pa8MdR {
        Pa8MdR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - Work Mode Configuration"]
    #[inline(always)]
    pub fn pa9_md(&self) -> Pa9MdR {
        Pa9MdR::new(((self.bits >> 18) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Work Mode Configuration"]
    #[inline(always)]
    pub fn pa0_md(&mut self) -> Pa0MdW<'_, Moder1Spec> {
        Pa0MdW::new(self, 0)
    }
    #[doc = "Bits 2:3 - Work Mode Configuration"]
    #[inline(always)]
    pub fn pa1_md(&mut self) -> Pa1MdW<'_, Moder1Spec> {
        Pa1MdW::new(self, 2)
    }
    #[doc = "Bits 4:5 - Work Mode Configuration"]
    #[inline(always)]
    pub fn pa2_md(&mut self) -> Pa2MdW<'_, Moder1Spec> {
        Pa2MdW::new(self, 4)
    }
    #[doc = "Bits 6:7 - Work Mode Configuration"]
    #[inline(always)]
    pub fn pa3_md(&mut self) -> Pa3MdW<'_, Moder1Spec> {
        Pa3MdW::new(self, 6)
    }
    #[doc = "Bits 8:9 - Work Mode Configuration"]
    #[inline(always)]
    pub fn pa4_md(&mut self) -> Pa4MdW<'_, Moder1Spec> {
        Pa4MdW::new(self, 8)
    }
    #[doc = "Bits 10:11 - Work Mode Configuration"]
    #[inline(always)]
    pub fn pa5_md(&mut self) -> Pa5MdW<'_, Moder1Spec> {
        Pa5MdW::new(self, 10)
    }
    #[doc = "Bits 12:13 - Work Mode Configuration"]
    #[inline(always)]
    pub fn pa6_md(&mut self) -> Pa6MdW<'_, Moder1Spec> {
        Pa6MdW::new(self, 12)
    }
    #[doc = "Bits 14:15 - Work Mode Configuration"]
    #[inline(always)]
    pub fn pa7_md(&mut self) -> Pa7MdW<'_, Moder1Spec> {
        Pa7MdW::new(self, 14)
    }
    #[doc = "Bits 16:17 - Work Mode Configuration"]
    #[inline(always)]
    pub fn pa8_md(&mut self) -> Pa8MdW<'_, Moder1Spec> {
        Pa8MdW::new(self, 16)
    }
    #[doc = "Bits 18:19 - Work Mode Configuration"]
    #[inline(always)]
    pub fn pa9_md(&mut self) -> Pa9MdW<'_, Moder1Spec> {
        Pa9MdW::new(self, 18)
    }
}
#[doc = "GPIO mode register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`moder1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`moder1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Moder1Spec;
impl crate::RegisterSpec for Moder1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`moder1::R`](R) reader structure"]
impl crate::Readable for Moder1Spec {}
#[doc = "`write(|w| ..)` method takes [`moder1::W`](W) writer structure"]
impl crate::Writable for Moder1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MODER1 to value 0"]
impl crate::Resettable for Moder1Spec {}
