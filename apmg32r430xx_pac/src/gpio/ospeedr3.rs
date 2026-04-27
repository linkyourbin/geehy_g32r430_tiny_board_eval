#[doc = "Register `OSPEEDR3` reader"]
pub type R = crate::R<Ospeedr3Spec>;
#[doc = "Register `OSPEEDR3` writer"]
pub type W = crate::W<Ospeedr3Spec>;
#[doc = "Field `PC0_OS` reader - PAD output speed configuration"]
pub type Pc0OsR = crate::FieldReader;
#[doc = "Field `PC0_OS` writer - PAD output speed configuration"]
pub type Pc0OsW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PC1_OS` reader - PAD output speed configuration"]
pub type Pc1OsR = crate::FieldReader;
#[doc = "Field `PC1_OS` writer - PAD output speed configuration"]
pub type Pc1OsW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PC2_OS` reader - PAD output speed configuration"]
pub type Pc2OsR = crate::FieldReader;
#[doc = "Field `PC2_OS` writer - PAD output speed configuration"]
pub type Pc2OsW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PC3_OS` reader - PAD output speed configuration"]
pub type Pc3OsR = crate::FieldReader;
#[doc = "Field `PC3_OS` writer - PAD output speed configuration"]
pub type Pc3OsW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PC4_OS` reader - PAD output speed configuration"]
pub type Pc4OsR = crate::FieldReader;
#[doc = "Field `PC4_OS` writer - PAD output speed configuration"]
pub type Pc4OsW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PC5_OS` reader - PAD output speed configuration"]
pub type Pc5OsR = crate::FieldReader;
#[doc = "Field `PC5_OS` writer - PAD output speed configuration"]
pub type Pc5OsW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PC6_OS` reader - PAD output speed configuration"]
pub type Pc6OsR = crate::FieldReader;
#[doc = "Field `PC6_OS` writer - PAD output speed configuration"]
pub type Pc6OsW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PC7_OS` reader - PAD output speed configuration"]
pub type Pc7OsR = crate::FieldReader;
#[doc = "Field `PC7_OS` writer - PAD output speed configuration"]
pub type Pc7OsW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PC8_OS` reader - PAD output speed configuration"]
pub type Pc8OsR = crate::FieldReader;
#[doc = "Field `PC8_OS` writer - PAD output speed configuration"]
pub type Pc8OsW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PC9_OS` reader - PAD output speed configuration"]
pub type Pc9OsR = crate::FieldReader;
#[doc = "Field `PC9_OS` writer - PAD output speed configuration"]
pub type Pc9OsW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PC10_OS` reader - PAD output speed configuration"]
pub type Pc10OsR = crate::FieldReader;
#[doc = "Field `PC10_OS` writer - PAD output speed configuration"]
pub type Pc10OsW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PC11_OS` reader - PAD output speed configuration"]
pub type Pc11OsR = crate::FieldReader;
#[doc = "Field `PC11_OS` writer - PAD output speed configuration"]
pub type Pc11OsW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PC12_OS` reader - PAD output speed configuration"]
pub type Pc12OsR = crate::FieldReader;
#[doc = "Field `PC12_OS` writer - PAD output speed configuration"]
pub type Pc12OsW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - PAD output speed configuration"]
    #[inline(always)]
    pub fn pc0_os(&self) -> Pc0OsR {
        Pc0OsR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - PAD output speed configuration"]
    #[inline(always)]
    pub fn pc1_os(&self) -> Pc1OsR {
        Pc1OsR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - PAD output speed configuration"]
    #[inline(always)]
    pub fn pc2_os(&self) -> Pc2OsR {
        Pc2OsR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - PAD output speed configuration"]
    #[inline(always)]
    pub fn pc3_os(&self) -> Pc3OsR {
        Pc3OsR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - PAD output speed configuration"]
    #[inline(always)]
    pub fn pc4_os(&self) -> Pc4OsR {
        Pc4OsR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - PAD output speed configuration"]
    #[inline(always)]
    pub fn pc5_os(&self) -> Pc5OsR {
        Pc5OsR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - PAD output speed configuration"]
    #[inline(always)]
    pub fn pc6_os(&self) -> Pc6OsR {
        Pc6OsR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - PAD output speed configuration"]
    #[inline(always)]
    pub fn pc7_os(&self) -> Pc7OsR {
        Pc7OsR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - PAD output speed configuration"]
    #[inline(always)]
    pub fn pc8_os(&self) -> Pc8OsR {
        Pc8OsR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - PAD output speed configuration"]
    #[inline(always)]
    pub fn pc9_os(&self) -> Pc9OsR {
        Pc9OsR::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - PAD output speed configuration"]
    #[inline(always)]
    pub fn pc10_os(&self) -> Pc10OsR {
        Pc10OsR::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - PAD output speed configuration"]
    #[inline(always)]
    pub fn pc11_os(&self) -> Pc11OsR {
        Pc11OsR::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - PAD output speed configuration"]
    #[inline(always)]
    pub fn pc12_os(&self) -> Pc12OsR {
        Pc12OsR::new(((self.bits >> 24) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - PAD output speed configuration"]
    #[inline(always)]
    pub fn pc0_os(&mut self) -> Pc0OsW<'_, Ospeedr3Spec> {
        Pc0OsW::new(self, 0)
    }
    #[doc = "Bits 2:3 - PAD output speed configuration"]
    #[inline(always)]
    pub fn pc1_os(&mut self) -> Pc1OsW<'_, Ospeedr3Spec> {
        Pc1OsW::new(self, 2)
    }
    #[doc = "Bits 4:5 - PAD output speed configuration"]
    #[inline(always)]
    pub fn pc2_os(&mut self) -> Pc2OsW<'_, Ospeedr3Spec> {
        Pc2OsW::new(self, 4)
    }
    #[doc = "Bits 6:7 - PAD output speed configuration"]
    #[inline(always)]
    pub fn pc3_os(&mut self) -> Pc3OsW<'_, Ospeedr3Spec> {
        Pc3OsW::new(self, 6)
    }
    #[doc = "Bits 8:9 - PAD output speed configuration"]
    #[inline(always)]
    pub fn pc4_os(&mut self) -> Pc4OsW<'_, Ospeedr3Spec> {
        Pc4OsW::new(self, 8)
    }
    #[doc = "Bits 10:11 - PAD output speed configuration"]
    #[inline(always)]
    pub fn pc5_os(&mut self) -> Pc5OsW<'_, Ospeedr3Spec> {
        Pc5OsW::new(self, 10)
    }
    #[doc = "Bits 12:13 - PAD output speed configuration"]
    #[inline(always)]
    pub fn pc6_os(&mut self) -> Pc6OsW<'_, Ospeedr3Spec> {
        Pc6OsW::new(self, 12)
    }
    #[doc = "Bits 14:15 - PAD output speed configuration"]
    #[inline(always)]
    pub fn pc7_os(&mut self) -> Pc7OsW<'_, Ospeedr3Spec> {
        Pc7OsW::new(self, 14)
    }
    #[doc = "Bits 16:17 - PAD output speed configuration"]
    #[inline(always)]
    pub fn pc8_os(&mut self) -> Pc8OsW<'_, Ospeedr3Spec> {
        Pc8OsW::new(self, 16)
    }
    #[doc = "Bits 18:19 - PAD output speed configuration"]
    #[inline(always)]
    pub fn pc9_os(&mut self) -> Pc9OsW<'_, Ospeedr3Spec> {
        Pc9OsW::new(self, 18)
    }
    #[doc = "Bits 20:21 - PAD output speed configuration"]
    #[inline(always)]
    pub fn pc10_os(&mut self) -> Pc10OsW<'_, Ospeedr3Spec> {
        Pc10OsW::new(self, 20)
    }
    #[doc = "Bits 22:23 - PAD output speed configuration"]
    #[inline(always)]
    pub fn pc11_os(&mut self) -> Pc11OsW<'_, Ospeedr3Spec> {
        Pc11OsW::new(self, 22)
    }
    #[doc = "Bits 24:25 - PAD output speed configuration"]
    #[inline(always)]
    pub fn pc12_os(&mut self) -> Pc12OsW<'_, Ospeedr3Spec> {
        Pc12OsW::new(self, 24)
    }
}
#[doc = "GPIO output speed register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`ospeedr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ospeedr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ospeedr3Spec;
impl crate::RegisterSpec for Ospeedr3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ospeedr3::R`](R) reader structure"]
impl crate::Readable for Ospeedr3Spec {}
#[doc = "`write(|w| ..)` method takes [`ospeedr3::W`](W) writer structure"]
impl crate::Writable for Ospeedr3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OSPEEDR3 to value 0x02aa_aaaa"]
impl crate::Resettable for Ospeedr3Spec {
    const RESET_VALUE: u32 = 0x02aa_aaaa;
}
