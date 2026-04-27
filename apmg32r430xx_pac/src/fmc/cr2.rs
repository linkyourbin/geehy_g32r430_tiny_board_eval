#[doc = "Register `CR2` reader"]
pub type R = crate::R<Cr2Spec>;
#[doc = "Register `CR2` writer"]
pub type W = crate::W<Cr2Spec>;
#[doc = "Field `RDWAIT` reader - Reading Flash-wait time setting"]
pub type RdwaitR = crate::FieldReader;
#[doc = "Field `RDWAIT` writer - Reading Flash-wait time setting"]
pub type RdwaitW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PREFETCHEN` reader - Prefetch Enable"]
pub type PrefetchenR = crate::BitReader;
#[doc = "Field `PREFETCHEN` writer - Prefetch Enable"]
pub type PrefetchenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHCHEN` reader - Cache enable operation."]
pub type ChchenR = crate::BitReader;
#[doc = "Field `CHCHEN` writer - Cache enable operation."]
pub type ChchenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CACHERST` reader - cache - reset operation"]
pub type CacherstR = crate::BitReader;
#[doc = "Field `CACHERST` writer - cache - reset operation"]
pub type CacherstW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - Reading Flash-wait time setting"]
    #[inline(always)]
    pub fn rdwait(&self) -> RdwaitR {
        RdwaitR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 8 - Prefetch Enable"]
    #[inline(always)]
    pub fn prefetchen(&self) -> PrefetchenR {
        PrefetchenR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Cache enable operation."]
    #[inline(always)]
    pub fn chchen(&self) -> ChchenR {
        ChchenR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - cache - reset operation"]
    #[inline(always)]
    pub fn cacherst(&self) -> CacherstR {
        CacherstR::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Reading Flash-wait time setting"]
    #[inline(always)]
    pub fn rdwait(&mut self) -> RdwaitW<'_, Cr2Spec> {
        RdwaitW::new(self, 0)
    }
    #[doc = "Bit 8 - Prefetch Enable"]
    #[inline(always)]
    pub fn prefetchen(&mut self) -> PrefetchenW<'_, Cr2Spec> {
        PrefetchenW::new(self, 8)
    }
    #[doc = "Bit 9 - Cache enable operation."]
    #[inline(always)]
    pub fn chchen(&mut self) -> ChchenW<'_, Cr2Spec> {
        ChchenW::new(self, 9)
    }
    #[doc = "Bit 10 - cache - reset operation"]
    #[inline(always)]
    pub fn cacherst(&mut self) -> CacherstW<'_, Cr2Spec> {
        CacherstW::new(self, 10)
    }
}
#[doc = "Control register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`cr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cr2Spec;
impl crate::RegisterSpec for Cr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr2::R`](R) reader structure"]
impl crate::Readable for Cr2Spec {}
#[doc = "`write(|w| ..)` method takes [`cr2::W`](W) writer structure"]
impl crate::Writable for Cr2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CR2 to value 0x01"]
impl crate::Resettable for Cr2Spec {
    const RESET_VALUE: u32 = 0x01;
}
