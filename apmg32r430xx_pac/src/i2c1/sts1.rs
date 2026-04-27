#[doc = "Register `STS1` reader"]
pub type R = crate::R<Sts1Spec>;
#[doc = "Register `STS1` writer"]
pub type W = crate::W<Sts1Spec>;
#[doc = "Field `STARTFLG` reader - Start bit (Master mode)"]
pub type StartflgR = crate::BitReader;
#[doc = "Field `ADDRFLG` reader - Address sent (master mode)/matched (slave mode)"]
pub type AddrflgR = crate::BitReader;
#[doc = "Field `BTCFLG` reader - Byte transfer finished"]
pub type BtcflgR = crate::BitReader;
#[doc = "Field `ADDR10FLG` reader - 10-bit header sent (Master mode)"]
pub type Addr10flgR = crate::BitReader;
#[doc = "Field `STOPFLG` reader - Stop detection (slave mode)"]
pub type StopflgR = crate::BitReader;
#[doc = "Field `RXBNEFLG` reader - Data register not empty (receivers)"]
pub type RxbneflgR = crate::BitReader;
#[doc = "Field `TXBEFLG` reader - Data register empty (transmitters)"]
pub type TxbeflgR = crate::BitReader;
#[doc = "Field `BERRFLG` reader - Bus error"]
pub type BerrflgR = crate::BitReader;
#[doc = "Field `BERRFLG` writer - Bus error"]
pub type BerrflgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ALFLG` reader - Arbitration lost (master mode)"]
pub type AlflgR = crate::BitReader;
#[doc = "Field `ALFLG` writer - Arbitration lost (master mode)"]
pub type AlflgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AEFLG` reader - Acknowledge failure"]
pub type AeflgR = crate::BitReader;
#[doc = "Field `AEFLG` writer - Acknowledge failure"]
pub type AeflgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVRURFLG` reader - Overrun/Underrun"]
pub type OvrurflgR = crate::BitReader;
#[doc = "Field `OVRURFLG` writer - Overrun/Underrun"]
pub type OvrurflgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PECEFLG` reader - PEC Error in reception"]
pub type PeceflgR = crate::BitReader;
#[doc = "Field `PECEFLG` writer - PEC Error in reception"]
pub type PeceflgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TTEFLG` reader - Timeout or Tlow error"]
pub type TteflgR = crate::BitReader;
#[doc = "Field `TTEFLG` writer - Timeout or Tlow error"]
pub type TteflgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SMBALTFLG` reader - SMBus alert"]
pub type SmbaltflgR = crate::BitReader;
#[doc = "Field `SMBALTFLG` writer - SMBus alert"]
pub type SmbaltflgW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Start bit (Master mode)"]
    #[inline(always)]
    pub fn startflg(&self) -> StartflgR {
        StartflgR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Address sent (master mode)/matched (slave mode)"]
    #[inline(always)]
    pub fn addrflg(&self) -> AddrflgR {
        AddrflgR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Byte transfer finished"]
    #[inline(always)]
    pub fn btcflg(&self) -> BtcflgR {
        BtcflgR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 10-bit header sent (Master mode)"]
    #[inline(always)]
    pub fn addr10flg(&self) -> Addr10flgR {
        Addr10flgR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Stop detection (slave mode)"]
    #[inline(always)]
    pub fn stopflg(&self) -> StopflgR {
        StopflgR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - Data register not empty (receivers)"]
    #[inline(always)]
    pub fn rxbneflg(&self) -> RxbneflgR {
        RxbneflgR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Data register empty (transmitters)"]
    #[inline(always)]
    pub fn txbeflg(&self) -> TxbeflgR {
        TxbeflgR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Bus error"]
    #[inline(always)]
    pub fn berrflg(&self) -> BerrflgR {
        BerrflgR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Arbitration lost (master mode)"]
    #[inline(always)]
    pub fn alflg(&self) -> AlflgR {
        AlflgR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Acknowledge failure"]
    #[inline(always)]
    pub fn aeflg(&self) -> AeflgR {
        AeflgR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Overrun/Underrun"]
    #[inline(always)]
    pub fn ovrurflg(&self) -> OvrurflgR {
        OvrurflgR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PEC Error in reception"]
    #[inline(always)]
    pub fn peceflg(&self) -> PeceflgR {
        PeceflgR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - Timeout or Tlow error"]
    #[inline(always)]
    pub fn tteflg(&self) -> TteflgR {
        TteflgR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - SMBus alert"]
    #[inline(always)]
    pub fn smbaltflg(&self) -> SmbaltflgR {
        SmbaltflgR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - Bus error"]
    #[inline(always)]
    pub fn berrflg(&mut self) -> BerrflgW<'_, Sts1Spec> {
        BerrflgW::new(self, 8)
    }
    #[doc = "Bit 9 - Arbitration lost (master mode)"]
    #[inline(always)]
    pub fn alflg(&mut self) -> AlflgW<'_, Sts1Spec> {
        AlflgW::new(self, 9)
    }
    #[doc = "Bit 10 - Acknowledge failure"]
    #[inline(always)]
    pub fn aeflg(&mut self) -> AeflgW<'_, Sts1Spec> {
        AeflgW::new(self, 10)
    }
    #[doc = "Bit 11 - Overrun/Underrun"]
    #[inline(always)]
    pub fn ovrurflg(&mut self) -> OvrurflgW<'_, Sts1Spec> {
        OvrurflgW::new(self, 11)
    }
    #[doc = "Bit 12 - PEC Error in reception"]
    #[inline(always)]
    pub fn peceflg(&mut self) -> PeceflgW<'_, Sts1Spec> {
        PeceflgW::new(self, 12)
    }
    #[doc = "Bit 14 - Timeout or Tlow error"]
    #[inline(always)]
    pub fn tteflg(&mut self) -> TteflgW<'_, Sts1Spec> {
        TteflgW::new(self, 14)
    }
    #[doc = "Bit 15 - SMBus alert"]
    #[inline(always)]
    pub fn smbaltflg(&mut self) -> SmbaltflgW<'_, Sts1Spec> {
        SmbaltflgW::new(self, 15)
    }
}
#[doc = "Status register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`sts1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sts1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sts1Spec;
impl crate::RegisterSpec for Sts1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sts1::R`](R) reader structure"]
impl crate::Readable for Sts1Spec {}
#[doc = "`write(|w| ..)` method takes [`sts1::W`](W) writer structure"]
impl crate::Writable for Sts1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets STS1 to value 0"]
impl crate::Resettable for Sts1Spec {}
