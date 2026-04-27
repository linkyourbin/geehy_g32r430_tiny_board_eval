#[doc = "Register `FCTRL6` reader"]
pub type R = crate::R<Fctrl6Spec>;
#[doc = "Register `FCTRL6` writer"]
pub type W = crate::W<Fctrl6Spec>;
#[doc = "Field `FTHSEL` reader - FIFO threshold selection"]
pub type FthselR = crate::FieldReader;
#[doc = "Field `FTHSEL` writer - FIFO threshold selection"]
pub type FthselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DMDEN` reader - Direct mode disable"]
pub type DmdenR = crate::BitReader;
#[doc = "Field `DMDEN` writer - Direct mode disable"]
pub type DmdenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FSTS` reader - FIFO status"]
pub type FstsR = crate::FieldReader;
#[doc = "Field `FEIEN` reader - FIFO error interrupt enable"]
pub type FeienR = crate::BitReader;
#[doc = "Field `FEIEN` writer - FIFO error interrupt enable"]
pub type FeienW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - FIFO threshold selection"]
    #[inline(always)]
    pub fn fthsel(&self) -> FthselR {
        FthselR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Direct mode disable"]
    #[inline(always)]
    pub fn dmden(&self) -> DmdenR {
        DmdenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:5 - FIFO status"]
    #[inline(always)]
    pub fn fsts(&self) -> FstsR {
        FstsR::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bit 7 - FIFO error interrupt enable"]
    #[inline(always)]
    pub fn feien(&self) -> FeienR {
        FeienR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - FIFO threshold selection"]
    #[inline(always)]
    pub fn fthsel(&mut self) -> FthselW<'_, Fctrl6Spec> {
        FthselW::new(self, 0)
    }
    #[doc = "Bit 2 - Direct mode disable"]
    #[inline(always)]
    pub fn dmden(&mut self) -> DmdenW<'_, Fctrl6Spec> {
        DmdenW::new(self, 2)
    }
    #[doc = "Bit 7 - FIFO error interrupt enable"]
    #[inline(always)]
    pub fn feien(&mut self) -> FeienW<'_, Fctrl6Spec> {
        FeienW::new(self, 7)
    }
}
#[doc = "stream x FIFO control register\n\nYou can [`read`](crate::Reg::read) this register and get [`fctrl6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fctrl6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fctrl6Spec;
impl crate::RegisterSpec for Fctrl6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fctrl6::R`](R) reader structure"]
impl crate::Readable for Fctrl6Spec {}
#[doc = "`write(|w| ..)` method takes [`fctrl6::W`](W) writer structure"]
impl crate::Writable for Fctrl6Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FCTRL6 to value 0x20"]
impl crate::Resettable for Fctrl6Spec {
    const RESET_VALUE: u32 = 0x20;
}
