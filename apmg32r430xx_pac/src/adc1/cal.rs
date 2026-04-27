#[doc = "Register `CAL` reader"]
pub type R = crate::R<CalSpec>;
#[doc = "Register `CAL` writer"]
pub type W = crate::W<CalSpec>;
#[doc = "Field `REF_SEL` reader - ADC REFERENCE Voltage Selection 1.65/AVDD Control Signal"]
pub type RefSelR = crate::BitReader;
#[doc = "Field `REF_SEL` writer - ADC REFERENCE Voltage Selection 1.65/AVDD Control Signal"]
pub type RefSelW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 3 - ADC REFERENCE Voltage Selection 1.65/AVDD Control Signal"]
    #[inline(always)]
    pub fn ref_sel(&self) -> RefSelR {
        RefSelR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - ADC REFERENCE Voltage Selection 1.65/AVDD Control Signal"]
    #[inline(always)]
    pub fn ref_sel(&mut self) -> RefSelW<'_, CalSpec> {
        RefSelW::new(self, 3)
    }
}
#[doc = "ADC Calibration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cal::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cal::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CalSpec;
impl crate::RegisterSpec for CalSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cal::R`](R) reader structure"]
impl crate::Readable for CalSpec {}
#[doc = "`write(|w| ..)` method takes [`cal::W`](W) writer structure"]
impl crate::Writable for CalSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CAL to value 0x02"]
impl crate::Resettable for CalSpec {
    const RESET_VALUE: u32 = 0x02;
}
