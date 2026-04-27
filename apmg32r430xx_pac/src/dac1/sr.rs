#[doc = "Register `SR` reader"]
pub type R = crate::R<SrSpec>;
#[doc = "Register `SR` writer"]
pub type W = crate::W<SrSpec>;
#[doc = "Field `DOWN_OW_FLAG` reader - DOR Data Underflow Flag"]
pub type DownOwFlagR = crate::BitReader;
#[doc = "Field `DOWN_OW_FLAG` writer - DOR Data Underflow Flag"]
pub type DownOwFlagW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UP_OW_FLAG` reader - DOR Data Upflow Flag"]
pub type UpOwFlagR = crate::BitReader;
#[doc = "Field `UP_OW_FLAG` writer - DOR Data Upflow Flag"]
pub type UpOwFlagW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - DOR Data Underflow Flag"]
    #[inline(always)]
    pub fn down_ow_flag(&self) -> DownOwFlagR {
        DownOwFlagR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DOR Data Upflow Flag"]
    #[inline(always)]
    pub fn up_ow_flag(&self) -> UpOwFlagR {
        UpOwFlagR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DOR Data Underflow Flag"]
    #[inline(always)]
    pub fn down_ow_flag(&mut self) -> DownOwFlagW<'_, SrSpec> {
        DownOwFlagW::new(self, 0)
    }
    #[doc = "Bit 1 - DOR Data Upflow Flag"]
    #[inline(always)]
    pub fn up_ow_flag(&mut self) -> UpOwFlagW<'_, SrSpec> {
        UpOwFlagW::new(self, 1)
    }
}
#[doc = "DAC status register\n\nYou can [`read`](crate::Reg::read) this register and get [`sr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
