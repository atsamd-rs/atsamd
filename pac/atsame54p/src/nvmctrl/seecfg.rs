#[doc = "Reader of register SEECFG"]
pub type R = crate::R<u8, super::SEECFG>;
#[doc = "Writer for register SEECFG"]
pub type W = crate::W<u8, super::SEECFG>;
#[doc = "Register SEECFG `reset()`'s with value 0"]
impl crate::ResetValue for super::SEECFG {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WMODE_A {
    #[doc = "0: A NVM write command is issued after each write in the pagebuffer"]
    UNBUFFERED = 0,
    #[doc = "1: A NVM write command is issued when a write to a new page is requested"]
    BUFFERED = 1,
}
impl From<WMODE_A> for bool {
    #[inline(always)]
    fn from(variant: WMODE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `WMODE`"]
pub type WMODE_R = crate::R<bool, WMODE_A>;
impl WMODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WMODE_A {
        match self.bits {
            false => WMODE_A::UNBUFFERED,
            true => WMODE_A::BUFFERED,
        }
    }
    #[doc = "Checks if the value of the field is `UNBUFFERED`"]
    #[inline(always)]
    pub fn is_unbuffered(&self) -> bool {
        *self == WMODE_A::UNBUFFERED
    }
    #[doc = "Checks if the value of the field is `BUFFERED`"]
    #[inline(always)]
    pub fn is_buffered(&self) -> bool {
        *self == WMODE_A::BUFFERED
    }
}
#[doc = "Write proxy for field `WMODE`"]
pub struct WMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> WMODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WMODE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "A NVM write command is issued after each write in the pagebuffer"]
    #[inline(always)]
    pub fn unbuffered(self) -> &'a mut W {
        self.variant(WMODE_A::UNBUFFERED)
    }
    #[doc = "A NVM write command is issued when a write to a new page is requested"]
    #[inline(always)]
    pub fn buffered(self) -> &'a mut W {
        self.variant(WMODE_A::BUFFERED)
    }
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u8) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `APRDIS`"]
pub type APRDIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `APRDIS`"]
pub struct APRDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> APRDIS_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Write Mode"]
    #[inline(always)]
    pub fn wmode(&self) -> WMODE_R {
        WMODE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Automatic Page Reallocation Disable"]
    #[inline(always)]
    pub fn aprdis(&self) -> APRDIS_R {
        APRDIS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Write Mode"]
    #[inline(always)]
    pub fn wmode(&mut self) -> WMODE_W {
        WMODE_W { w: self }
    }
    #[doc = "Bit 1 - Automatic Page Reallocation Disable"]
    #[inline(always)]
    pub fn aprdis(&mut self) -> APRDIS_W {
        APRDIS_W { w: self }
    }
}
