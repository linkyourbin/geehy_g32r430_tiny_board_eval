#[doc = "Register `TMCR` reader"]
pub type R = crate::R<TmcrSpec>;
#[doc = "Register `TMCR` writer"]
pub type W = crate::W<TmcrSpec>;
#[doc = "Field `UNIT` reader - Flash/EEPROM 1us Unit Counter"]
pub type UnitR = crate::FieldReader;
#[doc = "Field `UNIT` writer - Flash/EEPROM 1us Unit Counter"]
pub type UnitW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Flash/EEPROM 1us Unit Counter"]
    #[inline(always)]
    pub fn unit(&self) -> UnitR {
        UnitR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Flash/EEPROM 1us Unit Counter"]
    #[inline(always)]
    pub fn unit(&mut self) -> UnitW<'_, TmcrSpec> {
        UnitW::new(self, 0)
    }
}
#[doc = "TMCR\n\nYou can [`read`](crate::Reg::read) this register and get [`tmcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tmcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TmcrSpec;
impl crate::RegisterSpec for TmcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tmcr::R`](R) reader structure"]
impl crate::Readable for TmcrSpec {}
#[doc = "`write(|w| ..)` method takes [`tmcr::W`](W) writer structure"]
impl crate::Writable for TmcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TMCR to value 0xff"]
impl crate::Resettable for TmcrSpec {
    const RESET_VALUE: u32 = 0xff;
}
