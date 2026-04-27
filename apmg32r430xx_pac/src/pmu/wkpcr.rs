#[doc = "Register `WKPCR` reader"]
pub type R = crate::R<WkpcrSpec>;
#[doc = "Register `WKPCR` writer"]
pub type W = crate::W<WkpcrSpec>;
#[doc = "Field `WKUPEN0` reader - Activate PC1-WUIO Enable"]
pub type Wkupen0R = crate::BitReader;
#[doc = "Field `WKUPEN0` writer - Activate PC1-WUIO Enable"]
pub type Wkupen0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WKUPEN1` reader - Activate PC2-WUIO Enable"]
pub type Wkupen1R = crate::BitReader;
#[doc = "Field `WKUPEN1` writer - Activate PC2-WUIO Enable"]
pub type Wkupen1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WKUPEN2` reader - Activate PD0-WUIO Enable"]
pub type Wkupen2R = crate::BitReader;
#[doc = "Field `WKUPEN2` writer - Activate PD0-WUIO Enable"]
pub type Wkupen2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WKUPEN3` reader - Activate PD9-WUIO Enable"]
pub type Wkupen3R = crate::BitReader;
#[doc = "Field `WKUPEN3` writer - Activate PD9-WUIO Enable"]
pub type Wkupen3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WKUPPOL0` reader - The Wake-up Polarity of PC1-WUIO"]
pub type Wkuppol0R = crate::FieldReader;
#[doc = "Field `WKUPPOL0` writer - The Wake-up Polarity of PC1-WUIO"]
pub type Wkuppol0W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `WKUPPOL1` reader - The Wake-up Polarity of PC2-WUIO"]
pub type Wkuppol1R = crate::FieldReader;
#[doc = "Field `WKUPPOL1` writer - The Wake-up Polarity of PC2-WUIO"]
pub type Wkuppol1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `WKUPPOL2` reader - The Wake-up Polarity of PD0-WUIO"]
pub type Wkuppol2R = crate::FieldReader;
#[doc = "Field `WKUPPOL3` reader - The Wake-up Polarity of PD9-WUIO"]
pub type Wkuppol3R = crate::FieldReader;
#[doc = "Field `WKUPPOL3` writer - The Wake-up Polarity of PD9-WUIO"]
pub type Wkuppol3W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `APC` reader - Select the status of GPIO when entering standby mode."]
pub type ApcR = crate::BitReader;
#[doc = "Field `APC` writer - Select the status of GPIO when entering standby mode."]
pub type ApcW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Activate PC1-WUIO Enable"]
    #[inline(always)]
    pub fn wkupen0(&self) -> Wkupen0R {
        Wkupen0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Activate PC2-WUIO Enable"]
    #[inline(always)]
    pub fn wkupen1(&self) -> Wkupen1R {
        Wkupen1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Activate PD0-WUIO Enable"]
    #[inline(always)]
    pub fn wkupen2(&self) -> Wkupen2R {
        Wkupen2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Activate PD9-WUIO Enable"]
    #[inline(always)]
    pub fn wkupen3(&self) -> Wkupen3R {
        Wkupen3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - The Wake-up Polarity of PC1-WUIO"]
    #[inline(always)]
    pub fn wkuppol0(&self) -> Wkuppol0R {
        Wkuppol0R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - The Wake-up Polarity of PC2-WUIO"]
    #[inline(always)]
    pub fn wkuppol1(&self) -> Wkuppol1R {
        Wkuppol1R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - The Wake-up Polarity of PD0-WUIO"]
    #[inline(always)]
    pub fn wkuppol2(&self) -> Wkuppol2R {
        Wkuppol2R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - The Wake-up Polarity of PD9-WUIO"]
    #[inline(always)]
    pub fn wkuppol3(&self) -> Wkuppol3R {
        Wkuppol3R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 15 - Select the status of GPIO when entering standby mode."]
    #[inline(always)]
    pub fn apc(&self) -> ApcR {
        ApcR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Activate PC1-WUIO Enable"]
    #[inline(always)]
    pub fn wkupen0(&mut self) -> Wkupen0W<'_, WkpcrSpec> {
        Wkupen0W::new(self, 0)
    }
    #[doc = "Bit 1 - Activate PC2-WUIO Enable"]
    #[inline(always)]
    pub fn wkupen1(&mut self) -> Wkupen1W<'_, WkpcrSpec> {
        Wkupen1W::new(self, 1)
    }
    #[doc = "Bit 2 - Activate PD0-WUIO Enable"]
    #[inline(always)]
    pub fn wkupen2(&mut self) -> Wkupen2W<'_, WkpcrSpec> {
        Wkupen2W::new(self, 2)
    }
    #[doc = "Bit 3 - Activate PD9-WUIO Enable"]
    #[inline(always)]
    pub fn wkupen3(&mut self) -> Wkupen3W<'_, WkpcrSpec> {
        Wkupen3W::new(self, 3)
    }
    #[doc = "Bits 4:5 - The Wake-up Polarity of PC1-WUIO"]
    #[inline(always)]
    pub fn wkuppol0(&mut self) -> Wkuppol0W<'_, WkpcrSpec> {
        Wkuppol0W::new(self, 4)
    }
    #[doc = "Bits 6:7 - The Wake-up Polarity of PC2-WUIO"]
    #[inline(always)]
    pub fn wkuppol1(&mut self) -> Wkuppol1W<'_, WkpcrSpec> {
        Wkuppol1W::new(self, 6)
    }
    #[doc = "Bits 10:11 - The Wake-up Polarity of PD9-WUIO"]
    #[inline(always)]
    pub fn wkuppol3(&mut self) -> Wkuppol3W<'_, WkpcrSpec> {
        Wkuppol3W::new(self, 10)
    }
    #[doc = "Bit 15 - Select the status of GPIO when entering standby mode."]
    #[inline(always)]
    pub fn apc(&mut self) -> ApcW<'_, WkpcrSpec> {
        ApcW::new(self, 15)
    }
}
#[doc = "Low Power Consumption Wake-Up Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`wkpcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wkpcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WkpcrSpec;
impl crate::RegisterSpec for WkpcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wkpcr::R`](R) reader structure"]
impl crate::Readable for WkpcrSpec {}
#[doc = "`write(|w| ..)` method takes [`wkpcr::W`](W) writer structure"]
impl crate::Writable for WkpcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WKPCR to value 0"]
impl crate::Resettable for WkpcrSpec {}
