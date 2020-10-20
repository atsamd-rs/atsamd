#[doc = "Reader of register INTENSET"]
pub type R = crate::R<u32, super::INTENSET>;
#[doc = "Writer for register INTENSET"]
pub type W = crate::W<u32, super::INTENSET>;
#[doc = "Register INTENSET `reset()`'s with value 0"]
impl crate::ResetValue for super::INTENSET {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `XOSCRDY0`"]
pub type XOSCRDY0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `XOSCRDY0`"]
pub struct XOSCRDY0_W<'a> {
    w: &'a mut W,
}
impl<'a> XOSCRDY0_W<'a> {
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
#[doc = "Reader of field `XOSCRDY1`"]
pub type XOSCRDY1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `XOSCRDY1`"]
pub struct XOSCRDY1_W<'a> {
    w: &'a mut W,
}
impl<'a> XOSCRDY1_W<'a> {
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
#[doc = "Reader of field `XOSCFAIL0`"]
pub type XOSCFAIL0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `XOSCFAIL0`"]
pub struct XOSCFAIL0_W<'a> {
    w: &'a mut W,
}
impl<'a> XOSCFAIL0_W<'a> {
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
#[doc = "Reader of field `XOSCFAIL1`"]
pub type XOSCFAIL1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `XOSCFAIL1`"]
pub struct XOSCFAIL1_W<'a> {
    w: &'a mut W,
}
impl<'a> XOSCFAIL1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `DPLL0LCKR`"]
pub type DPLL0LCKR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DPLL0LCKR`"]
pub struct DPLL0LCKR_W<'a> {
    w: &'a mut W,
}
impl<'a> DPLL0LCKR_W<'a> {
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
#[doc = "Reader of field `DPLL0LCKF`"]
pub type DPLL0LCKF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DPLL0LCKF`"]
pub struct DPLL0LCKF_W<'a> {
    w: &'a mut W,
}
impl<'a> DPLL0LCKF_W<'a> {
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
#[doc = "Reader of field `DPLL0LTO`"]
pub type DPLL0LTO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DPLL0LTO`"]
pub struct DPLL0LTO_W<'a> {
    w: &'a mut W,
}
impl<'a> DPLL0LTO_W<'a> {
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
#[doc = "Reader of field `DPLL0LDRTO`"]
pub type DPLL0LDRTO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DPLL0LDRTO`"]
pub struct DPLL0LDRTO_W<'a> {
    w: &'a mut W,
}
impl<'a> DPLL0LDRTO_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Reader of field `DPLL1LCKR`"]
pub type DPLL1LCKR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DPLL1LCKR`"]
pub struct DPLL1LCKR_W<'a> {
    w: &'a mut W,
}
impl<'a> DPLL1LCKR_W<'a> {
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
#[doc = "Reader of field `DPLL1LCKF`"]
pub type DPLL1LCKF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DPLL1LCKF`"]
pub struct DPLL1LCKF_W<'a> {
    w: &'a mut W,
}
impl<'a> DPLL1LCKF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "Reader of field `DPLL1LTO`"]
pub type DPLL1LTO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DPLL1LTO`"]
pub struct DPLL1LTO_W<'a> {
    w: &'a mut W,
}
impl<'a> DPLL1LTO_W<'a> {
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
#[doc = "Reader of field `DPLL1LDRTO`"]
pub type DPLL1LDRTO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DPLL1LDRTO`"]
pub struct DPLL1LDRTO_W<'a> {
    w: &'a mut W,
}
impl<'a> DPLL1LDRTO_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - XOSC 0 Ready Interrupt Enable"]
    #[inline(always)]
    pub fn xoscrdy0(&self) -> XOSCRDY0_R {
        XOSCRDY0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - XOSC 1 Ready Interrupt Enable"]
    #[inline(always)]
    pub fn xoscrdy1(&self) -> XOSCRDY1_R {
        XOSCRDY1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - XOSC 0 Clock Failure Detector Interrupt Enable"]
    #[inline(always)]
    pub fn xoscfail0(&self) -> XOSCFAIL0_R {
        XOSCFAIL0_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - XOSC 1 Clock Failure Detector Interrupt Enable"]
    #[inline(always)]
    pub fn xoscfail1(&self) -> XOSCFAIL1_R {
        XOSCFAIL1_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 8 - DFLL Ready Interrupt Enable"]
    #[inline(always)]
    pub fn dfllrdy(&self) -> DFLLRDY_R {
        DFLLRDY_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - DFLL Out Of Bounds Interrupt Enable"]
    #[inline(always)]
    pub fn dflloob(&self) -> DFLLOOB_R {
        DFLLOOB_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - DFLL Lock Fine Interrupt Enable"]
    #[inline(always)]
    pub fn dflllckf(&self) -> DFLLLCKF_R {
        DFLLLCKF_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - DFLL Lock Coarse Interrupt Enable"]
    #[inline(always)]
    pub fn dflllckc(&self) -> DFLLLCKC_R {
        DFLLLCKC_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - DFLL Reference Clock Stopped Interrupt Enable"]
    #[inline(always)]
    pub fn dfllrcs(&self) -> DFLLRCS_R {
        DFLLRCS_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 16 - DPLL0 Lock Rise Interrupt Enable"]
    #[inline(always)]
    pub fn dpll0lckr(&self) -> DPLL0LCKR_R {
        DPLL0LCKR_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - DPLL0 Lock Fall Interrupt Enable"]
    #[inline(always)]
    pub fn dpll0lckf(&self) -> DPLL0LCKF_R {
        DPLL0LCKF_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - DPLL0 Lock Timeout Interrupt Enable"]
    #[inline(always)]
    pub fn dpll0lto(&self) -> DPLL0LTO_R {
        DPLL0LTO_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - DPLL0 Loop Divider Ratio Update Complete Interrupt Enable"]
    #[inline(always)]
    pub fn dpll0ldrto(&self) -> DPLL0LDRTO_R {
        DPLL0LDRTO_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 24 - DPLL1 Lock Rise Interrupt Enable"]
    #[inline(always)]
    pub fn dpll1lckr(&self) -> DPLL1LCKR_R {
        DPLL1LCKR_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - DPLL1 Lock Fall Interrupt Enable"]
    #[inline(always)]
    pub fn dpll1lckf(&self) -> DPLL1LCKF_R {
        DPLL1LCKF_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - DPLL1 Lock Timeout Interrupt Enable"]
    #[inline(always)]
    pub fn dpll1lto(&self) -> DPLL1LTO_R {
        DPLL1LTO_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - DPLL1 Loop Divider Ratio Update Complete Interrupt Enable"]
    #[inline(always)]
    pub fn dpll1ldrto(&self) -> DPLL1LDRTO_R {
        DPLL1LDRTO_R::new(((self.bits >> 27) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - XOSC 0 Ready Interrupt Enable"]
    #[inline(always)]
    pub fn xoscrdy0(&mut self) -> XOSCRDY0_W {
        XOSCRDY0_W { w: self }
    }
    #[doc = "Bit 1 - XOSC 1 Ready Interrupt Enable"]
    #[inline(always)]
    pub fn xoscrdy1(&mut self) -> XOSCRDY1_W {
        XOSCRDY1_W { w: self }
    }
    #[doc = "Bit 2 - XOSC 0 Clock Failure Detector Interrupt Enable"]
    #[inline(always)]
    pub fn xoscfail0(&mut self) -> XOSCFAIL0_W {
        XOSCFAIL0_W { w: self }
    }
    #[doc = "Bit 3 - XOSC 1 Clock Failure Detector Interrupt Enable"]
    #[inline(always)]
    pub fn xoscfail1(&mut self) -> XOSCFAIL1_W {
        XOSCFAIL1_W { w: self }
    }
    #[doc = "Bit 8 - DFLL Ready Interrupt Enable"]
    #[inline(always)]
    pub fn dfllrdy(&mut self) -> DFLLRDY_W {
        DFLLRDY_W { w: self }
    }
    #[doc = "Bit 9 - DFLL Out Of Bounds Interrupt Enable"]
    #[inline(always)]
    pub fn dflloob(&mut self) -> DFLLOOB_W {
        DFLLOOB_W { w: self }
    }
    #[doc = "Bit 10 - DFLL Lock Fine Interrupt Enable"]
    #[inline(always)]
    pub fn dflllckf(&mut self) -> DFLLLCKF_W {
        DFLLLCKF_W { w: self }
    }
    #[doc = "Bit 11 - DFLL Lock Coarse Interrupt Enable"]
    #[inline(always)]
    pub fn dflllckc(&mut self) -> DFLLLCKC_W {
        DFLLLCKC_W { w: self }
    }
    #[doc = "Bit 12 - DFLL Reference Clock Stopped Interrupt Enable"]
    #[inline(always)]
    pub fn dfllrcs(&mut self) -> DFLLRCS_W {
        DFLLRCS_W { w: self }
    }
    #[doc = "Bit 16 - DPLL0 Lock Rise Interrupt Enable"]
    #[inline(always)]
    pub fn dpll0lckr(&mut self) -> DPLL0LCKR_W {
        DPLL0LCKR_W { w: self }
    }
    #[doc = "Bit 17 - DPLL0 Lock Fall Interrupt Enable"]
    #[inline(always)]
    pub fn dpll0lckf(&mut self) -> DPLL0LCKF_W {
        DPLL0LCKF_W { w: self }
    }
    #[doc = "Bit 18 - DPLL0 Lock Timeout Interrupt Enable"]
    #[inline(always)]
    pub fn dpll0lto(&mut self) -> DPLL0LTO_W {
        DPLL0LTO_W { w: self }
    }
    #[doc = "Bit 19 - DPLL0 Loop Divider Ratio Update Complete Interrupt Enable"]
    #[inline(always)]
    pub fn dpll0ldrto(&mut self) -> DPLL0LDRTO_W {
        DPLL0LDRTO_W { w: self }
    }
    #[doc = "Bit 24 - DPLL1 Lock Rise Interrupt Enable"]
    #[inline(always)]
    pub fn dpll1lckr(&mut self) -> DPLL1LCKR_W {
        DPLL1LCKR_W { w: self }
    }
    #[doc = "Bit 25 - DPLL1 Lock Fall Interrupt Enable"]
    #[inline(always)]
    pub fn dpll1lckf(&mut self) -> DPLL1LCKF_W {
        DPLL1LCKF_W { w: self }
    }
    #[doc = "Bit 26 - DPLL1 Lock Timeout Interrupt Enable"]
    #[inline(always)]
    pub fn dpll1lto(&mut self) -> DPLL1LTO_W {
        DPLL1LTO_W { w: self }
    }
    #[doc = "Bit 27 - DPLL1 Loop Divider Ratio Update Complete Interrupt Enable"]
    #[inline(always)]
    pub fn dpll1ldrto(&mut self) -> DPLL1LDRTO_W {
        DPLL1LDRTO_W { w: self }
    }
}
