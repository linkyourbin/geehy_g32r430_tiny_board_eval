#[doc = "Register `CFG` reader"]
pub type R = crate::R<CfgSpec>;
#[doc = "Register `CFG` writer"]
pub type W = crate::W<CfgSpec>;
#[doc = "Field `WIN` reader - 7-bit window value"]
pub type WinR = crate::FieldReader;
#[doc = "Field `WIN` writer - 7-bit window value"]
pub type WinW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `TBPSC` reader - Timer Base Prescaler Factor Configure"]
pub type TbpscR = crate::FieldReader;
#[doc = "Field `TBPSC` writer - Timer Base Prescaler Factor Configure"]
pub type TbpscW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `EWIEN` reader - Early wakeup interrupt"]
pub type EwienR = crate::BitReader;
#[doc = "Field `EWIEN` writer - Early wakeup interrupt"]
pub type EwienW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:6 - 7-bit window value"]
    #[inline(always)]
    pub fn win(&self) -> WinR {
        WinR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 7:8 - Timer Base Prescaler Factor Configure"]
    #[inline(always)]
    pub fn tbpsc(&self) -> TbpscR {
        TbpscR::new(((self.bits >> 7) & 3) as u8)
    }
    #[doc = "Bit 9 - Early wakeup interrupt"]
    #[inline(always)]
    pub fn ewien(&self) -> EwienR {
        EwienR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - 7-bit window value"]
    #[inline(always)]
    pub fn win(&mut self) -> WinW<'_, CfgSpec> {
        WinW::new(self, 0)
    }
    #[doc = "Bits 7:8 - Timer Base Prescaler Factor Configure"]
    #[inline(always)]
    pub fn tbpsc(&mut self) -> TbpscW<'_, CfgSpec> {
        TbpscW::new(self, 7)
    }
    #[doc = "Bit 9 - Early wakeup interrupt"]
    #[inline(always)]
    pub fn ewien(&mut self) -> EwienW<'_, CfgSpec> {
        EwienW::new(self, 9)
    }
}
#[doc = "Configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgSpec;
impl crate::RegisterSpec for CfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg::R`](R) reader structure"]
impl crate::Readable for CfgSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg::W`](W) writer structure"]
impl crate::Writable for CfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CFG to value 0x01ff"]
impl crate::Resettable for CfgSpec {
    const RESET_VALUE: u32 = 0x01ff;
}
