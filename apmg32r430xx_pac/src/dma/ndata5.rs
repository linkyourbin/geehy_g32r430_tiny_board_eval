#[doc = "Register `NDATA5` reader"]
pub type R = crate::R<Ndata5Spec>;
#[doc = "Register `NDATA5` writer"]
pub type W = crate::W<Ndata5Spec>;
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
    pub fn ndata(&mut self) -> NdataW<'_, Ndata5Spec> {
        NdataW::new(self, 0)
    }
}
#[doc = "stream x number of data register\n\nYou can [`read`](crate::Reg::read) this register and get [`ndata5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ndata5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ndata5Spec;
impl crate::RegisterSpec for Ndata5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ndata5::R`](R) reader structure"]
impl crate::Readable for Ndata5Spec {}
#[doc = "`write(|w| ..)` method takes [`ndata5::W`](W) writer structure"]
impl crate::Writable for Ndata5Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets NDATA5 to value 0"]
impl crate::Resettable for Ndata5Spec {}
