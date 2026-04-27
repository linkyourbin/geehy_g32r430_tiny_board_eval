#[doc = "Register `IMASK` reader"]
pub type R = crate::R<ImaskSpec>;
#[doc = "Register `IMASK` writer"]
pub type W = crate::W<ImaskSpec>;
#[doc = "Field `IMASK0` reader - Interrupt Mask on line 0"]
pub type Imask0R = crate::BitReader;
#[doc = "Field `IMASK0` writer - Interrupt Mask on line 0"]
pub type Imask0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IMASK1` reader - Interrupt Mask on line 1"]
pub type Imask1R = crate::BitReader;
#[doc = "Field `IMASK1` writer - Interrupt Mask on line 1"]
pub type Imask1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IMASK2` reader - Interrupt Mask on line 2"]
pub type Imask2R = crate::BitReader;
#[doc = "Field `IMASK2` writer - Interrupt Mask on line 2"]
pub type Imask2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IMASK3` reader - Interrupt Mask on line 3"]
pub type Imask3R = crate::BitReader;
#[doc = "Field `IMASK3` writer - Interrupt Mask on line 3"]
pub type Imask3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IMASK4` reader - Interrupt Mask on line 4"]
pub type Imask4R = crate::BitReader;
#[doc = "Field `IMASK4` writer - Interrupt Mask on line 4"]
pub type Imask4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IMASK5` reader - Interrupt Mask on line 5"]
pub type Imask5R = crate::BitReader;
#[doc = "Field `IMASK5` writer - Interrupt Mask on line 5"]
pub type Imask5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IMASK6` reader - Interrupt Mask on line 6"]
pub type Imask6R = crate::BitReader;
#[doc = "Field `IMASK6` writer - Interrupt Mask on line 6"]
pub type Imask6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IMASK7` reader - Interrupt Mask on line 7"]
pub type Imask7R = crate::BitReader;
#[doc = "Field `IMASK7` writer - Interrupt Mask on line 7"]
pub type Imask7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IMASK8` reader - Interrupt Mask on line 8"]
pub type Imask8R = crate::BitReader;
#[doc = "Field `IMASK8` writer - Interrupt Mask on line 8"]
pub type Imask8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IMASK9` reader - Interrupt Mask on line 9"]
pub type Imask9R = crate::BitReader;
#[doc = "Field `IMASK9` writer - Interrupt Mask on line 9"]
pub type Imask9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IMASK10` reader - Interrupt Mask on line 10"]
pub type Imask10R = crate::BitReader;
#[doc = "Field `IMASK10` writer - Interrupt Mask on line 10"]
pub type Imask10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IMASK11` reader - Interrupt Mask on line 11"]
pub type Imask11R = crate::BitReader;
#[doc = "Field `IMASK11` writer - Interrupt Mask on line 11"]
pub type Imask11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IMASK12` reader - Interrupt Mask on line 12"]
pub type Imask12R = crate::BitReader;
#[doc = "Field `IMASK12` writer - Interrupt Mask on line 12"]
pub type Imask12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IMASK13` reader - Interrupt Mask on line 13"]
pub type Imask13R = crate::BitReader;
#[doc = "Field `IMASK13` writer - Interrupt Mask on line 13"]
pub type Imask13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IMASK14` reader - Interrupt Mask on line 14"]
pub type Imask14R = crate::BitReader;
#[doc = "Field `IMASK14` writer - Interrupt Mask on line 14"]
pub type Imask14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IMASK15` reader - Interrupt Mask on line 15"]
pub type Imask15R = crate::BitReader;
#[doc = "Field `IMASK15` writer - Interrupt Mask on line 15"]
pub type Imask15W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IMASK16` reader - Interrupt Mask on line 16"]
pub type Imask16R = crate::BitReader;
#[doc = "Field `IMASK16` writer - Interrupt Mask on line 16"]
pub type Imask16W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IMASK17` reader - Interrupt Mask on line 17"]
pub type Imask17R = crate::BitReader;
#[doc = "Field `IMASK17` writer - Interrupt Mask on line 17"]
pub type Imask17W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IMASK18` reader - Interrupt Mask on line 18"]
pub type Imask18R = crate::BitReader;
#[doc = "Field `IMASK18` writer - Interrupt Mask on line 18"]
pub type Imask18W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IMASK21` reader - Interrupt Mask on line 21"]
pub type Imask21R = crate::BitReader;
#[doc = "Field `IMASK21` writer - Interrupt Mask on line 21"]
pub type Imask21W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IMASK22` reader - Interrupt Mask on line 22"]
pub type Imask22R = crate::BitReader;
#[doc = "Field `IMASK22` writer - Interrupt Mask on line 22"]
pub type Imask22W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Interrupt Mask on line 0"]
    #[inline(always)]
    pub fn imask0(&self) -> Imask0R {
        Imask0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Interrupt Mask on line 1"]
    #[inline(always)]
    pub fn imask1(&self) -> Imask1R {
        Imask1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Interrupt Mask on line 2"]
    #[inline(always)]
    pub fn imask2(&self) -> Imask2R {
        Imask2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Interrupt Mask on line 3"]
    #[inline(always)]
    pub fn imask3(&self) -> Imask3R {
        Imask3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Interrupt Mask on line 4"]
    #[inline(always)]
    pub fn imask4(&self) -> Imask4R {
        Imask4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Interrupt Mask on line 5"]
    #[inline(always)]
    pub fn imask5(&self) -> Imask5R {
        Imask5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Interrupt Mask on line 6"]
    #[inline(always)]
    pub fn imask6(&self) -> Imask6R {
        Imask6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Interrupt Mask on line 7"]
    #[inline(always)]
    pub fn imask7(&self) -> Imask7R {
        Imask7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Interrupt Mask on line 8"]
    #[inline(always)]
    pub fn imask8(&self) -> Imask8R {
        Imask8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Interrupt Mask on line 9"]
    #[inline(always)]
    pub fn imask9(&self) -> Imask9R {
        Imask9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Interrupt Mask on line 10"]
    #[inline(always)]
    pub fn imask10(&self) -> Imask10R {
        Imask10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Interrupt Mask on line 11"]
    #[inline(always)]
    pub fn imask11(&self) -> Imask11R {
        Imask11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Interrupt Mask on line 12"]
    #[inline(always)]
    pub fn imask12(&self) -> Imask12R {
        Imask12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Interrupt Mask on line 13"]
    #[inline(always)]
    pub fn imask13(&self) -> Imask13R {
        Imask13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Interrupt Mask on line 14"]
    #[inline(always)]
    pub fn imask14(&self) -> Imask14R {
        Imask14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Interrupt Mask on line 15"]
    #[inline(always)]
    pub fn imask15(&self) -> Imask15R {
        Imask15R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Interrupt Mask on line 16"]
    #[inline(always)]
    pub fn imask16(&self) -> Imask16R {
        Imask16R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Interrupt Mask on line 17"]
    #[inline(always)]
    pub fn imask17(&self) -> Imask17R {
        Imask17R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Interrupt Mask on line 18"]
    #[inline(always)]
    pub fn imask18(&self) -> Imask18R {
        Imask18R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 21 - Interrupt Mask on line 21"]
    #[inline(always)]
    pub fn imask21(&self) -> Imask21R {
        Imask21R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Interrupt Mask on line 22"]
    #[inline(always)]
    pub fn imask22(&self) -> Imask22R {
        Imask22R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Interrupt Mask on line 0"]
    #[inline(always)]
    pub fn imask0(&mut self) -> Imask0W<'_, ImaskSpec> {
        Imask0W::new(self, 0)
    }
    #[doc = "Bit 1 - Interrupt Mask on line 1"]
    #[inline(always)]
    pub fn imask1(&mut self) -> Imask1W<'_, ImaskSpec> {
        Imask1W::new(self, 1)
    }
    #[doc = "Bit 2 - Interrupt Mask on line 2"]
    #[inline(always)]
    pub fn imask2(&mut self) -> Imask2W<'_, ImaskSpec> {
        Imask2W::new(self, 2)
    }
    #[doc = "Bit 3 - Interrupt Mask on line 3"]
    #[inline(always)]
    pub fn imask3(&mut self) -> Imask3W<'_, ImaskSpec> {
        Imask3W::new(self, 3)
    }
    #[doc = "Bit 4 - Interrupt Mask on line 4"]
    #[inline(always)]
    pub fn imask4(&mut self) -> Imask4W<'_, ImaskSpec> {
        Imask4W::new(self, 4)
    }
    #[doc = "Bit 5 - Interrupt Mask on line 5"]
    #[inline(always)]
    pub fn imask5(&mut self) -> Imask5W<'_, ImaskSpec> {
        Imask5W::new(self, 5)
    }
    #[doc = "Bit 6 - Interrupt Mask on line 6"]
    #[inline(always)]
    pub fn imask6(&mut self) -> Imask6W<'_, ImaskSpec> {
        Imask6W::new(self, 6)
    }
    #[doc = "Bit 7 - Interrupt Mask on line 7"]
    #[inline(always)]
    pub fn imask7(&mut self) -> Imask7W<'_, ImaskSpec> {
        Imask7W::new(self, 7)
    }
    #[doc = "Bit 8 - Interrupt Mask on line 8"]
    #[inline(always)]
    pub fn imask8(&mut self) -> Imask8W<'_, ImaskSpec> {
        Imask8W::new(self, 8)
    }
    #[doc = "Bit 9 - Interrupt Mask on line 9"]
    #[inline(always)]
    pub fn imask9(&mut self) -> Imask9W<'_, ImaskSpec> {
        Imask9W::new(self, 9)
    }
    #[doc = "Bit 10 - Interrupt Mask on line 10"]
    #[inline(always)]
    pub fn imask10(&mut self) -> Imask10W<'_, ImaskSpec> {
        Imask10W::new(self, 10)
    }
    #[doc = "Bit 11 - Interrupt Mask on line 11"]
    #[inline(always)]
    pub fn imask11(&mut self) -> Imask11W<'_, ImaskSpec> {
        Imask11W::new(self, 11)
    }
    #[doc = "Bit 12 - Interrupt Mask on line 12"]
    #[inline(always)]
    pub fn imask12(&mut self) -> Imask12W<'_, ImaskSpec> {
        Imask12W::new(self, 12)
    }
    #[doc = "Bit 13 - Interrupt Mask on line 13"]
    #[inline(always)]
    pub fn imask13(&mut self) -> Imask13W<'_, ImaskSpec> {
        Imask13W::new(self, 13)
    }
    #[doc = "Bit 14 - Interrupt Mask on line 14"]
    #[inline(always)]
    pub fn imask14(&mut self) -> Imask14W<'_, ImaskSpec> {
        Imask14W::new(self, 14)
    }
    #[doc = "Bit 15 - Interrupt Mask on line 15"]
    #[inline(always)]
    pub fn imask15(&mut self) -> Imask15W<'_, ImaskSpec> {
        Imask15W::new(self, 15)
    }
    #[doc = "Bit 16 - Interrupt Mask on line 16"]
    #[inline(always)]
    pub fn imask16(&mut self) -> Imask16W<'_, ImaskSpec> {
        Imask16W::new(self, 16)
    }
    #[doc = "Bit 17 - Interrupt Mask on line 17"]
    #[inline(always)]
    pub fn imask17(&mut self) -> Imask17W<'_, ImaskSpec> {
        Imask17W::new(self, 17)
    }
    #[doc = "Bit 18 - Interrupt Mask on line 18"]
    #[inline(always)]
    pub fn imask18(&mut self) -> Imask18W<'_, ImaskSpec> {
        Imask18W::new(self, 18)
    }
    #[doc = "Bit 21 - Interrupt Mask on line 21"]
    #[inline(always)]
    pub fn imask21(&mut self) -> Imask21W<'_, ImaskSpec> {
        Imask21W::new(self, 21)
    }
    #[doc = "Bit 22 - Interrupt Mask on line 22"]
    #[inline(always)]
    pub fn imask22(&mut self) -> Imask22W<'_, ImaskSpec> {
        Imask22W::new(self, 22)
    }
}
#[doc = "Interrupt mask register\n\nYou can [`read`](crate::Reg::read) this register and get [`imask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`imask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ImaskSpec;
impl crate::RegisterSpec for ImaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`imask::R`](R) reader structure"]
impl crate::Readable for ImaskSpec {}
#[doc = "`write(|w| ..)` method takes [`imask::W`](W) writer structure"]
impl crate::Writable for ImaskSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IMASK to value 0"]
impl crate::Resettable for ImaskSpec {}
