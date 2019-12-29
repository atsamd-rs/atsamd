#[doc = "Reader of register INTCPUSEL2"]
pub type R = crate::R<u32, super::INTCPUSEL2>;
#[doc = "Writer for register INTCPUSEL2"]
pub type W = crate::W<u32, super::INTCPUSEL2>;
#[doc = "Register INTCPUSEL2 `reset()`'s with value 0"]
impl crate::ResetValue for super::INTCPUSEL2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `USB`"]
pub type USB_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USB`"]
pub struct USB_W<'a> {
    w: &'a mut W,
}
impl<'a> USB_W<'a> {
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
#[doc = "Reader of field `NVMCTRL`"]
pub type NVMCTRL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `NVMCTRL`"]
pub struct NVMCTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> NVMCTRL_W<'a> {
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
#[doc = "Reader of field `DMAC`"]
pub type DMAC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMAC`"]
pub struct DMAC_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAC_W<'a> {
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
#[doc = "Reader of field `EVSYS`"]
pub type EVSYS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EVSYS`"]
pub struct EVSYS_W<'a> {
    w: &'a mut W,
}
impl<'a> EVSYS_W<'a> {
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
#[doc = "Reader of field `PICOP`"]
pub type PICOP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PICOP`"]
pub struct PICOP_W<'a> {
    w: &'a mut W,
}
impl<'a> PICOP_W<'a> {
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
#[doc = "Reader of field `SERCOM2`"]
pub type SERCOM2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SERCOM2`"]
pub struct SERCOM2_W<'a> {
    w: &'a mut W,
}
impl<'a> SERCOM2_W<'a> {
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
#[doc = "Reader of field `SERCOM3`"]
pub type SERCOM3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SERCOM3`"]
pub struct SERCOM3_W<'a> {
    w: &'a mut W,
}
impl<'a> SERCOM3_W<'a> {
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
#[doc = "Reader of field `TCC0`"]
pub type TCC0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TCC0`"]
pub struct TCC0_W<'a> {
    w: &'a mut W,
}
impl<'a> TCC0_W<'a> {
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
#[doc = "Reader of field `TCC1`"]
pub type TCC1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TCC1`"]
pub struct TCC1_W<'a> {
    w: &'a mut W,
}
impl<'a> TCC1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Reader of field `TC2`"]
pub type TC2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TC2`"]
pub struct TC2_W<'a> {
    w: &'a mut W,
}
impl<'a> TC2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "Reader of field `TC3`"]
pub type TC3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TC3`"]
pub struct TC3_W<'a> {
    w: &'a mut W,
}
impl<'a> TC3_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "Reader of field `TAL`"]
pub type TAL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TAL`"]
pub struct TAL_W<'a> {
    w: &'a mut W,
}
impl<'a> TAL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - USB Interrupt CPU Select"]
    #[inline(always)]
    pub fn usb(&self) -> USB_R {
        USB_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 4 - NVMCTRL Interrupt CPU Select"]
    #[inline(always)]
    pub fn nvmctrl(&self) -> NVMCTRL_R {
        NVMCTRL_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 10 - DMAC Interrupt CPU Select"]
    #[inline(always)]
    pub fn dmac(&self) -> DMAC_R {
        DMAC_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 14 - EVSYS Interrupt CPU Select"]
    #[inline(always)]
    pub fn evsys(&self) -> EVSYS_R {
        EVSYS_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 16 - PICOP Interrupt CPU Select"]
    #[inline(always)]
    pub fn picop(&self) -> PICOP_R {
        PICOP_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 18 - SERCOM2 Interrupt CPU Select"]
    #[inline(always)]
    pub fn sercom2(&self) -> SERCOM2_R {
        SERCOM2_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 20 - SERCOM3 Interrupt CPU Select"]
    #[inline(always)]
    pub fn sercom3(&self) -> SERCOM3_R {
        SERCOM3_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 22 - TCC0 Interrupt CPU Select"]
    #[inline(always)]
    pub fn tcc0(&self) -> TCC0_R {
        TCC0_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 24 - TCC1 Interrupt CPU Select"]
    #[inline(always)]
    pub fn tcc1(&self) -> TCC1_R {
        TCC1_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 26 - TC2 Interrupt CPU Select"]
    #[inline(always)]
    pub fn tc2(&self) -> TC2_R {
        TC2_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 28 - TC3 Interrupt CPU Select"]
    #[inline(always)]
    pub fn tc3(&self) -> TC3_R {
        TC3_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 30 - TAL Interrupt CPU Select"]
    #[inline(always)]
    pub fn tal(&self) -> TAL_R {
        TAL_R::new(((self.bits >> 30) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - USB Interrupt CPU Select"]
    #[inline(always)]
    pub fn usb(&mut self) -> USB_W {
        USB_W { w: self }
    }
    #[doc = "Bit 4 - NVMCTRL Interrupt CPU Select"]
    #[inline(always)]
    pub fn nvmctrl(&mut self) -> NVMCTRL_W {
        NVMCTRL_W { w: self }
    }
    #[doc = "Bit 10 - DMAC Interrupt CPU Select"]
    #[inline(always)]
    pub fn dmac(&mut self) -> DMAC_W {
        DMAC_W { w: self }
    }
    #[doc = "Bit 14 - EVSYS Interrupt CPU Select"]
    #[inline(always)]
    pub fn evsys(&mut self) -> EVSYS_W {
        EVSYS_W { w: self }
    }
    #[doc = "Bit 16 - PICOP Interrupt CPU Select"]
    #[inline(always)]
    pub fn picop(&mut self) -> PICOP_W {
        PICOP_W { w: self }
    }
    #[doc = "Bit 18 - SERCOM2 Interrupt CPU Select"]
    #[inline(always)]
    pub fn sercom2(&mut self) -> SERCOM2_W {
        SERCOM2_W { w: self }
    }
    #[doc = "Bit 20 - SERCOM3 Interrupt CPU Select"]
    #[inline(always)]
    pub fn sercom3(&mut self) -> SERCOM3_W {
        SERCOM3_W { w: self }
    }
    #[doc = "Bit 22 - TCC0 Interrupt CPU Select"]
    #[inline(always)]
    pub fn tcc0(&mut self) -> TCC0_W {
        TCC0_W { w: self }
    }
    #[doc = "Bit 24 - TCC1 Interrupt CPU Select"]
    #[inline(always)]
    pub fn tcc1(&mut self) -> TCC1_W {
        TCC1_W { w: self }
    }
    #[doc = "Bit 26 - TC2 Interrupt CPU Select"]
    #[inline(always)]
    pub fn tc2(&mut self) -> TC2_W {
        TC2_W { w: self }
    }
    #[doc = "Bit 28 - TC3 Interrupt CPU Select"]
    #[inline(always)]
    pub fn tc3(&mut self) -> TC3_W {
        TC3_W { w: self }
    }
    #[doc = "Bit 30 - TAL Interrupt CPU Select"]
    #[inline(always)]
    pub fn tal(&mut self) -> TAL_W {
        TAL_W { w: self }
    }
}
