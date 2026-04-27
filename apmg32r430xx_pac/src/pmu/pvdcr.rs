#[doc = "Register `PVDCR` reader"]
pub type R = crate::R<PvdcrSpec>;
#[doc = "Register `PVDCR` writer"]
pub type W = crate::W<PvdcrSpec>;
#[doc = "Field `PVDEN` reader - PVD Enable Control Bit"]
pub type PvdenR = crate::BitReader;
#[doc = "Field `PVDEN` writer - PVD Enable Control Bit"]
pub type PvdenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PVDTHSEL` reader - The detection threshold switch of the programmable voltage detector ranges from a minimum of 2.15V to a maximum of 2.98V, with a default of 000."]
pub type PvdthselR = crate::FieldReader;
#[doc = "Field `PVDTHSEL` writer - The detection threshold switch of the programmable voltage detector ranges from a minimum of 2.15V to a maximum of 2.98V, with a default of 000."]
pub type PvdthselW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bit 4 - PVD Enable Control Bit"]
    #[inline(always)]
    pub fn pvden(&self) -> PvdenR {
        PvdenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:7 - The detection threshold switch of the programmable voltage detector ranges from a minimum of 2.15V to a maximum of 2.98V, with a default of 000."]
    #[inline(always)]
    pub fn pvdthsel(&self) -> PvdthselR {
        PvdthselR::new(((self.bits >> 5) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 4 - PVD Enable Control Bit"]
    #[inline(always)]
    pub fn pvden(&mut self) -> PvdenW<'_, PvdcrSpec> {
        PvdenW::new(self, 4)
    }
    #[doc = "Bits 5:7 - The detection threshold switch of the programmable voltage detector ranges from a minimum of 2.15V to a maximum of 2.98V, with a default of 000."]
    #[inline(always)]
    pub fn pvdthsel(&mut self) -> PvdthselW<'_, PvdcrSpec> {
        PvdthselW::new(self, 5)
    }
}
#[doc = "PVD Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pvdcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pvdcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PvdcrSpec;
impl crate::RegisterSpec for PvdcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pvdcr::R`](R) reader structure"]
impl crate::Readable for PvdcrSpec {}
#[doc = "`write(|w| ..)` method takes [`pvdcr::W`](W) writer structure"]
impl crate::Writable for PvdcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PVDCR to value 0"]
impl crate::Resettable for PvdcrSpec {}
