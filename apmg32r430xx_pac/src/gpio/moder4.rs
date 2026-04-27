#[doc = "Register `MODER4` reader"]
pub type R = crate::R<Moder4Spec>;
#[doc = "Register `MODER4` writer"]
pub type W = crate::W<Moder4Spec>;
#[doc = "Field `PD0_MD` reader - Work Mode Configuration"]
pub type Pd0MdR = crate::FieldReader;
#[doc = "Field `PD0_MD` writer - Work Mode Configuration"]
pub type Pd0MdW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PD1_MD` reader - Work Mode Configuration"]
pub type Pd1MdR = crate::FieldReader;
#[doc = "Field `PD1_MD` writer - Work Mode Configuration"]
pub type Pd1MdW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PD2_MD` reader - Work Mode Configuration"]
pub type Pd2MdR = crate::FieldReader;
#[doc = "Field `PD2_MD` writer - Work Mode Configuration"]
pub type Pd2MdW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PD3_MD` reader - Work Mode Configuration"]
pub type Pd3MdR = crate::FieldReader;
#[doc = "Field `PD3_MD` writer - Work Mode Configuration"]
pub type Pd3MdW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PD4_MD` reader - Work Mode Configuration"]
pub type Pd4MdR = crate::FieldReader;
#[doc = "Field `PD4_MD` writer - Work Mode Configuration"]
pub type Pd4MdW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PD5_MD` reader - Work Mode Configuration"]
pub type Pd5MdR = crate::FieldReader;
#[doc = "Field `PD5_MD` writer - Work Mode Configuration"]
pub type Pd5MdW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PD6_MD` reader - Work Mode Configuration"]
pub type Pd6MdR = crate::FieldReader;
#[doc = "Field `PD6_MD` writer - Work Mode Configuration"]
pub type Pd6MdW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PD7_MD` reader - Work Mode Configuration"]
pub type Pd7MdR = crate::FieldReader;
#[doc = "Field `PD7_MD` writer - Work Mode Configuration"]
pub type Pd7MdW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PD8_MD` reader - Work Mode Configuration"]
pub type Pd8MdR = crate::FieldReader;
#[doc = "Field `PD8_MD` writer - Work Mode Configuration"]
pub type Pd8MdW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PD9_MD` reader - Work Mode Configuration"]
pub type Pd9MdR = crate::FieldReader;
#[doc = "Field `PD9_MD` writer - Work Mode Configuration"]
pub type Pd9MdW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PD10_MD` reader - Work Mode Configuration"]
pub type Pd10MdR = crate::FieldReader;
#[doc = "Field `PD10_MD` writer - Work Mode Configuration"]
pub type Pd10MdW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PD11_MD` reader - Work Mode Configuration"]
pub type Pd11MdR = crate::FieldReader;
#[doc = "Field `PD11_MD` writer - Work Mode Configuration"]
pub type Pd11MdW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PD12_MD` reader - Work Mode Configuration"]
pub type Pd12MdR = crate::FieldReader;
#[doc = "Field `PD12_MD` writer - Work Mode Configuration"]
pub type Pd12MdW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PD13_MD` reader - Work Mode Configuration"]
pub type Pd13MdR = crate::FieldReader;
#[doc = "Field `PD13_MD` writer - Work Mode Configuration"]
pub type Pd13MdW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PD14_MD` reader - Work Mode Configuration"]
pub type Pd14MdR = crate::FieldReader;
#[doc = "Field `PD14_MD` writer - Work Mode Configuration"]
pub type Pd14MdW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PD15_MD` reader - Work Mode Configuration"]
pub type Pd15MdR = crate::FieldReader;
#[doc = "Field `PD15_MD` writer - Work Mode Configuration"]
pub type Pd15MdW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - Work Mode Configuration"]
    #[inline(always)]
    pub fn pd0_md(&self) -> Pd0MdR {
        Pd0MdR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Work Mode Configuration"]
    #[inline(always)]
    pub fn pd1_md(&self) -> Pd1MdR {
        Pd1MdR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Work Mode Configuration"]
    #[inline(always)]
    pub fn pd2_md(&self) -> Pd2MdR {
        Pd2MdR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Work Mode Configuration"]
    #[inline(always)]
    pub fn pd3_md(&self) -> Pd3MdR {
        Pd3MdR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Work Mode Configuration"]
    #[inline(always)]
    pub fn pd4_md(&self) -> Pd4MdR {
        Pd4MdR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Work Mode Configuration"]
    #[inline(always)]
    pub fn pd5_md(&self) -> Pd5MdR {
        Pd5MdR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Work Mode Configuration"]
    #[inline(always)]
    pub fn pd6_md(&self) -> Pd6MdR {
        Pd6MdR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Work Mode Configuration"]
    #[inline(always)]
    pub fn pd7_md(&self) -> Pd7MdR {
        Pd7MdR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Work Mode Configuration"]
    #[inline(always)]
    pub fn pd8_md(&self) -> Pd8MdR {
        Pd8MdR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - Work Mode Configuration"]
    #[inline(always)]
    pub fn pd9_md(&self) -> Pd9MdR {
        Pd9MdR::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - Work Mode Configuration"]
    #[inline(always)]
    pub fn pd10_md(&self) -> Pd10MdR {
        Pd10MdR::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - Work Mode Configuration"]
    #[inline(always)]
    pub fn pd11_md(&self) -> Pd11MdR {
        Pd11MdR::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - Work Mode Configuration"]
    #[inline(always)]
    pub fn pd12_md(&self) -> Pd12MdR {
        Pd12MdR::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - Work Mode Configuration"]
    #[inline(always)]
    pub fn pd13_md(&self) -> Pd13MdR {
        Pd13MdR::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - Work Mode Configuration"]
    #[inline(always)]
    pub fn pd14_md(&self) -> Pd14MdR {
        Pd14MdR::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - Work Mode Configuration"]
    #[inline(always)]
    pub fn pd15_md(&self) -> Pd15MdR {
        Pd15MdR::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Work Mode Configuration"]
    #[inline(always)]
    pub fn pd0_md(&mut self) -> Pd0MdW<'_, Moder4Spec> {
        Pd0MdW::new(self, 0)
    }
    #[doc = "Bits 2:3 - Work Mode Configuration"]
    #[inline(always)]
    pub fn pd1_md(&mut self) -> Pd1MdW<'_, Moder4Spec> {
        Pd1MdW::new(self, 2)
    }
    #[doc = "Bits 4:5 - Work Mode Configuration"]
    #[inline(always)]
    pub fn pd2_md(&mut self) -> Pd2MdW<'_, Moder4Spec> {
        Pd2MdW::new(self, 4)
    }
    #[doc = "Bits 6:7 - Work Mode Configuration"]
    #[inline(always)]
    pub fn pd3_md(&mut self) -> Pd3MdW<'_, Moder4Spec> {
        Pd3MdW::new(self, 6)
    }
    #[doc = "Bits 8:9 - Work Mode Configuration"]
    #[inline(always)]
    pub fn pd4_md(&mut self) -> Pd4MdW<'_, Moder4Spec> {
        Pd4MdW::new(self, 8)
    }
    #[doc = "Bits 10:11 - Work Mode Configuration"]
    #[inline(always)]
    pub fn pd5_md(&mut self) -> Pd5MdW<'_, Moder4Spec> {
        Pd5MdW::new(self, 10)
    }
    #[doc = "Bits 12:13 - Work Mode Configuration"]
    #[inline(always)]
    pub fn pd6_md(&mut self) -> Pd6MdW<'_, Moder4Spec> {
        Pd6MdW::new(self, 12)
    }
    #[doc = "Bits 14:15 - Work Mode Configuration"]
    #[inline(always)]
    pub fn pd7_md(&mut self) -> Pd7MdW<'_, Moder4Spec> {
        Pd7MdW::new(self, 14)
    }
    #[doc = "Bits 16:17 - Work Mode Configuration"]
    #[inline(always)]
    pub fn pd8_md(&mut self) -> Pd8MdW<'_, Moder4Spec> {
        Pd8MdW::new(self, 16)
    }
    #[doc = "Bits 18:19 - Work Mode Configuration"]
    #[inline(always)]
    pub fn pd9_md(&mut self) -> Pd9MdW<'_, Moder4Spec> {
        Pd9MdW::new(self, 18)
    }
    #[doc = "Bits 20:21 - Work Mode Configuration"]
    #[inline(always)]
    pub fn pd10_md(&mut self) -> Pd10MdW<'_, Moder4Spec> {
        Pd10MdW::new(self, 20)
    }
    #[doc = "Bits 22:23 - Work Mode Configuration"]
    #[inline(always)]
    pub fn pd11_md(&mut self) -> Pd11MdW<'_, Moder4Spec> {
        Pd11MdW::new(self, 22)
    }
    #[doc = "Bits 24:25 - Work Mode Configuration"]
    #[inline(always)]
    pub fn pd12_md(&mut self) -> Pd12MdW<'_, Moder4Spec> {
        Pd12MdW::new(self, 24)
    }
    #[doc = "Bits 26:27 - Work Mode Configuration"]
    #[inline(always)]
    pub fn pd13_md(&mut self) -> Pd13MdW<'_, Moder4Spec> {
        Pd13MdW::new(self, 26)
    }
    #[doc = "Bits 28:29 - Work Mode Configuration"]
    #[inline(always)]
    pub fn pd14_md(&mut self) -> Pd14MdW<'_, Moder4Spec> {
        Pd14MdW::new(self, 28)
    }
    #[doc = "Bits 30:31 - Work Mode Configuration"]
    #[inline(always)]
    pub fn pd15_md(&mut self) -> Pd15MdW<'_, Moder4Spec> {
        Pd15MdW::new(self, 30)
    }
}
#[doc = "GPIO mode register 4\n\nYou can [`read`](crate::Reg::read) this register and get [`moder4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`moder4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Moder4Spec;
impl crate::RegisterSpec for Moder4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`moder4::R`](R) reader structure"]
impl crate::Readable for Moder4Spec {}
#[doc = "`write(|w| ..)` method takes [`moder4::W`](W) writer structure"]
impl crate::Writable for Moder4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MODER4 to value 0"]
impl crate::Resettable for Moder4Spec {}
