#[doc = "Register `SR` reader"]
pub type R = crate::R<SrSpec>;
#[doc = "Register `SR` writer"]
pub type W = crate::W<SrSpec>;
#[doc = "Field `ERD` reader - Erase completion"]
pub type ErdR = crate::BitReader;
#[doc = "Field `ERD` writer - Erase completion"]
pub type ErdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRD` reader - Programming completion"]
pub type PrdR = crate::BitReader;
#[doc = "Field `PRD` writer - Programming completion"]
pub type PrdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUSY` reader - Busy"]
pub type BusyR = crate::BitReader;
#[doc = "Field `KEYERR` reader - Flash KEY error"]
pub type KeyerrR = crate::BitReader;
#[doc = "Field `KEYERR` writer - Flash KEY error"]
pub type KeyerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRPRTERR` reader - Write protection error flag."]
pub type WrprterrR = crate::BitReader;
#[doc = "Field `WRPRTERR` writer - Write protection error flag."]
pub type WrprterrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WADRERR` reader - Incorrect address written."]
pub type WadrerrR = crate::BitReader;
#[doc = "Field `WADRERR` writer - Incorrect address written."]
pub type WadrerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OPBPRD` reader - Option byte programming completion flag"]
pub type OpbprdR = crate::BitReader;
#[doc = "Field `OPBPRD` writer - Option byte programming completion flag"]
pub type OpbprdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OPTERR` reader - option_byte does not pass the positive and negative code verification."]
pub type OpterrR = crate::BitReader;
#[doc = "Field `MAINUNLOCK` reader - Main program area lock flag."]
pub type MainunlockR = crate::BitReader;
#[doc = "Field `MAINUNLOCK` writer - Main program area lock flag."]
pub type MainunlockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OPUNLOCK` reader - Option byte area lock flag"]
pub type OpunlockR = crate::BitReader;
#[doc = "Field `OPUNLOCK` writer - Option byte area lock flag"]
pub type OpunlockW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Erase completion"]
    #[inline(always)]
    pub fn erd(&self) -> ErdR {
        ErdR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Programming completion"]
    #[inline(always)]
    pub fn prd(&self) -> PrdR {
        PrdR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Busy"]
    #[inline(always)]
    pub fn busy(&self) -> BusyR {
        BusyR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Flash KEY error"]
    #[inline(always)]
    pub fn keyerr(&self) -> KeyerrR {
        KeyerrR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Write protection error flag."]
    #[inline(always)]
    pub fn wrprterr(&self) -> WrprterrR {
        WrprterrR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Incorrect address written."]
    #[inline(always)]
    pub fn wadrerr(&self) -> WadrerrR {
        WadrerrR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 14 - Option byte programming completion flag"]
    #[inline(always)]
    pub fn opbprd(&self) -> OpbprdR {
        OpbprdR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - option_byte does not pass the positive and negative code verification."]
    #[inline(always)]
    pub fn opterr(&self) -> OpterrR {
        OpterrR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 17 - Main program area lock flag."]
    #[inline(always)]
    pub fn mainunlock(&self) -> MainunlockR {
        MainunlockR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Option byte area lock flag"]
    #[inline(always)]
    pub fn opunlock(&self) -> OpunlockR {
        OpunlockR::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Erase completion"]
    #[inline(always)]
    pub fn erd(&mut self) -> ErdW<'_, SrSpec> {
        ErdW::new(self, 0)
    }
    #[doc = "Bit 1 - Programming completion"]
    #[inline(always)]
    pub fn prd(&mut self) -> PrdW<'_, SrSpec> {
        PrdW::new(self, 1)
    }
    #[doc = "Bit 3 - Flash KEY error"]
    #[inline(always)]
    pub fn keyerr(&mut self) -> KeyerrW<'_, SrSpec> {
        KeyerrW::new(self, 3)
    }
    #[doc = "Bit 4 - Write protection error flag."]
    #[inline(always)]
    pub fn wrprterr(&mut self) -> WrprterrW<'_, SrSpec> {
        WrprterrW::new(self, 4)
    }
    #[doc = "Bit 5 - Incorrect address written."]
    #[inline(always)]
    pub fn wadrerr(&mut self) -> WadrerrW<'_, SrSpec> {
        WadrerrW::new(self, 5)
    }
    #[doc = "Bit 14 - Option byte programming completion flag"]
    #[inline(always)]
    pub fn opbprd(&mut self) -> OpbprdW<'_, SrSpec> {
        OpbprdW::new(self, 14)
    }
    #[doc = "Bit 17 - Main program area lock flag."]
    #[inline(always)]
    pub fn mainunlock(&mut self) -> MainunlockW<'_, SrSpec> {
        MainunlockW::new(self, 17)
    }
    #[doc = "Bit 18 - Option byte area lock flag"]
    #[inline(always)]
    pub fn opunlock(&mut self) -> OpunlockW<'_, SrSpec> {
        OpunlockW::new(self, 18)
    }
}
#[doc = "status register\n\nYou can [`read`](crate::Reg::read) this register and get [`sr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
