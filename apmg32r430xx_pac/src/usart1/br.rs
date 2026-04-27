#[doc = "Register `BR` reader"]
pub type R = crate::R<BrSpec>;
#[doc = "Register `BR` writer"]
pub type W = crate::W<BrSpec>;
#[doc = "Field `FBR` reader - fraction of USARTDIV"]
pub type FbrR = crate::FieldReader;
#[doc = "Field `FBR` writer - fraction of USARTDIV"]
pub type FbrW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `IBR` reader - mantissa of USARTDIV"]
pub type IbrR = crate::FieldReader<u16>;
#[doc = "Field `IBR` writer - mantissa of USARTDIV"]
pub type IbrW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:3 - fraction of USARTDIV"]
    #[inline(always)]
    pub fn fbr(&self) -> FbrR {
        FbrR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:15 - mantissa of USARTDIV"]
    #[inline(always)]
    pub fn ibr(&self) -> IbrR {
        IbrR::new(((self.bits >> 4) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:3 - fraction of USARTDIV"]
    #[inline(always)]
    pub fn fbr(&mut self) -> FbrW<'_, BrSpec> {
        FbrW::new(self, 0)
    }
    #[doc = "Bits 4:15 - mantissa of USARTDIV"]
    #[inline(always)]
    pub fn ibr(&mut self) -> IbrW<'_, BrSpec> {
        IbrW::new(self, 4)
    }
}
#[doc = "Baud rate register\n\nYou can [`read`](crate::Reg::read) this register and get [`br::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`br::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BrSpec;
impl crate::RegisterSpec for BrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`br::R`](R) reader structure"]
impl crate::Readable for BrSpec {}
#[doc = "`write(|w| ..)` method takes [`br::W`](W) writer structure"]
impl crate::Writable for BrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BR to value 0"]
impl crate::Resettable for BrSpec {}
