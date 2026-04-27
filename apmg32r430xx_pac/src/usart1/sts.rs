#[doc = "Register `STS` reader"]
pub type R = crate::R<StsSpec>;
#[doc = "Register `STS` writer"]
pub type W = crate::W<StsSpec>;
#[doc = "Field `PEFLG` reader - Parity error"]
pub type PeflgR = crate::BitReader;
#[doc = "Field `FEFLG` reader - Framing error"]
pub type FeflgR = crate::BitReader;
#[doc = "Field `NEFLG` reader - Noise detected flag"]
pub type NeflgR = crate::BitReader;
#[doc = "Field `OVREFLG` reader - Overrun error"]
pub type OvreflgR = crate::BitReader;
#[doc = "Field `IDLEFLG` reader - IDLE line detected"]
pub type IdleflgR = crate::BitReader;
#[doc = "Field `RXBNEFLG` reader - Read data register not empty"]
pub type RxbneflgR = crate::BitReader;
#[doc = "Field `RXBNEFLG` writer - Read data register not empty"]
pub type RxbneflgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXCFLG` reader - Transmission complete"]
pub type TxcflgR = crate::BitReader;
#[doc = "Field `TXCFLG` writer - Transmission complete"]
pub type TxcflgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXBEFLG` reader - Transmit data register empty"]
pub type TxbeflgR = crate::BitReader;
#[doc = "Field `LBDFLG` reader - LIN break detection flag"]
pub type LbdflgR = crate::BitReader;
#[doc = "Field `LBDFLG` writer - LIN break detection flag"]
pub type LbdflgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTSFLG` reader - CTS flag"]
pub type CtsflgR = crate::BitReader;
#[doc = "Field `CTSFLG` writer - CTS flag"]
pub type CtsflgW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Parity error"]
    #[inline(always)]
    pub fn peflg(&self) -> PeflgR {
        PeflgR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Framing error"]
    #[inline(always)]
    pub fn feflg(&self) -> FeflgR {
        FeflgR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Noise detected flag"]
    #[inline(always)]
    pub fn neflg(&self) -> NeflgR {
        NeflgR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Overrun error"]
    #[inline(always)]
    pub fn ovreflg(&self) -> OvreflgR {
        OvreflgR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - IDLE line detected"]
    #[inline(always)]
    pub fn idleflg(&self) -> IdleflgR {
        IdleflgR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Read data register not empty"]
    #[inline(always)]
    pub fn rxbneflg(&self) -> RxbneflgR {
        RxbneflgR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Transmission complete"]
    #[inline(always)]
    pub fn txcflg(&self) -> TxcflgR {
        TxcflgR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Transmit data register empty"]
    #[inline(always)]
    pub fn txbeflg(&self) -> TxbeflgR {
        TxbeflgR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - LIN break detection flag"]
    #[inline(always)]
    pub fn lbdflg(&self) -> LbdflgR {
        LbdflgR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - CTS flag"]
    #[inline(always)]
    pub fn ctsflg(&self) -> CtsflgR {
        CtsflgR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - Read data register not empty"]
    #[inline(always)]
    pub fn rxbneflg(&mut self) -> RxbneflgW<'_, StsSpec> {
        RxbneflgW::new(self, 5)
    }
    #[doc = "Bit 6 - Transmission complete"]
    #[inline(always)]
    pub fn txcflg(&mut self) -> TxcflgW<'_, StsSpec> {
        TxcflgW::new(self, 6)
    }
    #[doc = "Bit 8 - LIN break detection flag"]
    #[inline(always)]
    pub fn lbdflg(&mut self) -> LbdflgW<'_, StsSpec> {
        LbdflgW::new(self, 8)
    }
    #[doc = "Bit 9 - CTS flag"]
    #[inline(always)]
    pub fn ctsflg(&mut self) -> CtsflgW<'_, StsSpec> {
        CtsflgW::new(self, 9)
    }
}
#[doc = "Status register\n\nYou can [`read`](crate::Reg::read) this register and get [`sts::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sts::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StsSpec;
impl crate::RegisterSpec for StsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sts::R`](R) reader structure"]
impl crate::Readable for StsSpec {}
#[doc = "`write(|w| ..)` method takes [`sts::W`](W) writer structure"]
impl crate::Writable for StsSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets STS to value 0xc0"]
impl crate::Resettable for StsSpec {
    const RESET_VALUE: u32 = 0xc0;
}
