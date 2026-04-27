#[doc = "Register `C4CSR` reader"]
pub type R = crate::R<C4csrSpec>;
#[doc = "Register `C4CSR` writer"]
pub type W = crate::W<C4csrSpec>;
#[doc = "Field `EN` reader - Module enablement"]
pub type EnR = crate::BitReader;
#[doc = "Field `EN` writer - Module enablement"]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FILTER` reader - Comparator Digital Filter Timing Selection"]
pub type FilterR = crate::FieldReader;
#[doc = "Field `FILTER` writer - Comparator Digital Filter Timing Selection"]
pub type FilterW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `VN_SEL` reader - Comparator 4 negative terminal input voltage select"]
pub type VnSelR = crate::FieldReader;
#[doc = "Field `VN_SEL` writer - Comparator 4 negative terminal input voltage select"]
pub type VnSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `VP_SEL` reader - Comparator 4 positive input voltage select"]
pub type VpSelR = crate::FieldReader;
#[doc = "Field `VP_SEL` writer - Comparator 4 positive input voltage select"]
pub type VpSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TMR_SEL` reader - TMR_SEL"]
pub type TmrSelR = crate::FieldReader;
#[doc = "Field `TMR_SEL` writer - TMR_SEL"]
pub type TmrSelW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `POL` reader - The output polarity of comparator ."]
pub type PolR = crate::BitReader;
#[doc = "Field `POL` writer - The output polarity of comparator ."]
pub type PolW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HYS` reader - Delay function enabled"]
pub type HysR = crate::FieldReader;
#[doc = "Field `HYS` writer - Delay function enabled"]
pub type HysW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BLANKSEL` reader - Shielded signal for comparator channel selection."]
pub type BlankselR = crate::FieldReader;
#[doc = "Field `BLANKSEL` writer - Shielded signal for comparator channel selection."]
pub type BlankselW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `VALUE` reader - Comparator outputs the level value before polarity selector and shielding."]
pub type ValueR = crate::BitReader;
#[doc = "Field `LOCK` reader - lock"]
pub type LockR = crate::BitReader;
#[doc = "Field `LOCK` writer - lock"]
pub type LockW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Module enablement"]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - Comparator Digital Filter Timing Selection"]
    #[inline(always)]
    pub fn filter(&self) -> FilterR {
        FilterR::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bits 4:5 - Comparator 4 negative terminal input voltage select"]
    #[inline(always)]
    pub fn vn_sel(&self) -> VnSelR {
        VnSelR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Comparator 4 positive input voltage select"]
    #[inline(always)]
    pub fn vp_sel(&self) -> VpSelR {
        VpSelR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:11 - TMR_SEL"]
    #[inline(always)]
    pub fn tmr_sel(&self) -> TmrSelR {
        TmrSelR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 15 - The output polarity of comparator ."]
    #[inline(always)]
    pub fn pol(&self) -> PolR {
        PolR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:17 - Delay function enabled"]
    #[inline(always)]
    pub fn hys(&self) -> HysR {
        HysR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:20 - Shielded signal for comparator channel selection."]
    #[inline(always)]
    pub fn blanksel(&self) -> BlankselR {
        BlankselR::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bit 30 - Comparator outputs the level value before polarity selector and shielding."]
    #[inline(always)]
    pub fn value(&self) -> ValueR {
        ValueR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - lock"]
    #[inline(always)]
    pub fn lock(&self) -> LockR {
        LockR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Module enablement"]
    #[inline(always)]
    pub fn en(&mut self) -> EnW<'_, C4csrSpec> {
        EnW::new(self, 0)
    }
    #[doc = "Bits 1:3 - Comparator Digital Filter Timing Selection"]
    #[inline(always)]
    pub fn filter(&mut self) -> FilterW<'_, C4csrSpec> {
        FilterW::new(self, 1)
    }
    #[doc = "Bits 4:5 - Comparator 4 negative terminal input voltage select"]
    #[inline(always)]
    pub fn vn_sel(&mut self) -> VnSelW<'_, C4csrSpec> {
        VnSelW::new(self, 4)
    }
    #[doc = "Bits 6:7 - Comparator 4 positive input voltage select"]
    #[inline(always)]
    pub fn vp_sel(&mut self) -> VpSelW<'_, C4csrSpec> {
        VpSelW::new(self, 6)
    }
    #[doc = "Bits 8:11 - TMR_SEL"]
    #[inline(always)]
    pub fn tmr_sel(&mut self) -> TmrSelW<'_, C4csrSpec> {
        TmrSelW::new(self, 8)
    }
    #[doc = "Bit 15 - The output polarity of comparator ."]
    #[inline(always)]
    pub fn pol(&mut self) -> PolW<'_, C4csrSpec> {
        PolW::new(self, 15)
    }
    #[doc = "Bits 16:17 - Delay function enabled"]
    #[inline(always)]
    pub fn hys(&mut self) -> HysW<'_, C4csrSpec> {
        HysW::new(self, 16)
    }
    #[doc = "Bits 18:20 - Shielded signal for comparator channel selection."]
    #[inline(always)]
    pub fn blanksel(&mut self) -> BlankselW<'_, C4csrSpec> {
        BlankselW::new(self, 18)
    }
    #[doc = "Bit 31 - lock"]
    #[inline(always)]
    pub fn lock(&mut self) -> LockW<'_, C4csrSpec> {
        LockW::new(self, 31)
    }
}
#[doc = "Comparator 4 control register.\n\nYou can [`read`](crate::Reg::read) this register and get [`c4csr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c4csr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct C4csrSpec;
impl crate::RegisterSpec for C4csrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`c4csr::R`](R) reader structure"]
impl crate::Readable for C4csrSpec {}
#[doc = "`write(|w| ..)` method takes [`c4csr::W`](W) writer structure"]
impl crate::Writable for C4csrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets C4CSR to value 0"]
impl crate::Resettable for C4csrSpec {}
