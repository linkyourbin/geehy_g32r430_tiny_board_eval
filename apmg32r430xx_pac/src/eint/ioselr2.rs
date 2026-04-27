#[doc = "Register `IOSELR2` reader"]
pub type R = crate::R<Ioselr2Spec>;
#[doc = "Register `IOSELR2` writer"]
pub type W = crate::W<Ioselr2Spec>;
#[doc = "Field `EXTI8` reader - EXTI8"]
pub type Exti8R = crate::FieldReader;
#[doc = "Field `EXTI8` writer - EXTI8"]
pub type Exti8W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `EXTI9` reader - EXTI9"]
pub type Exti9R = crate::FieldReader;
#[doc = "Field `EXTI9` writer - EXTI9"]
pub type Exti9W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `EXTI10` reader - EXTI10"]
pub type Exti10R = crate::FieldReader;
#[doc = "Field `EXTI10` writer - EXTI10"]
pub type Exti10W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `EXTI11` reader - EXTI11"]
pub type Exti11R = crate::FieldReader;
#[doc = "Field `EXTI11` writer - EXTI11"]
pub type Exti11W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `EXTI12` reader - EXTI12"]
pub type Exti12R = crate::FieldReader;
#[doc = "Field `EXTI12` writer - EXTI12"]
pub type Exti12W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `EXTI13` reader - EXTI13"]
pub type Exti13R = crate::FieldReader;
#[doc = "Field `EXTI13` writer - EXTI13"]
pub type Exti13W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `EXTI14` reader - EXTI14"]
pub type Exti14R = crate::FieldReader;
#[doc = "Field `EXTI14` writer - EXTI14"]
pub type Exti14W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `EXTI15` reader - EXTI15"]
pub type Exti15R = crate::FieldReader;
#[doc = "Field `EXTI15` writer - EXTI15"]
pub type Exti15W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - EXTI8"]
    #[inline(always)]
    pub fn exti8(&self) -> Exti8R {
        Exti8R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - EXTI9"]
    #[inline(always)]
    pub fn exti9(&self) -> Exti9R {
        Exti9R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - EXTI10"]
    #[inline(always)]
    pub fn exti10(&self) -> Exti10R {
        Exti10R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - EXTI11"]
    #[inline(always)]
    pub fn exti11(&self) -> Exti11R {
        Exti11R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - EXTI12"]
    #[inline(always)]
    pub fn exti12(&self) -> Exti12R {
        Exti12R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - EXTI13"]
    #[inline(always)]
    pub fn exti13(&self) -> Exti13R {
        Exti13R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - EXTI14"]
    #[inline(always)]
    pub fn exti14(&self) -> Exti14R {
        Exti14R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - EXTI15"]
    #[inline(always)]
    pub fn exti15(&self) -> Exti15R {
        Exti15R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - EXTI8"]
    #[inline(always)]
    pub fn exti8(&mut self) -> Exti8W<'_, Ioselr2Spec> {
        Exti8W::new(self, 0)
    }
    #[doc = "Bits 4:7 - EXTI9"]
    #[inline(always)]
    pub fn exti9(&mut self) -> Exti9W<'_, Ioselr2Spec> {
        Exti9W::new(self, 4)
    }
    #[doc = "Bits 8:11 - EXTI10"]
    #[inline(always)]
    pub fn exti10(&mut self) -> Exti10W<'_, Ioselr2Spec> {
        Exti10W::new(self, 8)
    }
    #[doc = "Bits 12:15 - EXTI11"]
    #[inline(always)]
    pub fn exti11(&mut self) -> Exti11W<'_, Ioselr2Spec> {
        Exti11W::new(self, 12)
    }
    #[doc = "Bits 16:19 - EXTI12"]
    #[inline(always)]
    pub fn exti12(&mut self) -> Exti12W<'_, Ioselr2Spec> {
        Exti12W::new(self, 16)
    }
    #[doc = "Bits 20:23 - EXTI13"]
    #[inline(always)]
    pub fn exti13(&mut self) -> Exti13W<'_, Ioselr2Spec> {
        Exti13W::new(self, 20)
    }
    #[doc = "Bits 24:27 - EXTI14"]
    #[inline(always)]
    pub fn exti14(&mut self) -> Exti14W<'_, Ioselr2Spec> {
        Exti14W::new(self, 24)
    }
    #[doc = "Bits 28:31 - EXTI15"]
    #[inline(always)]
    pub fn exti15(&mut self) -> Exti15W<'_, Ioselr2Spec> {
        Exti15W::new(self, 28)
    }
}
#[doc = "Configure Register 2.\n\nYou can [`read`](crate::Reg::read) this register and get [`ioselr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ioselr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ioselr2Spec;
impl crate::RegisterSpec for Ioselr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ioselr2::R`](R) reader structure"]
impl crate::Readable for Ioselr2Spec {}
#[doc = "`write(|w| ..)` method takes [`ioselr2::W`](W) writer structure"]
impl crate::Writable for Ioselr2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IOSELR2 to value 0"]
impl crate::Resettable for Ioselr2Spec {}
