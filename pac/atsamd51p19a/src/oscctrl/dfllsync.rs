#[doc = "Reader of register DFLLSYNC"]
pub type R = crate::R<u8, super::DFLLSYNC>;
#[doc = "Writer for register DFLLSYNC"]
pub type W = crate::W<u8, super::DFLLSYNC>;
#[doc = "Register DFLLSYNC `reset()`'s with value 0"]
impl crate::ResetValue for super::DFLLSYNC {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ENABLE`"]
pub type ENABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENABLE`"]
pub struct ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u8) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `DFLLCTRLB`"]
pub type DFLLCTRLB_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DFLLCTRLB`"]
pub struct DFLLCTRLB_W<'a> {
    w: &'a mut W,
}
impl<'a> DFLLCTRLB_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u8) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `DFLLVAL`"]
pub type DFLLVAL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DFLLVAL`"]
pub struct DFLLVAL_W<'a> {
    w: &'a mut W,
}
impl<'a> DFLLVAL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u8) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `DFLLMUL`"]
pub type DFLLMUL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DFLLMUL`"]
pub struct DFLLMUL_W<'a> {
    w: &'a mut W,
}
impl<'a> DFLLMUL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u8) & 0x01) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bit 1 - ENABLE Synchronization Busy"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - DFLLCTRLB Synchronization Busy"]
    #[inline(always)]
    pub fn dfllctrlb(&self) -> DFLLCTRLB_R {
        DFLLCTRLB_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - DFLLVAL Synchronization Busy"]
    #[inline(always)]
    pub fn dfllval(&self) -> DFLLVAL_R {
        DFLLVAL_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - DFLLMUL Synchronization Busy"]
    #[inline(always)]
    pub fn dfllmul(&self) -> DFLLMUL_R {
        DFLLMUL_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - ENABLE Synchronization Busy"]
    #[inline(always)]
    pub fn enable(&mut self) -> ENABLE_W {
        ENABLE_W { w: self }
    }
    #[doc = "Bit 2 - DFLLCTRLB Synchronization Busy"]
    #[inline(always)]
    pub fn dfllctrlb(&mut self) -> DFLLCTRLB_W {
        DFLLCTRLB_W { w: self }
    }
    #[doc = "Bit 3 - DFLLVAL Synchronization Busy"]
    #[inline(always)]
    pub fn dfllval(&mut self) -> DFLLVAL_W {
        DFLLVAL_W { w: self }
    }
    #[doc = "Bit 4 - DFLLMUL Synchronization Busy"]
    #[inline(always)]
    pub fn dfllmul(&mut self) -> DFLLMUL_W {
        DFLLMUL_W { w: self }
    }
}
