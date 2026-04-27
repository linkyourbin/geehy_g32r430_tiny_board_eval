#[doc = "Register `LINTSTS` reader"]
pub type R = crate::R<LintstsSpec>;
#[doc = "Field `FEIFLG0` reader - Stream x FIFO error interrupt flag (x=3..0)"]
pub type Feiflg0R = crate::BitReader;
#[doc = "Field `DMEIFLG0` reader - Stream x direct mode error interrupt flag (x=3..0)"]
pub type Dmeiflg0R = crate::BitReader;
#[doc = "Field `TXEIFLG0` reader - Stream x transfer error interrupt flag (x=3..0)"]
pub type Txeiflg0R = crate::BitReader;
#[doc = "Field `HTXIFLG0` reader - Stream x half transfer interrupt flag (x=3..0)"]
pub type Htxiflg0R = crate::BitReader;
#[doc = "Field `TXCIFLG0` reader - Stream x transfer complete interrupt flag (x = 3..0)"]
pub type Txciflg0R = crate::BitReader;
#[doc = "Field `FEIFLG1` reader - Stream x FIFO error interrupt flag (x=3..0)"]
pub type Feiflg1R = crate::BitReader;
#[doc = "Field `DMEIFLG1` reader - Stream x direct mode error interrupt flag (x=3..0)"]
pub type Dmeiflg1R = crate::BitReader;
#[doc = "Field `TXEIFLG1` reader - Stream x transfer error interrupt flag (x=3..0)"]
pub type Txeiflg1R = crate::BitReader;
#[doc = "Field `HTXIFLG1` reader - Stream x half transfer interrupt flag (x=3..0)"]
pub type Htxiflg1R = crate::BitReader;
#[doc = "Field `TXCIFLG1` reader - Stream x transfer complete interrupt flag (x = 3..0)"]
pub type Txciflg1R = crate::BitReader;
#[doc = "Field `FEIFLG2` reader - Stream x FIFO error interrupt flag (x=3..0)"]
pub type Feiflg2R = crate::BitReader;
#[doc = "Field `DMEIFLG2` reader - Stream x direct mode error interrupt flag (x=3..0)"]
pub type Dmeiflg2R = crate::BitReader;
#[doc = "Field `TXEIFLG2` reader - Stream x transfer error interrupt flag (x=3..0)"]
pub type Txeiflg2R = crate::BitReader;
#[doc = "Field `HTXIFLG2` reader - Stream x half transfer interrupt flag (x=3..0)"]
pub type Htxiflg2R = crate::BitReader;
#[doc = "Field `TXCIFLG2` reader - Stream x transfer complete interrupt flag (x = 3..0)"]
pub type Txciflg2R = crate::BitReader;
#[doc = "Field `FEIFLG3` reader - Stream x FIFO error interrupt flag (x=3..0)"]
pub type Feiflg3R = crate::BitReader;
#[doc = "Field `DMEIFLG3` reader - Stream x direct mode error interrupt flag (x=3..0)"]
pub type Dmeiflg3R = crate::BitReader;
#[doc = "Field `TXEIFLG3` reader - Stream x transfer error interrupt flag (x=3..0)"]
pub type Txeiflg3R = crate::BitReader;
#[doc = "Field `HTXIFLG3` reader - Stream x half transfer interrupt flag (x=3..0)"]
pub type Htxiflg3R = crate::BitReader;
#[doc = "Field `TXCIFLG3` reader - Stream x transfer complete interrupt flag (x = 3..0)"]
pub type Txciflg3R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Stream x FIFO error interrupt flag (x=3..0)"]
    #[inline(always)]
    pub fn feiflg0(&self) -> Feiflg0R {
        Feiflg0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Stream x direct mode error interrupt flag (x=3..0)"]
    #[inline(always)]
    pub fn dmeiflg0(&self) -> Dmeiflg0R {
        Dmeiflg0R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Stream x transfer error interrupt flag (x=3..0)"]
    #[inline(always)]
    pub fn txeiflg0(&self) -> Txeiflg0R {
        Txeiflg0R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Stream x half transfer interrupt flag (x=3..0)"]
    #[inline(always)]
    pub fn htxiflg0(&self) -> Htxiflg0R {
        Htxiflg0R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Stream x transfer complete interrupt flag (x = 3..0)"]
    #[inline(always)]
    pub fn txciflg0(&self) -> Txciflg0R {
        Txciflg0R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Stream x FIFO error interrupt flag (x=3..0)"]
    #[inline(always)]
    pub fn feiflg1(&self) -> Feiflg1R {
        Feiflg1R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Stream x direct mode error interrupt flag (x=3..0)"]
    #[inline(always)]
    pub fn dmeiflg1(&self) -> Dmeiflg1R {
        Dmeiflg1R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Stream x transfer error interrupt flag (x=3..0)"]
    #[inline(always)]
    pub fn txeiflg1(&self) -> Txeiflg1R {
        Txeiflg1R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Stream x half transfer interrupt flag (x=3..0)"]
    #[inline(always)]
    pub fn htxiflg1(&self) -> Htxiflg1R {
        Htxiflg1R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Stream x transfer complete interrupt flag (x = 3..0)"]
    #[inline(always)]
    pub fn txciflg1(&self) -> Txciflg1R {
        Txciflg1R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 16 - Stream x FIFO error interrupt flag (x=3..0)"]
    #[inline(always)]
    pub fn feiflg2(&self) -> Feiflg2R {
        Feiflg2R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 18 - Stream x direct mode error interrupt flag (x=3..0)"]
    #[inline(always)]
    pub fn dmeiflg2(&self) -> Dmeiflg2R {
        Dmeiflg2R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Stream x transfer error interrupt flag (x=3..0)"]
    #[inline(always)]
    pub fn txeiflg2(&self) -> Txeiflg2R {
        Txeiflg2R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Stream x half transfer interrupt flag (x=3..0)"]
    #[inline(always)]
    pub fn htxiflg2(&self) -> Htxiflg2R {
        Htxiflg2R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Stream x transfer complete interrupt flag (x = 3..0)"]
    #[inline(always)]
    pub fn txciflg2(&self) -> Txciflg2R {
        Txciflg2R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Stream x FIFO error interrupt flag (x=3..0)"]
    #[inline(always)]
    pub fn feiflg3(&self) -> Feiflg3R {
        Feiflg3R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 24 - Stream x direct mode error interrupt flag (x=3..0)"]
    #[inline(always)]
    pub fn dmeiflg3(&self) -> Dmeiflg3R {
        Dmeiflg3R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Stream x transfer error interrupt flag (x=3..0)"]
    #[inline(always)]
    pub fn txeiflg3(&self) -> Txeiflg3R {
        Txeiflg3R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Stream x half transfer interrupt flag (x=3..0)"]
    #[inline(always)]
    pub fn htxiflg3(&self) -> Htxiflg3R {
        Htxiflg3R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Stream x transfer complete interrupt flag (x = 3..0)"]
    #[inline(always)]
    pub fn txciflg3(&self) -> Txciflg3R {
        Txciflg3R::new(((self.bits >> 27) & 1) != 0)
    }
}
#[doc = "low interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`lintsts::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LintstsSpec;
impl crate::RegisterSpec for LintstsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lintsts::R`](R) reader structure"]
impl crate::Readable for LintstsSpec {}
#[doc = "`reset()` method sets LINTSTS to value 0"]
impl crate::Resettable for LintstsSpec {}
