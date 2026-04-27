#[doc = "Register `SADDR2` reader"]
pub type R = crate::R<Saddr2Spec>;
#[doc = "Register `SADDR2` writer"]
pub type W = crate::W<Saddr2Spec>;
#[doc = "Field `ADDRNUM` reader - Slave Address Number Configure"]
pub type AddrnumR = crate::BitReader;
#[doc = "Field `ADDRNUM` writer - Slave Address Number Configure"]
pub type AddrnumW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDR2` reader - Interface address"]
pub type Addr2R = crate::FieldReader;
#[doc = "Field `ADDR2` writer - Interface address"]
pub type Addr2W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bit 0 - Slave Address Number Configure"]
    #[inline(always)]
    pub fn addrnum(&self) -> AddrnumR {
        AddrnumR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:7 - Interface address"]
    #[inline(always)]
    pub fn addr2(&self) -> Addr2R {
        Addr2R::new(((self.bits >> 1) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Slave Address Number Configure"]
    #[inline(always)]
    pub fn addrnum(&mut self) -> AddrnumW<'_, Saddr2Spec> {
        AddrnumW::new(self, 0)
    }
    #[doc = "Bits 1:7 - Interface address"]
    #[inline(always)]
    pub fn addr2(&mut self) -> Addr2W<'_, Saddr2Spec> {
        Addr2W::new(self, 1)
    }
}
#[doc = "Own address register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`saddr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`saddr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Saddr2Spec;
impl crate::RegisterSpec for Saddr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`saddr2::R`](R) reader structure"]
impl crate::Readable for Saddr2Spec {}
#[doc = "`write(|w| ..)` method takes [`saddr2::W`](W) writer structure"]
impl crate::Writable for Saddr2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SADDR2 to value 0"]
impl crate::Resettable for Saddr2Spec {}
