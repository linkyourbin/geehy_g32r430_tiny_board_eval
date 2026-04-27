#[doc = "Register `OFFSET5` reader"]
pub type R = crate::R<Offset5Spec>;
#[doc = "Register `OFFSET5` writer"]
pub type W = crate::W<Offset5Spec>;
#[doc = "Field `OFFSET5` reader - The data offset compensation corresponding to the channel (single-ended mode INP<0>/differential mode<0>) These bits are written by software, used to define the offset subtracted from the original conversion data when converting the channel (can be regular channel or injection channel) Note: Writing to these bits through software is only allowed when ADSTART=0 and JADSTART=0 (this ensures that no conversion is currently taking place)."]
pub type Offset5R = crate::FieldReader<u16>;
#[doc = "Field `OFFSET5` writer - The data offset compensation corresponding to the channel (single-ended mode INP<0>/differential mode<0>) These bits are written by software, used to define the offset subtracted from the original conversion data when converting the channel (can be regular channel or injection channel) Note: Writing to these bits through software is only allowed when ADSTART=0 and JADSTART=0 (this ensures that no conversion is currently taking place)."]
pub type Offset5W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `OFFSET5_EN` reader - This bit is enabled and disabled by software writing for programming into the OFFSET5 bit. Note: Writing to this bit through software is only allowed when ADSTART=0 and JADSTART=0 (this ensures that no conversion is currently taking place)."]
pub type Offset5EnR = crate::BitReader;
#[doc = "Field `OFFSET5_EN` writer - This bit is enabled and disabled by software writing for programming into the OFFSET5 bit. Note: Writing to this bit through software is only allowed when ADSTART=0 and JADSTART=0 (this ensures that no conversion is currently taking place)."]
pub type Offset5EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OFFSET5_POS` reader - POSITIVE OFFSET THIS BIT IS SET AND CLEARED BY SOFTWARE TO ENABLE THE POSITIVE OFFSET. 0: NEGATIVE OFFSET 1: POSITIVE OFFSET NOTE: THE SOFTWARE IS ALLOWED TO WRITE THESE BITS ONLY WHEN ADSTART = 0 AND JADSTART = 0 (WHICH ENSURES THAT NO CONVERSION IS ONGOING)"]
pub type Offset5PosR = crate::BitReader;
#[doc = "Field `OFFSET5_POS` writer - POSITIVE OFFSET THIS BIT IS SET AND CLEARED BY SOFTWARE TO ENABLE THE POSITIVE OFFSET. 0: NEGATIVE OFFSET 1: POSITIVE OFFSET NOTE: THE SOFTWARE IS ALLOWED TO WRITE THESE BITS ONLY WHEN ADSTART = 0 AND JADSTART = 0 (WHICH ENSURES THAT NO CONVERSION IS ONGOING)"]
pub type Offset5PosW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - The data offset compensation corresponding to the channel (single-ended mode INP<0>/differential mode<0>) These bits are written by software, used to define the offset subtracted from the original conversion data when converting the channel (can be regular channel or injection channel) Note: Writing to these bits through software is only allowed when ADSTART=0 and JADSTART=0 (this ensures that no conversion is currently taking place)."]
    #[inline(always)]
    pub fn offset5(&self) -> Offset5R {
        Offset5R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - This bit is enabled and disabled by software writing for programming into the OFFSET5 bit. Note: Writing to this bit through software is only allowed when ADSTART=0 and JADSTART=0 (this ensures that no conversion is currently taking place)."]
    #[inline(always)]
    pub fn offset5_en(&self) -> Offset5EnR {
        Offset5EnR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - POSITIVE OFFSET THIS BIT IS SET AND CLEARED BY SOFTWARE TO ENABLE THE POSITIVE OFFSET. 0: NEGATIVE OFFSET 1: POSITIVE OFFSET NOTE: THE SOFTWARE IS ALLOWED TO WRITE THESE BITS ONLY WHEN ADSTART = 0 AND JADSTART = 0 (WHICH ENSURES THAT NO CONVERSION IS ONGOING)"]
    #[inline(always)]
    pub fn offset5_pos(&self) -> Offset5PosR {
        Offset5PosR::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - The data offset compensation corresponding to the channel (single-ended mode INP<0>/differential mode<0>) These bits are written by software, used to define the offset subtracted from the original conversion data when converting the channel (can be regular channel or injection channel) Note: Writing to these bits through software is only allowed when ADSTART=0 and JADSTART=0 (this ensures that no conversion is currently taking place)."]
    #[inline(always)]
    pub fn offset5(&mut self) -> Offset5W<'_, Offset5Spec> {
        Offset5W::new(self, 0)
    }
    #[doc = "Bit 16 - This bit is enabled and disabled by software writing for programming into the OFFSET5 bit. Note: Writing to this bit through software is only allowed when ADSTART=0 and JADSTART=0 (this ensures that no conversion is currently taking place)."]
    #[inline(always)]
    pub fn offset5_en(&mut self) -> Offset5EnW<'_, Offset5Spec> {
        Offset5EnW::new(self, 16)
    }
    #[doc = "Bit 17 - POSITIVE OFFSET THIS BIT IS SET AND CLEARED BY SOFTWARE TO ENABLE THE POSITIVE OFFSET. 0: NEGATIVE OFFSET 1: POSITIVE OFFSET NOTE: THE SOFTWARE IS ALLOWED TO WRITE THESE BITS ONLY WHEN ADSTART = 0 AND JADSTART = 0 (WHICH ENSURES THAT NO CONVERSION IS ONGOING)"]
    #[inline(always)]
    pub fn offset5_pos(&mut self) -> Offset5PosW<'_, Offset5Spec> {
        Offset5PosW::new(self, 17)
    }
}
#[doc = "ADC offset register 5\n\nYou can [`read`](crate::Reg::read) this register and get [`offset5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`offset5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Offset5Spec;
impl crate::RegisterSpec for Offset5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`offset5::R`](R) reader structure"]
impl crate::Readable for Offset5Spec {}
#[doc = "`write(|w| ..)` method takes [`offset5::W`](W) writer structure"]
impl crate::Writable for Offset5Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OFFSET5 to value 0"]
impl crate::Resettable for Offset5Spec {}
