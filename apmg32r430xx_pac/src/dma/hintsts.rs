#[doc = "Register `HINTSTS` reader"]
pub type R = crate::R<HintstsSpec>;
#[doc = "Field `FEIFLG4` reader - Stream x FIFO error interrupt flag (x=7..4)"]
pub type Feiflg4R = crate::BitReader;
#[doc = "Field `DMEIFLG4` reader - Stream x direct mode error interrupt flag (x=7..4)"]
pub type Dmeiflg4R = crate::BitReader;
#[doc = "Field `TXEIFLG4` reader - Stream x transfer error interrupt flag (x=7..4)"]
pub type Txeiflg4R = crate::BitReader;
#[doc = "Field `HTXIFLG4` reader - Stream x half transfer interrupt flag (x=7..4)"]
pub type Htxiflg4R = crate::BitReader;
#[doc = "Field `TXCIFLG4` reader - Stream x transfer complete interrupt flag (x=7..4)"]
pub type Txciflg4R = crate::BitReader;
#[doc = "Field `FEIFLG5` reader - Stream x FIFO error interrupt flag (x=7..4)"]
pub type Feiflg5R = crate::BitReader;
#[doc = "Field `DMEIFLG5` reader - Stream x direct mode error interrupt flag (x=7..4)"]
pub type Dmeiflg5R = crate::BitReader;
#[doc = "Field `TXEIFLG5` reader - Stream x transfer error interrupt flag (x=7..4)"]
pub type Txeiflg5R = crate::BitReader;
#[doc = "Field `HTXIFLG5` reader - Stream x half transfer interrupt flag (x=7..4)"]
pub type Htxiflg5R = crate::BitReader;
#[doc = "Field `TXCIFLG5` reader - Stream x transfer complete interrupt flag (x=7..4)"]
pub type Txciflg5R = crate::BitReader;
#[doc = "Field `FEIFLG6` reader - Stream x FIFO error interrupt flag (x=7..4)"]
pub type Feiflg6R = crate::BitReader;
#[doc = "Field `DMEIFLG6` reader - Stream x direct mode error interrupt flag (x=7..4)"]
pub type Dmeiflg6R = crate::BitReader;
#[doc = "Field `TXEIFLG6` reader - Stream x transfer error interrupt flag (x=7..4)"]
pub type Txeiflg6R = crate::BitReader;
#[doc = "Field `HTXIFLG6` reader - Stream x half transfer interrupt flag (x=7..4)"]
pub type Htxiflg6R = crate::BitReader;
#[doc = "Field `TXCIFLG6` reader - Stream x transfer complete interrupt flag (x=7..4)"]
pub type Txciflg6R = crate::BitReader;
#[doc = "Field `FEIFLG7` reader - Stream x FIFO error interrupt flag (x=7..4)"]
pub type Feiflg7R = crate::BitReader;
#[doc = "Field `DMEIFLG7` reader - Stream x direct mode error interrupt flag (x=7..4)"]
pub type Dmeiflg7R = crate::BitReader;
#[doc = "Field `TXEIFLG7` reader - Stream x transfer error interrupt flag (x=7..4)"]
pub type Txeiflg7R = crate::BitReader;
#[doc = "Field `HTXIFLG7` reader - Stream x half transfer interrupt flag (x=7..4)"]
pub type Htxiflg7R = crate::BitReader;
#[doc = "Field `TXCIFLG7` reader - Stream x transfer complete interrupt flag (x=7..4)"]
pub type Txciflg7R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Stream x FIFO error interrupt flag (x=7..4)"]
    #[inline(always)]
    pub fn feiflg4(&self) -> Feiflg4R {
        Feiflg4R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Stream x direct mode error interrupt flag (x=7..4)"]
    #[inline(always)]
    pub fn dmeiflg4(&self) -> Dmeiflg4R {
        Dmeiflg4R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Stream x transfer error interrupt flag (x=7..4)"]
    #[inline(always)]
    pub fn txeiflg4(&self) -> Txeiflg4R {
        Txeiflg4R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Stream x half transfer interrupt flag (x=7..4)"]
    #[inline(always)]
    pub fn htxiflg4(&self) -> Htxiflg4R {
        Htxiflg4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Stream x transfer complete interrupt flag (x=7..4)"]
    #[inline(always)]
    pub fn txciflg4(&self) -> Txciflg4R {
        Txciflg4R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Stream x FIFO error interrupt flag (x=7..4)"]
    #[inline(always)]
    pub fn feiflg5(&self) -> Feiflg5R {
        Feiflg5R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Stream x direct mode error interrupt flag (x=7..4)"]
    #[inline(always)]
    pub fn dmeiflg5(&self) -> Dmeiflg5R {
        Dmeiflg5R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Stream x transfer error interrupt flag (x=7..4)"]
    #[inline(always)]
    pub fn txeiflg5(&self) -> Txeiflg5R {
        Txeiflg5R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Stream x half transfer interrupt flag (x=7..4)"]
    #[inline(always)]
    pub fn htxiflg5(&self) -> Htxiflg5R {
        Htxiflg5R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Stream x transfer complete interrupt flag (x=7..4)"]
    #[inline(always)]
    pub fn txciflg5(&self) -> Txciflg5R {
        Txciflg5R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 16 - Stream x FIFO error interrupt flag (x=7..4)"]
    #[inline(always)]
    pub fn feiflg6(&self) -> Feiflg6R {
        Feiflg6R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 18 - Stream x direct mode error interrupt flag (x=7..4)"]
    #[inline(always)]
    pub fn dmeiflg6(&self) -> Dmeiflg6R {
        Dmeiflg6R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Stream x transfer error interrupt flag (x=7..4)"]
    #[inline(always)]
    pub fn txeiflg6(&self) -> Txeiflg6R {
        Txeiflg6R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Stream x half transfer interrupt flag (x=7..4)"]
    #[inline(always)]
    pub fn htxiflg6(&self) -> Htxiflg6R {
        Htxiflg6R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Stream x transfer complete interrupt flag (x=7..4)"]
    #[inline(always)]
    pub fn txciflg6(&self) -> Txciflg6R {
        Txciflg6R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Stream x FIFO error interrupt flag (x=7..4)"]
    #[inline(always)]
    pub fn feiflg7(&self) -> Feiflg7R {
        Feiflg7R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 24 - Stream x direct mode error interrupt flag (x=7..4)"]
    #[inline(always)]
    pub fn dmeiflg7(&self) -> Dmeiflg7R {
        Dmeiflg7R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Stream x transfer error interrupt flag (x=7..4)"]
    #[inline(always)]
    pub fn txeiflg7(&self) -> Txeiflg7R {
        Txeiflg7R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Stream x half transfer interrupt flag (x=7..4)"]
    #[inline(always)]
    pub fn htxiflg7(&self) -> Htxiflg7R {
        Htxiflg7R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Stream x transfer complete interrupt flag (x=7..4)"]
    #[inline(always)]
    pub fn txciflg7(&self) -> Txciflg7R {
        Txciflg7R::new(((self.bits >> 27) & 1) != 0)
    }
}
#[doc = "high interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`hintsts::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HintstsSpec;
impl crate::RegisterSpec for HintstsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hintsts::R`](R) reader structure"]
impl crate::Readable for HintstsSpec {}
#[doc = "`reset()` method sets HINTSTS to value 0"]
impl crate::Resettable for HintstsSpec {}
