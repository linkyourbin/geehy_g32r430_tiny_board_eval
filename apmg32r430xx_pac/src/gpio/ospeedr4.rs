#[doc = "Register `OSPEEDR4` reader"]
pub type R = crate::R<Ospeedr4Spec>;
#[doc = "Register `OSPEEDR4` writer"]
pub type W = crate::W<Ospeedr4Spec>;
#[doc = "Field `PD0_OS` reader - PAD output speed configuration"]
pub type Pd0OsR = crate::FieldReader;
#[doc = "Field `PD0_OS` writer - PAD output speed configuration"]
pub type Pd0OsW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PD1_OS` reader - PAD output speed configuration"]
pub type Pd1OsR = crate::FieldReader;
#[doc = "Field `PD1_OS` writer - PAD output speed configuration"]
pub type Pd1OsW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PD2_OS` reader - PAD output speed configuration"]
pub type Pd2OsR = crate::FieldReader;
#[doc = "Field `PD2_OS` writer - PAD output speed configuration"]
pub type Pd2OsW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PD3_OS` reader - PAD output speed configuration"]
pub type Pd3OsR = crate::FieldReader;
#[doc = "Field `PD3_OS` writer - PAD output speed configuration"]
pub type Pd3OsW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PD4_OS` reader - PAD output speed configuration"]
pub type Pd4OsR = crate::FieldReader;
#[doc = "Field `PD4_OS` writer - PAD output speed configuration"]
pub type Pd4OsW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PD5_OS` reader - PAD output speed configuration"]
pub type Pd5OsR = crate::FieldReader;
#[doc = "Field `PD5_OS` writer - PAD output speed configuration"]
pub type Pd5OsW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PD6_OS` reader - PAD output speed configuration"]
pub type Pd6OsR = crate::FieldReader;
#[doc = "Field `PD6_OS` writer - PAD output speed configuration"]
pub type Pd6OsW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PD7_OS` reader - PAD output speed configuration"]
pub type Pd7OsR = crate::FieldReader;
#[doc = "Field `PD7_OS` writer - PAD output speed configuration"]
pub type Pd7OsW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PD8_OS` reader - PAD output speed configuration"]
pub type Pd8OsR = crate::FieldReader;
#[doc = "Field `PD8_OS` writer - PAD output speed configuration"]
pub type Pd8OsW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PD9_OS` reader - PAD output speed configuration"]
pub type Pd9OsR = crate::FieldReader;
#[doc = "Field `PD9_OS` writer - PAD output speed configuration"]
pub type Pd9OsW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PD10_OS` reader - PAD output speed configuration"]
pub type Pd10OsR = crate::FieldReader;
#[doc = "Field `PD10_OS` writer - PAD output speed configuration"]
pub type Pd10OsW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PD11_OS` reader - PAD output speed configuration"]
pub type Pd11OsR = crate::FieldReader;
#[doc = "Field `PD11_OS` writer - PAD output speed configuration"]
pub type Pd11OsW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PD12_OS` reader - PAD output speed configuration"]
pub type Pd12OsR = crate::FieldReader;
#[doc = "Field `PD12_OS` writer - PAD output speed configuration"]
pub type Pd12OsW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PD13_OS` reader - PAD output speed configuration"]
pub type Pd13OsR = crate::FieldReader;
#[doc = "Field `PD13_OS` writer - PAD output speed configuration"]
pub type Pd13OsW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PD14_OS` reader - PAD output speed configuration"]
pub type Pd14OsR = crate::FieldReader;
#[doc = "Field `PD14_OS` writer - PAD output speed configuration"]
pub type Pd14OsW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PD15_OS` reader - PAD output speed configuration"]
pub type Pd15OsR = crate::FieldReader;
#[doc = "Field `PD15_OS` writer - PAD output speed configuration"]
pub type Pd15OsW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - PAD output speed configuration"]
    #[inline(always)]
    pub fn pd0_os(&self) -> Pd0OsR {
        Pd0OsR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - PAD output speed configuration"]
    #[inline(always)]
    pub fn pd1_os(&self) -> Pd1OsR {
        Pd1OsR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - PAD output speed configuration"]
    #[inline(always)]
    pub fn pd2_os(&self) -> Pd2OsR {
        Pd2OsR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - PAD output speed configuration"]
    #[inline(always)]
    pub fn pd3_os(&self) -> Pd3OsR {
        Pd3OsR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - PAD output speed configuration"]
    #[inline(always)]
    pub fn pd4_os(&self) -> Pd4OsR {
        Pd4OsR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - PAD output speed configuration"]
    #[inline(always)]
    pub fn pd5_os(&self) -> Pd5OsR {
        Pd5OsR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - PAD output speed configuration"]
    #[inline(always)]
    pub fn pd6_os(&self) -> Pd6OsR {
        Pd6OsR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - PAD output speed configuration"]
    #[inline(always)]
    pub fn pd7_os(&self) -> Pd7OsR {
        Pd7OsR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - PAD output speed configuration"]
    #[inline(always)]
    pub fn pd8_os(&self) -> Pd8OsR {
        Pd8OsR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - PAD output speed configuration"]
    #[inline(always)]
    pub fn pd9_os(&self) -> Pd9OsR {
        Pd9OsR::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - PAD output speed configuration"]
    #[inline(always)]
    pub fn pd10_os(&self) -> Pd10OsR {
        Pd10OsR::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - PAD output speed configuration"]
    #[inline(always)]
    pub fn pd11_os(&self) -> Pd11OsR {
        Pd11OsR::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - PAD output speed configuration"]
    #[inline(always)]
    pub fn pd12_os(&self) -> Pd12OsR {
        Pd12OsR::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - PAD output speed configuration"]
    #[inline(always)]
    pub fn pd13_os(&self) -> Pd13OsR {
        Pd13OsR::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - PAD output speed configuration"]
    #[inline(always)]
    pub fn pd14_os(&self) -> Pd14OsR {
        Pd14OsR::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - PAD output speed configuration"]
    #[inline(always)]
    pub fn pd15_os(&self) -> Pd15OsR {
        Pd15OsR::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - PAD output speed configuration"]
    #[inline(always)]
    pub fn pd0_os(&mut self) -> Pd0OsW<'_, Ospeedr4Spec> {
        Pd0OsW::new(self, 0)
    }
    #[doc = "Bits 2:3 - PAD output speed configuration"]
    #[inline(always)]
    pub fn pd1_os(&mut self) -> Pd1OsW<'_, Ospeedr4Spec> {
        Pd1OsW::new(self, 2)
    }
    #[doc = "Bits 4:5 - PAD output speed configuration"]
    #[inline(always)]
    pub fn pd2_os(&mut self) -> Pd2OsW<'_, Ospeedr4Spec> {
        Pd2OsW::new(self, 4)
    }
    #[doc = "Bits 6:7 - PAD output speed configuration"]
    #[inline(always)]
    pub fn pd3_os(&mut self) -> Pd3OsW<'_, Ospeedr4Spec> {
        Pd3OsW::new(self, 6)
    }
    #[doc = "Bits 8:9 - PAD output speed configuration"]
    #[inline(always)]
    pub fn pd4_os(&mut self) -> Pd4OsW<'_, Ospeedr4Spec> {
        Pd4OsW::new(self, 8)
    }
    #[doc = "Bits 10:11 - PAD output speed configuration"]
    #[inline(always)]
    pub fn pd5_os(&mut self) -> Pd5OsW<'_, Ospeedr4Spec> {
        Pd5OsW::new(self, 10)
    }
    #[doc = "Bits 12:13 - PAD output speed configuration"]
    #[inline(always)]
    pub fn pd6_os(&mut self) -> Pd6OsW<'_, Ospeedr4Spec> {
        Pd6OsW::new(self, 12)
    }
    #[doc = "Bits 14:15 - PAD output speed configuration"]
    #[inline(always)]
    pub fn pd7_os(&mut self) -> Pd7OsW<'_, Ospeedr4Spec> {
        Pd7OsW::new(self, 14)
    }
    #[doc = "Bits 16:17 - PAD output speed configuration"]
    #[inline(always)]
    pub fn pd8_os(&mut self) -> Pd8OsW<'_, Ospeedr4Spec> {
        Pd8OsW::new(self, 16)
    }
    #[doc = "Bits 18:19 - PAD output speed configuration"]
    #[inline(always)]
    pub fn pd9_os(&mut self) -> Pd9OsW<'_, Ospeedr4Spec> {
        Pd9OsW::new(self, 18)
    }
    #[doc = "Bits 20:21 - PAD output speed configuration"]
    #[inline(always)]
    pub fn pd10_os(&mut self) -> Pd10OsW<'_, Ospeedr4Spec> {
        Pd10OsW::new(self, 20)
    }
    #[doc = "Bits 22:23 - PAD output speed configuration"]
    #[inline(always)]
    pub fn pd11_os(&mut self) -> Pd11OsW<'_, Ospeedr4Spec> {
        Pd11OsW::new(self, 22)
    }
    #[doc = "Bits 24:25 - PAD output speed configuration"]
    #[inline(always)]
    pub fn pd12_os(&mut self) -> Pd12OsW<'_, Ospeedr4Spec> {
        Pd12OsW::new(self, 24)
    }
    #[doc = "Bits 26:27 - PAD output speed configuration"]
    #[inline(always)]
    pub fn pd13_os(&mut self) -> Pd13OsW<'_, Ospeedr4Spec> {
        Pd13OsW::new(self, 26)
    }
    #[doc = "Bits 28:29 - PAD output speed configuration"]
    #[inline(always)]
    pub fn pd14_os(&mut self) -> Pd14OsW<'_, Ospeedr4Spec> {
        Pd14OsW::new(self, 28)
    }
    #[doc = "Bits 30:31 - PAD output speed configuration"]
    #[inline(always)]
    pub fn pd15_os(&mut self) -> Pd15OsW<'_, Ospeedr4Spec> {
        Pd15OsW::new(self, 30)
    }
}
#[doc = "GPIO output speed register 4\n\nYou can [`read`](crate::Reg::read) this register and get [`ospeedr4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ospeedr4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ospeedr4Spec;
impl crate::RegisterSpec for Ospeedr4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ospeedr4::R`](R) reader structure"]
impl crate::Readable for Ospeedr4Spec {}
#[doc = "`write(|w| ..)` method takes [`ospeedr4::W`](W) writer structure"]
impl crate::Writable for Ospeedr4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OSPEEDR4 to value 0xaaaa_aaaa"]
impl crate::Resettable for Ospeedr4Spec {
    const RESET_VALUE: u32 = 0xaaaa_aaaa;
}
