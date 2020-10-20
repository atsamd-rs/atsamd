#[doc = "Reader of register SRR"]
pub type R = crate::R<u8, super::SRR>;
#[doc = "Writer for register SRR"]
pub type W = crate::W<u8, super::SRR>;
#[doc = "Register SRR `reset()`'s with value 0"]
impl crate::ResetValue for super::SRR {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Software Reset For All\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWRSTALL_A {
    #[doc = "0: Work"]
    WORK = 0,
    #[doc = "1: Reset"]
    RESET = 1,
}
impl From<SWRSTALL_A> for bool {
    #[inline(always)]
    fn from(variant: SWRSTALL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SWRSTALL`"]
pub type SWRSTALL_R = crate::R<bool, SWRSTALL_A>;
impl SWRSTALL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SWRSTALL_A {
        match self.bits {
            false => SWRSTALL_A::WORK,
            true => SWRSTALL_A::RESET,
        }
    }
    #[doc = "Checks if the value of the field is `WORK`"]
    #[inline(always)]
    pub fn is_work(&self) -> bool {
        *self == SWRSTALL_A::WORK
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == SWRSTALL_A::RESET
    }
}
#[doc = "Write proxy for field `SWRSTALL`"]
pub struct SWRSTALL_W<'a> {
    w: &'a mut W,
}
impl<'a> SWRSTALL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SWRSTALL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Work"]
    #[inline(always)]
    pub fn work(self) -> &'a mut W {
        self.variant(SWRSTALL_A::WORK)
    }
    #[doc = "Reset"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(SWRSTALL_A::RESET)
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
#[doc = "Software Reset For CMD Line\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWRSTCMD_A {
    #[doc = "0: Work"]
    WORK = 0,
    #[doc = "1: Reset"]
    RESET = 1,
}
impl From<SWRSTCMD_A> for bool {
    #[inline(always)]
    fn from(variant: SWRSTCMD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SWRSTCMD`"]
pub type SWRSTCMD_R = crate::R<bool, SWRSTCMD_A>;
impl SWRSTCMD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SWRSTCMD_A {
        match self.bits {
            false => SWRSTCMD_A::WORK,
            true => SWRSTCMD_A::RESET,
        }
    }
    #[doc = "Checks if the value of the field is `WORK`"]
    #[inline(always)]
    pub fn is_work(&self) -> bool {
        *self == SWRSTCMD_A::WORK
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == SWRSTCMD_A::RESET
    }
}
#[doc = "Write proxy for field `SWRSTCMD`"]
pub struct SWRSTCMD_W<'a> {
    w: &'a mut W,
}
impl<'a> SWRSTCMD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SWRSTCMD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Work"]
    #[inline(always)]
    pub fn work(self) -> &'a mut W {
        self.variant(SWRSTCMD_A::WORK)
    }
    #[doc = "Reset"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(SWRSTCMD_A::RESET)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u8) & 0x01) << 1);
        self.w
    }
}
#[doc = "Software Reset For DAT Line\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWRSTDAT_A {
    #[doc = "0: Work"]
    WORK = 0,
    #[doc = "1: Reset"]
    RESET = 1,
}
impl From<SWRSTDAT_A> for bool {
    #[inline(always)]
    fn from(variant: SWRSTDAT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SWRSTDAT`"]
pub type SWRSTDAT_R = crate::R<bool, SWRSTDAT_A>;
impl SWRSTDAT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SWRSTDAT_A {
        match self.bits {
            false => SWRSTDAT_A::WORK,
            true => SWRSTDAT_A::RESET,
        }
    }
    #[doc = "Checks if the value of the field is `WORK`"]
    #[inline(always)]
    pub fn is_work(&self) -> bool {
        *self == SWRSTDAT_A::WORK
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == SWRSTDAT_A::RESET
    }
}
#[doc = "Write proxy for field `SWRSTDAT`"]
pub struct SWRSTDAT_W<'a> {
    w: &'a mut W,
}
impl<'a> SWRSTDAT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SWRSTDAT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Work"]
    #[inline(always)]
    pub fn work(self) -> &'a mut W {
        self.variant(SWRSTDAT_A::WORK)
    }
    #[doc = "Reset"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(SWRSTDAT_A::RESET)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u8) & 0x01) << 2);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Software Reset For All"]
    #[inline(always)]
    pub fn swrstall(&self) -> SWRSTALL_R {
        SWRSTALL_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Software Reset For CMD Line"]
    #[inline(always)]
    pub fn swrstcmd(&self) -> SWRSTCMD_R {
        SWRSTCMD_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Software Reset For DAT Line"]
    #[inline(always)]
    pub fn swrstdat(&self) -> SWRSTDAT_R {
        SWRSTDAT_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Software Reset For All"]
    #[inline(always)]
    pub fn swrstall(&mut self) -> SWRSTALL_W {
        SWRSTALL_W { w: self }
    }
    #[doc = "Bit 1 - Software Reset For CMD Line"]
    #[inline(always)]
    pub fn swrstcmd(&mut self) -> SWRSTCMD_W {
        SWRSTCMD_W { w: self }
    }
    #[doc = "Bit 2 - Software Reset For DAT Line"]
    #[inline(always)]
    pub fn swrstdat(&mut self) -> SWRSTDAT_W {
        SWRSTDAT_W { w: self }
    }
}
