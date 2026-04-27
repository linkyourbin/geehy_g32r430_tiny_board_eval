#[doc = "Register `BCFG` reader"]
pub type R = crate::R<BcfgSpec>;
#[doc = "Register `BCFG` writer"]
pub type W = crate::W<BcfgSpec>;
#[doc = "Field `FREQ_CLKNUM` reader - FREQ_CLKNUM"]
pub type FreqClknumR = crate::FieldReader;
#[doc = "Field `FREQ_CLKNUM` writer - FREQ_CLKNUM"]
pub type FreqClknumW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `FREQ_TIMEOUTNUM` reader - FREQ_TIMEOUTNUM"]
pub type FreqTimeoutnumR = crate::FieldReader;
#[doc = "Field `FREQ_TIMEOUTNUM` writer - FREQ_TIMEOUTNUM"]
pub type FreqTimeoutnumW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:7 - FREQ_CLKNUM"]
    #[inline(always)]
    pub fn freq_clknum(&self) -> FreqClknumR {
        FreqClknumR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:13 - FREQ_TIMEOUTNUM"]
    #[inline(always)]
    pub fn freq_timeoutnum(&self) -> FreqTimeoutnumR {
        FreqTimeoutnumR::new(((self.bits >> 8) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - FREQ_CLKNUM"]
    #[inline(always)]
    pub fn freq_clknum(&mut self) -> FreqClknumW<'_, BcfgSpec> {
        FreqClknumW::new(self, 0)
    }
    #[doc = "Bits 8:13 - FREQ_TIMEOUTNUM"]
    #[inline(always)]
    pub fn freq_timeoutnum(&mut self) -> FreqTimeoutnumW<'_, BcfgSpec> {
        FreqTimeoutnumW::new(self, 8)
    }
}
#[doc = "BISS-C Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`bcfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bcfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BcfgSpec;
impl crate::RegisterSpec for BcfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bcfg::R`](R) reader structure"]
impl crate::Readable for BcfgSpec {}
#[doc = "`write(|w| ..)` method takes [`bcfg::W`](W) writer structure"]
impl crate::Writable for BcfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BCFG to value 0"]
impl crate::Resettable for BcfgSpec {}
