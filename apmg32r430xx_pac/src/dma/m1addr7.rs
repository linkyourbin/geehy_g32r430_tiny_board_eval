#[doc = "Register `M1ADDR7` reader"]
pub type R = crate::R<M1addr7Spec>;
#[doc = "Register `M1ADDR7` writer"]
pub type W = crate::W<M1addr7Spec>;
#[doc = "Field `M1ADDR` reader - Memory 1 address (used in case of Double buffer mode)"]
pub type M1addrR = crate::FieldReader<u32>;
#[doc = "Field `M1ADDR` writer - Memory 1 address (used in case of Double buffer mode)"]
pub type M1addrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Memory 1 address (used in case of Double buffer mode)"]
    #[inline(always)]
    pub fn m1addr(&self) -> M1addrR {
        M1addrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Memory 1 address (used in case of Double buffer mode)"]
    #[inline(always)]
    pub fn m1addr(&mut self) -> M1addrW<'_, M1addr7Spec> {
        M1addrW::new(self, 0)
    }
}
#[doc = "stream x memory 1 address register\n\nYou can [`read`](crate::Reg::read) this register and get [`m1addr7::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m1addr7::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct M1addr7Spec;
impl crate::RegisterSpec for M1addr7Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`m1addr7::R`](R) reader structure"]
impl crate::Readable for M1addr7Spec {}
#[doc = "`write(|w| ..)` method takes [`m1addr7::W`](W) writer structure"]
impl crate::Writable for M1addr7Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets M1ADDR7 to value 0"]
impl crate::Resettable for M1addr7Spec {}
