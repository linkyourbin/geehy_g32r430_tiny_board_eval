#[doc = "Register `OFFSET3` reader"]
pub type R = crate::R<Offset3Spec>;
#[doc = "Register `OFFSET3` writer"]
pub type W = crate::W<Offset3Spec>;
#[doc = "Field `OFFSET3` reader - The data offset compensation corresponding to the channel (single-ended mode INP<0>/differential mode<0>) These bits are written by software, used to define the offset subtracted from the original conversion data when converting the channel (can be regular channel or injection channel) Note: Writing to these bits through software is only allowed when ADSTART=0 and JADSTART=0 (this ensures that no conversion is currently taking place)."]
pub type Offset3R = crate::FieldReader<u16>;
#[doc = "Field `OFFSET3` writer - The data offset compensation corresponding to the channel (single-ended mode INP<0>/differential mode<0>) These bits are written by software, used to define the offset subtracted from the original conversion data when converting the channel (can be regular channel or injection channel) Note: Writing to these bits through software is only allowed when ADSTART=0 and JADSTART=0 (this ensures that no conversion is currently taking place)."]
pub type Offset3W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `OFFSET3_EN` reader - This bit is enabled and disabled by software writing for programming into the OFFSET3 bit. Note: Writing to this bit through software is only allowed when ADSTART=0 and JADSTART=0 (this ensures that no conversion is currently taking place)."]
pub type Offset3EnR = crate::BitReader;
#[doc = "Field `OFFSET3_EN` writer - This bit is enabled and disabled by software writing for programming into the OFFSET3 bit. Note: Writing to this bit through software is only allowed when ADSTART=0 and JADSTART=0 (this ensures that no conversion is currently taking place)."]
pub type Offset3EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OFFSET3_POS` reader - POSITIVE OFFSET THIS BIT IS SET AND CLEARED BY SOFTWARE TO ENABLE THE POSITIVE OFFSET. 0: NEGATIVE OFFSET 1: POSITIVE OFFSET NOTE: THE SOFTWARE IS ALLOWED TO WRITE THESE BITS ONLY WHEN ADSTART = 0 AND JADSTART = 0 (WHICH ENSURES THAT NO CONVERSION IS ONGOING)"]
pub type Offset3PosR = crate::BitReader;
#[doc = "Field `OFFSET3_POS` writer - POSITIVE OFFSET THIS BIT IS SET AND CLEARED BY SOFTWARE TO ENABLE THE POSITIVE OFFSET. 0: NEGATIVE OFFSET 1: POSITIVE OFFSET NOTE: THE SOFTWARE IS ALLOWED TO WRITE THESE BITS ONLY WHEN ADSTART = 0 AND JADSTART = 0 (WHICH ENSURES THAT NO CONVERSION IS ONGOING)"]
pub type Offset3PosW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - The data offset compensation corresponding to the channel (single-ended mode INP<0>/differential mode<0>) These bits are written by software, used to define the offset subtracted from the original conversion data when converting the channel (can be regular channel or injection channel) Note: Writing to these bits through software is only allowed when ADSTART=0 and JADSTART=0 (this ensures that no conversion is currently taking place)."]
    #[inline(always)]
    pub fn offset3(&self) -> Offset3R {
        Offset3R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - This bit is enabled and disabled by software writing for programming into the OFFSET3 bit. Note: Writing to this bit through software is only allowed when ADSTART=0 and JADSTART=0 (this ensures that no conversion is currently taking place)."]
    #[inline(always)]
    pub fn offset3_en(&self) -> Offset3EnR {
        Offset3EnR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - POSITIVE OFFSET THIS BIT IS SET AND CLEARED BY SOFTWARE TO ENABLE THE POSITIVE OFFSET. 0: NEGATIVE OFFSET 1: POSITIVE OFFSET NOTE: THE SOFTWARE IS ALLOWED TO WRITE THESE BITS ONLY WHEN ADSTART = 0 AND JADSTART = 0 (WHICH ENSURES THAT NO CONVERSION IS ONGOING)"]
    #[inline(always)]
    pub fn offset3_pos(&self) -> Offset3PosR {
        Offset3PosR::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - The data offset compensation corresponding to the channel (single-ended mode INP<0>/differential mode<0>) These bits are written by software, used to define the offset subtracted from the original conversion data when converting the channel (can be regular channel or injection channel) Note: Writing to these bits through software is only allowed when ADSTART=0 and JADSTART=0 (this ensures that no conversion is currently taking place)."]
    #[inline(always)]
    pub fn offset3(&mut self) -> Offset3W<'_, Offset3Spec> {
        Offset3W::new(self, 0)
    }
    #[doc = "Bit 16 - This bit is enabled and disabled by software writing for programming into the OFFSET3 bit. Note: Writing to this bit through software is only allowed when ADSTART=0 and JADSTART=0 (this ensures that no conversion is currently taking place)."]
    #[inline(always)]
    pub fn offset3_en(&mut self) -> Offset3EnW<'_, Offset3Spec> {
        Offset3EnW::new(self, 16)
    }
    #[doc = "Bit 17 - POSITIVE OFFSET THIS BIT IS SET AND CLEARED BY SOFTWARE TO ENABLE THE POSITIVE OFFSET. 0: NEGATIVE OFFSET 1: POSITIVE OFFSET NOTE: THE SOFTWARE IS ALLOWED TO WRITE THESE BITS ONLY WHEN ADSTART = 0 AND JADSTART = 0 (WHICH ENSURES THAT NO CONVERSION IS ONGOING)"]
    #[inline(always)]
    pub fn offset3_pos(&mut self) -> Offset3PosW<'_, Offset3Spec> {
        Offset3PosW::new(self, 17)
    }
}
#[doc = "ADC offset register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`offset3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`offset3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Offset3Spec;
impl crate::RegisterSpec for Offset3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`offset3::R`](R) reader structure"]
impl crate::Readable for Offset3Spec {}
#[doc = "`write(|w| ..)` method takes [`offset3::W`](W) writer structure"]
impl crate::Writable for Offset3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OFFSET3 to value 0"]
impl crate::Resettable for Offset3Spec {}
