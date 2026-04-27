#[doc = "Register `EMASK` reader"]
pub type R = crate::R<EmaskSpec>;
#[doc = "Register `EMASK` writer"]
pub type W = crate::W<EmaskSpec>;
#[doc = "Field `EMASK0` reader - Event Mask on line 0"]
pub type Emask0R = crate::BitReader;
#[doc = "Field `EMASK0` writer - Event Mask on line 0"]
pub type Emask0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EMASK1` reader - Event Mask on line 1"]
pub type Emask1R = crate::BitReader;
#[doc = "Field `EMASK1` writer - Event Mask on line 1"]
pub type Emask1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EMASK2` reader - Event Mask on line 2"]
pub type Emask2R = crate::BitReader;
#[doc = "Field `EMASK2` writer - Event Mask on line 2"]
pub type Emask2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EMASK3` reader - Event Mask on line 3"]
pub type Emask3R = crate::BitReader;
#[doc = "Field `EMASK3` writer - Event Mask on line 3"]
pub type Emask3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EMASK4` reader - Event Mask on line 4"]
pub type Emask4R = crate::BitReader;
#[doc = "Field `EMASK4` writer - Event Mask on line 4"]
pub type Emask4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EMASK5` reader - Event Mask on line 5"]
pub type Emask5R = crate::BitReader;
#[doc = "Field `EMASK5` writer - Event Mask on line 5"]
pub type Emask5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EMASK6` reader - Event Mask on line 6"]
pub type Emask6R = crate::BitReader;
#[doc = "Field `EMASK6` writer - Event Mask on line 6"]
pub type Emask6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EMASK7` reader - Event Mask on line 7"]
pub type Emask7R = crate::BitReader;
#[doc = "Field `EMASK7` writer - Event Mask on line 7"]
pub type Emask7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EMASK8` reader - Event Mask on line 8"]
pub type Emask8R = crate::BitReader;
#[doc = "Field `EMASK8` writer - Event Mask on line 8"]
pub type Emask8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EMASK9` reader - Event Mask on line 9"]
pub type Emask9R = crate::BitReader;
#[doc = "Field `EMASK9` writer - Event Mask on line 9"]
pub type Emask9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EMASK10` reader - Event Mask on line 10"]
pub type Emask10R = crate::BitReader;
#[doc = "Field `EMASK10` writer - Event Mask on line 10"]
pub type Emask10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EMASK11` reader - Event Mask on line 11"]
pub type Emask11R = crate::BitReader;
#[doc = "Field `EMASK11` writer - Event Mask on line 11"]
pub type Emask11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EMASK12` reader - Event Mask on line 12"]
pub type Emask12R = crate::BitReader;
#[doc = "Field `EMASK12` writer - Event Mask on line 12"]
pub type Emask12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EMASK13` reader - Event Mask on line 13"]
pub type Emask13R = crate::BitReader;
#[doc = "Field `EMASK13` writer - Event Mask on line 13"]
pub type Emask13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EMASK14` reader - Event Mask on line 14"]
pub type Emask14R = crate::BitReader;
#[doc = "Field `EMASK14` writer - Event Mask on line 14"]
pub type Emask14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EMASK15` reader - Event Mask on line 15"]
pub type Emask15R = crate::BitReader;
#[doc = "Field `EMASK15` writer - Event Mask on line 15"]
pub type Emask15W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EMASK16` reader - Event Mask on line 16"]
pub type Emask16R = crate::BitReader;
#[doc = "Field `EMASK16` writer - Event Mask on line 16"]
pub type Emask16W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EMASK17` reader - Event Mask on line 17"]
pub type Emask17R = crate::BitReader;
#[doc = "Field `EMASK17` writer - Event Mask on line 17"]
pub type Emask17W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EMASK18` reader - Event Mask on line 18"]
pub type Emask18R = crate::BitReader;
#[doc = "Field `EMASK18` writer - Event Mask on line 18"]
pub type Emask18W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EMASK21` reader - Event Mask on line 21"]
pub type Emask21R = crate::BitReader;
#[doc = "Field `EMASK21` writer - Event Mask on line 21"]
pub type Emask21W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EMASK22` reader - Event Mask on line 22"]
pub type Emask22R = crate::BitReader;
#[doc = "Field `EMASK22` writer - Event Mask on line 22"]
pub type Emask22W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Event Mask on line 0"]
    #[inline(always)]
    pub fn emask0(&self) -> Emask0R {
        Emask0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Event Mask on line 1"]
    #[inline(always)]
    pub fn emask1(&self) -> Emask1R {
        Emask1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Event Mask on line 2"]
    #[inline(always)]
    pub fn emask2(&self) -> Emask2R {
        Emask2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Event Mask on line 3"]
    #[inline(always)]
    pub fn emask3(&self) -> Emask3R {
        Emask3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Event Mask on line 4"]
    #[inline(always)]
    pub fn emask4(&self) -> Emask4R {
        Emask4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Event Mask on line 5"]
    #[inline(always)]
    pub fn emask5(&self) -> Emask5R {
        Emask5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Event Mask on line 6"]
    #[inline(always)]
    pub fn emask6(&self) -> Emask6R {
        Emask6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Event Mask on line 7"]
    #[inline(always)]
    pub fn emask7(&self) -> Emask7R {
        Emask7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Event Mask on line 8"]
    #[inline(always)]
    pub fn emask8(&self) -> Emask8R {
        Emask8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Event Mask on line 9"]
    #[inline(always)]
    pub fn emask9(&self) -> Emask9R {
        Emask9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Event Mask on line 10"]
    #[inline(always)]
    pub fn emask10(&self) -> Emask10R {
        Emask10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Event Mask on line 11"]
    #[inline(always)]
    pub fn emask11(&self) -> Emask11R {
        Emask11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Event Mask on line 12"]
    #[inline(always)]
    pub fn emask12(&self) -> Emask12R {
        Emask12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Event Mask on line 13"]
    #[inline(always)]
    pub fn emask13(&self) -> Emask13R {
        Emask13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Event Mask on line 14"]
    #[inline(always)]
    pub fn emask14(&self) -> Emask14R {
        Emask14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Event Mask on line 15"]
    #[inline(always)]
    pub fn emask15(&self) -> Emask15R {
        Emask15R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Event Mask on line 16"]
    #[inline(always)]
    pub fn emask16(&self) -> Emask16R {
        Emask16R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Event Mask on line 17"]
    #[inline(always)]
    pub fn emask17(&self) -> Emask17R {
        Emask17R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Event Mask on line 18"]
    #[inline(always)]
    pub fn emask18(&self) -> Emask18R {
        Emask18R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 21 - Event Mask on line 21"]
    #[inline(always)]
    pub fn emask21(&self) -> Emask21R {
        Emask21R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Event Mask on line 22"]
    #[inline(always)]
    pub fn emask22(&self) -> Emask22R {
        Emask22R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Event Mask on line 0"]
    #[inline(always)]
    pub fn emask0(&mut self) -> Emask0W<'_, EmaskSpec> {
        Emask0W::new(self, 0)
    }
    #[doc = "Bit 1 - Event Mask on line 1"]
    #[inline(always)]
    pub fn emask1(&mut self) -> Emask1W<'_, EmaskSpec> {
        Emask1W::new(self, 1)
    }
    #[doc = "Bit 2 - Event Mask on line 2"]
    #[inline(always)]
    pub fn emask2(&mut self) -> Emask2W<'_, EmaskSpec> {
        Emask2W::new(self, 2)
    }
    #[doc = "Bit 3 - Event Mask on line 3"]
    #[inline(always)]
    pub fn emask3(&mut self) -> Emask3W<'_, EmaskSpec> {
        Emask3W::new(self, 3)
    }
    #[doc = "Bit 4 - Event Mask on line 4"]
    #[inline(always)]
    pub fn emask4(&mut self) -> Emask4W<'_, EmaskSpec> {
        Emask4W::new(self, 4)
    }
    #[doc = "Bit 5 - Event Mask on line 5"]
    #[inline(always)]
    pub fn emask5(&mut self) -> Emask5W<'_, EmaskSpec> {
        Emask5W::new(self, 5)
    }
    #[doc = "Bit 6 - Event Mask on line 6"]
    #[inline(always)]
    pub fn emask6(&mut self) -> Emask6W<'_, EmaskSpec> {
        Emask6W::new(self, 6)
    }
    #[doc = "Bit 7 - Event Mask on line 7"]
    #[inline(always)]
    pub fn emask7(&mut self) -> Emask7W<'_, EmaskSpec> {
        Emask7W::new(self, 7)
    }
    #[doc = "Bit 8 - Event Mask on line 8"]
    #[inline(always)]
    pub fn emask8(&mut self) -> Emask8W<'_, EmaskSpec> {
        Emask8W::new(self, 8)
    }
    #[doc = "Bit 9 - Event Mask on line 9"]
    #[inline(always)]
    pub fn emask9(&mut self) -> Emask9W<'_, EmaskSpec> {
        Emask9W::new(self, 9)
    }
    #[doc = "Bit 10 - Event Mask on line 10"]
    #[inline(always)]
    pub fn emask10(&mut self) -> Emask10W<'_, EmaskSpec> {
        Emask10W::new(self, 10)
    }
    #[doc = "Bit 11 - Event Mask on line 11"]
    #[inline(always)]
    pub fn emask11(&mut self) -> Emask11W<'_, EmaskSpec> {
        Emask11W::new(self, 11)
    }
    #[doc = "Bit 12 - Event Mask on line 12"]
    #[inline(always)]
    pub fn emask12(&mut self) -> Emask12W<'_, EmaskSpec> {
        Emask12W::new(self, 12)
    }
    #[doc = "Bit 13 - Event Mask on line 13"]
    #[inline(always)]
    pub fn emask13(&mut self) -> Emask13W<'_, EmaskSpec> {
        Emask13W::new(self, 13)
    }
    #[doc = "Bit 14 - Event Mask on line 14"]
    #[inline(always)]
    pub fn emask14(&mut self) -> Emask14W<'_, EmaskSpec> {
        Emask14W::new(self, 14)
    }
    #[doc = "Bit 15 - Event Mask on line 15"]
    #[inline(always)]
    pub fn emask15(&mut self) -> Emask15W<'_, EmaskSpec> {
        Emask15W::new(self, 15)
    }
    #[doc = "Bit 16 - Event Mask on line 16"]
    #[inline(always)]
    pub fn emask16(&mut self) -> Emask16W<'_, EmaskSpec> {
        Emask16W::new(self, 16)
    }
    #[doc = "Bit 17 - Event Mask on line 17"]
    #[inline(always)]
    pub fn emask17(&mut self) -> Emask17W<'_, EmaskSpec> {
        Emask17W::new(self, 17)
    }
    #[doc = "Bit 18 - Event Mask on line 18"]
    #[inline(always)]
    pub fn emask18(&mut self) -> Emask18W<'_, EmaskSpec> {
        Emask18W::new(self, 18)
    }
    #[doc = "Bit 21 - Event Mask on line 21"]
    #[inline(always)]
    pub fn emask21(&mut self) -> Emask21W<'_, EmaskSpec> {
        Emask21W::new(self, 21)
    }
    #[doc = "Bit 22 - Event Mask on line 22"]
    #[inline(always)]
    pub fn emask22(&mut self) -> Emask22W<'_, EmaskSpec> {
        Emask22W::new(self, 22)
    }
}
#[doc = "Event mask register\n\nYou can [`read`](crate::Reg::read) this register and get [`emask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`emask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EmaskSpec;
impl crate::RegisterSpec for EmaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`emask::R`](R) reader structure"]
impl crate::Readable for EmaskSpec {}
#[doc = "`write(|w| ..)` method takes [`emask::W`](W) writer structure"]
impl crate::Writable for EmaskSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EMASK to value 0"]
impl crate::Resettable for EmaskSpec {}
