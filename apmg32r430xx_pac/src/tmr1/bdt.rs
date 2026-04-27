#[doc = "Register `BDT` reader"]
pub type R = crate::R<BdtSpec>;
#[doc = "Register `BDT` writer"]
pub type W = crate::W<BdtSpec>;
#[doc = "Field `DTS` reader - Dead-time generator setup"]
pub type DtsR = crate::FieldReader;
#[doc = "Field `DTS` writer - Dead-time generator setup"]
pub type DtsW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `LOCKCFG` reader - Lock configuration"]
pub type LockcfgR = crate::FieldReader;
#[doc = "Field `LOCKCFG` writer - Lock configuration"]
pub type LockcfgW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IMOS` reader - Off-state selection for Idle mode"]
pub type ImosR = crate::BitReader;
#[doc = "Field `IMOS` writer - Off-state selection for Idle mode"]
pub type ImosW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RMOS` reader - Off-state selection for Run mode"]
pub type RmosR = crate::BitReader;
#[doc = "Field `RMOS` writer - Off-state selection for Run mode"]
pub type RmosW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BRKEN` reader - Break enable"]
pub type BrkenR = crate::BitReader;
#[doc = "Field `BRKEN` writer - Break enable"]
pub type BrkenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BRKPOL` reader - Break polarity"]
pub type BrkpolR = crate::BitReader;
#[doc = "Field `BRKPOL` writer - Break polarity"]
pub type BrkpolW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AOEN` reader - Automatic output enable"]
pub type AoenR = crate::BitReader;
#[doc = "Field `AOEN` writer - Automatic output enable"]
pub type AoenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MOEN` reader - Main output enable"]
pub type MoenR = crate::BitReader;
#[doc = "Field `MOEN` writer - Main output enable"]
pub type MoenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - Dead-time generator setup"]
    #[inline(always)]
    pub fn dts(&self) -> DtsR {
        DtsR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:9 - Lock configuration"]
    #[inline(always)]
    pub fn lockcfg(&self) -> LockcfgR {
        LockcfgR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - Off-state selection for Idle mode"]
    #[inline(always)]
    pub fn imos(&self) -> ImosR {
        ImosR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Off-state selection for Run mode"]
    #[inline(always)]
    pub fn rmos(&self) -> RmosR {
        RmosR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Break enable"]
    #[inline(always)]
    pub fn brken(&self) -> BrkenR {
        BrkenR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Break polarity"]
    #[inline(always)]
    pub fn brkpol(&self) -> BrkpolR {
        BrkpolR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Automatic output enable"]
    #[inline(always)]
    pub fn aoen(&self) -> AoenR {
        AoenR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Main output enable"]
    #[inline(always)]
    pub fn moen(&self) -> MoenR {
        MoenR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Dead-time generator setup"]
    #[inline(always)]
    pub fn dts(&mut self) -> DtsW<'_, BdtSpec> {
        DtsW::new(self, 0)
    }
    #[doc = "Bits 8:9 - Lock configuration"]
    #[inline(always)]
    pub fn lockcfg(&mut self) -> LockcfgW<'_, BdtSpec> {
        LockcfgW::new(self, 8)
    }
    #[doc = "Bit 10 - Off-state selection for Idle mode"]
    #[inline(always)]
    pub fn imos(&mut self) -> ImosW<'_, BdtSpec> {
        ImosW::new(self, 10)
    }
    #[doc = "Bit 11 - Off-state selection for Run mode"]
    #[inline(always)]
    pub fn rmos(&mut self) -> RmosW<'_, BdtSpec> {
        RmosW::new(self, 11)
    }
    #[doc = "Bit 12 - Break enable"]
    #[inline(always)]
    pub fn brken(&mut self) -> BrkenW<'_, BdtSpec> {
        BrkenW::new(self, 12)
    }
    #[doc = "Bit 13 - Break polarity"]
    #[inline(always)]
    pub fn brkpol(&mut self) -> BrkpolW<'_, BdtSpec> {
        BrkpolW::new(self, 13)
    }
    #[doc = "Bit 14 - Automatic output enable"]
    #[inline(always)]
    pub fn aoen(&mut self) -> AoenW<'_, BdtSpec> {
        AoenW::new(self, 14)
    }
    #[doc = "Bit 15 - Main output enable"]
    #[inline(always)]
    pub fn moen(&mut self) -> MoenW<'_, BdtSpec> {
        MoenW::new(self, 15)
    }
}
#[doc = "break and dead-time register\n\nYou can [`read`](crate::Reg::read) this register and get [`bdt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bdt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BdtSpec;
impl crate::RegisterSpec for BdtSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bdt::R`](R) reader structure"]
impl crate::Readable for BdtSpec {}
#[doc = "`write(|w| ..)` method takes [`bdt::W`](W) writer structure"]
impl crate::Writable for BdtSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BDT to value 0"]
impl crate::Resettable for BdtSpec {}
