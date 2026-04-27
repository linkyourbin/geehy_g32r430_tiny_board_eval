#[doc = "Register `CTRL1` reader"]
pub type R = crate::R<Ctrl1Spec>;
#[doc = "Register `CTRL1` writer"]
pub type W = crate::W<Ctrl1Spec>;
#[doc = "Field `TXBF` reader - Send break"]
pub type TxbfR = crate::BitReader;
#[doc = "Field `TXBF` writer - Send break"]
pub type TxbfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXMUTEEN` reader - Receiver wakeup"]
pub type RxmuteenR = crate::BitReader;
#[doc = "Field `RXMUTEEN` writer - Receiver wakeup"]
pub type RxmuteenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXEN` reader - Receiver enable"]
pub type RxenR = crate::BitReader;
#[doc = "Field `RXEN` writer - Receiver enable"]
pub type RxenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXEN` reader - Transmitter enable"]
pub type TxenR = crate::BitReader;
#[doc = "Field `TXEN` writer - Transmitter enable"]
pub type TxenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IDLEIEN` reader - IDLE interrupt enable"]
pub type IdleienR = crate::BitReader;
#[doc = "Field `IDLEIEN` writer - IDLE interrupt enable"]
pub type IdleienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXBNEIEN` reader - RXNE interrupt enable"]
pub type RxbneienR = crate::BitReader;
#[doc = "Field `RXBNEIEN` writer - RXNE interrupt enable"]
pub type RxbneienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXCIEN` reader - Transmission complete interrupt enable"]
pub type TxcienR = crate::BitReader;
#[doc = "Field `TXCIEN` writer - Transmission complete interrupt enable"]
pub type TxcienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXBEIEN` reader - Transmit Buffer Empty Interrupt Enable"]
pub type TxbeienR = crate::BitReader;
#[doc = "Field `TXBEIEN` writer - Transmit Buffer Empty Interrupt Enable"]
pub type TxbeienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PEIEN` reader - Parity Error Interrupt Enable"]
pub type PeienR = crate::BitReader;
#[doc = "Field `PEIEN` writer - Parity Error Interrupt Enable"]
pub type PeienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCFG` reader - Parity selection"]
pub type PcfgR = crate::BitReader;
#[doc = "Field `PCFG` writer - Parity selection"]
pub type PcfgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCEN` reader - Parity control enable"]
pub type PcenR = crate::BitReader;
#[doc = "Field `PCEN` writer - Parity control enable"]
pub type PcenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WUPMCFG` reader - Wakeup method"]
pub type WupmcfgR = crate::BitReader;
#[doc = "Field `WUPMCFG` writer - Wakeup method"]
pub type WupmcfgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBLCFG` reader - Word length"]
pub type DblcfgR = crate::BitReader;
#[doc = "Field `DBLCFG` writer - Word length"]
pub type DblcfgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UEN` reader - USART enable"]
pub type UenR = crate::BitReader;
#[doc = "Field `UEN` writer - USART enable"]
pub type UenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OSMCFG` reader - Oversampling mode"]
pub type OsmcfgR = crate::BitReader;
#[doc = "Field `OSMCFG` writer - Oversampling mode"]
pub type OsmcfgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DEDT` reader - Driver Enable deassertion time"]
pub type DedtR = crate::FieldReader;
#[doc = "Field `DEDT` writer - Driver Enable deassertion time"]
pub type DedtW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `DEAT` reader - Driver Enable assertion time"]
pub type DeatR = crate::FieldReader;
#[doc = "Field `DEAT` writer - Driver Enable assertion time"]
pub type DeatW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bit 0 - Send break"]
    #[inline(always)]
    pub fn txbf(&self) -> TxbfR {
        TxbfR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Receiver wakeup"]
    #[inline(always)]
    pub fn rxmuteen(&self) -> RxmuteenR {
        RxmuteenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Receiver enable"]
    #[inline(always)]
    pub fn rxen(&self) -> RxenR {
        RxenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Transmitter enable"]
    #[inline(always)]
    pub fn txen(&self) -> TxenR {
        TxenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - IDLE interrupt enable"]
    #[inline(always)]
    pub fn idleien(&self) -> IdleienR {
        IdleienR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - RXNE interrupt enable"]
    #[inline(always)]
    pub fn rxbneien(&self) -> RxbneienR {
        RxbneienR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Transmission complete interrupt enable"]
    #[inline(always)]
    pub fn txcien(&self) -> TxcienR {
        TxcienR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Transmit Buffer Empty Interrupt Enable"]
    #[inline(always)]
    pub fn txbeien(&self) -> TxbeienR {
        TxbeienR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Parity Error Interrupt Enable"]
    #[inline(always)]
    pub fn peien(&self) -> PeienR {
        PeienR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Parity selection"]
    #[inline(always)]
    pub fn pcfg(&self) -> PcfgR {
        PcfgR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Parity control enable"]
    #[inline(always)]
    pub fn pcen(&self) -> PcenR {
        PcenR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Wakeup method"]
    #[inline(always)]
    pub fn wupmcfg(&self) -> WupmcfgR {
        WupmcfgR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Word length"]
    #[inline(always)]
    pub fn dblcfg(&self) -> DblcfgR {
        DblcfgR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - USART enable"]
    #[inline(always)]
    pub fn uen(&self) -> UenR {
        UenR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 15 - Oversampling mode"]
    #[inline(always)]
    pub fn osmcfg(&self) -> OsmcfgR {
        OsmcfgR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:20 - Driver Enable deassertion time"]
    #[inline(always)]
    pub fn dedt(&self) -> DedtR {
        DedtR::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 21:25 - Driver Enable assertion time"]
    #[inline(always)]
    pub fn deat(&self) -> DeatR {
        DeatR::new(((self.bits >> 21) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Send break"]
    #[inline(always)]
    pub fn txbf(&mut self) -> TxbfW<'_, Ctrl1Spec> {
        TxbfW::new(self, 0)
    }
    #[doc = "Bit 1 - Receiver wakeup"]
    #[inline(always)]
    pub fn rxmuteen(&mut self) -> RxmuteenW<'_, Ctrl1Spec> {
        RxmuteenW::new(self, 1)
    }
    #[doc = "Bit 2 - Receiver enable"]
    #[inline(always)]
    pub fn rxen(&mut self) -> RxenW<'_, Ctrl1Spec> {
        RxenW::new(self, 2)
    }
    #[doc = "Bit 3 - Transmitter enable"]
    #[inline(always)]
    pub fn txen(&mut self) -> TxenW<'_, Ctrl1Spec> {
        TxenW::new(self, 3)
    }
    #[doc = "Bit 4 - IDLE interrupt enable"]
    #[inline(always)]
    pub fn idleien(&mut self) -> IdleienW<'_, Ctrl1Spec> {
        IdleienW::new(self, 4)
    }
    #[doc = "Bit 5 - RXNE interrupt enable"]
    #[inline(always)]
    pub fn rxbneien(&mut self) -> RxbneienW<'_, Ctrl1Spec> {
        RxbneienW::new(self, 5)
    }
    #[doc = "Bit 6 - Transmission complete interrupt enable"]
    #[inline(always)]
    pub fn txcien(&mut self) -> TxcienW<'_, Ctrl1Spec> {
        TxcienW::new(self, 6)
    }
    #[doc = "Bit 7 - Transmit Buffer Empty Interrupt Enable"]
    #[inline(always)]
    pub fn txbeien(&mut self) -> TxbeienW<'_, Ctrl1Spec> {
        TxbeienW::new(self, 7)
    }
    #[doc = "Bit 8 - Parity Error Interrupt Enable"]
    #[inline(always)]
    pub fn peien(&mut self) -> PeienW<'_, Ctrl1Spec> {
        PeienW::new(self, 8)
    }
    #[doc = "Bit 9 - Parity selection"]
    #[inline(always)]
    pub fn pcfg(&mut self) -> PcfgW<'_, Ctrl1Spec> {
        PcfgW::new(self, 9)
    }
    #[doc = "Bit 10 - Parity control enable"]
    #[inline(always)]
    pub fn pcen(&mut self) -> PcenW<'_, Ctrl1Spec> {
        PcenW::new(self, 10)
    }
    #[doc = "Bit 11 - Wakeup method"]
    #[inline(always)]
    pub fn wupmcfg(&mut self) -> WupmcfgW<'_, Ctrl1Spec> {
        WupmcfgW::new(self, 11)
    }
    #[doc = "Bit 12 - Word length"]
    #[inline(always)]
    pub fn dblcfg(&mut self) -> DblcfgW<'_, Ctrl1Spec> {
        DblcfgW::new(self, 12)
    }
    #[doc = "Bit 13 - USART enable"]
    #[inline(always)]
    pub fn uen(&mut self) -> UenW<'_, Ctrl1Spec> {
        UenW::new(self, 13)
    }
    #[doc = "Bit 15 - Oversampling mode"]
    #[inline(always)]
    pub fn osmcfg(&mut self) -> OsmcfgW<'_, Ctrl1Spec> {
        OsmcfgW::new(self, 15)
    }
    #[doc = "Bits 16:20 - Driver Enable deassertion time"]
    #[inline(always)]
    pub fn dedt(&mut self) -> DedtW<'_, Ctrl1Spec> {
        DedtW::new(self, 16)
    }
    #[doc = "Bits 21:25 - Driver Enable assertion time"]
    #[inline(always)]
    pub fn deat(&mut self) -> DeatW<'_, Ctrl1Spec> {
        DeatW::new(self, 21)
    }
}
#[doc = "Control register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
