#[doc = "Register `MCCR` reader"]
pub type R = crate::R<MccrSpec>;
#[doc = "Register `MCCR` writer"]
pub type W = crate::W<MccrSpec>;
#[doc = "Field `CSEN` reader - Main clock switch enable."]
pub type CsenR = crate::BitReader;
#[doc = "Field `CSEN` writer - Main clock switch enable."]
pub type CsenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWST` reader - The current status of the master clock"]
pub type SwstR = crate::FieldReader;
#[doc = "Field `SWDONE` reader - Switching completion flag."]
pub type SwdoneR = crate::BitReader;
#[doc = "Field `SWERR` reader - Toggle error flag."]
pub type SwerrR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Main clock switch enable."]
    #[inline(always)]
    pub fn csen(&self) -> CsenR {
        CsenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - The current status of the master clock"]
    #[inline(always)]
    pub fn swst(&self) -> SwstR {
        SwstR::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 3 - Switching completion flag."]
    #[inline(always)]
    pub fn swdone(&self) -> SwdoneR {
        SwdoneR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Toggle error flag."]
    #[inline(always)]
    pub fn swerr(&self) -> SwerrR {
        SwerrR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Main clock switch enable."]
    #[inline(always)]
    pub fn csen(&mut self) -> CsenW<'_, MccrSpec> {
        CsenW::new(self, 0)
    }
}
#[doc = "Main clock control register.\n\nYou can [`read`](crate::Reg::read) this register and get [`mccr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mccr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MccrSpec;
impl crate::RegisterSpec for MccrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mccr::R`](R) reader structure"]
impl crate::Readable for MccrSpec {}
#[doc = "`write(|w| ..)` method takes [`mccr::W`](W) writer structure"]
impl crate::Writable for MccrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MCCR to value 0"]
impl crate::Resettable for MccrSpec {}
