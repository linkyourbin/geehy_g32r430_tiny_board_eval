#[doc = "Register `CTRL1` reader"]
pub type R = crate::R<Ctrl1Spec>;
#[doc = "Register `CTRL1` writer"]
pub type W = crate::W<Ctrl1Spec>;
#[doc = "Field `CNTEN` reader - Counter enable"]
pub type CntenR = crate::BitReader;
#[doc = "Field `CNTEN` writer - Counter enable"]
pub type CntenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UD` reader - Update disable"]
pub type UdR = crate::BitReader;
#[doc = "Field `UD` writer - Update disable"]
pub type UdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `URSSEL` reader - Update request source"]
pub type UrsselR = crate::BitReader;
#[doc = "Field `URSSEL` writer - Update request source"]
pub type UrsselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPMEN` reader - One-pulse mode"]
pub type SpmenR = crate::BitReader;
#[doc = "Field `SPMEN` writer - One-pulse mode"]
pub type SpmenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CNTDIR` reader - Direction"]
pub type CntdirR = crate::BitReader;
#[doc = "Field `CNTDIR` writer - Direction"]
pub type CntdirW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAMSEL` reader - Center-aligned mode selection"]
pub type CamselR = crate::FieldReader;
#[doc = "Field `CAMSEL` writer - Center-aligned mode selection"]
pub type CamselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ARPEN` reader - Auto-reload preload enable"]
pub type ArpenR = crate::BitReader;
#[doc = "Field `ARPEN` writer - Auto-reload preload enable"]
pub type ArpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLKDIV` reader - Clock division"]
pub type ClkdivR = crate::FieldReader;
#[doc = "Field `CLKDIV` writer - Clock division"]
pub type ClkdivW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - Counter enable"]
    #[inline(always)]
    pub fn cnten(&self) -> CntenR {
        CntenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Update disable"]
    #[inline(always)]
    pub fn ud(&self) -> UdR {
        UdR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Update request source"]
    #[inline(always)]
    pub fn urssel(&self) -> UrsselR {
        UrsselR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - One-pulse mode"]
    #[inline(always)]
    pub fn spmen(&self) -> SpmenR {
        SpmenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Direction"]
    #[inline(always)]
    pub fn cntdir(&self) -> CntdirR {
        CntdirR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - Center-aligned mode selection"]
    #[inline(always)]
    pub fn camsel(&self) -> CamselR {
        CamselR::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - Auto-reload preload enable"]
    #[inline(always)]
    pub fn arpen(&self) -> ArpenR {
        ArpenR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Clock division"]
    #[inline(always)]
    pub fn clkdiv(&self) -> ClkdivR {
        ClkdivR::new(((self.bits >> 8) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Counter enable"]
    #[inline(always)]
    pub fn cnten(&mut self) -> CntenW<'_, Ctrl1Spec> {
        CntenW::new(self, 0)
    }
    #[doc = "Bit 1 - Update disable"]
    #[inline(always)]
    pub fn ud(&mut self) -> UdW<'_, Ctrl1Spec> {
        UdW::new(self, 1)
    }
    #[doc = "Bit 2 - Update request source"]
    #[inline(always)]
    pub fn urssel(&mut self) -> UrsselW<'_, Ctrl1Spec> {
        UrsselW::new(self, 2)
    }
    #[doc = "Bit 3 - One-pulse mode"]
    #[inline(always)]
    pub fn spmen(&mut self) -> SpmenW<'_, Ctrl1Spec> {
        SpmenW::new(self, 3)
    }
    #[doc = "Bit 4 - Direction"]
    #[inline(always)]
    pub fn cntdir(&mut self) -> CntdirW<'_, Ctrl1Spec> {
        CntdirW::new(self, 4)
    }
    #[doc = "Bits 5:6 - Center-aligned mode selection"]
    #[inline(always)]
    pub fn camsel(&mut self) -> CamselW<'_, Ctrl1Spec> {
        CamselW::new(self, 5)
    }
    #[doc = "Bit 7 - Auto-reload preload enable"]
    #[inline(always)]
    pub fn arpen(&mut self) -> ArpenW<'_, Ctrl1Spec> {
        ArpenW::new(self, 7)
    }
    #[doc = "Bits 8:9 - Clock division"]
    #[inline(always)]
    pub fn clkdiv(&mut self) -> ClkdivW<'_, Ctrl1Spec> {
        ClkdivW::new(self, 8)
    }
}
#[doc = "control register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ctrl1Spec;
impl crate::RegisterSpec for Ctrl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl1::R`](R) reader structure"]
impl crate::Readable for Ctrl1Spec {}
#[doc = "`write(|w| ..)` method takes [`ctrl1::W`](W) writer structure"]
impl crate::Writable for Ctrl1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTRL1 to value 0"]
impl crate::Resettable for Ctrl1Spec {}
