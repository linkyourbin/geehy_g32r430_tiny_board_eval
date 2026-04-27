#[doc = "Register `SR` reader"]
pub type R = crate::R<SrSpec>;
#[doc = "Register `SR` writer"]
pub type W = crate::W<SrSpec>;
#[doc = "Field `RDY` reader - Sampling data valid flag, set to 1 by the hardware when the sampling data is ready. After reading the TS_DR register, this bit is automatically cleared by the hardware."]
pub type RdyR = crate::BitReader;
#[doc = "Field `RDY` writer - Sampling data valid flag, set to 1 by the hardware when the sampling data is ready. After reading the TS_DR register, this bit is automatically cleared by the hardware."]
pub type RdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVR` reader - Overflow Flag Bit When the RDY flag bit is 1, new data is generated, and the hardware sets this bit to 1. Writing 1 to this bit in the software can clear this bit."]
pub type OvrR = crate::BitReader;
#[doc = "Field `OVR` writer - Overflow Flag Bit When the RDY flag bit is 1, new data is generated, and the hardware sets this bit to 1. Writing 1 to this bit in the software can clear this bit."]
pub type OvrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Sampling data valid flag, set to 1 by the hardware when the sampling data is ready. After reading the TS_DR register, this bit is automatically cleared by the hardware."]
    #[inline(always)]
    pub fn rdy(&self) -> RdyR {
        RdyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Overflow Flag Bit When the RDY flag bit is 1, new data is generated, and the hardware sets this bit to 1. Writing 1 to this bit in the software can clear this bit."]
    #[inline(always)]
    pub fn ovr(&self) -> OvrR {
        OvrR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Sampling data valid flag, set to 1 by the hardware when the sampling data is ready. After reading the TS_DR register, this bit is automatically cleared by the hardware."]
    #[inline(always)]
    pub fn rdy(&mut self) -> RdyW<'_, SrSpec> {
        RdyW::new(self, 0)
    }
    #[doc = "Bit 1 - Overflow Flag Bit When the RDY flag bit is 1, new data is generated, and the hardware sets this bit to 1. Writing 1 to this bit in the software can clear this bit."]
    #[inline(always)]
    pub fn ovr(&mut self) -> OvrW<'_, SrSpec> {
        OvrW::new(self, 1)
    }
}
#[doc = "Status register\n\nYou can [`read`](crate::Reg::read) this register and get [`sr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SrSpec;
impl crate::RegisterSpec for SrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr::R`](R) reader structure"]
impl crate::Readable for SrSpec {}
#[doc = "`write(|w| ..)` method takes [`sr::W`](W) writer structure"]
impl crate::Writable for SrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SR to value 0"]
impl crate::Resettable for SrSpec {}
