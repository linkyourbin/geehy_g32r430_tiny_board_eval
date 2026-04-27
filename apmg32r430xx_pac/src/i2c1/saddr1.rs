#[doc = "Register `SADDR1` reader"]
pub type R = crate::R<Saddr1Spec>;
#[doc = "Register `SADDR1` writer"]
pub type W = crate::W<Saddr1Spec>;
#[doc = "Field `ADDR0` reader - Interface address"]
pub type Addr0R = crate::BitReader;
#[doc = "Field `ADDR0` writer - Interface address"]
pub type Addr0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDR7` reader - Interface address"]
pub type Addr7R = crate::FieldReader;
#[doc = "Field `ADDR7` writer - Interface address"]
pub type Addr7W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `ADDR10` reader - Interface address"]
pub type Addr10R = crate::FieldReader;
#[doc = "Field `ADDR10` writer - Interface address"]
pub type Addr10W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ADDRLEN` reader - Addressing mode (slave mode)"]
pub type AddrlenR = crate::BitReader;
#[doc = "Field `ADDRLEN` writer - Addressing mode (slave mode)"]
pub type AddrlenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Interface address"]
    #[inline(always)]
    pub fn addr0(&self) -> Addr0R {
        Addr0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:7 - Interface address"]
    #[inline(always)]
    pub fn addr7(&self) -> Addr7R {
        Addr7R::new(((self.bits >> 1) & 0x7f) as u8)
    }
    #[doc = "Bits 8:9 - Interface address"]
    #[inline(always)]
    pub fn addr10(&self) -> Addr10R {
        Addr10R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 15 - Addressing mode (slave mode)"]
    #[inline(always)]
    pub fn addrlen(&self) -> AddrlenR {
        AddrlenR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Interface address"]
    #[inline(always)]
    pub fn addr0(&mut self) -> Addr0W<'_, Saddr1Spec> {
        Addr0W::new(self, 0)
    }
    #[doc = "Bits 1:7 - Interface address"]
    #[inline(always)]
    pub fn addr7(&mut self) -> Addr7W<'_, Saddr1Spec> {
        Addr7W::new(self, 1)
    }
    #[doc = "Bits 8:9 - Interface address"]
    #[inline(always)]
    pub fn addr10(&mut self) -> Addr10W<'_, Saddr1Spec> {
        Addr10W::new(self, 8)
    }
    #[doc = "Bit 15 - Addressing mode (slave mode)"]
    #[inline(always)]
    pub fn addrlen(&mut self) -> AddrlenW<'_, Saddr1Spec> {
        AddrlenW::new(self, 15)
    }
}
#[doc = "Own address register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`saddr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`saddr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Saddr1Spec;
impl crate::RegisterSpec for Saddr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`saddr1::R`](R) reader structure"]
impl crate::Readable for Saddr1Spec {}
#[doc = "`write(|w| ..)` method takes [`saddr1::W`](W) writer structure"]
impl crate::Writable for Saddr1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SADDR1 to value 0"]
impl crate::Resettable for Saddr1Spec {}
