#[doc = "Register `CTRL3` reader"]
pub type R = crate::R<Ctrl3Spec>;
#[doc = "Register `CTRL3` writer"]
pub type W = crate::W<Ctrl3Spec>;
#[doc = "Field `ERRIEN` reader - Error interrupt enable"]
pub type ErrienR = crate::BitReader;
#[doc = "Field `ERRIEN` writer - Error interrupt enable"]
pub type ErrienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IREN` reader - IrDA mode enable"]
pub type IrenR = crate::BitReader;
#[doc = "Field `IREN` writer - IrDA mode enable"]
pub type IrenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IRLPEN` reader - IrDA low-power"]
pub type IrlpenR = crate::BitReader;
#[doc = "Field `IRLPEN` writer - IrDA low-power"]
pub type IrlpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HDEN` reader - Half-duplex selection"]
pub type HdenR = crate::BitReader;
#[doc = "Field `HDEN` writer - Half-duplex selection"]
pub type HdenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCNACKEN` reader - Smartcard NACK enable"]
pub type ScnackenR = crate::BitReader;
#[doc = "Field `SCNACKEN` writer - Smartcard NACK enable"]
pub type ScnackenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCEN` reader - Smartcard mode enable"]
pub type ScenR = crate::BitReader;
#[doc = "Field `SCEN` writer - Smartcard mode enable"]
pub type ScenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMARXEN` reader - DMA enable receiver"]
pub type DmarxenR = crate::BitReader;
#[doc = "Field `DMARXEN` writer - DMA enable receiver"]
pub type DmarxenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMATXEN` reader - DMA enable transmitter"]
pub type DmatxenR = crate::BitReader;
#[doc = "Field `DMATXEN` writer - DMA enable transmitter"]
pub type DmatxenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTSEN` reader - RTS enable"]
pub type RtsenR = crate::BitReader;
#[doc = "Field `RTSEN` writer - RTS enable"]
pub type RtsenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTSEN` reader - CTS enable"]
pub type CtsenR = crate::BitReader;
#[doc = "Field `CTSEN` writer - CTS enable"]
pub type CtsenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTSIEN` reader - CTS interrupt enable"]
pub type CtsienR = crate::BitReader;
#[doc = "Field `CTSIEN` writer - CTS interrupt enable"]
pub type CtsienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SAMCFG` reader - Sample Method Configuration"]
pub type SamcfgR = crate::BitReader;
#[doc = "Field `SAMCFG` writer - Sample Method Configuration"]
pub type SamcfgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DEM` reader - Driver enable mode"]
pub type DemR = crate::BitReader;
#[doc = "Field `DEM` writer - Driver enable mode"]
pub type DemW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Error interrupt enable"]
    #[inline(always)]
    pub fn errien(&self) -> ErrienR {
        ErrienR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - IrDA mode enable"]
    #[inline(always)]
    pub fn iren(&self) -> IrenR {
        IrenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - IrDA low-power"]
    #[inline(always)]
    pub fn irlpen(&self) -> IrlpenR {
        IrlpenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Half-duplex selection"]
    #[inline(always)]
    pub fn hden(&self) -> HdenR {
        HdenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Smartcard NACK enable"]
    #[inline(always)]
    pub fn scnacken(&self) -> ScnackenR {
        ScnackenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Smartcard mode enable"]
    #[inline(always)]
    pub fn scen(&self) -> ScenR {
        ScenR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - DMA enable receiver"]
    #[inline(always)]
    pub fn dmarxen(&self) -> DmarxenR {
        DmarxenR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - DMA enable transmitter"]
    #[inline(always)]
    pub fn dmatxen(&self) -> DmatxenR {
        DmatxenR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - RTS enable"]
    #[inline(always)]
    pub fn rtsen(&self) -> RtsenR {
        RtsenR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - CTS enable"]
    #[inline(always)]
    pub fn ctsen(&self) -> CtsenR {
        CtsenR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - CTS interrupt enable"]
    #[inline(always)]
    pub fn ctsien(&self) -> CtsienR {
        CtsienR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Sample Method Configuration"]
    #[inline(always)]
    pub fn samcfg(&self) -> SamcfgR {
        SamcfgR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Driver enable mode"]
    #[inline(always)]
    pub fn dem(&self) -> DemR {
        DemR::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Error interrupt enable"]
    #[inline(always)]
    pub fn errien(&mut self) -> ErrienW<'_, Ctrl3Spec> {
        ErrienW::new(self, 0)
    }
    #[doc = "Bit 1 - IrDA mode enable"]
    #[inline(always)]
    pub fn iren(&mut self) -> IrenW<'_, Ctrl3Spec> {
        IrenW::new(self, 1)
    }
    #[doc = "Bit 2 - IrDA low-power"]
    #[inline(always)]
    pub fn irlpen(&mut self) -> IrlpenW<'_, Ctrl3Spec> {
        IrlpenW::new(self, 2)
    }
    #[doc = "Bit 3 - Half-duplex selection"]
    #[inline(always)]
    pub fn hden(&mut self) -> HdenW<'_, Ctrl3Spec> {
        HdenW::new(self, 3)
    }
    #[doc = "Bit 4 - Smartcard NACK enable"]
    #[inline(always)]
    pub fn scnacken(&mut self) -> ScnackenW<'_, Ctrl3Spec> {
        ScnackenW::new(self, 4)
    }
    #[doc = "Bit 5 - Smartcard mode enable"]
    #[inline(always)]
    pub fn scen(&mut self) -> ScenW<'_, Ctrl3Spec> {
        ScenW::new(self, 5)
    }
    #[doc = "Bit 6 - DMA enable receiver"]
    #[inline(always)]
    pub fn dmarxen(&mut self) -> DmarxenW<'_, Ctrl3Spec> {
        DmarxenW::new(self, 6)
    }
    #[doc = "Bit 7 - DMA enable transmitter"]
    #[inline(always)]
    pub fn dmatxen(&mut self) -> DmatxenW<'_, Ctrl3Spec> {
        DmatxenW::new(self, 7)
    }
    #[doc = "Bit 8 - RTS enable"]
    #[inline(always)]
    pub fn rtsen(&mut self) -> RtsenW<'_, Ctrl3Spec> {
        RtsenW::new(self, 8)
    }
    #[doc = "Bit 9 - CTS enable"]
    #[inline(always)]
    pub fn ctsen(&mut self) -> CtsenW<'_, Ctrl3Spec> {
        CtsenW::new(self, 9)
    }
    #[doc = "Bit 10 - CTS interrupt enable"]
    #[inline(always)]
    pub fn ctsien(&mut self) -> CtsienW<'_, Ctrl3Spec> {
        CtsienW::new(self, 10)
    }
    #[doc = "Bit 11 - Sample Method Configuration"]
    #[inline(always)]
    pub fn samcfg(&mut self) -> SamcfgW<'_, Ctrl3Spec> {
        SamcfgW::new(self, 11)
    }
    #[doc = "Bit 12 - Driver enable mode"]
    #[inline(always)]
    pub fn dem(&mut self) -> DemW<'_, Ctrl3Spec> {
        DemW::new(self, 12)
    }
}
#[doc = "Control register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ctrl3Spec;
impl crate::RegisterSpec for Ctrl3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl3::R`](R) reader structure"]
impl crate::Readable for Ctrl3Spec {}
#[doc = "`write(|w| ..)` method takes [`ctrl3::W`](W) writer structure"]
impl crate::Writable for Ctrl3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTRL3 to value 0"]
impl crate::Resettable for Ctrl3Spec {}
