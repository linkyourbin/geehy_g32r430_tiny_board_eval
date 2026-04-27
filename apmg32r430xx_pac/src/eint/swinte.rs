#[doc = "Register `SWINTE` reader"]
pub type R = crate::R<SwinteSpec>;
#[doc = "Register `SWINTE` writer"]
pub type W = crate::W<SwinteSpec>;
#[doc = "Field `SWINTE0` reader - Software Interrupt on line 0"]
pub type Swinte0R = crate::BitReader;
#[doc = "Field `SWINTE0` writer - Software Interrupt on line 0"]
pub type Swinte0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWINTE1` reader - Software Interrupt on line 1"]
pub type Swinte1R = crate::BitReader;
#[doc = "Field `SWINTE1` writer - Software Interrupt on line 1"]
pub type Swinte1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWINTE2` reader - Software Interrupt on line 2"]
pub type Swinte2R = crate::BitReader;
#[doc = "Field `SWINTE2` writer - Software Interrupt on line 2"]
pub type Swinte2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWINTE3` reader - Software Interrupt on line 3"]
pub type Swinte3R = crate::BitReader;
#[doc = "Field `SWINTE3` writer - Software Interrupt on line 3"]
pub type Swinte3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWINTE4` reader - Software Interrupt on line 4"]
pub type Swinte4R = crate::BitReader;
#[doc = "Field `SWINTE4` writer - Software Interrupt on line 4"]
pub type Swinte4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWINTE5` reader - Software Interrupt on line 5"]
pub type Swinte5R = crate::BitReader;
#[doc = "Field `SWINTE5` writer - Software Interrupt on line 5"]
pub type Swinte5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWINTE6` reader - Software Interrupt on line 6"]
pub type Swinte6R = crate::BitReader;
#[doc = "Field `SWINTE6` writer - Software Interrupt on line 6"]
pub type Swinte6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWINTE7` reader - Software Interrupt on line 7"]
pub type Swinte7R = crate::BitReader;
#[doc = "Field `SWINTE7` writer - Software Interrupt on line 7"]
pub type Swinte7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWINTE8` reader - Software Interrupt on line 8"]
pub type Swinte8R = crate::BitReader;
#[doc = "Field `SWINTE8` writer - Software Interrupt on line 8"]
pub type Swinte8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWINTE9` reader - Software Interrupt on line 9"]
pub type Swinte9R = crate::BitReader;
#[doc = "Field `SWINTE9` writer - Software Interrupt on line 9"]
pub type Swinte9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWINTE10` reader - Software Interrupt on line 10"]
pub type Swinte10R = crate::BitReader;
#[doc = "Field `SWINTE10` writer - Software Interrupt on line 10"]
pub type Swinte10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWINTE11` reader - Software Interrupt on line 11"]
pub type Swinte11R = crate::BitReader;
#[doc = "Field `SWINTE11` writer - Software Interrupt on line 11"]
pub type Swinte11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWINTE12` reader - Software Interrupt on line 12"]
pub type Swinte12R = crate::BitReader;
#[doc = "Field `SWINTE12` writer - Software Interrupt on line 12"]
pub type Swinte12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWINTE13` reader - Software Interrupt on line 13"]
pub type Swinte13R = crate::BitReader;
#[doc = "Field `SWINTE13` writer - Software Interrupt on line 13"]
pub type Swinte13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWINTE14` reader - Software Interrupt on line 14"]
pub type Swinte14R = crate::BitReader;
#[doc = "Field `SWINTE14` writer - Software Interrupt on line 14"]
pub type Swinte14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWINTE15` reader - Software Interrupt on line 15"]
pub type Swinte15R = crate::BitReader;
#[doc = "Field `SWINTE15` writer - Software Interrupt on line 15"]
pub type Swinte15W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWINTE16` reader - Software Interrupt on line 16"]
pub type Swinte16R = crate::BitReader;
#[doc = "Field `SWINTE16` writer - Software Interrupt on line 16"]
pub type Swinte16W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWINTE17` reader - Software Interrupt on line 17"]
pub type Swinte17R = crate::BitReader;
#[doc = "Field `SWINTE17` writer - Software Interrupt on line 17"]
pub type Swinte17W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWINTE18` reader - Software Interrupt on line 18"]
pub type Swinte18R = crate::BitReader;
#[doc = "Field `SWINTE18` writer - Software Interrupt on line 18"]
pub type Swinte18W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWINTE21` reader - Software Interrupt on line 21"]
pub type Swinte21R = crate::BitReader;
#[doc = "Field `SWINTE21` writer - Software Interrupt on line 21"]
pub type Swinte21W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWINTE22` reader - Software Interrupt on line 22"]
pub type Swinte22R = crate::BitReader;
#[doc = "Field `SWINTE22` writer - Software Interrupt on line 22"]
pub type Swinte22W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Software Interrupt on line 0"]
    #[inline(always)]
    pub fn swinte0(&self) -> Swinte0R {
        Swinte0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Software Interrupt on line 1"]
    #[inline(always)]
    pub fn swinte1(&self) -> Swinte1R {
        Swinte1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Software Interrupt on line 2"]
    #[inline(always)]
    pub fn swinte2(&self) -> Swinte2R {
        Swinte2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Software Interrupt on line 3"]
    #[inline(always)]
    pub fn swinte3(&self) -> Swinte3R {
        Swinte3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Software Interrupt on line 4"]
    #[inline(always)]
    pub fn swinte4(&self) -> Swinte4R {
        Swinte4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Software Interrupt on line 5"]
    #[inline(always)]
    pub fn swinte5(&self) -> Swinte5R {
        Swinte5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Software Interrupt on line 6"]
    #[inline(always)]
    pub fn swinte6(&self) -> Swinte6R {
        Swinte6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Software Interrupt on line 7"]
    #[inline(always)]
    pub fn swinte7(&self) -> Swinte7R {
        Swinte7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Software Interrupt on line 8"]
    #[inline(always)]
    pub fn swinte8(&self) -> Swinte8R {
        Swinte8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Software Interrupt on line 9"]
    #[inline(always)]
    pub fn swinte9(&self) -> Swinte9R {
        Swinte9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Software Interrupt on line 10"]
    #[inline(always)]
    pub fn swinte10(&self) -> Swinte10R {
        Swinte10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Software Interrupt on line 11"]
    #[inline(always)]
    pub fn swinte11(&self) -> Swinte11R {
        Swinte11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Software Interrupt on line 12"]
    #[inline(always)]
    pub fn swinte12(&self) -> Swinte12R {
        Swinte12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Software Interrupt on line 13"]
    #[inline(always)]
    pub fn swinte13(&self) -> Swinte13R {
        Swinte13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Software Interrupt on line 14"]
    #[inline(always)]
    pub fn swinte14(&self) -> Swinte14R {
        Swinte14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Software Interrupt on line 15"]
    #[inline(always)]
    pub fn swinte15(&self) -> Swinte15R {
        Swinte15R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Software Interrupt on line 16"]
    #[inline(always)]
    pub fn swinte16(&self) -> Swinte16R {
        Swinte16R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Software Interrupt on line 17"]
    #[inline(always)]
    pub fn swinte17(&self) -> Swinte17R {
        Swinte17R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Software Interrupt on line 18"]
    #[inline(always)]
    pub fn swinte18(&self) -> Swinte18R {
        Swinte18R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 21 - Software Interrupt on line 21"]
    #[inline(always)]
    pub fn swinte21(&self) -> Swinte21R {
        Swinte21R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Software Interrupt on line 22"]
    #[inline(always)]
    pub fn swinte22(&self) -> Swinte22R {
        Swinte22R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Software Interrupt on line 0"]
    #[inline(always)]
    pub fn swinte0(&mut self) -> Swinte0W<'_, SwinteSpec> {
        Swinte0W::new(self, 0)
    }
    #[doc = "Bit 1 - Software Interrupt on line 1"]
    #[inline(always)]
    pub fn swinte1(&mut self) -> Swinte1W<'_, SwinteSpec> {
        Swinte1W::new(self, 1)
    }
    #[doc = "Bit 2 - Software Interrupt on line 2"]
    #[inline(always)]
    pub fn swinte2(&mut self) -> Swinte2W<'_, SwinteSpec> {
        Swinte2W::new(self, 2)
    }
    #[doc = "Bit 3 - Software Interrupt on line 3"]
    #[inline(always)]
    pub fn swinte3(&mut self) -> Swinte3W<'_, SwinteSpec> {
        Swinte3W::new(self, 3)
    }
    #[doc = "Bit 4 - Software Interrupt on line 4"]
    #[inline(always)]
    pub fn swinte4(&mut self) -> Swinte4W<'_, SwinteSpec> {
        Swinte4W::new(self, 4)
    }
    #[doc = "Bit 5 - Software Interrupt on line 5"]
    #[inline(always)]
    pub fn swinte5(&mut self) -> Swinte5W<'_, SwinteSpec> {
        Swinte5W::new(self, 5)
    }
    #[doc = "Bit 6 - Software Interrupt on line 6"]
    #[inline(always)]
    pub fn swinte6(&mut self) -> Swinte6W<'_, SwinteSpec> {
        Swinte6W::new(self, 6)
    }
    #[doc = "Bit 7 - Software Interrupt on line 7"]
    #[inline(always)]
    pub fn swinte7(&mut self) -> Swinte7W<'_, SwinteSpec> {
        Swinte7W::new(self, 7)
    }
    #[doc = "Bit 8 - Software Interrupt on line 8"]
    #[inline(always)]
    pub fn swinte8(&mut self) -> Swinte8W<'_, SwinteSpec> {
        Swinte8W::new(self, 8)
    }
    #[doc = "Bit 9 - Software Interrupt on line 9"]
    #[inline(always)]
    pub fn swinte9(&mut self) -> Swinte9W<'_, SwinteSpec> {
        Swinte9W::new(self, 9)
    }
    #[doc = "Bit 10 - Software Interrupt on line 10"]
    #[inline(always)]
    pub fn swinte10(&mut self) -> Swinte10W<'_, SwinteSpec> {
        Swinte10W::new(self, 10)
    }
    #[doc = "Bit 11 - Software Interrupt on line 11"]
    #[inline(always)]
    pub fn swinte11(&mut self) -> Swinte11W<'_, SwinteSpec> {
        Swinte11W::new(self, 11)
    }
    #[doc = "Bit 12 - Software Interrupt on line 12"]
    #[inline(always)]
    pub fn swinte12(&mut self) -> Swinte12W<'_, SwinteSpec> {
        Swinte12W::new(self, 12)
    }
    #[doc = "Bit 13 - Software Interrupt on line 13"]
    #[inline(always)]
    pub fn swinte13(&mut self) -> Swinte13W<'_, SwinteSpec> {
        Swinte13W::new(self, 13)
    }
    #[doc = "Bit 14 - Software Interrupt on line 14"]
    #[inline(always)]
    pub fn swinte14(&mut self) -> Swinte14W<'_, SwinteSpec> {
        Swinte14W::new(self, 14)
    }
    #[doc = "Bit 15 - Software Interrupt on line 15"]
    #[inline(always)]
    pub fn swinte15(&mut self) -> Swinte15W<'_, SwinteSpec> {
        Swinte15W::new(self, 15)
    }
    #[doc = "Bit 16 - Software Interrupt on line 16"]
    #[inline(always)]
    pub fn swinte16(&mut self) -> Swinte16W<'_, SwinteSpec> {
        Swinte16W::new(self, 16)
    }
    #[doc = "Bit 17 - Software Interrupt on line 17"]
    #[inline(always)]
    pub fn swinte17(&mut self) -> Swinte17W<'_, SwinteSpec> {
        Swinte17W::new(self, 17)
    }
    #[doc = "Bit 18 - Software Interrupt on line 18"]
    #[inline(always)]
    pub fn swinte18(&mut self) -> Swinte18W<'_, SwinteSpec> {
        Swinte18W::new(self, 18)
    }
    #[doc = "Bit 21 - Software Interrupt on line 21"]
    #[inline(always)]
    pub fn swinte21(&mut self) -> Swinte21W<'_, SwinteSpec> {
        Swinte21W::new(self, 21)
    }
    #[doc = "Bit 22 - Software Interrupt on line 22"]
    #[inline(always)]
    pub fn swinte22(&mut self) -> Swinte22W<'_, SwinteSpec> {
        Swinte22W::new(self, 22)
    }
}
#[doc = "Software interrupt event register\n\nYou can [`read`](crate::Reg::read) this register and get [`swinte::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swinte::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SwinteSpec;
impl crate::RegisterSpec for SwinteSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swinte::R`](R) reader structure"]
impl crate::Readable for SwinteSpec {}
#[doc = "`write(|w| ..)` method takes [`swinte::W`](W) writer structure"]
impl crate::Writable for SwinteSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SWINTE to value 0"]
impl crate::Resettable for SwinteSpec {}
