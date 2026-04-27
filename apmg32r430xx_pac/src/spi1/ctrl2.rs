#[doc = "Register `CTRL2` reader"]
pub type R = crate::R<Ctrl2Spec>;
#[doc = "Register `CTRL2` writer"]
pub type W = crate::W<Ctrl2Spec>;
#[doc = "Field `RXDEN` reader - Rx buffer DMA enable"]
pub type RxdenR = crate::BitReader;
#[doc = "Field `RXDEN` writer - Rx buffer DMA enable"]
pub type RxdenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXDEN` reader - Tx buffer DMA enable"]
pub type TxdenR = crate::BitReader;
#[doc = "Field `TXDEN` writer - Tx buffer DMA enable"]
pub type TxdenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SSOEN` reader - SS output enable"]
pub type SsoenR = crate::BitReader;
#[doc = "Field `SSOEN` writer - SS output enable"]
pub type SsoenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FRFCFG` reader - Frame Format Configure"]
pub type FrfcfgR = crate::BitReader;
#[doc = "Field `FRFCFG` writer - Frame Format Configure"]
pub type FrfcfgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERRIEN` reader - Error interrupt enable"]
pub type ErrienR = crate::BitReader;
#[doc = "Field `ERRIEN` writer - Error interrupt enable"]
pub type ErrienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXBNEIEN` reader - RX buffer not empty interrupt enable"]
pub type RxbneienR = crate::BitReader;
#[doc = "Field `RXBNEIEN` writer - RX buffer not empty interrupt enable"]
pub type RxbneienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXBEIEN` reader - Tx buffer empty interrupt enable"]
pub type TxbeienR = crate::BitReader;
#[doc = "Field `TXBEIEN` writer - Tx buffer empty interrupt enable"]
pub type TxbeienW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Rx buffer DMA enable"]
    #[inline(always)]
    pub fn rxden(&self) -> RxdenR {
        RxdenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Tx buffer DMA enable"]
    #[inline(always)]
    pub fn txden(&self) -> TxdenR {
        TxdenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SS output enable"]
    #[inline(always)]
    pub fn ssoen(&self) -> SsoenR {
        SsoenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Frame Format Configure"]
    #[inline(always)]
    pub fn frfcfg(&self) -> FrfcfgR {
        FrfcfgR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Error interrupt enable"]
    #[inline(always)]
    pub fn errien(&self) -> ErrienR {
        ErrienR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - RX buffer not empty interrupt enable"]
    #[inline(always)]
    pub fn rxbneien(&self) -> RxbneienR {
        RxbneienR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Tx buffer empty interrupt enable"]
    #[inline(always)]
    pub fn txbeien(&self) -> TxbeienR {
        TxbeienR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Rx buffer DMA enable"]
    #[inline(always)]
    pub fn rxden(&mut self) -> RxdenW<'_, Ctrl2Spec> {
        RxdenW::new(self, 0)
    }
    #[doc = "Bit 1 - Tx buffer DMA enable"]
    #[inline(always)]
    pub fn txden(&mut self) -> TxdenW<'_, Ctrl2Spec> {
        TxdenW::new(self, 1)
    }
    #[doc = "Bit 2 - SS output enable"]
    #[inline(always)]
    pub fn ssoen(&mut self) -> SsoenW<'_, Ctrl2Spec> {
        SsoenW::new(self, 2)
    }
    #[doc = "Bit 4 - Frame Format Configure"]
    #[inline(always)]
    pub fn frfcfg(&mut self) -> FrfcfgW<'_, Ctrl2Spec> {
        FrfcfgW::new(self, 4)
    }
    #[doc = "Bit 5 - Error interrupt enable"]
    #[inline(always)]
    pub fn errien(&mut self) -> ErrienW<'_, Ctrl2Spec> {
        ErrienW::new(self, 5)
    }
    #[doc = "Bit 6 - RX buffer not empty interrupt enable"]
    #[inline(always)]
    pub fn rxbneien(&mut self) -> RxbneienW<'_, Ctrl2Spec> {
        RxbneienW::new(self, 6)
    }
    #[doc = "Bit 7 - Tx buffer empty interrupt enable"]
    #[inline(always)]
    pub fn txbeien(&mut self) -> TxbeienW<'_, Ctrl2Spec> {
        TxbeienW::new(self, 7)
    }
}
#[doc = "control register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
