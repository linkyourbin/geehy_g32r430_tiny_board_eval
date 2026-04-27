#[doc = "Register `CTRL1` reader"]
pub type R = crate::R<Ctrl1Spec>;
#[doc = "Register `CTRL1` writer"]
pub type W = crate::W<Ctrl1Spec>;
#[doc = "Field `CPHA` reader - Clock phase"]
pub type CphaR = crate::BitReader;
#[doc = "Field `CPHA` writer - Clock phase"]
pub type CphaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPOL` reader - Clock polarity"]
pub type CpolR = crate::BitReader;
#[doc = "Field `CPOL` writer - Clock polarity"]
pub type CpolW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MSMCFG` reader - Master selection"]
pub type MsmcfgR = crate::BitReader;
#[doc = "Field `MSMCFG` writer - Master selection"]
pub type MsmcfgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BRSEL` reader - Baud rate control"]
pub type BrselR = crate::FieldReader;
#[doc = "Field `BRSEL` writer - Baud rate control"]
pub type BrselW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SPIEN` reader - SPI enable"]
pub type SpienR = crate::BitReader;
#[doc = "Field `SPIEN` writer - SPI enable"]
pub type SpienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LSBSEL` reader - Frame format"]
pub type LsbselR = crate::BitReader;
#[doc = "Field `LSBSEL` writer - Frame format"]
pub type LsbselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ISSEL` reader - Internal slave select"]
pub type IsselR = crate::BitReader;
#[doc = "Field `ISSEL` writer - Internal slave select"]
pub type IsselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SSEN` reader - Software slave management"]
pub type SsenR = crate::BitReader;
#[doc = "Field `SSEN` writer - Software slave management"]
pub type SsenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXOMEN` reader - Receive only"]
pub type RxomenR = crate::BitReader;
#[doc = "Field `RXOMEN` writer - Receive only"]
pub type RxomenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DFLSEL` reader - Data frame format"]
pub type DflselR = crate::BitReader;
#[doc = "Field `DFLSEL` writer - Data frame format"]
pub type DflselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRCNXT` reader - CRC transfer next"]
pub type CrcnxtR = crate::BitReader;
#[doc = "Field `CRCNXT` writer - CRC transfer next"]
pub type CrcnxtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRCEN` reader - Hardware CRC calculation enable"]
pub type CrcenR = crate::BitReader;
#[doc = "Field `CRCEN` writer - Hardware CRC calculation enable"]
pub type CrcenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BMOEN` reader - Output enable in bidirectional mode"]
pub type BmoenR = crate::BitReader;
#[doc = "Field `BMOEN` writer - Output enable in bidirectional mode"]
pub type BmoenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BMEN` reader - Bidirectional data mode enable"]
pub type BmenR = crate::BitReader;
#[doc = "Field `BMEN` writer - Bidirectional data mode enable"]
pub type BmenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Clock phase"]
    #[inline(always)]
    pub fn cpha(&self) -> CphaR {
        CphaR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Clock polarity"]
    #[inline(always)]
    pub fn cpol(&self) -> CpolR {
        CpolR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Master selection"]
    #[inline(always)]
    pub fn msmcfg(&self) -> MsmcfgR {
        MsmcfgR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:5 - Baud rate control"]
    #[inline(always)]
    pub fn brsel(&self) -> BrselR {
        BrselR::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bit 6 - SPI enable"]
    #[inline(always)]
    pub fn spien(&self) -> SpienR {
        SpienR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Frame format"]
    #[inline(always)]
    pub fn lsbsel(&self) -> LsbselR {
        LsbselR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Internal slave select"]
    #[inline(always)]
    pub fn issel(&self) -> IsselR {
        IsselR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Software slave management"]
    #[inline(always)]
    pub fn ssen(&self) -> SsenR {
        SsenR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Receive only"]
    #[inline(always)]
    pub fn rxomen(&self) -> RxomenR {
        RxomenR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Data frame format"]
    #[inline(always)]
    pub fn dflsel(&self) -> DflselR {
        DflselR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - CRC transfer next"]
    #[inline(always)]
    pub fn crcnxt(&self) -> CrcnxtR {
        CrcnxtR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Hardware CRC calculation enable"]
    #[inline(always)]
    pub fn crcen(&self) -> CrcenR {
        CrcenR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Output enable in bidirectional mode"]
    #[inline(always)]
    pub fn bmoen(&self) -> BmoenR {
        BmoenR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Bidirectional data mode enable"]
    #[inline(always)]
    pub fn bmen(&self) -> BmenR {
        BmenR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Clock phase"]
    #[inline(always)]
    pub fn cpha(&mut self) -> CphaW<'_, Ctrl1Spec> {
        CphaW::new(self, 0)
    }
    #[doc = "Bit 1 - Clock polarity"]
    #[inline(always)]
    pub fn cpol(&mut self) -> CpolW<'_, Ctrl1Spec> {
        CpolW::new(self, 1)
    }
    #[doc = "Bit 2 - Master selection"]
    #[inline(always)]
    pub fn msmcfg(&mut self) -> MsmcfgW<'_, Ctrl1Spec> {
        MsmcfgW::new(self, 2)
    }
    #[doc = "Bits 3:5 - Baud rate control"]
    #[inline(always)]
    pub fn brsel(&mut self) -> BrselW<'_, Ctrl1Spec> {
        BrselW::new(self, 3)
    }
    #[doc = "Bit 6 - SPI enable"]
    #[inline(always)]
    pub fn spien(&mut self) -> SpienW<'_, Ctrl1Spec> {
        SpienW::new(self, 6)
    }
    #[doc = "Bit 7 - Frame format"]
    #[inline(always)]
    pub fn lsbsel(&mut self) -> LsbselW<'_, Ctrl1Spec> {
        LsbselW::new(self, 7)
    }
    #[doc = "Bit 8 - Internal slave select"]
    #[inline(always)]
    pub fn issel(&mut self) -> IsselW<'_, Ctrl1Spec> {
        IsselW::new(self, 8)
    }
    #[doc = "Bit 9 - Software slave management"]
    #[inline(always)]
    pub fn ssen(&mut self) -> SsenW<'_, Ctrl1Spec> {
        SsenW::new(self, 9)
    }
    #[doc = "Bit 10 - Receive only"]
    #[inline(always)]
    pub fn rxomen(&mut self) -> RxomenW<'_, Ctrl1Spec> {
        RxomenW::new(self, 10)
    }
    #[doc = "Bit 11 - Data frame format"]
    #[inline(always)]
    pub fn dflsel(&mut self) -> DflselW<'_, Ctrl1Spec> {
        DflselW::new(self, 11)
    }
    #[doc = "Bit 12 - CRC transfer next"]
    #[inline(always)]
    pub fn crcnxt(&mut self) -> CrcnxtW<'_, Ctrl1Spec> {
        CrcnxtW::new(self, 12)
    }
    #[doc = "Bit 13 - Hardware CRC calculation enable"]
    #[inline(always)]
    pub fn crcen(&mut self) -> CrcenW<'_, Ctrl1Spec> {
        CrcenW::new(self, 13)
    }
    #[doc = "Bit 14 - Output enable in bidirectional mode"]
    #[inline(always)]
    pub fn bmoen(&mut self) -> BmoenW<'_, Ctrl1Spec> {
        BmoenW::new(self, 14)
    }
    #[doc = "Bit 15 - Bidirectional data mode enable"]
    #[inline(always)]
    pub fn bmen(&mut self) -> BmenW<'_, Ctrl1Spec> {
        BmenW::new(self, 15)
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
