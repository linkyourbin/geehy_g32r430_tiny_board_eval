#[doc = "Register `EVSCR` reader"]
pub type R = crate::R<EvscrSpec>;
#[doc = "Register `EVSCR` writer"]
pub type W = crate::W<EvscrSpec>;
#[doc = "Field `VDCPVDSEL` reader - VDC-VPD Threshold Selection"]
pub type VdcpvdselR = crate::FieldReader;
#[doc = "Field `VDCPVDSEL` writer - VDC-VPD Threshold Selection"]
pub type VdcpvdselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `VDCPVDFILTER` reader - EVS Pin Filter Configuration"]
pub type VdcpvdfilterR = crate::FieldReader;
#[doc = "Field `VDCPVDFILTER` writer - EVS Pin Filter Configuration"]
pub type VdcpvdfilterW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `VDCPVDEN` reader - EVS Pin Wake-up Enable"]
pub type VdcpvdenR = crate::BitReader;
#[doc = "Field `VDCPVDEN` writer - EVS Pin Wake-up Enable"]
pub type VdcpvdenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VDCPVDWKPEN` reader - EVS Pin STANDBY Mode Wake-Up Enable"]
pub type VdcpvdwkpenR = crate::BitReader;
#[doc = "Field `VDCPVDWKPEN` writer - EVS Pin STANDBY Mode Wake-Up Enable"]
pub type VdcpvdwkpenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - VDC-VPD Threshold Selection"]
    #[inline(always)]
    pub fn vdcpvdsel(&self) -> VdcpvdselR {
        VdcpvdselR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - EVS Pin Filter Configuration"]
    #[inline(always)]
    pub fn vdcpvdfilter(&self) -> VdcpvdfilterR {
        VdcpvdfilterR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - EVS Pin Wake-up Enable"]
    #[inline(always)]
    pub fn vdcpvden(&self) -> VdcpvdenR {
        VdcpvdenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - EVS Pin STANDBY Mode Wake-Up Enable"]
    #[inline(always)]
    pub fn vdcpvdwkpen(&self) -> VdcpvdwkpenR {
        VdcpvdwkpenR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - VDC-VPD Threshold Selection"]
    #[inline(always)]
    pub fn vdcpvdsel(&mut self) -> VdcpvdselW<'_, EvscrSpec> {
        VdcpvdselW::new(self, 0)
    }
    #[doc = "Bits 2:3 - EVS Pin Filter Configuration"]
    #[inline(always)]
    pub fn vdcpvdfilter(&mut self) -> VdcpvdfilterW<'_, EvscrSpec> {
        VdcpvdfilterW::new(self, 2)
    }
    #[doc = "Bit 4 - EVS Pin Wake-up Enable"]
    #[inline(always)]
    pub fn vdcpvden(&mut self) -> VdcpvdenW<'_, EvscrSpec> {
        VdcpvdenW::new(self, 4)
    }
    #[doc = "Bit 5 - EVS Pin STANDBY Mode Wake-Up Enable"]
    #[inline(always)]
    pub fn vdcpvdwkpen(&mut self) -> VdcpvdwkpenW<'_, EvscrSpec> {
        VdcpvdwkpenW::new(self, 5)
    }
}
#[doc = "Power-off Wake-up Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`evscr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`evscr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EvscrSpec;
impl crate::RegisterSpec for EvscrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`evscr::R`](R) reader structure"]
impl crate::Readable for EvscrSpec {}
#[doc = "`write(|w| ..)` method takes [`evscr::W`](W) writer structure"]
impl crate::Writable for EvscrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVSCR to value 0"]
impl crate::Resettable for EvscrSpec {}
