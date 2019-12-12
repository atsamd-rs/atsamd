#[doc = "Reader of register INTFLAG"]
pub type R = crate::R<u32, super::INTFLAG>;
#[doc = "Writer for register INTFLAG"]
pub type W = crate::W<u32, super::INTFLAG>;
#[doc = "Register INTFLAG `reset()`'s with value 0"]
impl crate::ResetValue for super::INTFLAG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `XOSCRDY`"]
pub type XOSCRDY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `XOSCRDY`"]
pub struct XOSCRDY_W<'a> {
    w: &'a mut W,
}
impl<'a> XOSCRDY_W<'a> {
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
#[doc = "Reader of field `XOSC32KRDY`"]
pub type XOSC32KRDY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `XOSC32KRDY`"]
pub struct XOSC32KRDY_W<'a> {
    w: &'a mut W,
}
impl<'a> XOSC32KRDY_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `OSC32KRDY`"]
pub type OSC32KRDY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OSC32KRDY`"]
pub struct OSC32KRDY_W<'a> {
    w: &'a mut W,
}
impl<'a> OSC32KRDY_W<'a> {
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
#[doc = "Reader of field `OSC8MRDY`"]
pub type OSC8MRDY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OSC8MRDY`"]
pub struct OSC8MRDY_W<'a> {
    w: &'a mut W,
}
impl<'a> OSC8MRDY_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `DFLLRDY`"]
pub type DFLLRDY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DFLLRDY`"]
pub struct DFLLRDY_W<'a> {
    w: &'a mut W,
}
impl<'a> DFLLRDY_W<'a> {
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
#[doc = "Reader of field `DFLLOOB`"]
pub type DFLLOOB_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DFLLOOB`"]
pub struct DFLLOOB_W<'a> {
    w: &'a mut W,
}
impl<'a> DFLLOOB_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `DFLLLCKF`"]
pub type DFLLLCKF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DFLLLCKF`"]
pub struct DFLLLCKF_W<'a> {
    w: &'a mut W,
}
impl<'a> DFLLLCKF_W<'a> {
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
#[doc = "Reader of field `DFLLLCKC`"]
pub type DFLLLCKC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DFLLLCKC`"]
pub struct DFLLLCKC_W<'a> {
    w: &'a mut W,
}
impl<'a> DFLLLCKC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `DFLLRCS`"]
pub type DFLLRCS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DFLLRCS`"]
pub struct DFLLRCS_W<'a> {
    w: &'a mut W,
}
impl<'a> DFLLRCS_W<'a> {
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
#[doc = "Reader of field `BOD33RDY`"]
pub type BOD33RDY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BOD33RDY`"]
pub struct BOD33RDY_W<'a> {
    w: &'a mut W,
}
impl<'a> BOD33RDY_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `BOD33DET`"]
pub type BOD33DET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BOD33DET`"]
pub struct BOD33DET_W<'a> {
    w: &'a mut W,
}
impl<'a> BOD33DET_W<'a> {
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
#[doc = "Reader of field `B33SRDY`"]
pub type B33SRDY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B33SRDY`"]
pub struct B33SRDY_W<'a> {
    w: &'a mut W,
}
impl<'a> B33SRDY_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Reader of field `DPLLLCKR`"]
pub type DPLLLCKR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DPLLLCKR`"]
pub struct DPLLLCKR_W<'a> {
    w: &'a mut W,
}
impl<'a> DPLLLCKR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Reader of field `DPLLLCKF`"]
pub type DPLLLCKF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DPLLLCKF`"]
pub struct DPLLLCKF_W<'a> {
    w: &'a mut W,
}
impl<'a> DPLLLCKF_W<'a> {
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
#[doc = "Reader of field `DPLLLTO`"]
pub type DPLLLTO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DPLLLTO`"]
pub struct DPLLLTO_W<'a> {
    w: &'a mut W,
}
impl<'a> DPLLLTO_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - XOSC Ready"]
    #[inline(always)]
    pub fn xoscrdy(&self) -> XOSCRDY_R {
        XOSCRDY_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - XOSC32K Ready"]
    #[inline(always)]
    pub fn xosc32krdy(&self) -> XOSC32KRDY_R {
        XOSC32KRDY_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - OSC32K Ready"]
    #[inline(always)]
    pub fn osc32krdy(&self) -> OSC32KRDY_R {
        OSC32KRDY_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - OSC8M Ready"]
    #[inline(always)]
    pub fn osc8mrdy(&self) -> OSC8MRDY_R {
        OSC8MRDY_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - DFLL Ready"]
    #[inline(always)]
    pub fn dfllrdy(&self) -> DFLLRDY_R {
        DFLLRDY_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - DFLL Out Of Bounds"]
    #[inline(always)]
    pub fn dflloob(&self) -> DFLLOOB_R {
        DFLLOOB_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - DFLL Lock Fine"]
    #[inline(always)]
    pub fn dflllckf(&self) -> DFLLLCKF_R {
        DFLLLCKF_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - DFLL Lock Coarse"]
    #[inline(always)]
    pub fn dflllckc(&self) -> DFLLLCKC_R {
        DFLLLCKC_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - DFLL Reference Clock Stopped"]
    #[inline(always)]
    pub fn dfllrcs(&self) -> DFLLRCS_R {
        DFLLRCS_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - BOD33 Ready"]
    #[inline(always)]
    pub fn bod33rdy(&self) -> BOD33RDY_R {
        BOD33RDY_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - BOD33 Detection"]
    #[inline(always)]
    pub fn bod33det(&self) -> BOD33DET_R {
        BOD33DET_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - BOD33 Synchronization Ready"]
    #[inline(always)]
    pub fn b33srdy(&self) -> B33SRDY_R {
        B33SRDY_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 15 - DPLL Lock Rise"]
    #[inline(always)]
    pub fn dplllckr(&self) -> DPLLLCKR_R {
        DPLLLCKR_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - DPLL Lock Fall"]
    #[inline(always)]
    pub fn dplllckf(&self) -> DPLLLCKF_R {
        DPLLLCKF_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - DPLL Lock Timeout"]
    #[inline(always)]
    pub fn dplllto(&self) -> DPLLLTO_R {
        DPLLLTO_R::new(((self.bits >> 17) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - XOSC Ready"]
    #[inline(always)]
    pub fn xoscrdy(&mut self) -> XOSCRDY_W {
        XOSCRDY_W { w: self }
    }
    #[doc = "Bit 1 - XOSC32K Ready"]
    #[inline(always)]
    pub fn xosc32krdy(&mut self) -> XOSC32KRDY_W {
        XOSC32KRDY_W { w: self }
    }
    #[doc = "Bit 2 - OSC32K Ready"]
    #[inline(always)]
    pub fn osc32krdy(&mut self) -> OSC32KRDY_W {
        OSC32KRDY_W { w: self }
    }
    #[doc = "Bit 3 - OSC8M Ready"]
    #[inline(always)]
    pub fn osc8mrdy(&mut self) -> OSC8MRDY_W {
        OSC8MRDY_W { w: self }
    }
    #[doc = "Bit 4 - DFLL Ready"]
    #[inline(always)]
    pub fn dfllrdy(&mut self) -> DFLLRDY_W {
        DFLLRDY_W { w: self }
    }
    #[doc = "Bit 5 - DFLL Out Of Bounds"]
    #[inline(always)]
    pub fn dflloob(&mut self) -> DFLLOOB_W {
        DFLLOOB_W { w: self }
    }
    #[doc = "Bit 6 - DFLL Lock Fine"]
    #[inline(always)]
    pub fn dflllckf(&mut self) -> DFLLLCKF_W {
        DFLLLCKF_W { w: self }
    }
    #[doc = "Bit 7 - DFLL Lock Coarse"]
    #[inline(always)]
    pub fn dflllckc(&mut self) -> DFLLLCKC_W {
        DFLLLCKC_W { w: self }
    }
    #[doc = "Bit 8 - DFLL Reference Clock Stopped"]
    #[inline(always)]
    pub fn dfllrcs(&mut self) -> DFLLRCS_W {
        DFLLRCS_W { w: self }
    }
    #[doc = "Bit 9 - BOD33 Ready"]
    #[inline(always)]
    pub fn bod33rdy(&mut self) -> BOD33RDY_W {
        BOD33RDY_W { w: self }
    }
    #[doc = "Bit 10 - BOD33 Detection"]
    #[inline(always)]
    pub fn bod33det(&mut self) -> BOD33DET_W {
        BOD33DET_W { w: self }
    }
    #[doc = "Bit 11 - BOD33 Synchronization Ready"]
    #[inline(always)]
    pub fn b33srdy(&mut self) -> B33SRDY_W {
        B33SRDY_W { w: self }
    }
    #[doc = "Bit 15 - DPLL Lock Rise"]
    #[inline(always)]
    pub fn dplllckr(&mut self) -> DPLLLCKR_W {
        DPLLLCKR_W { w: self }
    }
    #[doc = "Bit 16 - DPLL Lock Fall"]
    #[inline(always)]
    pub fn dplllckf(&mut self) -> DPLLLCKF_W {
        DPLLLCKF_W { w: self }
    }
    #[doc = "Bit 17 - DPLL Lock Timeout"]
    #[inline(always)]
    pub fn dplllto(&mut self) -> DPLLLTO_W {
        DPLLLTO_W { w: self }
    }
}
