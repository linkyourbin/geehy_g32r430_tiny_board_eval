#[doc = "Register `SCFG7` reader"]
pub type R = crate::R<Scfg7Spec>;
#[doc = "Register `SCFG7` writer"]
pub type W = crate::W<Scfg7Spec>;
#[doc = "Field `EN` reader - Stream enable / flag stream ready when read low"]
pub type EnR = crate::BitReader;
#[doc = "Field `EN` writer - Stream enable / flag stream ready when read low"]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMEIEN` reader - Direct mode error interrupt enable"]
pub type DmeienR = crate::BitReader;
#[doc = "Field `DMEIEN` writer - Direct mode error interrupt enable"]
pub type DmeienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXEIEN` reader - Transfer error interrupt enable"]
pub type TxeienR = crate::BitReader;
#[doc = "Field `TXEIEN` writer - Transfer error interrupt enable"]
pub type TxeienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HTXIEN` reader - Half transfer interrupt enable"]
pub type HtxienR = crate::BitReader;
#[doc = "Field `HTXIEN` writer - Half transfer interrupt enable"]
pub type HtxienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXCIEN` reader - Transfer complete interrupt enable"]
pub type TxcienR = crate::BitReader;
#[doc = "Field `TXCIEN` writer - Transfer complete interrupt enable"]
pub type TxcienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PERFC` reader - Peripheral flow controller"]
pub type PerfcR = crate::BitReader;
#[doc = "Field `PERFC` writer - Peripheral flow controller"]
pub type PerfcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIRCFG` reader - Data transfer direction"]
pub type DircfgR = crate::FieldReader;
#[doc = "Field `DIRCFG` writer - Data transfer direction"]
pub type DircfgW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CIRCMEN` reader - Circular mode"]
pub type CircmenR = crate::BitReader;
#[doc = "Field `CIRCMEN` writer - Circular mode"]
pub type CircmenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PERIM` reader - Peripheral increment mode"]
pub type PerimR = crate::BitReader;
#[doc = "Field `PERIM` writer - Peripheral increment mode"]
pub type PerimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MEMIM` reader - Memory increment mode"]
pub type MemimR = crate::BitReader;
#[doc = "Field `MEMIM` writer - Memory increment mode"]
pub type MemimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PERSIZECFG` reader - Peripheral data size"]
pub type PersizecfgR = crate::FieldReader;
#[doc = "Field `PERSIZECFG` writer - Peripheral data size"]
pub type PersizecfgW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MEMSIZECFG` reader - Memory data size"]
pub type MemsizecfgR = crate::FieldReader;
#[doc = "Field `MEMSIZECFG` writer - Memory data size"]
pub type MemsizecfgW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PERIOSIZE` reader - Peripheral increment offset size"]
pub type PeriosizeR = crate::BitReader;
#[doc = "Field `PERIOSIZE` writer - Peripheral increment offset size"]
pub type PeriosizeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRILCFG` reader - Priority level"]
pub type PrilcfgR = crate::FieldReader;
#[doc = "Field `PRILCFG` writer - Priority level"]
pub type PrilcfgW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DBM` reader - Double buffer mode"]
pub type DbmR = crate::BitReader;
#[doc = "Field `DBM` writer - Double buffer mode"]
pub type DbmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTARG` reader - Current target (only in double buffer mode)"]
pub type CtargR = crate::BitReader;
#[doc = "Field `CTARG` writer - Current target (only in double buffer mode)"]
pub type CtargW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACK` reader - ACK"]
pub type AckR = crate::BitReader;
#[doc = "Field `ACK` writer - ACK"]
pub type AckW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PBCFG` reader - Peripheral burst transfer configuration"]
pub type PbcfgR = crate::FieldReader;
#[doc = "Field `PBCFG` writer - Peripheral burst transfer configuration"]
pub type PbcfgW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MBCFG` reader - Memory burst transfer configuration"]
pub type MbcfgR = crate::FieldReader;
#[doc = "Field `MBCFG` writer - Memory burst transfer configuration"]
pub type MbcfgW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CHSEL` reader - Channel selection"]
pub type ChselR = crate::FieldReader;
#[doc = "Field `CHSEL` writer - Channel selection"]
pub type ChselW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bit 0 - Stream enable / flag stream ready when read low"]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Direct mode error interrupt enable"]
    #[inline(always)]
    pub fn dmeien(&self) -> DmeienR {
        DmeienR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Transfer error interrupt enable"]
    #[inline(always)]
    pub fn txeien(&self) -> TxeienR {
        TxeienR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Half transfer interrupt enable"]
    #[inline(always)]
    pub fn htxien(&self) -> HtxienR {
        HtxienR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Transfer complete interrupt enable"]
    #[inline(always)]
    pub fn txcien(&self) -> TxcienR {
        TxcienR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Peripheral flow controller"]
    #[inline(always)]
    pub fn perfc(&self) -> PerfcR {
        PerfcR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Data transfer direction"]
    #[inline(always)]
    pub fn dircfg(&self) -> DircfgR {
        DircfgR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - Circular mode"]
    #[inline(always)]
    pub fn circmen(&self) -> CircmenR {
        CircmenR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Peripheral increment mode"]
    #[inline(always)]
    pub fn perim(&self) -> PerimR {
        PerimR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Memory increment mode"]
    #[inline(always)]
    pub fn memim(&self) -> MemimR {
        MemimR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:12 - Peripheral data size"]
    #[inline(always)]
    pub fn persizecfg(&self) -> PersizecfgR {
        PersizecfgR::new(((self.bits >> 11) & 3) as u8)
    }
    #[doc = "Bits 13:14 - Memory data size"]
    #[inline(always)]
    pub fn memsizecfg(&self) -> MemsizecfgR {
        MemsizecfgR::new(((self.bits >> 13) & 3) as u8)
    }
    #[doc = "Bit 15 - Peripheral increment offset size"]
    #[inline(always)]
    pub fn periosize(&self) -> PeriosizeR {
        PeriosizeR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:17 - Priority level"]
    #[inline(always)]
    pub fn prilcfg(&self) -> PrilcfgR {
        PrilcfgR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 18 - Double buffer mode"]
    #[inline(always)]
    pub fn dbm(&self) -> DbmR {
        DbmR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Current target (only in double buffer mode)"]
    #[inline(always)]
    pub fn ctarg(&self) -> CtargR {
        CtargR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - ACK"]
    #[inline(always)]
    pub fn ack(&self) -> AckR {
        AckR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 21:22 - Peripheral burst transfer configuration"]
    #[inline(always)]
    pub fn pbcfg(&self) -> PbcfgR {
        PbcfgR::new(((self.bits >> 21) & 3) as u8)
    }
    #[doc = "Bits 23:24 - Memory burst transfer configuration"]
    #[inline(always)]
    pub fn mbcfg(&self) -> MbcfgR {
        MbcfgR::new(((self.bits >> 23) & 3) as u8)
    }
    #[doc = "Bits 25:27 - Channel selection"]
    #[inline(always)]
    pub fn chsel(&self) -> ChselR {
        ChselR::new(((self.bits >> 25) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Stream enable / flag stream ready when read low"]
    #[inline(always)]
    pub fn en(&mut self) -> EnW<'_, Scfg7Spec> {
        EnW::new(self, 0)
    }
    #[doc = "Bit 1 - Direct mode error interrupt enable"]
    #[inline(always)]
    pub fn dmeien(&mut self) -> DmeienW<'_, Scfg7Spec> {
        DmeienW::new(self, 1)
    }
    #[doc = "Bit 2 - Transfer error interrupt enable"]
    #[inline(always)]
    pub fn txeien(&mut self) -> TxeienW<'_, Scfg7Spec> {
        TxeienW::new(self, 2)
    }
    #[doc = "Bit 3 - Half transfer interrupt enable"]
    #[inline(always)]
    pub fn htxien(&mut self) -> HtxienW<'_, Scfg7Spec> {
        HtxienW::new(self, 3)
    }
    #[doc = "Bit 4 - Transfer complete interrupt enable"]
    #[inline(always)]
    pub fn txcien(&mut self) -> TxcienW<'_, Scfg7Spec> {
        TxcienW::new(self, 4)
    }
    #[doc = "Bit 5 - Peripheral flow controller"]
    #[inline(always)]
    pub fn perfc(&mut self) -> PerfcW<'_, Scfg7Spec> {
        PerfcW::new(self, 5)
    }
    #[doc = "Bits 6:7 - Data transfer direction"]
    #[inline(always)]
    pub fn dircfg(&mut self) -> DircfgW<'_, Scfg7Spec> {
        DircfgW::new(self, 6)
    }
    #[doc = "Bit 8 - Circular mode"]
    #[inline(always)]
    pub fn circmen(&mut self) -> CircmenW<'_, Scfg7Spec> {
        CircmenW::new(self, 8)
    }
    #[doc = "Bit 9 - Peripheral increment mode"]
    #[inline(always)]
    pub fn perim(&mut self) -> PerimW<'_, Scfg7Spec> {
        PerimW::new(self, 9)
    }
    #[doc = "Bit 10 - Memory increment mode"]
    #[inline(always)]
    pub fn memim(&mut self) -> MemimW<'_, Scfg7Spec> {
        MemimW::new(self, 10)
    }
    #[doc = "Bits 11:12 - Peripheral data size"]
    #[inline(always)]
    pub fn persizecfg(&mut self) -> PersizecfgW<'_, Scfg7Spec> {
        PersizecfgW::new(self, 11)
    }
    #[doc = "Bits 13:14 - Memory data size"]
    #[inline(always)]
    pub fn memsizecfg(&mut self) -> MemsizecfgW<'_, Scfg7Spec> {
        MemsizecfgW::new(self, 13)
    }
    #[doc = "Bit 15 - Peripheral increment offset size"]
    #[inline(always)]
    pub fn periosize(&mut self) -> PeriosizeW<'_, Scfg7Spec> {
        PeriosizeW::new(self, 15)
    }
    #[doc = "Bits 16:17 - Priority level"]
    #[inline(always)]
    pub fn prilcfg(&mut self) -> PrilcfgW<'_, Scfg7Spec> {
        PrilcfgW::new(self, 16)
    }
    #[doc = "Bit 18 - Double buffer mode"]
    #[inline(always)]
    pub fn dbm(&mut self) -> DbmW<'_, Scfg7Spec> {
        DbmW::new(self, 18)
    }
    #[doc = "Bit 19 - Current target (only in double buffer mode)"]
    #[inline(always)]
    pub fn ctarg(&mut self) -> CtargW<'_, Scfg7Spec> {
        CtargW::new(self, 19)
    }
    #[doc = "Bit 20 - ACK"]
    #[inline(always)]
    pub fn ack(&mut self) -> AckW<'_, Scfg7Spec> {
        AckW::new(self, 20)
    }
    #[doc = "Bits 21:22 - Peripheral burst transfer configuration"]
    #[inline(always)]
    pub fn pbcfg(&mut self) -> PbcfgW<'_, Scfg7Spec> {
        PbcfgW::new(self, 21)
    }
    #[doc = "Bits 23:24 - Memory burst transfer configuration"]
    #[inline(always)]
    pub fn mbcfg(&mut self) -> MbcfgW<'_, Scfg7Spec> {
        MbcfgW::new(self, 23)
    }
    #[doc = "Bits 25:27 - Channel selection"]
    #[inline(always)]
    pub fn chsel(&mut self) -> ChselW<'_, Scfg7Spec> {
        ChselW::new(self, 25)
    }
}
#[doc = "stream x configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`scfg7::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scfg7::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scfg7Spec;
impl crate::RegisterSpec for Scfg7Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scfg7::R`](R) reader structure"]
impl crate::Readable for Scfg7Spec {}
#[doc = "`write(|w| ..)` method takes [`scfg7::W`](W) writer structure"]
impl crate::Writable for Scfg7Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCFG7 to value 0"]
impl crate::Resettable for Scfg7Spec {}
