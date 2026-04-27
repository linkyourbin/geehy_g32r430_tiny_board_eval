#[doc = "Register `MODER2` reader"]
pub type R = crate::R<Moder2Spec>;
#[doc = "Register `MODER2` writer"]
pub type W = crate::W<Moder2Spec>;
#[doc = "Field `PB0_MD` reader - Work Mode Configuration"]
pub type Pb0MdR = crate::FieldReader;
#[doc = "Field `PB0_MD` writer - Work Mode Configuration"]
pub type Pb0MdW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PB1_MD` reader - Work Mode Configuration"]
pub type Pb1MdR = crate::FieldReader;
#[doc = "Field `PB1_MD` writer - Work Mode Configuration"]
pub type Pb1MdW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PB2_MD` reader - Work Mode Configuration"]
pub type Pb2MdR = crate::FieldReader;
#[doc = "Field `PB2_MD` writer - Work Mode Configuration"]
pub type Pb2MdW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PB3_MD` reader - Work Mode Configuration"]
pub type Pb3MdR = crate::FieldReader;
#[doc = "Field `PB3_MD` writer - Work Mode Configuration"]
pub type Pb3MdW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PB4_MD` reader - Work Mode Configuration"]
pub type Pb4MdR = crate::FieldReader;
#[doc = "Field `PB4_MD` writer - Work Mode Configuration"]
pub type Pb4MdW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PB5_MD` reader - Work Mode Configuration"]
pub type Pb5MdR = crate::FieldReader;
#[doc = "Field `PB5_MD` writer - Work Mode Configuration"]
pub type Pb5MdW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PB6_MD` reader - Work Mode Configuration"]
pub type Pb6MdR = crate::FieldReader;
#[doc = "Field `PB6_MD` writer - Work Mode Configuration"]
pub type Pb6MdW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PB7_MD` reader - Work Mode Configuration"]
pub type Pb7MdR = crate::FieldReader;
#[doc = "Field `PB7_MD` writer - Work Mode Configuration"]
pub type Pb7MdW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PB8_MD` reader - Work Mode Configuration"]
pub type Pb8MdR = crate::FieldReader;
#[doc = "Field `PB8_MD` writer - Work Mode Configuration"]
pub type Pb8MdW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PB9_MD` reader - Work Mode Configuration"]
pub type Pb9MdR = crate::FieldReader;
#[doc = "Field `PB9_MD` writer - Work Mode Configuration"]
pub type Pb9MdW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PB10_MD` reader - Work Mode Configuration"]
pub type Pb10MdR = crate::FieldReader;
#[doc = "Field `PB10_MD` writer - Work Mode Configuration"]
pub type Pb10MdW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PB11_MD` reader - Work Mode Configuration"]
pub type Pb11MdR = crate::FieldReader;
#[doc = "Field `PB11_MD` writer - Work Mode Configuration"]
pub type Pb11MdW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PB12_MD` reader - Work Mode Configuration"]
pub type Pb12MdR = crate::FieldReader;
#[doc = "Field `PB12_MD` writer - Work Mode Configuration"]
pub type Pb12MdW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PB13_MD` reader - Work Mode Configuration"]
pub type Pb13MdR = crate::FieldReader;
#[doc = "Field `PB13_MD` writer - Work Mode Configuration"]
pub type Pb13MdW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - Work Mode Configuration"]
    #[inline(always)]
    pub fn pb0_md(&self) -> Pb0MdR {
        Pb0MdR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Work Mode Configuration"]
    #[inline(always)]
    pub fn pb1_md(&self) -> Pb1MdR {
        Pb1MdR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Work Mode Configuration"]
    #[inline(always)]
    pub fn pb2_md(&self) -> Pb2MdR {
        Pb2MdR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Work Mode Configuration"]
    #[inline(always)]
    pub fn pb3_md(&self) -> Pb3MdR {
        Pb3MdR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Work Mode Configuration"]
    #[inline(always)]
    pub fn pb4_md(&self) -> Pb4MdR {
        Pb4MdR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Work Mode Configuration"]
    #[inline(always)]
    pub fn pb5_md(&self) -> Pb5MdR {
        Pb5MdR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Work Mode Configuration"]
    #[inline(always)]
    pub fn pb6_md(&self) -> Pb6MdR {
        Pb6MdR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Work Mode Configuration"]
    #[inline(always)]
    pub fn pb7_md(&self) -> Pb7MdR {
        Pb7MdR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Work Mode Configuration"]
    #[inline(always)]
    pub fn pb8_md(&self) -> Pb8MdR {
        Pb8MdR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - Work Mode Configuration"]
    #[inline(always)]
    pub fn pb9_md(&self) -> Pb9MdR {
        Pb9MdR::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - Work Mode Configuration"]
    #[inline(always)]
    pub fn pb10_md(&self) -> Pb10MdR {
        Pb10MdR::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - Work Mode Configuration"]
    #[inline(always)]
    pub fn pb11_md(&self) -> Pb11MdR {
        Pb11MdR::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - Work Mode Configuration"]
    #[inline(always)]
    pub fn pb12_md(&self) -> Pb12MdR {
        Pb12MdR::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - Work Mode Configuration"]
    #[inline(always)]
    pub fn pb13_md(&self) -> Pb13MdR {
        Pb13MdR::new(((self.bits >> 26) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Work Mode Configuration"]
    #[inline(always)]
    pub fn pb0_md(&mut self) -> Pb0MdW<'_, Moder2Spec> {
        Pb0MdW::new(self, 0)
    }
    #[doc = "Bits 2:3 - Work Mode Configuration"]
    #[inline(always)]
    pub fn pb1_md(&mut self) -> Pb1MdW<'_, Moder2Spec> {
        Pb1MdW::new(self, 2)
    }
    #[doc = "Bits 4:5 - Work Mode Configuration"]
    #[inline(always)]
    pub fn pb2_md(&mut self) -> Pb2MdW<'_, Moder2Spec> {
        Pb2MdW::new(self, 4)
    }
    #[doc = "Bits 6:7 - Work Mode Configuration"]
    #[inline(always)]
    pub fn pb3_md(&mut self) -> Pb3MdW<'_, Moder2Spec> {
        Pb3MdW::new(self, 6)
    }
    #[doc = "Bits 8:9 - Work Mode Configuration"]
    #[inline(always)]
    pub fn pb4_md(&mut self) -> Pb4MdW<'_, Moder2Spec> {
        Pb4MdW::new(self, 8)
    }
    #[doc = "Bits 10:11 - Work Mode Configuration"]
    #[inline(always)]
    pub fn pb5_md(&mut self) -> Pb5MdW<'_, Moder2Spec> {
        Pb5MdW::new(self, 10)
    }
    #[doc = "Bits 12:13 - Work Mode Configuration"]
    #[inline(always)]
    pub fn pb6_md(&mut self) -> Pb6MdW<'_, Moder2Spec> {
        Pb6MdW::new(self, 12)
    }
    #[doc = "Bits 14:15 - Work Mode Configuration"]
    #[inline(always)]
    pub fn pb7_md(&mut self) -> Pb7MdW<'_, Moder2Spec> {
        Pb7MdW::new(self, 14)
    }
    #[doc = "Bits 16:17 - Work Mode Configuration"]
    #[inline(always)]
    pub fn pb8_md(&mut self) -> Pb8MdW<'_, Moder2Spec> {
        Pb8MdW::new(self, 16)
    }
    #[doc = "Bits 18:19 - Work Mode Configuration"]
    #[inline(always)]
    pub fn pb9_md(&mut self) -> Pb9MdW<'_, Moder2Spec> {
        Pb9MdW::new(self, 18)
    }
    #[doc = "Bits 20:21 - Work Mode Configuration"]
    #[inline(always)]
    pub fn pb10_md(&mut self) -> Pb10MdW<'_, Moder2Spec> {
        Pb10MdW::new(self, 20)
    }
    #[doc = "Bits 22:23 - Work Mode Configuration"]
    #[inline(always)]
    pub fn pb11_md(&mut self) -> Pb11MdW<'_, Moder2Spec> {
        Pb11MdW::new(self, 22)
    }
    #[doc = "Bits 24:25 - Work Mode Configuration"]
    #[inline(always)]
    pub fn pb12_md(&mut self) -> Pb12MdW<'_, Moder2Spec> {
        Pb12MdW::new(self, 24)
    }
    #[doc = "Bits 26:27 - Work Mode Configuration"]
    #[inline(always)]
    pub fn pb13_md(&mut self) -> Pb13MdW<'_, Moder2Spec> {
        Pb13MdW::new(self, 26)
    }
}
#[doc = "GPIO mode register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`moder2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`moder2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Moder2Spec;
impl crate::RegisterSpec for Moder2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`moder2::R`](R) reader structure"]
impl crate::Readable for Moder2Spec {}
#[doc = "`write(|w| ..)` method takes [`moder2::W`](W) writer structure"]
impl crate::Writable for Moder2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MODER2 to value 0x00f0_0c00"]
impl crate::Resettable for Moder2Spec {
    const RESET_VALUE: u32 = 0x00f0_0c00;
}
