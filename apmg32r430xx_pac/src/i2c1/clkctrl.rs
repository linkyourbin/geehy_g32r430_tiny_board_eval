#[doc = "Register `CLKCTRL` reader"]
pub type R = crate::R<ClkctrlSpec>;
#[doc = "Register `CLKCTRL` writer"]
pub type W = crate::W<ClkctrlSpec>;
#[doc = "Field `CLKS` reader - Clock control register in Fast/Standard mode (Master mode)"]
pub type ClksR = crate::FieldReader<u16>;
#[doc = "Field `CLKS` writer - Clock control register in Fast/Standard mode (Master mode)"]
pub type ClksW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `FDUTYCFG` reader - Fast mode duty cycle"]
pub type FdutycfgR = crate::BitReader;
#[doc = "Field `FDUTYCFG` writer - Fast mode duty cycle"]
pub type FdutycfgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPEEDCFG` reader - Master Mode Speed Configure"]
pub type SpeedcfgR = crate::BitReader;
#[doc = "Field `SPEEDCFG` writer - Master Mode Speed Configure"]
pub type SpeedcfgW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:11 - Clock control register in Fast/Standard mode (Master mode)"]
    #[inline(always)]
    pub fn clks(&self) -> ClksR {
        ClksR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bit 14 - Fast mode duty cycle"]
    #[inline(always)]
    pub fn fdutycfg(&self) -> FdutycfgR {
        FdutycfgR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Master Mode Speed Configure"]
    #[inline(always)]
    pub fn speedcfg(&self) -> SpeedcfgR {
        SpeedcfgR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:11 - Clock control register in Fast/Standard mode (Master mode)"]
    #[inline(always)]
    pub fn clks(&mut self) -> ClksW<'_, ClkctrlSpec> {
        ClksW::new(self, 0)
    }
    #[doc = "Bit 14 - Fast mode duty cycle"]
    #[inline(always)]
    pub fn fdutycfg(&mut self) -> FdutycfgW<'_, ClkctrlSpec> {
        FdutycfgW::new(self, 14)
    }
    #[doc = "Bit 15 - Master Mode Speed Configure"]
    #[inline(always)]
    pub fn speedcfg(&mut self) -> SpeedcfgW<'_, ClkctrlSpec> {
        SpeedcfgW::new(self, 15)
    }
}
#[doc = "Clock control register\n\nYou can [`read`](crate::Reg::read) this register and get [`clkctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clkctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkctrlSpec;
impl crate::RegisterSpec for ClkctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clkctrl::R`](R) reader structure"]
impl crate::Readable for ClkctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`clkctrl::W`](W) writer structure"]
impl crate::Writable for ClkctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CLKCTRL to value 0"]
impl crate::Resettable for ClkctrlSpec {}
