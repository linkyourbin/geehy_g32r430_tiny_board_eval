#[doc = "Register `CR` reader"]
pub type R = crate::R<CrSpec>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CrSpec>;
#[doc = "Field `EN` reader - DAC channel1 enable"]
pub type EnR = crate::BitReader;
#[doc = "Field `EN` writer - DAC channel1 enable"]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSEL` reader - DAC channel trigger selection"]
pub type TselR = crate::FieldReader;
#[doc = "Field `TSEL` writer - DAC channel trigger selection"]
pub type TselW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `WAVE` reader - DAC channel noise/triangle wave generation enable"]
pub type WaveR = crate::FieldReader;
#[doc = "Field `WAVE` writer - DAC channel noise/triangle wave generation enable"]
pub type WaveW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MAMP` reader - DAC channel mute/amplitude selector"]
pub type MampR = crate::FieldReader;
#[doc = "Field `MAMP` writer - DAC channel mute/amplitude selector"]
pub type MampW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `EN_RDAC` reader - Switch to control the DAC resistor ladder output signal to the op-amp input."]
pub type EnRdacR = crate::FieldReader;
#[doc = "Field `EN_RDAC` writer - Switch to control the DAC resistor ladder output signal to the op-amp input."]
pub type EnRdacW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `EXT_SEL` reader - BUFFER output connected to the PAD"]
pub type ExtSelR = crate::BitReader;
#[doc = "Field `EXT_SEL` writer - BUFFER output connected to the PAD"]
pub type ExtSelW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - DAC channel1 enable"]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 2:5 - DAC channel trigger selection"]
    #[inline(always)]
    pub fn tsel(&self) -> TselR {
        TselR::new(((self.bits >> 2) & 0x0f) as u8)
    }
    #[doc = "Bits 6:7 - DAC channel noise/triangle wave generation enable"]
    #[inline(always)]
    pub fn wave(&self) -> WaveR {
        WaveR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:11 - DAC channel mute/amplitude selector"]
    #[inline(always)]
    pub fn mamp(&self) -> MampR {
        MampR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:13 - Switch to control the DAC resistor ladder output signal to the op-amp input."]
    #[inline(always)]
    pub fn en_rdac(&self) -> EnRdacR {
        EnRdacR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - BUFFER output connected to the PAD"]
    #[inline(always)]
    pub fn ext_sel(&self) -> ExtSelR {
        ExtSelR::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DAC channel1 enable"]
    #[inline(always)]
    pub fn en(&mut self) -> EnW<'_, CrSpec> {
        EnW::new(self, 0)
    }
    #[doc = "Bits 2:5 - DAC channel trigger selection"]
    #[inline(always)]
    pub fn tsel(&mut self) -> TselW<'_, CrSpec> {
        TselW::new(self, 2)
    }
    #[doc = "Bits 6:7 - DAC channel noise/triangle wave generation enable"]
    #[inline(always)]
    pub fn wave(&mut self) -> WaveW<'_, CrSpec> {
        WaveW::new(self, 6)
    }
    #[doc = "Bits 8:11 - DAC channel mute/amplitude selector"]
    #[inline(always)]
    pub fn mamp(&mut self) -> MampW<'_, CrSpec> {
        MampW::new(self, 8)
    }
    #[doc = "Bits 12:13 - Switch to control the DAC resistor ladder output signal to the op-amp input."]
    #[inline(always)]
    pub fn en_rdac(&mut self) -> EnRdacW<'_, CrSpec> {
        EnRdacW::new(self, 12)
    }
    #[doc = "Bit 14 - BUFFER output connected to the PAD"]
    #[inline(always)]
    pub fn ext_sel(&mut self) -> ExtSelW<'_, CrSpec> {
        ExtSelW::new(self, 14)
    }
}
#[doc = "Control register (DAC_CTRL)\n\nYou can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrSpec;
impl crate::RegisterSpec for CrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr::R`](R) reader structure"]
impl crate::Readable for CrSpec {}
#[doc = "`write(|w| ..)` method takes [`cr::W`](W) writer structure"]
impl crate::Writable for CrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CrSpec {}
