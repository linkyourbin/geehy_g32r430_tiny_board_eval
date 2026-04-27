#[doc = "Register `IPEND` reader"]
pub type R = crate::R<IpendSpec>;
#[doc = "Register `IPEND` writer"]
pub type W = crate::W<IpendSpec>;
#[doc = "Field `IPEND0` reader - Pending bit 0"]
pub type Ipend0R = crate::BitReader;
#[doc = "Field `IPEND0` writer - Pending bit 0"]
pub type Ipend0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IPEND1` reader - Pending bit 1"]
pub type Ipend1R = crate::BitReader;
#[doc = "Field `IPEND1` writer - Pending bit 1"]
pub type Ipend1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IPEND2` reader - Pending bit 2"]
pub type Ipend2R = crate::BitReader;
#[doc = "Field `IPEND2` writer - Pending bit 2"]
pub type Ipend2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IPEND3` reader - Pending bit 3"]
pub type Ipend3R = crate::BitReader;
#[doc = "Field `IPEND3` writer - Pending bit 3"]
pub type Ipend3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IPEND4` reader - Pending bit 4"]
pub type Ipend4R = crate::BitReader;
#[doc = "Field `IPEND4` writer - Pending bit 4"]
pub type Ipend4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IPEND5` reader - Pending bit 5"]
pub type Ipend5R = crate::BitReader;
#[doc = "Field `IPEND5` writer - Pending bit 5"]
pub type Ipend5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IPEND6` reader - Pending bit 6"]
pub type Ipend6R = crate::BitReader;
#[doc = "Field `IPEND6` writer - Pending bit 6"]
pub type Ipend6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IPEND7` reader - Pending bit 7"]
pub type Ipend7R = crate::BitReader;
#[doc = "Field `IPEND7` writer - Pending bit 7"]
pub type Ipend7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IPEND8` reader - Pending bit 8"]
pub type Ipend8R = crate::BitReader;
#[doc = "Field `IPEND8` writer - Pending bit 8"]
pub type Ipend8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IPEND9` reader - Pending bit 9"]
pub type Ipend9R = crate::BitReader;
#[doc = "Field `IPEND9` writer - Pending bit 9"]
pub type Ipend9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IPEND10` reader - Pending bit 10"]
pub type Ipend10R = crate::BitReader;
#[doc = "Field `IPEND10` writer - Pending bit 10"]
pub type Ipend10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IPEND11` reader - Pending bit 11"]
pub type Ipend11R = crate::BitReader;
#[doc = "Field `IPEND11` writer - Pending bit 11"]
pub type Ipend11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IPEND12` reader - Pending bit 12"]
pub type Ipend12R = crate::BitReader;
#[doc = "Field `IPEND12` writer - Pending bit 12"]
pub type Ipend12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IPEND13` reader - Pending bit 13"]
pub type Ipend13R = crate::BitReader;
#[doc = "Field `IPEND13` writer - Pending bit 13"]
pub type Ipend13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IPEND14` reader - Pending bit 14"]
pub type Ipend14R = crate::BitReader;
#[doc = "Field `IPEND14` writer - Pending bit 14"]
pub type Ipend14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IPEND15` reader - Pending bit 15"]
pub type Ipend15R = crate::BitReader;
#[doc = "Field `IPEND15` writer - Pending bit 15"]
pub type Ipend15W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IPEND16` reader - Pending bit 16"]
pub type Ipend16R = crate::BitReader;
#[doc = "Field `IPEND16` writer - Pending bit 16"]
pub type Ipend16W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IPEND17` reader - Pending bit 17"]
pub type Ipend17R = crate::BitReader;
#[doc = "Field `IPEND17` writer - Pending bit 17"]
pub type Ipend17W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IPEND18` reader - Pending bit 18"]
pub type Ipend18R = crate::BitReader;
#[doc = "Field `IPEND18` writer - Pending bit 18"]
pub type Ipend18W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IPEND21` reader - Pending bit 21"]
pub type Ipend21R = crate::BitReader;
#[doc = "Field `IPEND21` writer - Pending bit 21"]
pub type Ipend21W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IPEND22` reader - Pending bit 22"]
pub type Ipend22R = crate::BitReader;
#[doc = "Field `IPEND22` writer - Pending bit 22"]
pub type Ipend22W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Pending bit 0"]
    #[inline(always)]
    pub fn ipend0(&self) -> Ipend0R {
        Ipend0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Pending bit 1"]
    #[inline(always)]
    pub fn ipend1(&self) -> Ipend1R {
        Ipend1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Pending bit 2"]
    #[inline(always)]
    pub fn ipend2(&self) -> Ipend2R {
        Ipend2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Pending bit 3"]
    #[inline(always)]
    pub fn ipend3(&self) -> Ipend3R {
        Ipend3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Pending bit 4"]
    #[inline(always)]
    pub fn ipend4(&self) -> Ipend4R {
        Ipend4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Pending bit 5"]
    #[inline(always)]
    pub fn ipend5(&self) -> Ipend5R {
        Ipend5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Pending bit 6"]
    #[inline(always)]
    pub fn ipend6(&self) -> Ipend6R {
        Ipend6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Pending bit 7"]
    #[inline(always)]
    pub fn ipend7(&self) -> Ipend7R {
        Ipend7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Pending bit 8"]
    #[inline(always)]
    pub fn ipend8(&self) -> Ipend8R {
        Ipend8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Pending bit 9"]
    #[inline(always)]
    pub fn ipend9(&self) -> Ipend9R {
        Ipend9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Pending bit 10"]
    #[inline(always)]
    pub fn ipend10(&self) -> Ipend10R {
        Ipend10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Pending bit 11"]
    #[inline(always)]
    pub fn ipend11(&self) -> Ipend11R {
        Ipend11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Pending bit 12"]
    #[inline(always)]
    pub fn ipend12(&self) -> Ipend12R {
        Ipend12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Pending bit 13"]
    #[inline(always)]
    pub fn ipend13(&self) -> Ipend13R {
        Ipend13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Pending bit 14"]
    #[inline(always)]
    pub fn ipend14(&self) -> Ipend14R {
        Ipend14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Pending bit 15"]
    #[inline(always)]
    pub fn ipend15(&self) -> Ipend15R {
        Ipend15R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Pending bit 16"]
    #[inline(always)]
    pub fn ipend16(&self) -> Ipend16R {
        Ipend16R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Pending bit 17"]
    #[inline(always)]
    pub fn ipend17(&self) -> Ipend17R {
        Ipend17R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Pending bit 18"]
    #[inline(always)]
    pub fn ipend18(&self) -> Ipend18R {
        Ipend18R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 21 - Pending bit 21"]
    #[inline(always)]
    pub fn ipend21(&self) -> Ipend21R {
        Ipend21R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Pending bit 22"]
    #[inline(always)]
    pub fn ipend22(&self) -> Ipend22R {
        Ipend22R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Pending bit 0"]
    #[inline(always)]
    pub fn ipend0(&mut self) -> Ipend0W<'_, IpendSpec> {
        Ipend0W::new(self, 0)
    }
    #[doc = "Bit 1 - Pending bit 1"]
    #[inline(always)]
    pub fn ipend1(&mut self) -> Ipend1W<'_, IpendSpec> {
        Ipend1W::new(self, 1)
    }
    #[doc = "Bit 2 - Pending bit 2"]
    #[inline(always)]
    pub fn ipend2(&mut self) -> Ipend2W<'_, IpendSpec> {
        Ipend2W::new(self, 2)
    }
    #[doc = "Bit 3 - Pending bit 3"]
    #[inline(always)]
    pub fn ipend3(&mut self) -> Ipend3W<'_, IpendSpec> {
        Ipend3W::new(self, 3)
    }
    #[doc = "Bit 4 - Pending bit 4"]
    #[inline(always)]
    pub fn ipend4(&mut self) -> Ipend4W<'_, IpendSpec> {
        Ipend4W::new(self, 4)
    }
    #[doc = "Bit 5 - Pending bit 5"]
    #[inline(always)]
    pub fn ipend5(&mut self) -> Ipend5W<'_, IpendSpec> {
        Ipend5W::new(self, 5)
    }
    #[doc = "Bit 6 - Pending bit 6"]
    #[inline(always)]
    pub fn ipend6(&mut self) -> Ipend6W<'_, IpendSpec> {
        Ipend6W::new(self, 6)
    }
    #[doc = "Bit 7 - Pending bit 7"]
    #[inline(always)]
    pub fn ipend7(&mut self) -> Ipend7W<'_, IpendSpec> {
        Ipend7W::new(self, 7)
    }
    #[doc = "Bit 8 - Pending bit 8"]
    #[inline(always)]
    pub fn ipend8(&mut self) -> Ipend8W<'_, IpendSpec> {
        Ipend8W::new(self, 8)
    }
    #[doc = "Bit 9 - Pending bit 9"]
    #[inline(always)]
    pub fn ipend9(&mut self) -> Ipend9W<'_, IpendSpec> {
        Ipend9W::new(self, 9)
    }
    #[doc = "Bit 10 - Pending bit 10"]
    #[inline(always)]
    pub fn ipend10(&mut self) -> Ipend10W<'_, IpendSpec> {
        Ipend10W::new(self, 10)
    }
    #[doc = "Bit 11 - Pending bit 11"]
    #[inline(always)]
    pub fn ipend11(&mut self) -> Ipend11W<'_, IpendSpec> {
        Ipend11W::new(self, 11)
    }
    #[doc = "Bit 12 - Pending bit 12"]
    #[inline(always)]
    pub fn ipend12(&mut self) -> Ipend12W<'_, IpendSpec> {
        Ipend12W::new(self, 12)
    }
    #[doc = "Bit 13 - Pending bit 13"]
    #[inline(always)]
    pub fn ipend13(&mut self) -> Ipend13W<'_, IpendSpec> {
        Ipend13W::new(self, 13)
    }
    #[doc = "Bit 14 - Pending bit 14"]
    #[inline(always)]
    pub fn ipend14(&mut self) -> Ipend14W<'_, IpendSpec> {
        Ipend14W::new(self, 14)
    }
    #[doc = "Bit 15 - Pending bit 15"]
    #[inline(always)]
    pub fn ipend15(&mut self) -> Ipend15W<'_, IpendSpec> {
        Ipend15W::new(self, 15)
    }
    #[doc = "Bit 16 - Pending bit 16"]
    #[inline(always)]
    pub fn ipend16(&mut self) -> Ipend16W<'_, IpendSpec> {
        Ipend16W::new(self, 16)
    }
    #[doc = "Bit 17 - Pending bit 17"]
    #[inline(always)]
    pub fn ipend17(&mut self) -> Ipend17W<'_, IpendSpec> {
        Ipend17W::new(self, 17)
    }
    #[doc = "Bit 18 - Pending bit 18"]
    #[inline(always)]
    pub fn ipend18(&mut self) -> Ipend18W<'_, IpendSpec> {
        Ipend18W::new(self, 18)
    }
    #[doc = "Bit 21 - Pending bit 21"]
    #[inline(always)]
    pub fn ipend21(&mut self) -> Ipend21W<'_, IpendSpec> {
        Ipend21W::new(self, 21)
    }
    #[doc = "Bit 22 - Pending bit 22"]
    #[inline(always)]
    pub fn ipend22(&mut self) -> Ipend22W<'_, IpendSpec> {
        Ipend22W::new(self, 22)
    }
}
#[doc = "Pending register\n\nYou can [`read`](crate::Reg::read) this register and get [`ipend::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipend::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IpendSpec;
impl crate::RegisterSpec for IpendSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ipend::R`](R) reader structure"]
impl crate::Readable for IpendSpec {}
#[doc = "`write(|w| ..)` method takes [`ipend::W`](W) writer structure"]
impl crate::Writable for IpendSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IPEND to value 0"]
impl crate::Resettable for IpendSpec {}
