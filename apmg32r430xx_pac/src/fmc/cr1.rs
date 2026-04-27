#[doc = "Register `CR1` reader"]
pub type R = crate::R<Cr1Spec>;
#[doc = "Register `CR1` writer"]
pub type W = crate::W<Cr1Spec>;
#[doc = "Field `EREQ` reader - Erase Request"]
pub type EreqR = crate::BitReader;
#[doc = "Field `EREQ` writer - Erase Request"]
pub type EreqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PREQ` reader - Program Request"]
pub type PreqR = crate::BitReader;
#[doc = "Field `PREQ` writer - Program Request"]
pub type PreqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERTYPE` reader - Flash erase type configuration"]
pub type ErtypeR = crate::FieldReader;
#[doc = "Field `ERTYPE` writer - Flash erase type configuration"]
pub type ErtypeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `EOPIE` reader - Programming or clearing end of interrupt enable."]
pub type EopieR = crate::BitReader;
#[doc = "Field `EOPIE` writer - Programming or clearing end of interrupt enable."]
pub type EopieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERRIE` reader - Error Interrupt Occurred"]
pub type ErrieR = crate::BitReader;
#[doc = "Field `ERRIE` writer - Error Interrupt Occurred"]
pub type ErrieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OPRELOAD` writer - Options Byte Forced Update"]
pub type OpreloadW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Erase Request"]
    #[inline(always)]
    pub fn ereq(&self) -> EreqR {
        EreqR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Program Request"]
    #[inline(always)]
    pub fn preq(&self) -> PreqR {
        PreqR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Flash erase type configuration"]
    #[inline(always)]
    pub fn ertype(&self) -> ErtypeR {
        ErtypeR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - Programming or clearing end of interrupt enable."]
    #[inline(always)]
    pub fn eopie(&self) -> EopieR {
        EopieR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Error Interrupt Occurred"]
    #[inline(always)]
    pub fn errie(&self) -> ErrieR {
        ErrieR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Erase Request"]
    #[inline(always)]
    pub fn ereq(&mut self) -> EreqW<'_, Cr1Spec> {
        EreqW::new(self, 0)
    }
    #[doc = "Bit 1 - Program Request"]
    #[inline(always)]
    pub fn preq(&mut self) -> PreqW<'_, Cr1Spec> {
        PreqW::new(self, 1)
    }
    #[doc = "Bits 2:3 - Flash erase type configuration"]
    #[inline(always)]
    pub fn ertype(&mut self) -> ErtypeW<'_, Cr1Spec> {
        ErtypeW::new(self, 2)
    }
    #[doc = "Bit 4 - Programming or clearing end of interrupt enable."]
    #[inline(always)]
    pub fn eopie(&mut self) -> EopieW<'_, Cr1Spec> {
        EopieW::new(self, 4)
    }
    #[doc = "Bit 5 - Error Interrupt Occurred"]
    #[inline(always)]
    pub fn errie(&mut self) -> ErrieW<'_, Cr1Spec> {
        ErrieW::new(self, 5)
    }
    #[doc = "Bit 15 - Options Byte Forced Update"]
    #[inline(always)]
    pub fn opreload(&mut self) -> OpreloadW<'_, Cr1Spec> {
        OpreloadW::new(self, 15)
    }
}
#[doc = "Control register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`cr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cr1Spec;
impl crate::RegisterSpec for Cr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr1::R`](R) reader structure"]
impl crate::Readable for Cr1Spec {}
#[doc = "`write(|w| ..)` method takes [`cr1::W`](W) writer structure"]
impl crate::Writable for Cr1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CR1 to value 0"]
impl crate::Resettable for Cr1Spec {}
