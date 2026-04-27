#[doc = "Register `OSPEEDR1` reader"]
pub type R = crate::R<Ospeedr1Spec>;
#[doc = "Register `OSPEEDR1` writer"]
pub type W = crate::W<Ospeedr1Spec>;
#[doc = "Field `PA0_OS` reader - PAD output speed configuration"]
pub type Pa0OsR = crate::FieldReader;
#[doc = "Field `PA0_OS` writer - PAD output speed configuration"]
pub type Pa0OsW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PA1_OS` reader - PAD output speed configuration"]
pub type Pa1OsR = crate::FieldReader;
#[doc = "Field `PA1_OS` writer - PAD output speed configuration"]
pub type Pa1OsW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PA2_OS` reader - PAD output speed configuration"]
pub type Pa2OsR = crate::FieldReader;
#[doc = "Field `PA2_OS` writer - PAD output speed configuration"]
pub type Pa2OsW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PA3_OS` reader - PAD output speed configuration"]
pub type Pa3OsR = crate::FieldReader;
#[doc = "Field `PA3_OS` writer - PAD output speed configuration"]
pub type Pa3OsW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PA4_OS` reader - PAD output speed configuration"]
pub type Pa4OsR = crate::FieldReader;
#[doc = "Field `PA4_OS` writer - PAD output speed configuration"]
pub type Pa4OsW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PA5_OS` reader - PAD output speed configuration"]
pub type Pa5OsR = crate::FieldReader;
#[doc = "Field `PA5_OS` writer - PAD output speed configuration"]
pub type Pa5OsW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PA6_OS` reader - PAD output speed configuration"]
pub type Pa6OsR = crate::FieldReader;
#[doc = "Field `PA6_OS` writer - PAD output speed configuration"]
pub type Pa6OsW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PA7_OS` reader - PAD output speed configuration"]
pub type Pa7OsR = crate::FieldReader;
#[doc = "Field `PA7_OS` writer - PAD output speed configuration"]
pub type Pa7OsW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PA8_OS` reader - PAD output speed configuration"]
pub type Pa8OsR = crate::FieldReader;
#[doc = "Field `PA8_OS` writer - PAD output speed configuration"]
pub type Pa8OsW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PA9_OS` reader - PAD output speed configuration"]
pub type Pa9OsR = crate::FieldReader;
#[doc = "Field `PA9_OS` writer - PAD output speed configuration"]
pub type Pa9OsW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - PAD output speed configuration"]
    #[inline(always)]
    pub fn pa0_os(&self) -> Pa0OsR {
        Pa0OsR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - PAD output speed configuration"]
    #[inline(always)]
    pub fn pa1_os(&self) -> Pa1OsR {
        Pa1OsR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - PAD output speed configuration"]
    #[inline(always)]
    pub fn pa2_os(&self) -> Pa2OsR {
        Pa2OsR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - PAD output speed configuration"]
    #[inline(always)]
    pub fn pa3_os(&self) -> Pa3OsR {
        Pa3OsR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - PAD output speed configuration"]
    #[inline(always)]
    pub fn pa4_os(&self) -> Pa4OsR {
        Pa4OsR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - PAD output speed configuration"]
    #[inline(always)]
    pub fn pa5_os(&self) -> Pa5OsR {
        Pa5OsR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - PAD output speed configuration"]
    #[inline(always)]
    pub fn pa6_os(&self) -> Pa6OsR {
        Pa6OsR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - PAD output speed configuration"]
    #[inline(always)]
    pub fn pa7_os(&self) -> Pa7OsR {
        Pa7OsR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - PAD output speed configuration"]
    #[inline(always)]
    pub fn pa8_os(&self) -> Pa8OsR {
        Pa8OsR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - PAD output speed configuration"]
    #[inline(always)]
    pub fn pa9_os(&self) -> Pa9OsR {
        Pa9OsR::new(((self.bits >> 18) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - PAD output speed configuration"]
    #[inline(always)]
    pub fn pa0_os(&mut self) -> Pa0OsW<'_, Ospeedr1Spec> {
        Pa0OsW::new(self, 0)
    }
    #[doc = "Bits 2:3 - PAD output speed configuration"]
    #[inline(always)]
    pub fn pa1_os(&mut self) -> Pa1OsW<'_, Ospeedr1Spec> {
        Pa1OsW::new(self, 2)
    }
    #[doc = "Bits 4:5 - PAD output speed configuration"]
    #[inline(always)]
    pub fn pa2_os(&mut self) -> Pa2OsW<'_, Ospeedr1Spec> {
        Pa2OsW::new(self, 4)
    }
    #[doc = "Bits 6:7 - PAD output speed configuration"]
    #[inline(always)]
    pub fn pa3_os(&mut self) -> Pa3OsW<'_, Ospeedr1Spec> {
        Pa3OsW::new(self, 6)
    }
    #[doc = "Bits 8:9 - PAD output speed configuration"]
    #[inline(always)]
    pub fn pa4_os(&mut self) -> Pa4OsW<'_, Ospeedr1Spec> {
        Pa4OsW::new(self, 8)
    }
    #[doc = "Bits 10:11 - PAD output speed configuration"]
    #[inline(always)]
    pub fn pa5_os(&mut self) -> Pa5OsW<'_, Ospeedr1Spec> {
        Pa5OsW::new(self, 10)
    }
    #[doc = "Bits 12:13 - PAD output speed configuration"]
    #[inline(always)]
    pub fn pa6_os(&mut self) -> Pa6OsW<'_, Ospeedr1Spec> {
        Pa6OsW::new(self, 12)
    }
    #[doc = "Bits 14:15 - PAD output speed configuration"]
    #[inline(always)]
    pub fn pa7_os(&mut self) -> Pa7OsW<'_, Ospeedr1Spec> {
        Pa7OsW::new(self, 14)
    }
    #[doc = "Bits 16:17 - PAD output speed configuration"]
    #[inline(always)]
    pub fn pa8_os(&mut self) -> Pa8OsW<'_, Ospeedr1Spec> {
        Pa8OsW::new(self, 16)
    }
    #[doc = "Bits 18:19 - PAD output speed configuration"]
    #[inline(always)]
    pub fn pa9_os(&mut self) -> Pa9OsW<'_, Ospeedr1Spec> {
        Pa9OsW::new(self, 18)
    }
}
#[doc = "GPIO output speed register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`ospeedr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ospeedr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ospeedr1Spec;
impl crate::RegisterSpec for Ospeedr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ospeedr1::R`](R) reader structure"]
impl crate::Readable for Ospeedr1Spec {}
#[doc = "`write(|w| ..)` method takes [`ospeedr1::W`](W) writer structure"]
impl crate::Writable for Ospeedr1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OSPEEDR1 to value 0x0aaa"]
impl crate::Resettable for Ospeedr1Spec {
    const RESET_VALUE: u32 = 0x0aaa;
}
