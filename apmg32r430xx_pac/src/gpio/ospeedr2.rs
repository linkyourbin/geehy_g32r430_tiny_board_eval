#[doc = "Register `OSPEEDR2` reader"]
pub type R = crate::R<Ospeedr2Spec>;
#[doc = "Register `OSPEEDR2` writer"]
pub type W = crate::W<Ospeedr2Spec>;
#[doc = "Field `PB0_OS` reader - PAD output speed configuration"]
pub type Pb0OsR = crate::FieldReader;
#[doc = "Field `PB0_OS` writer - PAD output speed configuration"]
pub type Pb0OsW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PB1_OS` reader - PAD output speed configuration"]
pub type Pb1OsR = crate::FieldReader;
#[doc = "Field `PB1_OS` writer - PAD output speed configuration"]
pub type Pb1OsW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PB2_OS` reader - PAD output speed configuration"]
pub type Pb2OsR = crate::FieldReader;
#[doc = "Field `PB2_OS` writer - PAD output speed configuration"]
pub type Pb2OsW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PB3_OS` reader - PAD output speed configuration"]
pub type Pb3OsR = crate::FieldReader;
#[doc = "Field `PB3_OS` writer - PAD output speed configuration"]
pub type Pb3OsW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PB4_OS` reader - PAD output speed configuration"]
pub type Pb4OsR = crate::FieldReader;
#[doc = "Field `PB4_OS` writer - PAD output speed configuration"]
pub type Pb4OsW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PB5_OS` reader - PAD output speed configuration"]
pub type Pb5OsR = crate::FieldReader;
#[doc = "Field `PB5_OS` writer - PAD output speed configuration"]
pub type Pb5OsW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PB6_OS` reader - PAD output speed configuration"]
pub type Pb6OsR = crate::FieldReader;
#[doc = "Field `PB6_OS` writer - PAD output speed configuration"]
pub type Pb6OsW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PB7_OS` reader - PAD output speed configuration"]
pub type Pb7OsR = crate::FieldReader;
#[doc = "Field `PB7_OS` writer - PAD output speed configuration"]
pub type Pb7OsW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PB8_OS` reader - PAD output speed configuration"]
pub type Pb8OsR = crate::FieldReader;
#[doc = "Field `PB8_OS` writer - PAD output speed configuration"]
pub type Pb8OsW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PB9_OS` reader - PAD output speed configuration"]
pub type Pb9OsR = crate::FieldReader;
#[doc = "Field `PB9_OS` writer - PAD output speed configuration"]
pub type Pb9OsW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PB10_OS` reader - PAD output speed configuration"]
pub type Pb10OsR = crate::FieldReader;
#[doc = "Field `PB10_OS` writer - PAD output speed configuration"]
pub type Pb10OsW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PB11_OS` reader - PAD output speed configuration"]
pub type Pb11OsR = crate::FieldReader;
#[doc = "Field `PB11_OS` writer - PAD output speed configuration"]
pub type Pb11OsW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PB12_OS` reader - PAD output speed configuration"]
pub type Pb12OsR = crate::FieldReader;
#[doc = "Field `PB12_OS` writer - PAD output speed configuration"]
pub type Pb12OsW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PB13_OS` reader - PAD output speed configuration"]
pub type Pb13OsR = crate::FieldReader;
#[doc = "Field `PB13_OS` writer - PAD output speed configuration"]
pub type Pb13OsW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - PAD output speed configuration"]
    #[inline(always)]
    pub fn pb0_os(&self) -> Pb0OsR {
        Pb0OsR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - PAD output speed configuration"]
    #[inline(always)]
    pub fn pb1_os(&self) -> Pb1OsR {
        Pb1OsR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - PAD output speed configuration"]
    #[inline(always)]
    pub fn pb2_os(&self) -> Pb2OsR {
        Pb2OsR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - PAD output speed configuration"]
    #[inline(always)]
    pub fn pb3_os(&self) -> Pb3OsR {
        Pb3OsR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - PAD output speed configuration"]
    #[inline(always)]
    pub fn pb4_os(&self) -> Pb4OsR {
        Pb4OsR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - PAD output speed configuration"]
    #[inline(always)]
    pub fn pb5_os(&self) -> Pb5OsR {
        Pb5OsR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - PAD output speed configuration"]
    #[inline(always)]
    pub fn pb6_os(&self) -> Pb6OsR {
        Pb6OsR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - PAD output speed configuration"]
    #[inline(always)]
    pub fn pb7_os(&self) -> Pb7OsR {
        Pb7OsR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - PAD output speed configuration"]
    #[inline(always)]
    pub fn pb8_os(&self) -> Pb8OsR {
        Pb8OsR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - PAD output speed configuration"]
    #[inline(always)]
    pub fn pb9_os(&self) -> Pb9OsR {
        Pb9OsR::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - PAD output speed configuration"]
    #[inline(always)]
    pub fn pb10_os(&self) -> Pb10OsR {
        Pb10OsR::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - PAD output speed configuration"]
    #[inline(always)]
    pub fn pb11_os(&self) -> Pb11OsR {
        Pb11OsR::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - PAD output speed configuration"]
    #[inline(always)]
    pub fn pb12_os(&self) -> Pb12OsR {
        Pb12OsR::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - PAD output speed configuration"]
    #[inline(always)]
    pub fn pb13_os(&self) -> Pb13OsR {
        Pb13OsR::new(((self.bits >> 26) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - PAD output speed configuration"]
    #[inline(always)]
    pub fn pb0_os(&mut self) -> Pb0OsW<'_, Ospeedr2Spec> {
        Pb0OsW::new(self, 0)
    }
    #[doc = "Bits 2:3 - PAD output speed configuration"]
    #[inline(always)]
    pub fn pb1_os(&mut self) -> Pb1OsW<'_, Ospeedr2Spec> {
        Pb1OsW::new(self, 2)
    }
    #[doc = "Bits 4:5 - PAD output speed configuration"]
    #[inline(always)]
    pub fn pb2_os(&mut self) -> Pb2OsW<'_, Ospeedr2Spec> {
        Pb2OsW::new(self, 4)
    }
    #[doc = "Bits 6:7 - PAD output speed configuration"]
    #[inline(always)]
    pub fn pb3_os(&mut self) -> Pb3OsW<'_, Ospeedr2Spec> {
        Pb3OsW::new(self, 6)
    }
    #[doc = "Bits 8:9 - PAD output speed configuration"]
    #[inline(always)]
    pub fn pb4_os(&mut self) -> Pb4OsW<'_, Ospeedr2Spec> {
        Pb4OsW::new(self, 8)
    }
    #[doc = "Bits 10:11 - PAD output speed configuration"]
    #[inline(always)]
    pub fn pb5_os(&mut self) -> Pb5OsW<'_, Ospeedr2Spec> {
        Pb5OsW::new(self, 10)
    }
    #[doc = "Bits 12:13 - PAD output speed configuration"]
    #[inline(always)]
    pub fn pb6_os(&mut self) -> Pb6OsW<'_, Ospeedr2Spec> {
        Pb6OsW::new(self, 12)
    }
    #[doc = "Bits 14:15 - PAD output speed configuration"]
    #[inline(always)]
    pub fn pb7_os(&mut self) -> Pb7OsW<'_, Ospeedr2Spec> {
        Pb7OsW::new(self, 14)
    }
    #[doc = "Bits 16:17 - PAD output speed configuration"]
    #[inline(always)]
    pub fn pb8_os(&mut self) -> Pb8OsW<'_, Ospeedr2Spec> {
        Pb8OsW::new(self, 16)
    }
    #[doc = "Bits 18:19 - PAD output speed configuration"]
    #[inline(always)]
    pub fn pb9_os(&mut self) -> Pb9OsW<'_, Ospeedr2Spec> {
        Pb9OsW::new(self, 18)
    }
    #[doc = "Bits 20:21 - PAD output speed configuration"]
    #[inline(always)]
    pub fn pb10_os(&mut self) -> Pb10OsW<'_, Ospeedr2Spec> {
        Pb10OsW::new(self, 20)
    }
    #[doc = "Bits 22:23 - PAD output speed configuration"]
    #[inline(always)]
    pub fn pb11_os(&mut self) -> Pb11OsW<'_, Ospeedr2Spec> {
        Pb11OsW::new(self, 22)
    }
    #[doc = "Bits 24:25 - PAD output speed configuration"]
    #[inline(always)]
    pub fn pb12_os(&mut self) -> Pb12OsW<'_, Ospeedr2Spec> {
        Pb12OsW::new(self, 24)
    }
    #[doc = "Bits 26:27 - PAD output speed configuration"]
    #[inline(always)]
    pub fn pb13_os(&mut self) -> Pb13OsW<'_, Ospeedr2Spec> {
        Pb13OsW::new(self, 26)
    }
}
#[doc = "GPIO output speed register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`ospeedr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ospeedr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ospeedr2Spec;
impl crate::RegisterSpec for Ospeedr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ospeedr2::R`](R) reader structure"]
impl crate::Readable for Ospeedr2Spec {}
#[doc = "`write(|w| ..)` method takes [`ospeedr2::W`](W) writer structure"]
impl crate::Writable for Ospeedr2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OSPEEDR2 to value 0x02ba_aaaa"]
impl crate::Resettable for Ospeedr2Spec {
    const RESET_VALUE: u32 = 0x02ba_aaaa;
}
