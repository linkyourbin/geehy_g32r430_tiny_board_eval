#[doc = "Register `OFFSET4` reader"]
pub type R = crate::R<Offset4Spec>;
#[doc = "Register `OFFSET4` writer"]
pub type W = crate::W<Offset4Spec>;
#[doc = "Field `OFFSET4` reader - The data offset compensation corresponding to the channel (single-ended mode INP<0>/differential mode<0>) These bits are written by software, used to define the offset subtracted from the original conversion data when converting the channel (can be regular channel or injection channel) Note: Writing to these bits through software is only allowed when ADSTART=0 and JADSTART=0 (this ensures that no conversion is currently taking place)."]
pub type Offset4R = crate::FieldReader<u16>;
#[doc = "Field `OFFSET4` writer - The data offset compensation corresponding to the channel (single-ended mode INP<0>/differential mode<0>) These bits are written by software, used to define the offset subtracted from the original conversion data when converting the channel (can be regular channel or injection channel) Note: Writing to these bits through software is only allowed when ADSTART=0 and JADSTART=0 (this ensures that no conversion is currently taking place)."]
pub type Offset4W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `OFFSET4_EN` reader - This bit is enabled and disabled by software writing for programming into the OFFSET4 bit. Note: Writing to this bit through software is only allowed when ADSTART=0 and JADSTART=0 (this ensures that no conversion is currently taking place)."]
pub type Offset4EnR = crate::BitReader;
#[doc = "Field `OFFSET4_EN` writer - This bit is enabled and disabled by software writing for programming into the OFFSET4 bit. Note: Writing to this bit through software is only allowed when ADSTART=0 and JADSTART=0 (this ensures that no conversion is currently taking place)."]
pub type Offset4EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OFFSET4_POS` reader - POSITIVE OFFSET THIS BIT IS SET AND CLEARED BY SOFTWARE TO ENABLE THE POSITIVE OFFSET. 0: NEGATIVE OFFSET 1: POSITIVE OFFSET NOTE: THE SOFTWARE IS ALLOWED TO WRITE THESE BITS ONLY WHEN ADSTART = 0 AND JADSTART = 0 (WHICH ENSURES THAT NO CONVERSION IS ONGOING)"]
pub type Offset4PosR = crate::BitReader;
#[doc = "Field `OFFSET4_POS` writer - POSITIVE OFFSET THIS BIT IS SET AND CLEARED BY SOFTWARE TO ENABLE THE POSITIVE OFFSET. 0: NEGATIVE OFFSET 1: POSITIVE OFFSET NOTE: THE SOFTWARE IS ALLOWED TO WRITE THESE BITS ONLY WHEN ADSTART = 0 AND JADSTART = 0 (WHICH ENSURES THAT NO CONVERSION IS ONGOING)"]
pub type Offset4PosW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - The data offset compensation corresponding to the channel (single-ended mode INP<0>/differential mode<0>) These bits are written by software, used to define the offset subtracted from the original conversion data when converting the channel (can be regular channel or injection channel) Note: Writing to these bits through software is only allowed when ADSTART=0 and JADSTART=0 (this ensures that no conversion is currently taking place)."]
    #[inline(always)]
    pub fn offset4(&self) -> Offset4R {
        Offset4R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - This bit is enabled and disabled by software writing for programming into the OFFSET4 bit. Note: Writing to this bit through software is only allowed when ADSTART=0 and JADSTART=0 (this ensures that no conversion is currently taking place)."]
    #[inline(always)]
    pub fn offset4_en(&self) -> Offset4EnR {
        Offset4EnR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - POSITIVE OFFSET THIS BIT IS SET AND CLEARED BY SOFTWARE TO ENABLE THE POSITIVE OFFSET. 0: NEGATIVE OFFSET 1: POSITIVE OFFSET NOTE: THE SOFTWARE IS ALLOWED TO WRITE THESE BITS ONLY WHEN ADSTART = 0 AND JADSTART = 0 (WHICH ENSURES THAT NO CONVERSION IS ONGOING)"]
    #[inline(always)]
    pub fn offset4_pos(&self) -> Offset4PosR {
        Offset4PosR::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - The data offset compensation corresponding to the channel (single-ended mode INP<0>/differential mode<0>) These bits are written by software, used to define the offset subtracted from the original conversion data when converting the channel (can be regular channel or injection channel) Note: Writing to these bits through software is only allowed when ADSTART=0 and JADSTART=0 (this ensures that no conversion is currently taking place)."]
    #[inline(always)]
    pub fn offset4(&mut self) -> Offset4W<'_, Offset4Spec> {
        Offset4W::new(self, 0)
    }
    #[doc = "Bit 16 - This bit is enabled and disabled by software writing for programming into the OFFSET4 bit. Note: Writing to this bit through software is only allowed when ADSTART=0 and JADSTART=0 (this ensures that no conversion is currently taking place)."]
    #[inline(always)]
    pub fn offset4_en(&mut self) -> Offset4EnW<'_, Offset4Spec> {
        Offset4EnW::new(self, 16)
    }
    #[doc = "Bit 17 - POSITIVE OFFSET THIS BIT IS SET AND CLEARED BY SOFTWARE TO ENABLE THE POSITIVE OFFSET. 0: NEGATIVE OFFSET 1: POSITIVE OFFSET NOTE: THE SOFTWARE IS ALLOWED TO WRITE THESE BITS ONLY WHEN ADSTART = 0 AND JADSTART = 0 (WHICH ENSURES THAT NO CONVERSION IS ONGOING)"]
    #[inline(always)]
    pub fn offset4_pos(&mut self) -> Offset4PosW<'_, Offset4Spec> {
        Offset4PosW::new(self, 17)
    }
}
#[doc = "ADC offset register 4\n\nYou can [`read`](crate::Reg::read) this register and get [`offset4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`offset4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Offset4Spec;
impl crate::RegisterSpec for Offset4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`offset4::R`](R) reader structure"]
impl crate::Readable for Offset4Spec {}
#[doc = "`write(|w| ..)` method takes [`offset4::W`](W) writer structure"]
impl crate::Writable for Offset4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OFFSET4 to value 0"]
impl crate::Resettable for Offset4Spec {}
