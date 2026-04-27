#[doc = "Register `AWD2CR` reader"]
pub type R = crate::R<Awd2crSpec>;
#[doc = "Register `AWD2CR` writer"]
pub type W = crate::W<Awd2crSpec>;
#[doc = "Field `AWD2CH` reader - ANALOG WATCHDOG 2 CHANNEL SELECTION"]
pub type Awd2chR = crate::FieldReader;
#[doc = "Field `AWD2CH` writer - ANALOG WATCHDOG 2 CHANNEL SELECTION"]
pub type Awd2chW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - ANALOG WATCHDOG 2 CHANNEL SELECTION"]
    #[inline(always)]
    pub fn awd2ch(&self) -> Awd2chR {
        Awd2chR::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - ANALOG WATCHDOG 2 CHANNEL SELECTION"]
    #[inline(always)]
    pub fn awd2ch(&mut self) -> Awd2chW<'_, Awd2crSpec> {
        Awd2chW::new(self, 0)
    }
}
#[doc = "Simulation Watchdog 2 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`awd2cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`awd2cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Awd2crSpec;
impl crate::RegisterSpec for Awd2crSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`awd2cr::R`](R) reader structure"]
impl crate::Readable for Awd2crSpec {}
#[doc = "`write(|w| ..)` method takes [`awd2cr::W`](W) writer structure"]
impl crate::Writable for Awd2crSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AWD2CR to value 0"]
impl crate::Resettable for Awd2crSpec {}
