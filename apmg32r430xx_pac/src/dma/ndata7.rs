#[doc = "Register `NDATA7` reader"]
pub type R = crate::R<Ndata7Spec>;
#[doc = "Register `NDATA7` writer"]
pub type W = crate::W<Ndata7Spec>;
#[doc = "Field `NDATA` reader - Number of data items to transfer"]
pub type NdataR = crate::FieldReader<u16>;
#[doc = "Field `NDATA` writer - Number of data items to transfer"]
pub type NdataW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Number of data items to transfer"]
    #[inline(always)]
    pub fn ndata(&self) -> NdataR {
        NdataR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Number of data items to transfer"]
    #[inline(always)]
    pub fn ndata(&mut self) -> NdataW<'_, Ndata7Spec> {
        NdataW::new(self, 0)
    }
}
#[doc = "stream x number of data register\n\nYou can [`read`](crate::Reg::read) this register and get [`ndata7::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ndata7::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ndata7Spec;
impl crate::RegisterSpec for Ndata7Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ndata7::R`](R) reader structure"]
impl crate::Readable for Ndata7Spec {}
#[doc = "`write(|w| ..)` method takes [`ndata7::W`](W) writer structure"]
impl crate::Writable for Ndata7Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets NDATA7 to value 0"]
impl crate::Resettable for Ndata7Spec {}
