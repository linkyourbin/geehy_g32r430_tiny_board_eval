#[doc = "Register `STS2` reader"]
pub type R = crate::R<Sts2Spec>;
#[doc = "Field `MSFLG` reader - Master Slave Mode Flag"]
pub type MsflgR = crate::BitReader;
#[doc = "Field `BUSBSYFLG` reader - Bus busy"]
pub type BusbsyflgR = crate::BitReader;
#[doc = "Field `TRFLG` reader - Transmitter/receiver"]
pub type TrflgR = crate::BitReader;
#[doc = "Field `GENCALLFLG` reader - General call address (Slave mode)"]
pub type GencallflgR = crate::BitReader;
#[doc = "Field `SMBDADDRFLG` reader - SMBus device default address (Slave mode)"]
pub type SmbdaddrflgR = crate::BitReader;
#[doc = "Field `SMMHADDR` reader - SMBus host header (Slave mode)"]
pub type SmmhaddrR = crate::BitReader;
#[doc = "Field `DUALADDRFLG` reader - Dual flag (Slave mode)"]
pub type DualaddrflgR = crate::BitReader;
#[doc = "Field `PECVALUE` reader - Save Packet Error Checking Value"]
pub type PecvalueR = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - Master Slave Mode Flag"]
    #[inline(always)]
    pub fn msflg(&self) -> MsflgR {
        MsflgR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Bus busy"]
    #[inline(always)]
    pub fn busbsyflg(&self) -> BusbsyflgR {
        BusbsyflgR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Transmitter/receiver"]
    #[inline(always)]
    pub fn trflg(&self) -> TrflgR {
        TrflgR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - General call address (Slave mode)"]
    #[inline(always)]
    pub fn gencallflg(&self) -> GencallflgR {
        GencallflgR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - SMBus device default address (Slave mode)"]
    #[inline(always)]
    pub fn smbdaddrflg(&self) -> SmbdaddrflgR {
        SmbdaddrflgR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - SMBus host header (Slave mode)"]
    #[inline(always)]
    pub fn smmhaddr(&self) -> SmmhaddrR {
        SmmhaddrR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Dual flag (Slave mode)"]
    #[inline(always)]
    pub fn dualaddrflg(&self) -> DualaddrflgR {
        DualaddrflgR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:15 - Save Packet Error Checking Value"]
    #[inline(always)]
    pub fn pecvalue(&self) -> PecvalueR {
        PecvalueR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
#[doc = "Status register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`sts2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sts2Spec;
impl crate::RegisterSpec for Sts2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sts2::R`](R) reader structure"]
impl crate::Readable for Sts2Spec {}
#[doc = "`reset()` method sets STS2 to value 0"]
impl crate::Resettable for Sts2Spec {}
