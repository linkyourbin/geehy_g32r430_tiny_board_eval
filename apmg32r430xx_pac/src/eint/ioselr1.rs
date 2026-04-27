#[doc = "Register `IOSELR1` reader"]
pub type R = crate::R<Ioselr1Spec>;
#[doc = "Register `IOSELR1` writer"]
pub type W = crate::W<Ioselr1Spec>;
#[doc = "Field `EXTI0` reader - EXTI0"]
pub type Exti0R = crate::FieldReader;
#[doc = "Field `EXTI0` writer - EXTI0"]
pub type Exti0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `EXTI1` reader - EXTI1"]
pub type Exti1R = crate::FieldReader;
#[doc = "Field `EXTI1` writer - EXTI1"]
pub type Exti1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `EXTI2` reader - EXTI2"]
pub type Exti2R = crate::FieldReader;
#[doc = "Field `EXTI2` writer - EXTI2"]
pub type Exti2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `EXTI3` reader - EXTI3"]
pub type Exti3R = crate::FieldReader;
#[doc = "Field `EXTI3` writer - EXTI3"]
pub type Exti3W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `EXTI4` reader - EXTI4"]
pub type Exti4R = crate::FieldReader;
#[doc = "Field `EXTI4` writer - EXTI4"]
pub type Exti4W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `EXTI5` reader - EXTI5"]
pub type Exti5R = crate::FieldReader;
#[doc = "Field `EXTI5` writer - EXTI5"]
pub type Exti5W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `EXTI6` reader - EXTI6"]
pub type Exti6R = crate::FieldReader;
#[doc = "Field `EXTI6` writer - EXTI6"]
pub type Exti6W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `EXTI7` reader - EXTI7"]
pub type Exti7R = crate::FieldReader;
#[doc = "Field `EXTI7` writer - EXTI7"]
pub type Exti7W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - EXTI0"]
    #[inline(always)]
    pub fn exti0(&self) -> Exti0R {
        Exti0R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - EXTI1"]
    #[inline(always)]
    pub fn exti1(&self) -> Exti1R {
        Exti1R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - EXTI2"]
    #[inline(always)]
    pub fn exti2(&self) -> Exti2R {
        Exti2R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - EXTI3"]
    #[inline(always)]
    pub fn exti3(&self) -> Exti3R {
        Exti3R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - EXTI4"]
    #[inline(always)]
    pub fn exti4(&self) -> Exti4R {
        Exti4R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - EXTI5"]
    #[inline(always)]
    pub fn exti5(&self) -> Exti5R {
        Exti5R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - EXTI6"]
    #[inline(always)]
    pub fn exti6(&self) -> Exti6R {
        Exti6R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - EXTI7"]
    #[inline(always)]
    pub fn exti7(&self) -> Exti7R {
        Exti7R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - EXTI0"]
    #[inline(always)]
    pub fn exti0(&mut self) -> Exti0W<'_, Ioselr1Spec> {
        Exti0W::new(self, 0)
    }
    #[doc = "Bits 4:7 - EXTI1"]
    #[inline(always)]
    pub fn exti1(&mut self) -> Exti1W<'_, Ioselr1Spec> {
        Exti1W::new(self, 4)
    }
    #[doc = "Bits 8:11 - EXTI2"]
    #[inline(always)]
    pub fn exti2(&mut self) -> Exti2W<'_, Ioselr1Spec> {
        Exti2W::new(self, 8)
    }
    #[doc = "Bits 12:15 - EXTI3"]
    #[inline(always)]
    pub fn exti3(&mut self) -> Exti3W<'_, Ioselr1Spec> {
        Exti3W::new(self, 12)
    }
    #[doc = "Bits 16:19 - EXTI4"]
    #[inline(always)]
    pub fn exti4(&mut self) -> Exti4W<'_, Ioselr1Spec> {
        Exti4W::new(self, 16)
    }
    #[doc = "Bits 20:23 - EXTI5"]
    #[inline(always)]
    pub fn exti5(&mut self) -> Exti5W<'_, Ioselr1Spec> {
        Exti5W::new(self, 20)
    }
    #[doc = "Bits 24:27 - EXTI6"]
    #[inline(always)]
    pub fn exti6(&mut self) -> Exti6W<'_, Ioselr1Spec> {
        Exti6W::new(self, 24)
    }
    #[doc = "Bits 28:31 - EXTI7"]
    #[inline(always)]
    pub fn exti7(&mut self) -> Exti7W<'_, Ioselr1Spec> {
        Exti7W::new(self, 28)
    }
}
#[doc = "Configure Register 1.\n\nYou can [`read`](crate::Reg::read) this register and get [`ioselr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ioselr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ioselr1Spec;
impl crate::RegisterSpec for Ioselr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ioselr1::R`](R) reader structure"]
impl crate::Readable for Ioselr1Spec {}
#[doc = "`write(|w| ..)` method takes [`ioselr1::W`](W) writer structure"]
impl crate::Writable for Ioselr1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IOSELR1 to value 0"]
impl crate::Resettable for Ioselr1Spec {}
