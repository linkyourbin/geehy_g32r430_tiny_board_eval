#[doc = "Register `CTRL1` reader"]
pub type R = crate::R<Ctrl1Spec>;
#[doc = "Register `CTRL1` writer"]
pub type W = crate::W<Ctrl1Spec>;
#[doc = "Field `I2CEN` reader - I2C enable"]
pub type I2cenR = crate::BitReader;
#[doc = "Field `I2CEN` writer - I2C enable"]
pub type I2cenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SMBEN` reader - SMBus mode"]
pub type SmbenR = crate::BitReader;
#[doc = "Field `SMBEN` writer - SMBus mode"]
pub type SmbenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SMBTCFG` reader - SMBus type"]
pub type SmbtcfgR = crate::BitReader;
#[doc = "Field `SMBTCFG` writer - SMBus type"]
pub type SmbtcfgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARPEN` reader - ARP enable"]
pub type ArpenR = crate::BitReader;
#[doc = "Field `ARPEN` writer - ARP enable"]
pub type ArpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PECEN` reader - PEC enable"]
pub type PecenR = crate::BitReader;
#[doc = "Field `PECEN` writer - PEC enable"]
pub type PecenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRBEN` reader - General call enable"]
pub type SrbenR = crate::BitReader;
#[doc = "Field `SRBEN` writer - General call enable"]
pub type SrbenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLKSTRETCHD` reader - Clock stretching disable (Slave mode)"]
pub type ClkstretchdR = crate::BitReader;
#[doc = "Field `CLKSTRETCHD` writer - Clock stretching disable (Slave mode)"]
pub type ClkstretchdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `START` reader - Start generation"]
pub type StartR = crate::BitReader;
#[doc = "Field `START` writer - Start generation"]
pub type StartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STOP` reader - Stop generation"]
pub type StopR = crate::BitReader;
#[doc = "Field `STOP` writer - Stop generation"]
pub type StopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACKEN` reader - Acknowledge enable"]
pub type AckenR = crate::BitReader;
#[doc = "Field `ACKEN` writer - Acknowledge enable"]
pub type AckenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACKPOS` reader - Acknowledge/PEC Position (for data reception)"]
pub type AckposR = crate::BitReader;
#[doc = "Field `ACKPOS` writer - Acknowledge/PEC Position (for data reception)"]
pub type AckposW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PEC` reader - Packet error checking"]
pub type PecR = crate::BitReader;
#[doc = "Field `PEC` writer - Packet error checking"]
pub type PecW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ALERTEN` reader - SMBus alert"]
pub type AlertenR = crate::BitReader;
#[doc = "Field `ALERTEN` writer - SMBus alert"]
pub type AlertenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWRST` reader - Software reset"]
pub type SwrstR = crate::BitReader;
#[doc = "Field `SWRST` writer - Software reset"]
pub type SwrstW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - I2C enable"]
    #[inline(always)]
    pub fn i2cen(&self) -> I2cenR {
        I2cenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SMBus mode"]
    #[inline(always)]
    pub fn smben(&self) -> SmbenR {
        SmbenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - SMBus type"]
    #[inline(always)]
    pub fn smbtcfg(&self) -> SmbtcfgR {
        SmbtcfgR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - ARP enable"]
    #[inline(always)]
    pub fn arpen(&self) -> ArpenR {
        ArpenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - PEC enable"]
    #[inline(always)]
    pub fn pecen(&self) -> PecenR {
        PecenR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - General call enable"]
    #[inline(always)]
    pub fn srben(&self) -> SrbenR {
        SrbenR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Clock stretching disable (Slave mode)"]
    #[inline(always)]
    pub fn clkstretchd(&self) -> ClkstretchdR {
        ClkstretchdR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Start generation"]
    #[inline(always)]
    pub fn start(&self) -> StartR {
        StartR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Stop generation"]
    #[inline(always)]
    pub fn stop(&self) -> StopR {
        StopR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Acknowledge enable"]
    #[inline(always)]
    pub fn acken(&self) -> AckenR {
        AckenR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Acknowledge/PEC Position (for data reception)"]
    #[inline(always)]
    pub fn ackpos(&self) -> AckposR {
        AckposR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Packet error checking"]
    #[inline(always)]
    pub fn pec(&self) -> PecR {
        PecR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - SMBus alert"]
    #[inline(always)]
    pub fn alerten(&self) -> AlertenR {
        AlertenR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 15 - Software reset"]
    #[inline(always)]
    pub fn swrst(&self) -> SwrstR {
        SwrstR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - I2C enable"]
    #[inline(always)]
    pub fn i2cen(&mut self) -> I2cenW<'_, Ctrl1Spec> {
        I2cenW::new(self, 0)
    }
    #[doc = "Bit 1 - SMBus mode"]
    #[inline(always)]
    pub fn smben(&mut self) -> SmbenW<'_, Ctrl1Spec> {
        SmbenW::new(self, 1)
    }
    #[doc = "Bit 3 - SMBus type"]
    #[inline(always)]
    pub fn smbtcfg(&mut self) -> SmbtcfgW<'_, Ctrl1Spec> {
        SmbtcfgW::new(self, 3)
    }
    #[doc = "Bit 4 - ARP enable"]
    #[inline(always)]
    pub fn arpen(&mut self) -> ArpenW<'_, Ctrl1Spec> {
        ArpenW::new(self, 4)
    }
    #[doc = "Bit 5 - PEC enable"]
    #[inline(always)]
    pub fn pecen(&mut self) -> PecenW<'_, Ctrl1Spec> {
        PecenW::new(self, 5)
    }
    #[doc = "Bit 6 - General call enable"]
    #[inline(always)]
    pub fn srben(&mut self) -> SrbenW<'_, Ctrl1Spec> {
        SrbenW::new(self, 6)
    }
    #[doc = "Bit 7 - Clock stretching disable (Slave mode)"]
    #[inline(always)]
    pub fn clkstretchd(&mut self) -> ClkstretchdW<'_, Ctrl1Spec> {
        ClkstretchdW::new(self, 7)
    }
    #[doc = "Bit 8 - Start generation"]
    #[inline(always)]
    pub fn start(&mut self) -> StartW<'_, Ctrl1Spec> {
        StartW::new(self, 8)
    }
    #[doc = "Bit 9 - Stop generation"]
    #[inline(always)]
    pub fn stop(&mut self) -> StopW<'_, Ctrl1Spec> {
        StopW::new(self, 9)
    }
    #[doc = "Bit 10 - Acknowledge enable"]
    #[inline(always)]
    pub fn acken(&mut self) -> AckenW<'_, Ctrl1Spec> {
        AckenW::new(self, 10)
    }
    #[doc = "Bit 11 - Acknowledge/PEC Position (for data reception)"]
    #[inline(always)]
    pub fn ackpos(&mut self) -> AckposW<'_, Ctrl1Spec> {
        AckposW::new(self, 11)
    }
    #[doc = "Bit 12 - Packet error checking"]
    #[inline(always)]
    pub fn pec(&mut self) -> PecW<'_, Ctrl1Spec> {
        PecW::new(self, 12)
    }
    #[doc = "Bit 13 - SMBus alert"]
    #[inline(always)]
    pub fn alerten(&mut self) -> AlertenW<'_, Ctrl1Spec> {
        AlertenW::new(self, 13)
    }
    #[doc = "Bit 15 - Software reset"]
    #[inline(always)]
    pub fn swrst(&mut self) -> SwrstW<'_, Ctrl1Spec> {
        SwrstW::new(self, 15)
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
