#[doc = "Register `OFFSET0` reader"]
pub type R = crate::R<Offset0Spec>;
#[doc = "Register `OFFSET0` writer"]
pub type W = crate::W<Offset0Spec>;
#[doc = "Field `OFFSET0` reader - The data offset compensation corresponding to the channel (single-ended mode INP<0>/differential mode<0>) These bits are written by software, used to define the offset subtracted from the original conversion data when converting the channel (can be regular channel or injection channel) Note: Writing to these bits through software is only allowed when ADSTART=0 and JADSTART=0 (this ensures that no conversion is currently taking place)."]
pub type Offset0R = crate::FieldReader<u16>;
#[doc = "Field `OFFSET0` writer - The data offset compensation corresponding to the channel (single-ended mode INP<0>/differential mode<0>) These bits are written by software, used to define the offset subtracted from the original conversion data when converting the channel (can be regular channel or injection channel) Note: Writing to these bits through software is only allowed when ADSTART=0 and JADSTART=0 (this ensures that no conversion is currently taking place)."]
pub type Offset0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `OFFSET0_EN` reader - This bit is enabled and disabled by software writing for programming into the OFFSET0 bit. Note: Writing to this bit through software is only allowed when ADSTART=0 and JADSTART=0 (this ensures that no conversion is currently taking place)."]
pub type Offset0EnR = crate::BitReader;
#[doc = "Field `OFFSET0_EN` writer - This bit is enabled and disabled by software writing for programming into the OFFSET0 bit. Note: Writing to this bit through software is only allowed when ADSTART=0 and JADSTART=0 (this ensures that no conversion is currently taking place)."]
pub type Offset0EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OFFSET0_POS` reader - POSITIVE OFFSET THIS BIT IS SET AND CLEARED BY SOFTWARE TO ENABLE THE POSITIVE OFFSET. 0: NEGATIVE OFFSET 1: POSITIVE OFFSET NOTE: THE SOFTWARE IS ALLOWED TO WRITE THESE BITS ONLY WHEN ADSTART = 0 AND JADSTART = 0 (WHICH ENSURES THAT NO CONVERSION IS ONGOING)"]
pub type Offset0PosR = crate::BitReader;
#[doc = "Field `OFFSET0_POS` writer - POSITIVE OFFSET THIS BIT IS SET AND CLEARED BY SOFTWARE TO ENABLE THE POSITIVE OFFSET. 0: NEGATIVE OFFSET 1: POSITIVE OFFSET NOTE: THE SOFTWARE IS ALLOWED TO WRITE THESE BITS ONLY WHEN ADSTART = 0 AND JADSTART = 0 (WHICH ENSURES THAT NO CONVERSION IS ONGOING)"]
pub type Offset0PosW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - The data offset compensation corresponding to the channel (single-ended mode INP<0>/differential mode<0>) These bits are written by software, used to define the offset subtracted from the original conversion data when converting the channel (can be regular channel or injection channel) Note: Writing to these bits through software is only allowed when ADSTART=0 and JADSTART=0 (this ensures that no conversion is currently taking place)."]
    #[inline(always)]
    pub fn offset0(&self) -> Offset0R {
        Offset0R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - This bit is enabled and disabled by software writing for programming into the OFFSET0 bit. Note: Writing to this bit through software is only allowed when ADSTART=0 and JADSTART=0 (this ensures that no conversion is currently taking place)."]
    #[inline(always)]
    pub fn offset0_en(&self) -> Offset0EnR {
        Offset0EnR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - POSITIVE OFFSET THIS BIT IS SET AND CLEARED BY SOFTWARE TO ENABLE THE POSITIVE OFFSET. 0: NEGATIVE OFFSET 1: POSITIVE OFFSET NOTE: THE SOFTWARE IS ALLOWED TO WRITE THESE BITS ONLY WHEN ADSTART = 0 AND JADSTART = 0 (WHICH ENSURES THAT NO CONVERSION IS ONGOING)"]
    #[inline(always)]
    pub fn offset0_pos(&self) -> Offset0PosR {
        Offset0PosR::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - The data offset compensation corresponding to the channel (single-ended mode INP<0>/differential mode<0>) These bits are written by software, used to define the offset subtracted from the original conversion data when converting the channel (can be regular channel or injection channel) Note: Writing to these bits through software is only allowed when ADSTART=0 and JADSTART=0 (this ensures that no conversion is currently taking place)."]
    #[inline(always)]
    pub fn offset0(&mut self) -> Offset0W<'_, Offset0Spec> {
        Offset0W::new(self, 0)
    }
    #[doc = "Bit 16 - This bit is enabled and disabled by software writing for programming into the OFFSET0 bit. Note: Writing to this bit through software is only allowed when ADSTART=0 and JADSTART=0 (this ensures that no conversion is currently taking place)."]
    #[inline(always)]
    pub fn offset0_en(&mut self) -> Offset0EnW<'_, Offset0Spec> {
        Offset0EnW::new(self, 16)
    }
    #[doc = "Bit 17 - POSITIVE OFFSET THIS BIT IS SET AND CLEARED BY SOFTWARE TO ENABLE THE POSITIVE OFFSET. 0: NEGATIVE OFFSET 1: POSITIVE OFFSET NOTE: THE SOFTWARE IS ALLOWED TO WRITE THESE BITS ONLY WHEN ADSTART = 0 AND JADSTART = 0 (WHICH ENSURES THAT NO CONVERSION IS ONGOING)"]
    #[inline(always)]
    pub fn offset0_pos(&mut self) -> Offset0PosW<'_, Offset0Spec> {
        Offset0PosW::new(self, 17)
    }
}
#[doc = "ADC offset register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`offset0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`offset0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Offset0Spec;
impl crate::RegisterSpec for Offset0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`offset0::R`](R) reader structure"]
impl crate::Readable for Offset0Spec {}
#[doc = "`write(|w| ..)` method takes [`offset0::W`](W) writer structure"]
impl crate::Writable for Offset0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OFFSET0 to value 0"]
impl crate::Resettable for Offset0Spec {}
