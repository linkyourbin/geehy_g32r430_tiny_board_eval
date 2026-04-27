#[doc = "Register `NDATA6` reader"]
pub type R = crate::R<Ndata6Spec>;
#[doc = "Register `NDATA6` writer"]
pub type W = crate::W<Ndata6Spec>;
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
    pub fn ndata(&mut self) -> NdataW<'_, Ndata6Spec> {
        NdataW::new(self, 0)
    }
}
#[doc = "stream x number of data register\n\nYou can [`read`](crate::Reg::read) this register and get [`ndata6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ndata6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ndata6Spec;
impl crate::RegisterSpec for Ndata6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ndata6::R`](R) reader structure"]
impl crate::Readable for Ndata6Spec {}
#[doc = "`write(|w| ..)` method takes [`ndata6::W`](W) writer structure"]
impl crate::Writable for Ndata6Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets NDATA6 to value 0"]
impl crate::Resettable for Ndata6Spec {}
