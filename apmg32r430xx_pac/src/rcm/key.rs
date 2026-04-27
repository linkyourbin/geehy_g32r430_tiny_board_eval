#[doc = "Register `KEY` reader"]
pub type R = crate::R<KeySpec>;
#[doc = "Register `KEY` writer"]
pub type W = crate::W<KeySpec>;
#[doc = "Field `LOCKKEY` writer - Configuration of password protection for system register write operations"]
pub type LockkeyW<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
#[doc = "Field `KEYST` reader - System Register KEY Protection Status Bit"]
pub type KeystR = crate::BitReader;
#[doc = "Field `KEYST` writer - System Register KEY Protection Status Bit"]
pub type KeystW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 16 - System Register KEY Protection Status Bit"]
    #[inline(always)]
    pub fn keyst(&self) -> KeystR {
        KeystR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:14 - Configuration of password protection for system register write operations"]
    #[inline(always)]
    pub fn lockkey(&mut self) -> LockkeyW<'_, KeySpec> {
        LockkeyW::new(self, 0)
    }
    #[doc = "Bit 16 - System Register KEY Protection Status Bit"]
    #[inline(always)]
    pub fn keyst(&mut self) -> KeystW<'_, KeySpec> {
        KeystW::new(self, 16)
    }
}
#[doc = "Key register\n\nYou can [`read`](crate::Reg::read) this register and get [`key::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`key::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct KeySpec;
impl crate::RegisterSpec for KeySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`key::R`](R) reader structure"]
impl crate::Readable for KeySpec {}
#[doc = "`write(|w| ..)` method takes [`key::W`](W) writer structure"]
impl crate::Writable for KeySpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets KEY to value 0"]
impl crate::Resettable for KeySpec {}
