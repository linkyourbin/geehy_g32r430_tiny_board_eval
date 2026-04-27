#[doc = "Register `CTRL2` reader"]
pub type R = crate::R<Ctrl2Spec>;
#[doc = "Register `CTRL2` writer"]
pub type W = crate::W<Ctrl2Spec>;
#[doc = "Field `ADDR` reader - Address of the USART node"]
pub type AddrR = crate::FieldReader;
#[doc = "Field `ADDR` writer - Address of the USART node"]
pub type AddrW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `LBDLCFG` reader - lin break detection length"]
pub type LbdlcfgR = crate::BitReader;
#[doc = "Field `LBDLCFG` writer - lin break detection length"]
pub type LbdlcfgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LBDIEN` reader - LIN break detection interrupt enable"]
pub type LbdienR = crate::BitReader;
#[doc = "Field `LBDIEN` writer - LIN break detection interrupt enable"]
pub type LbdienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LBCPOEN` reader - Last bit clock pulse"]
pub type LbcpoenR = crate::BitReader;
#[doc = "Field `LBCPOEN` writer - Last bit clock pulse"]
pub type LbcpoenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPHA` reader - Clock phase"]
pub type CphaR = crate::BitReader;
#[doc = "Field `CPHA` writer - Clock phase"]
pub type CphaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPOL` reader - Clock polarity"]
pub type CpolR = crate::BitReader;
#[doc = "Field `CPOL` writer - Clock polarity"]
pub type CpolW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLKEN` reader - Clock enable"]
pub type ClkenR = crate::BitReader;
#[doc = "Field `CLKEN` writer - Clock enable"]
pub type ClkenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STOPCFG` reader - STOP bits"]
pub type StopcfgR = crate::FieldReader;
#[doc = "Field `STOPCFG` writer - STOP bits"]
pub type StopcfgW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `LINMEN` reader - LIN mode enable"]
pub type LinmenR = crate::BitReader;
#[doc = "Field `LINMEN` writer - LIN mode enable"]
pub type LinmenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - Address of the USART node"]
    #[inline(always)]
    pub fn addr(&self) -> AddrR {
        AddrR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 5 - lin break detection length"]
    #[inline(always)]
    pub fn lbdlcfg(&self) -> LbdlcfgR {
        LbdlcfgR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - LIN break detection interrupt enable"]
    #[inline(always)]
    pub fn lbdien(&self) -> LbdienR {
        LbdienR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Last bit clock pulse"]
    #[inline(always)]
    pub fn lbcpoen(&self) -> LbcpoenR {
        LbcpoenR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Clock phase"]
    #[inline(always)]
    pub fn cpha(&self) -> CphaR {
        CphaR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Clock polarity"]
    #[inline(always)]
    pub fn cpol(&self) -> CpolR {
        CpolR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Clock enable"]
    #[inline(always)]
    pub fn clken(&self) -> ClkenR {
        ClkenR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:13 - STOP bits"]
    #[inline(always)]
    pub fn stopcfg(&self) -> StopcfgR {
        StopcfgR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - LIN mode enable"]
    #[inline(always)]
    pub fn linmen(&self) -> LinmenR {
        LinmenR::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Address of the USART node"]
    #[inline(always)]
    pub fn addr(&mut self) -> AddrW<'_, Ctrl2Spec> {
        AddrW::new(self, 0)
    }
    #[doc = "Bit 5 - lin break detection length"]
    #[inline(always)]
    pub fn lbdlcfg(&mut self) -> LbdlcfgW<'_, Ctrl2Spec> {
        LbdlcfgW::new(self, 5)
    }
    #[doc = "Bit 6 - LIN break detection interrupt enable"]
    #[inline(always)]
    pub fn lbdien(&mut self) -> LbdienW<'_, Ctrl2Spec> {
        LbdienW::new(self, 6)
    }
    #[doc = "Bit 8 - Last bit clock pulse"]
    #[inline(always)]
    pub fn lbcpoen(&mut self) -> LbcpoenW<'_, Ctrl2Spec> {
        LbcpoenW::new(self, 8)
    }
    #[doc = "Bit 9 - Clock phase"]
    #[inline(always)]
    pub fn cpha(&mut self) -> CphaW<'_, Ctrl2Spec> {
        CphaW::new(self, 9)
    }
    #[doc = "Bit 10 - Clock polarity"]
    #[inline(always)]
    pub fn cpol(&mut self) -> CpolW<'_, Ctrl2Spec> {
        CpolW::new(self, 10)
    }
    #[doc = "Bit 11 - Clock enable"]
    #[inline(always)]
    pub fn clken(&mut self) -> ClkenW<'_, Ctrl2Spec> {
        ClkenW::new(self, 11)
    }
    #[doc = "Bits 12:13 - STOP bits"]
    #[inline(always)]
    pub fn stopcfg(&mut self) -> StopcfgW<'_, Ctrl2Spec> {
        StopcfgW::new(self, 12)
    }
    #[doc = "Bit 14 - LIN mode enable"]
    #[inline(always)]
    pub fn linmen(&mut self) -> LinmenW<'_, Ctrl2Spec> {
        LinmenW::new(self, 14)
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
