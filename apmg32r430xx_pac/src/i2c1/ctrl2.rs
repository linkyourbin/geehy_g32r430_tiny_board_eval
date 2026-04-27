#[doc = "Register `CTRL2` reader"]
pub type R = crate::R<Ctrl2Spec>;
#[doc = "Register `CTRL2` writer"]
pub type W = crate::W<Ctrl2Spec>;
#[doc = "Field `CLKFCFG` reader - I2C clock frequency"]
pub type ClkfcfgR = crate::FieldReader;
#[doc = "Field `CLKFCFG` writer - I2C clock frequency"]
pub type ClkfcfgW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `ERRIEN` reader - Error interrupt enable"]
pub type ErrienR = crate::BitReader;
#[doc = "Field `ERRIEN` writer - Error interrupt enable"]
pub type ErrienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EVIEN` reader - Event interrupt enable"]
pub type EvienR = crate::BitReader;
#[doc = "Field `EVIEN` writer - Event interrupt enable"]
pub type EvienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUFIEN` reader - Buffer interrupt enable"]
pub type BufienR = crate::BitReader;
#[doc = "Field `BUFIEN` writer - Buffer interrupt enable"]
pub type BufienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMAEN` reader - DMA requests enable"]
pub type DmaenR = crate::BitReader;
#[doc = "Field `DMAEN` writer - DMA requests enable"]
pub type DmaenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LTCFG` reader - DMA last transfer"]
pub type LtcfgR = crate::BitReader;
#[doc = "Field `LTCFG` writer - DMA last transfer"]
pub type LtcfgW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:5 - I2C clock frequency"]
    #[inline(always)]
    pub fn clkfcfg(&self) -> ClkfcfgR {
        ClkfcfgR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 8 - Error interrupt enable"]
    #[inline(always)]
    pub fn errien(&self) -> ErrienR {
        ErrienR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Event interrupt enable"]
    #[inline(always)]
    pub fn evien(&self) -> EvienR {
        EvienR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Buffer interrupt enable"]
    #[inline(always)]
    pub fn bufien(&self) -> BufienR {
        BufienR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - DMA requests enable"]
    #[inline(always)]
    pub fn dmaen(&self) -> DmaenR {
        DmaenR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - DMA last transfer"]
    #[inline(always)]
    pub fn ltcfg(&self) -> LtcfgR {
        LtcfgR::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - I2C clock frequency"]
    #[inline(always)]
    pub fn clkfcfg(&mut self) -> ClkfcfgW<'_, Ctrl2Spec> {
        ClkfcfgW::new(self, 0)
    }
    #[doc = "Bit 8 - Error interrupt enable"]
    #[inline(always)]
    pub fn errien(&mut self) -> ErrienW<'_, Ctrl2Spec> {
        ErrienW::new(self, 8)
    }
    #[doc = "Bit 9 - Event interrupt enable"]
    #[inline(always)]
    pub fn evien(&mut self) -> EvienW<'_, Ctrl2Spec> {
        EvienW::new(self, 9)
    }
    #[doc = "Bit 10 - Buffer interrupt enable"]
    #[inline(always)]
    pub fn bufien(&mut self) -> BufienW<'_, Ctrl2Spec> {
        BufienW::new(self, 10)
    }
    #[doc = "Bit 11 - DMA requests enable"]
    #[inline(always)]
    pub fn dmaen(&mut self) -> DmaenW<'_, Ctrl2Spec> {
        DmaenW::new(self, 11)
    }
    #[doc = "Bit 12 - DMA last transfer"]
    #[inline(always)]
    pub fn ltcfg(&mut self) -> LtcfgW<'_, Ctrl2Spec> {
        LtcfgW::new(self, 12)
    }
}
#[doc = "Control register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ctrl2Spec;
impl crate::RegisterSpec for Ctrl2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl2::R`](R) reader structure"]
impl crate::Readable for Ctrl2Spec {}
#[doc = "`write(|w| ..)` method takes [`ctrl2::W`](W) writer structure"]
impl crate::Writable for Ctrl2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTRL2 to value 0"]
impl crate::Resettable for Ctrl2Spec {}
