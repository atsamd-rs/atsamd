#[doc = "Reader of register INTCPUSEL6"]
pub type R = crate::R<u32, super::INTCPUSEL6>;
#[doc = "Writer for register INTCPUSEL6"]
pub type W = crate::W<u32, super::INTCPUSEL6>;
#[doc = "Register INTCPUSEL6 `reset()`'s with value 0"]
impl crate::ResetValue for super::INTCPUSEL6 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SERCOM4`"]
pub type SERCOM4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SERCOM4`"]
pub struct SERCOM4_W<'a> {
    w: &'a mut W,
}
impl<'a> SERCOM4_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `SERCOM5`"]
pub type SERCOM5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SERCOM5`"]
pub struct SERCOM5_W<'a> {
    w: &'a mut W,
}
impl<'a> SERCOM5_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `SERCOM6`"]
pub type SERCOM6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SERCOM6`"]
pub struct SERCOM6_W<'a> {
    w: &'a mut W,
}
impl<'a> SERCOM6_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `SERCOM7`"]
pub type SERCOM7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SERCOM7`"]
pub struct SERCOM7_W<'a> {
    w: &'a mut W,
}
impl<'a> SERCOM7_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `TCC4`"]
pub type TCC4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TCC4`"]
pub struct TCC4_W<'a> {
    w: &'a mut W,
}
impl<'a> TCC4_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `TC6`"]
pub type TC6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TC6`"]
pub struct TC6_W<'a> {
    w: &'a mut W,
}
impl<'a> TC6_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `TC7`"]
pub type TC7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TC7`"]
pub struct TC7_W<'a> {
    w: &'a mut W,
}
impl<'a> TC7_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `ADC0`"]
pub type ADC0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC0`"]
pub struct ADC0_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC0_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Reader of field `ADC1`"]
pub type ADC1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC1`"]
pub struct ADC1_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC1_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `DAC`"]
pub type DAC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DAC`"]
pub struct DAC_W<'a> {
    w: &'a mut W,
}
impl<'a> DAC_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Reader of field `I2S`"]
pub type I2S_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2S`"]
pub struct I2S_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Reader of field `PCC`"]
pub type PCC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PCC`"]
pub struct PCC_W<'a> {
    w: &'a mut W,
}
impl<'a> PCC_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - SERCOM4 Interrupt CPU Select"]
    #[inline(always)]
    pub fn sercom4(&self) -> SERCOM4_R {
        SERCOM4_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 2 - SERCOM5 Interrupt CPU Select"]
    #[inline(always)]
    pub fn sercom5(&self) -> SERCOM5_R {
        SERCOM5_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 4 - SERCOM6 Interrupt CPU Select"]
    #[inline(always)]
    pub fn sercom6(&self) -> SERCOM6_R {
        SERCOM6_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 6 - SERCOM7 Interrupt CPU Select"]
    #[inline(always)]
    pub fn sercom7(&self) -> SERCOM7_R {
        SERCOM7_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 8 - TCC4 Interrupt CPU Select"]
    #[inline(always)]
    pub fn tcc4(&self) -> TCC4_R {
        TCC4_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 10 - TC6 Interrupt CPU Select"]
    #[inline(always)]
    pub fn tc6(&self) -> TC6_R {
        TC6_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 12 - TC7 Interrupt CPU Select"]
    #[inline(always)]
    pub fn tc7(&self) -> TC7_R {
        TC7_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 14 - ADC0 Interrupt CPU Select"]
    #[inline(always)]
    pub fn adc0(&self) -> ADC0_R {
        ADC0_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 16 - ADC1 Interrupt CPU Select"]
    #[inline(always)]
    pub fn adc1(&self) -> ADC1_R {
        ADC1_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 18 - DAC Interrupt CPU Select"]
    #[inline(always)]
    pub fn dac(&self) -> DAC_R {
        DAC_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 20 - I2S Interrupt CPU Select"]
    #[inline(always)]
    pub fn i2s(&self) -> I2S_R {
        I2S_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 22 - PCC Interrupt CPU Select"]
    #[inline(always)]
    pub fn pcc(&self) -> PCC_R {
        PCC_R::new(((self.bits >> 22) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SERCOM4 Interrupt CPU Select"]
    #[inline(always)]
    pub fn sercom4(&mut self) -> SERCOM4_W {
        SERCOM4_W { w: self }
    }
    #[doc = "Bit 2 - SERCOM5 Interrupt CPU Select"]
    #[inline(always)]
    pub fn sercom5(&mut self) -> SERCOM5_W {
        SERCOM5_W { w: self }
    }
    #[doc = "Bit 4 - SERCOM6 Interrupt CPU Select"]
    #[inline(always)]
    pub fn sercom6(&mut self) -> SERCOM6_W {
        SERCOM6_W { w: self }
    }
    #[doc = "Bit 6 - SERCOM7 Interrupt CPU Select"]
    #[inline(always)]
    pub fn sercom7(&mut self) -> SERCOM7_W {
        SERCOM7_W { w: self }
    }
    #[doc = "Bit 8 - TCC4 Interrupt CPU Select"]
    #[inline(always)]
    pub fn tcc4(&mut self) -> TCC4_W {
        TCC4_W { w: self }
    }
    #[doc = "Bit 10 - TC6 Interrupt CPU Select"]
    #[inline(always)]
    pub fn tc6(&mut self) -> TC6_W {
        TC6_W { w: self }
    }
    #[doc = "Bit 12 - TC7 Interrupt CPU Select"]
    #[inline(always)]
    pub fn tc7(&mut self) -> TC7_W {
        TC7_W { w: self }
    }
    #[doc = "Bit 14 - ADC0 Interrupt CPU Select"]
    #[inline(always)]
    pub fn adc0(&mut self) -> ADC0_W {
        ADC0_W { w: self }
    }
    #[doc = "Bit 16 - ADC1 Interrupt CPU Select"]
    #[inline(always)]
    pub fn adc1(&mut self) -> ADC1_W {
        ADC1_W { w: self }
    }
    #[doc = "Bit 18 - DAC Interrupt CPU Select"]
    #[inline(always)]
    pub fn dac(&mut self) -> DAC_W {
        DAC_W { w: self }
    }
    #[doc = "Bit 20 - I2S Interrupt CPU Select"]
    #[inline(always)]
    pub fn i2s(&mut self) -> I2S_W {
        I2S_W { w: self }
    }
    #[doc = "Bit 22 - PCC Interrupt CPU Select"]
    #[inline(always)]
    pub fn pcc(&mut self) -> PCC_W {
        PCC_W { w: self }
    }
}
