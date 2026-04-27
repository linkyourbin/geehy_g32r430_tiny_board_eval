#[doc = "Register `CTRL2` reader"]
pub type R = crate::R<Ctrl2Spec>;
#[doc = "Register `CTRL2` writer"]
pub type W = crate::W<Ctrl2Spec>;
#[doc = "Field `CCPEN` reader - Capture/compare preloaded control"]
pub type CcpenR = crate::BitReader;
#[doc = "Field `CCPEN` writer - Capture/compare preloaded control"]
pub type CcpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCUSEL` reader - Capture/compare control update selection"]
pub type CcuselR = crate::BitReader;
#[doc = "Field `CCUSEL` writer - Capture/compare control update selection"]
pub type CcuselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCDSEL` reader - Capture/compare DMA selection"]
pub type CcdselR = crate::BitReader;
#[doc = "Field `CCDSEL` writer - Capture/compare DMA selection"]
pub type CcdselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MMSEL` reader - Master mode selection"]
pub type MmselR = crate::FieldReader;
#[doc = "Field `MMSEL` writer - Master mode selection"]
pub type MmselW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `TI1SEL` reader - TI1 selection"]
pub type Ti1selR = crate::BitReader;
#[doc = "Field `TI1SEL` writer - TI1 selection"]
pub type Ti1selW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OC1OIS` reader - Output Idle state 1"]
pub type Oc1oisR = crate::BitReader;
#[doc = "Field `OC1OIS` writer - Output Idle state 1"]
pub type Oc1oisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OC1NOIS` reader - Output Idle state 1"]
pub type Oc1noisR = crate::BitReader;
#[doc = "Field `OC1NOIS` writer - Output Idle state 1"]
pub type Oc1noisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OC2OIS` reader - Output Idle state 2"]
pub type Oc2oisR = crate::BitReader;
#[doc = "Field `OC2OIS` writer - Output Idle state 2"]
pub type Oc2oisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OC2NOIS` reader - Output Idle state 2"]
pub type Oc2noisR = crate::BitReader;
#[doc = "Field `OC2NOIS` writer - Output Idle state 2"]
pub type Oc2noisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OC3OIS` reader - Output Idle state 3"]
pub type Oc3oisR = crate::BitReader;
#[doc = "Field `OC3OIS` writer - Output Idle state 3"]
pub type Oc3oisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OC3NOIS` reader - Output Idle state 3"]
pub type Oc3noisR = crate::BitReader;
#[doc = "Field `OC3NOIS` writer - Output Idle state 3"]
pub type Oc3noisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OC4OIS` reader - Output Idle state 4"]
pub type Oc4oisR = crate::BitReader;
#[doc = "Field `OC4OIS` writer - Output Idle state 4"]
pub type Oc4oisW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Capture/compare preloaded control"]
    #[inline(always)]
    pub fn ccpen(&self) -> CcpenR {
        CcpenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Capture/compare control update selection"]
    #[inline(always)]
    pub fn ccusel(&self) -> CcuselR {
        CcuselR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Capture/compare DMA selection"]
    #[inline(always)]
    pub fn ccdsel(&self) -> CcdselR {
        CcdselR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - Master mode selection"]
    #[inline(always)]
    pub fn mmsel(&self) -> MmselR {
        MmselR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - TI1 selection"]
    #[inline(always)]
    pub fn ti1sel(&self) -> Ti1selR {
        Ti1selR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Output Idle state 1"]
    #[inline(always)]
    pub fn oc1ois(&self) -> Oc1oisR {
        Oc1oisR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Output Idle state 1"]
    #[inline(always)]
    pub fn oc1nois(&self) -> Oc1noisR {
        Oc1noisR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Output Idle state 2"]
    #[inline(always)]
    pub fn oc2ois(&self) -> Oc2oisR {
        Oc2oisR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Output Idle state 2"]
    #[inline(always)]
    pub fn oc2nois(&self) -> Oc2noisR {
        Oc2noisR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Output Idle state 3"]
    #[inline(always)]
    pub fn oc3ois(&self) -> Oc3oisR {
        Oc3oisR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Output Idle state 3"]
    #[inline(always)]
    pub fn oc3nois(&self) -> Oc3noisR {
        Oc3noisR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Output Idle state 4"]
    #[inline(always)]
    pub fn oc4ois(&self) -> Oc4oisR {
        Oc4oisR::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Capture/compare preloaded control"]
    #[inline(always)]
    pub fn ccpen(&mut self) -> CcpenW<'_, Ctrl2Spec> {
        CcpenW::new(self, 0)
    }
    #[doc = "Bit 2 - Capture/compare control update selection"]
    #[inline(always)]
    pub fn ccusel(&mut self) -> CcuselW<'_, Ctrl2Spec> {
        CcuselW::new(self, 2)
    }
    #[doc = "Bit 3 - Capture/compare DMA selection"]
    #[inline(always)]
    pub fn ccdsel(&mut self) -> CcdselW<'_, Ctrl2Spec> {
        CcdselW::new(self, 3)
    }
    #[doc = "Bits 4:6 - Master mode selection"]
    #[inline(always)]
    pub fn mmsel(&mut self) -> MmselW<'_, Ctrl2Spec> {
        MmselW::new(self, 4)
    }
    #[doc = "Bit 7 - TI1 selection"]
    #[inline(always)]
    pub fn ti1sel(&mut self) -> Ti1selW<'_, Ctrl2Spec> {
        Ti1selW::new(self, 7)
    }
    #[doc = "Bit 8 - Output Idle state 1"]
    #[inline(always)]
    pub fn oc1ois(&mut self) -> Oc1oisW<'_, Ctrl2Spec> {
        Oc1oisW::new(self, 8)
    }
    #[doc = "Bit 9 - Output Idle state 1"]
    #[inline(always)]
    pub fn oc1nois(&mut self) -> Oc1noisW<'_, Ctrl2Spec> {
        Oc1noisW::new(self, 9)
    }
    #[doc = "Bit 10 - Output Idle state 2"]
    #[inline(always)]
    pub fn oc2ois(&mut self) -> Oc2oisW<'_, Ctrl2Spec> {
        Oc2oisW::new(self, 10)
    }
    #[doc = "Bit 11 - Output Idle state 2"]
    #[inline(always)]
    pub fn oc2nois(&mut self) -> Oc2noisW<'_, Ctrl2Spec> {
        Oc2noisW::new(self, 11)
    }
    #[doc = "Bit 12 - Output Idle state 3"]
    #[inline(always)]
    pub fn oc3ois(&mut self) -> Oc3oisW<'_, Ctrl2Spec> {
        Oc3oisW::new(self, 12)
    }
    #[doc = "Bit 13 - Output Idle state 3"]
    #[inline(always)]
    pub fn oc3nois(&mut self) -> Oc3noisW<'_, Ctrl2Spec> {
        Oc3noisW::new(self, 13)
    }
    #[doc = "Bit 14 - Output Idle state 4"]
    #[inline(always)]
    pub fn oc4ois(&mut self) -> Oc4oisW<'_, Ctrl2Spec> {
        Oc4oisW::new(self, 14)
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
