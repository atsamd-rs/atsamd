#[doc = "Writer for register IDR"]
pub type W = crate::W<u32, super::IDR>;
#[doc = "Register IDR `reset()`'s with value 0"]
impl crate::ResetValue for super::IDR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `RHC`"]
pub struct RHC_W<'a> {
    w: &'a mut W,
}
impl<'a> RHC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Write proxy for field `RDM`"]
pub struct RDM_W<'a> {
    w: &'a mut W,
}
impl<'a> RDM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Write proxy for field `RBE`"]
pub struct RBE_W<'a> {
    w: &'a mut W,
}
impl<'a> RBE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Write proxy for field `RWC`"]
pub struct RWC_W<'a> {
    w: &'a mut W,
}
impl<'a> RWC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u32) & 0x0f) << 12);
        self.w
    }
}
#[doc = "Write proxy for field `REC`"]
pub struct REC_W<'a> {
    w: &'a mut W,
}
impl<'a> REC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "Write proxy for field `RSU`"]
pub struct RSU_W<'a> {
    w: &'a mut W,
}
impl<'a> RSU_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | (((value as u32) & 0x0f) << 20);
        self.w
    }
}
#[doc = "Write proxy for field `URAD`"]
pub struct URAD_W<'a> {
    w: &'a mut W,
}
impl<'a> URAD_W<'a> {
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
impl W {
    #[doc = "Bits 0:3 - Region Hash Completed Interrupt Disable"]
    #[inline(always)]
    pub fn rhc(&mut self) -> RHC_W {
        RHC_W { w: self }
    }
    #[doc = "Bits 4:7 - Region Digest Mismatch Interrupt Disable"]
    #[inline(always)]
    pub fn rdm(&mut self) -> RDM_W {
        RDM_W { w: self }
    }
    #[doc = "Bits 8:11 - Region Bus Error Interrupt Disable"]
    #[inline(always)]
    pub fn rbe(&mut self) -> RBE_W {
        RBE_W { w: self }
    }
    #[doc = "Bits 12:15 - Region Wrap Condition Detected Interrupt Disable"]
    #[inline(always)]
    pub fn rwc(&mut self) -> RWC_W {
        RWC_W { w: self }
    }
    #[doc = "Bits 16:19 - Region End bit Condition detected Interrupt Disable"]
    #[inline(always)]
    pub fn rec(&mut self) -> REC_W {
        REC_W { w: self }
    }
    #[doc = "Bits 20:23 - Region Status Updated Interrupt Disable"]
    #[inline(always)]
    pub fn rsu(&mut self) -> RSU_W {
        RSU_W { w: self }
    }
    #[doc = "Bit 24 - Undefined Register Access Detection Interrupt Disable"]
    #[inline(always)]
    pub fn urad(&mut self) -> URAD_W {
        URAD_W { w: self }
    }
}
