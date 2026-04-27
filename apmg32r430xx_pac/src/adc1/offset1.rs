#[doc = "Register `OFFSET1` reader"]
pub type R = crate::R<Offset1Spec>;
#[doc = "Register `OFFSET1` writer"]
pub type W = crate::W<Offset1Spec>;
#[doc = "Field `OFFSET1` reader - The data offset compensation corresponding to the channel (single-ended mode INP<0>/differential mode<0>) These bits are written by software, used to define the offset subtracted from the original conversion data when converting the channel (can be regular channel or injection channel) Note: Writing to these bits through software is only allowed when ADSTART=0 and JADSTART=0 (this ensures that no conversion is currently taking place)."]
pub type Offset1R = crate::FieldReader<u16>;
#[doc = "Field `OFFSET1` writer - The data offset compensation corresponding to the channel (single-ended mode INP<0>/differential mode<0>) These bits are written by software, used to define the offset subtracted from the original conversion data when converting the channel (can be regular channel or injection channel) Note: Writing to these bits through software is only allowed when ADSTART=0 and JADSTART=0 (this ensures that no conversion is currently taking place)."]
pub type Offset1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `OFFSET1_EN` reader - This bit is enabled and disabled by software writing for programming into the OFFSET1 bit. Note: Writing to this bit through software is only allowed when ADSTART=0 and JADSTART=0 (this ensures that no conversion is currently taking place)."]
pub type Offset1EnR = crate::BitReader;
#[doc = "Field `OFFSET1_EN` writer - This bit is enabled and disabled by software writing for programming into the OFFSET1 bit. Note: Writing to this bit through software is only allowed when ADSTART=0 and JADSTART=0 (this ensures that no conversion is currently taking place)."]
pub type Offset1EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OFFSET1_POS` reader - POSITIVE OFFSET THIS BIT IS SET AND CLEARED BY SOFTWARE TO ENABLE THE POSITIVE OFFSET. 0: NEGATIVE OFFSET 1: POSITIVE OFFSET NOTE: THE SOFTWARE IS ALLOWED TO WRITE THESE BITS ONLY WHEN ADSTART = 0 AND JADSTART = 0 (WHICH ENSURES THAT NO CONVERSION IS ONGOING)"]
pub type Offset1PosR = crate::BitReader;
#[doc = "Field `OFFSET1_POS` writer - POSITIVE OFFSET THIS BIT IS SET AND CLEARED BY SOFTWARE TO ENABLE THE POSITIVE OFFSET. 0: NEGATIVE OFFSET 1: POSITIVE OFFSET NOTE: THE SOFTWARE IS ALLOWED TO WRITE THESE BITS ONLY WHEN ADSTART = 0 AND JADSTART = 0 (WHICH ENSURES THAT NO CONVERSION IS ONGOING)"]
pub type Offset1PosW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - The data offset compensation corresponding to the channel (single-ended mode INP<0>/differential mode<0>) These bits are written by software, used to define the offset subtracted from the original conversion data when converting the channel (can be regular channel or injection channel) Note: Writing to these bits through software is only allowed when ADSTART=0 and JADSTART=0 (this ensures that no conversion is currently taking place)."]
    #[inline(always)]
    pub fn offset1(&self) -> Offset1R {
        Offset1R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - This bit is enabled and disabled by software writing for programming into the OFFSET1 bit. Note: Writing to this bit through software is only allowed when ADSTART=0 and JADSTART=0 (this ensures that no conversion is currently taking place)."]
    #[inline(always)]
    pub fn offset1_en(&self) -> Offset1EnR {
        Offset1EnR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - POSITIVE OFFSET THIS BIT IS SET AND CLEARED BY SOFTWARE TO ENABLE THE POSITIVE OFFSET. 0: NEGATIVE OFFSET 1: POSITIVE OFFSET NOTE: THE SOFTWARE IS ALLOWED TO WRITE THESE BITS ONLY WHEN ADSTART = 0 AND JADSTART = 0 (WHICH ENSURES THAT NO CONVERSION IS ONGOING)"]
    #[inline(always)]
    pub fn offset1_pos(&self) -> Offset1PosR {
        Offset1PosR::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - The data offset compensation corresponding to the channel (single-ended mode INP<0>/differential mode<0>) These bits are written by software, used to define the offset subtracted from the original conversion data when converting the channel (can be regular channel or injection channel) Note: Writing to these bits through software is only allowed when ADSTART=0 and JADSTART=0 (this ensures that no conversion is currently taking place)."]
    #[inline(always)]
    pub fn offset1(&mut self) -> Offset1W<'_, Offset1Spec> {
        Offset1W::new(self, 0)
    }
    #[doc = "Bit 16 - This bit is enabled and disabled by software writing for programming into the OFFSET1 bit. Note: Writing to this bit through software is only allowed when ADSTART=0 and JADSTART=0 (this ensures that no conversion is currently taking place)."]
    #[inline(always)]
    pub fn offset1_en(&mut self) -> Offset1EnW<'_, Offset1Spec> {
        Offset1EnW::new(self, 16)
    }
    #[doc = "Bit 17 - POSITIVE OFFSET THIS BIT IS SET AND CLEARED BY SOFTWARE TO ENABLE THE POSITIVE OFFSET. 0: NEGATIVE OFFSET 1: POSITIVE OFFSET NOTE: THE SOFTWARE IS ALLOWED TO WRITE THESE BITS ONLY WHEN ADSTART = 0 AND JADSTART = 0 (WHICH ENSURES THAT NO CONVERSION IS ONGOING)"]
    #[inline(always)]
    pub fn offset1_pos(&mut self) -> Offset1PosW<'_, Offset1Spec> {
        Offset1PosW::new(self, 17)
    }
}
#[doc = "ADC offset register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`offset1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`offset1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Offset1Spec;
impl crate::RegisterSpec for Offset1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`offset1::R`](R) reader structure"]
impl crate::Readable for Offset1Spec {}
#[doc = "`write(|w| ..)` method takes [`offset1::W`](W) writer structure"]
impl crate::Writable for Offset1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OFFSET1 to value 0"]
impl crate::Resettable for Offset1Spec {}
