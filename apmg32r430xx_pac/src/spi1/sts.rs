#[doc = "Register `STS` reader"]
pub type R = crate::R<StsSpec>;
#[doc = "Register `STS` writer"]
pub type W = crate::W<StsSpec>;
#[doc = "Field `RXBNEFLG` reader - Receive buffer not empty"]
pub type RxbneflgR = crate::BitReader;
#[doc = "Field `TXBEFLG` reader - Transmit buffer empty"]
pub type TxbeflgR = crate::BitReader;
#[doc = "Field `SCHDIR` reader - Channel side"]
pub type SchdirR = crate::BitReader;
#[doc = "Field `UDRFLG` reader - Underrun flag"]
pub type UdrflgR = crate::BitReader;
#[doc = "Field `CRCEFLG` reader - CRC error flag"]
pub type CrceflgR = crate::BitReader;
#[doc = "Field `CRCEFLG` writer - CRC error flag"]
pub type CrceflgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MEFLG` reader - Mode fault"]
pub type MeflgR = crate::BitReader;
#[doc = "Field `OVRFLG` reader - Overrun flag"]
pub type OvrflgR = crate::BitReader;
#[doc = "Field `BSYFLG` reader - Busy flag"]
pub type BsyflgR = crate::BitReader;
#[doc = "Field `FFERR` reader - TI frame format error"]
pub type FferrR = crate::BitReader;
#[doc = "Field `CDM_NE` reader - CDM Non-Empty Status"]
pub type CdmNeR = crate::BitReader;
#[doc = "Field `BSBUSY` reader - Busy in Biss-c/ssi mode"]
pub type BsbusyR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Receive buffer not empty"]
    #[inline(always)]
    pub fn rxbneflg(&self) -> RxbneflgR {
        RxbneflgR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit buffer empty"]
    #[inline(always)]
    pub fn txbeflg(&self) -> TxbeflgR {
        TxbeflgR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Channel side"]
    #[inline(always)]
    pub fn schdir(&self) -> SchdirR {
        SchdirR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Underrun flag"]
    #[inline(always)]
    pub fn udrflg(&self) -> UdrflgR {
        UdrflgR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - CRC error flag"]
    #[inline(always)]
    pub fn crceflg(&self) -> CrceflgR {
        CrceflgR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Mode fault"]
    #[inline(always)]
    pub fn meflg(&self) -> MeflgR {
        MeflgR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Overrun flag"]
    #[inline(always)]
    pub fn ovrflg(&self) -> OvrflgR {
        OvrflgR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Busy flag"]
    #[inline(always)]
    pub fn bsyflg(&self) -> BsyflgR {
        BsyflgR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - TI frame format error"]
    #[inline(always)]
    pub fn fferr(&self) -> FferrR {
        FferrR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - CDM Non-Empty Status"]
    #[inline(always)]
    pub fn cdm_ne(&self) -> CdmNeR {
        CdmNeR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Busy in Biss-c/ssi mode"]
    #[inline(always)]
    pub fn bsbusy(&self) -> BsbusyR {
        BsbusyR::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - CRC error flag"]
    #[inline(always)]
    pub fn crceflg(&mut self) -> CrceflgW<'_, StsSpec> {
        CrceflgW::new(self, 4)
    }
}
#[doc = "status register\n\nYou can [`read`](crate::Reg::read) this register and get [`sts::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sts::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
#[doc = "`reset()` method sets STS to value 0x02"]
impl crate::Resettable for StsSpec {
    const RESET_VALUE: u32 = 0x02;
}
