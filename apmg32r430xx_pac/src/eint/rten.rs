#[doc = "Register `RTEN` reader"]
pub type R = crate::R<RtenSpec>;
#[doc = "Register `RTEN` writer"]
pub type W = crate::W<RtenSpec>;
#[doc = "Field `RTEN0` reader - Rising trigger event configuration of line 0"]
pub type Rten0R = crate::BitReader;
#[doc = "Field `RTEN0` writer - Rising trigger event configuration of line 0"]
pub type Rten0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTEN1` reader - Rising trigger event configuration of line 1"]
pub type Rten1R = crate::BitReader;
#[doc = "Field `RTEN1` writer - Rising trigger event configuration of line 1"]
pub type Rten1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTEN2` reader - Rising trigger event configuration of line 2"]
pub type Rten2R = crate::BitReader;
#[doc = "Field `RTEN2` writer - Rising trigger event configuration of line 2"]
pub type Rten2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTEN3` reader - Rising trigger event configuration of line 3"]
pub type Rten3R = crate::BitReader;
#[doc = "Field `RTEN3` writer - Rising trigger event configuration of line 3"]
pub type Rten3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTEN4` reader - Rising trigger event configuration of line 4"]
pub type Rten4R = crate::BitReader;
#[doc = "Field `RTEN4` writer - Rising trigger event configuration of line 4"]
pub type Rten4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTEN5` reader - Rising trigger event configuration of line 5"]
pub type Rten5R = crate::BitReader;
#[doc = "Field `RTEN5` writer - Rising trigger event configuration of line 5"]
pub type Rten5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTEN6` reader - Rising trigger event configuration of line 6"]
pub type Rten6R = crate::BitReader;
#[doc = "Field `RTEN6` writer - Rising trigger event configuration of line 6"]
pub type Rten6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTEN7` reader - Rising trigger event configuration of line 7"]
pub type Rten7R = crate::BitReader;
#[doc = "Field `RTEN7` writer - Rising trigger event configuration of line 7"]
pub type Rten7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTEN8` reader - Rising trigger event configuration of line 8"]
pub type Rten8R = crate::BitReader;
#[doc = "Field `RTEN8` writer - Rising trigger event configuration of line 8"]
pub type Rten8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTEN9` reader - Rising trigger event configuration of line 9"]
pub type Rten9R = crate::BitReader;
#[doc = "Field `RTEN9` writer - Rising trigger event configuration of line 9"]
pub type Rten9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTEN10` reader - Rising trigger event configuration of line 10"]
pub type Rten10R = crate::BitReader;
#[doc = "Field `RTEN10` writer - Rising trigger event configuration of line 10"]
pub type Rten10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTEN11` reader - Rising trigger event configuration of line 11"]
pub type Rten11R = crate::BitReader;
#[doc = "Field `RTEN11` writer - Rising trigger event configuration of line 11"]
pub type Rten11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTEN12` reader - Rising trigger event configuration of line 12"]
pub type Rten12R = crate::BitReader;
#[doc = "Field `RTEN12` writer - Rising trigger event configuration of line 12"]
pub type Rten12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTEN13` reader - Rising trigger event configuration of line 13"]
pub type Rten13R = crate::BitReader;
#[doc = "Field `RTEN13` writer - Rising trigger event configuration of line 13"]
pub type Rten13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTEN14` reader - Rising trigger event configuration of line 14"]
pub type Rten14R = crate::BitReader;
#[doc = "Field `RTEN14` writer - Rising trigger event configuration of line 14"]
pub type Rten14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTEN15` reader - Rising trigger event configuration of line 15"]
pub type Rten15R = crate::BitReader;
#[doc = "Field `RTEN15` writer - Rising trigger event configuration of line 15"]
pub type Rten15W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTEN16` reader - Rising trigger event configuration of line 16"]
pub type Rten16R = crate::BitReader;
#[doc = "Field `RTEN16` writer - Rising trigger event configuration of line 16"]
pub type Rten16W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTEN17` reader - Rising trigger event configuration of line 17"]
pub type Rten17R = crate::BitReader;
#[doc = "Field `RTEN17` writer - Rising trigger event configuration of line 17"]
pub type Rten17W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTEN18` reader - Rising trigger event configuration of line 18"]
pub type Rten18R = crate::BitReader;
#[doc = "Field `RTEN18` writer - Rising trigger event configuration of line 18"]
pub type Rten18W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTEN21` reader - Rising trigger event configuration of line 21"]
pub type Rten21R = crate::BitReader;
#[doc = "Field `RTEN21` writer - Rising trigger event configuration of line 21"]
pub type Rten21W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTEN22` reader - Rising trigger event configuration of line 22"]
pub type Rten22R = crate::BitReader;
#[doc = "Field `RTEN22` writer - Rising trigger event configuration of line 22"]
pub type Rten22W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Rising trigger event configuration of line 0"]
    #[inline(always)]
    pub fn rten0(&self) -> Rten0R {
        Rten0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Rising trigger event configuration of line 1"]
    #[inline(always)]
    pub fn rten1(&self) -> Rten1R {
        Rten1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Rising trigger event configuration of line 2"]
    #[inline(always)]
    pub fn rten2(&self) -> Rten2R {
        Rten2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Rising trigger event configuration of line 3"]
    #[inline(always)]
    pub fn rten3(&self) -> Rten3R {
        Rten3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Rising trigger event configuration of line 4"]
    #[inline(always)]
    pub fn rten4(&self) -> Rten4R {
        Rten4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Rising trigger event configuration of line 5"]
    #[inline(always)]
    pub fn rten5(&self) -> Rten5R {
        Rten5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Rising trigger event configuration of line 6"]
    #[inline(always)]
    pub fn rten6(&self) -> Rten6R {
        Rten6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Rising trigger event configuration of line 7"]
    #[inline(always)]
    pub fn rten7(&self) -> Rten7R {
        Rten7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Rising trigger event configuration of line 8"]
    #[inline(always)]
    pub fn rten8(&self) -> Rten8R {
        Rten8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Rising trigger event configuration of line 9"]
    #[inline(always)]
    pub fn rten9(&self) -> Rten9R {
        Rten9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Rising trigger event configuration of line 10"]
    #[inline(always)]
    pub fn rten10(&self) -> Rten10R {
        Rten10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Rising trigger event configuration of line 11"]
    #[inline(always)]
    pub fn rten11(&self) -> Rten11R {
        Rten11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Rising trigger event configuration of line 12"]
    #[inline(always)]
    pub fn rten12(&self) -> Rten12R {
        Rten12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Rising trigger event configuration of line 13"]
    #[inline(always)]
    pub fn rten13(&self) -> Rten13R {
        Rten13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Rising trigger event configuration of line 14"]
    #[inline(always)]
    pub fn rten14(&self) -> Rten14R {
        Rten14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Rising trigger event configuration of line 15"]
    #[inline(always)]
    pub fn rten15(&self) -> Rten15R {
        Rten15R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Rising trigger event configuration of line 16"]
    #[inline(always)]
    pub fn rten16(&self) -> Rten16R {
        Rten16R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Rising trigger event configuration of line 17"]
    #[inline(always)]
    pub fn rten17(&self) -> Rten17R {
        Rten17R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Rising trigger event configuration of line 18"]
    #[inline(always)]
    pub fn rten18(&self) -> Rten18R {
        Rten18R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 21 - Rising trigger event configuration of line 21"]
    #[inline(always)]
    pub fn rten21(&self) -> Rten21R {
        Rten21R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Rising trigger event configuration of line 22"]
    #[inline(always)]
    pub fn rten22(&self) -> Rten22R {
        Rten22R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Rising trigger event configuration of line 0"]
    #[inline(always)]
    pub fn rten0(&mut self) -> Rten0W<'_, RtenSpec> {
        Rten0W::new(self, 0)
    }
    #[doc = "Bit 1 - Rising trigger event configuration of line 1"]
    #[inline(always)]
    pub fn rten1(&mut self) -> Rten1W<'_, RtenSpec> {
        Rten1W::new(self, 1)
    }
    #[doc = "Bit 2 - Rising trigger event configuration of line 2"]
    #[inline(always)]
    pub fn rten2(&mut self) -> Rten2W<'_, RtenSpec> {
        Rten2W::new(self, 2)
    }
    #[doc = "Bit 3 - Rising trigger event configuration of line 3"]
    #[inline(always)]
    pub fn rten3(&mut self) -> Rten3W<'_, RtenSpec> {
        Rten3W::new(self, 3)
    }
    #[doc = "Bit 4 - Rising trigger event configuration of line 4"]
    #[inline(always)]
    pub fn rten4(&mut self) -> Rten4W<'_, RtenSpec> {
        Rten4W::new(self, 4)
    }
    #[doc = "Bit 5 - Rising trigger event configuration of line 5"]
    #[inline(always)]
    pub fn rten5(&mut self) -> Rten5W<'_, RtenSpec> {
        Rten5W::new(self, 5)
    }
    #[doc = "Bit 6 - Rising trigger event configuration of line 6"]
    #[inline(always)]
    pub fn rten6(&mut self) -> Rten6W<'_, RtenSpec> {
        Rten6W::new(self, 6)
    }
    #[doc = "Bit 7 - Rising trigger event configuration of line 7"]
    #[inline(always)]
    pub fn rten7(&mut self) -> Rten7W<'_, RtenSpec> {
        Rten7W::new(self, 7)
    }
    #[doc = "Bit 8 - Rising trigger event configuration of line 8"]
    #[inline(always)]
    pub fn rten8(&mut self) -> Rten8W<'_, RtenSpec> {
        Rten8W::new(self, 8)
    }
    #[doc = "Bit 9 - Rising trigger event configuration of line 9"]
    #[inline(always)]
    pub fn rten9(&mut self) -> Rten9W<'_, RtenSpec> {
        Rten9W::new(self, 9)
    }
    #[doc = "Bit 10 - Rising trigger event configuration of line 10"]
    #[inline(always)]
    pub fn rten10(&mut self) -> Rten10W<'_, RtenSpec> {
        Rten10W::new(self, 10)
    }
    #[doc = "Bit 11 - Rising trigger event configuration of line 11"]
    #[inline(always)]
    pub fn rten11(&mut self) -> Rten11W<'_, RtenSpec> {
        Rten11W::new(self, 11)
    }
    #[doc = "Bit 12 - Rising trigger event configuration of line 12"]
    #[inline(always)]
    pub fn rten12(&mut self) -> Rten12W<'_, RtenSpec> {
        Rten12W::new(self, 12)
    }
    #[doc = "Bit 13 - Rising trigger event configuration of line 13"]
    #[inline(always)]
    pub fn rten13(&mut self) -> Rten13W<'_, RtenSpec> {
        Rten13W::new(self, 13)
    }
    #[doc = "Bit 14 - Rising trigger event configuration of line 14"]
    #[inline(always)]
    pub fn rten14(&mut self) -> Rten14W<'_, RtenSpec> {
        Rten14W::new(self, 14)
    }
    #[doc = "Bit 15 - Rising trigger event configuration of line 15"]
    #[inline(always)]
    pub fn rten15(&mut self) -> Rten15W<'_, RtenSpec> {
        Rten15W::new(self, 15)
    }
    #[doc = "Bit 16 - Rising trigger event configuration of line 16"]
    #[inline(always)]
    pub fn rten16(&mut self) -> Rten16W<'_, RtenSpec> {
        Rten16W::new(self, 16)
    }
    #[doc = "Bit 17 - Rising trigger event configuration of line 17"]
    #[inline(always)]
    pub fn rten17(&mut self) -> Rten17W<'_, RtenSpec> {
        Rten17W::new(self, 17)
    }
    #[doc = "Bit 18 - Rising trigger event configuration of line 18"]
    #[inline(always)]
    pub fn rten18(&mut self) -> Rten18W<'_, RtenSpec> {
        Rten18W::new(self, 18)
    }
    #[doc = "Bit 21 - Rising trigger event configuration of line 21"]
    #[inline(always)]
    pub fn rten21(&mut self) -> Rten21W<'_, RtenSpec> {
        Rten21W::new(self, 21)
    }
    #[doc = "Bit 22 - Rising trigger event configuration of line 22"]
    #[inline(always)]
    pub fn rten22(&mut self) -> Rten22W<'_, RtenSpec> {
        Rten22W::new(self, 22)
    }
}
#[doc = "Rising Trigger selection register\n\nYou can [`read`](crate::Reg::read) this register and get [`rten::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rten::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RtenSpec;
impl crate::RegisterSpec for RtenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rten::R`](R) reader structure"]
impl crate::Readable for RtenSpec {}
#[doc = "`write(|w| ..)` method takes [`rten::W`](W) writer structure"]
impl crate::Writable for RtenSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RTEN to value 0"]
impl crate::Resettable for RtenSpec {}
