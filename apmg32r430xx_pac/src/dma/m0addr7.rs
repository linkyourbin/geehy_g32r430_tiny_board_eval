#[doc = "Register `M0ADDR7` reader"]
pub type R = crate::R<M0addr7Spec>;
#[doc = "Register `M0ADDR7` writer"]
pub type W = crate::W<M0addr7Spec>;
#[doc = "Field `M0ADDR` reader - Memory 0 address"]
pub type M0addrR = crate::FieldReader<u32>;
#[doc = "Field `M0ADDR` writer - Memory 0 address"]
pub type M0addrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Memory 0 address"]
    #[inline(always)]
    pub fn m0addr(&self) -> M0addrR {
        M0addrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Memory 0 address"]
    #[inline(always)]
    pub fn m0addr(&mut self) -> M0addrW<'_, M0addr7Spec> {
        M0addrW::new(self, 0)
    }
}
#[doc = "stream x memory 0 address register\n\nYou can [`read`](crate::Reg::read) this register and get [`m0addr7::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m0addr7::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct M0addr7Spec;
impl crate::RegisterSpec for M0addr7Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`m0addr7::R`](R) reader structure"]
impl crate::Readable for M0addr7Spec {}
#[doc = "`write(|w| ..)` method takes [`m0addr7::W`](W) writer structure"]
impl crate::Writable for M0addr7Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets M0ADDR7 to value 0"]
impl crate::Resettable for M0addr7Spec {}
