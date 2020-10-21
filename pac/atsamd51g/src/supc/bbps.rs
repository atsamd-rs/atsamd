#[doc = "Reader of register BBPS"]
pub type R = crate::R<u32, super::BBPS>;
#[doc = "Writer for register BBPS"]
pub type W = crate::W<u32, super::BBPS>;
#[doc = "Register BBPS `reset()`'s with value 0"]
impl crate::ResetValue for super::BBPS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Battery Backup Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CONF_A {
    #[doc = "0: The power switch is handled by the BOD33"]
    BOD33 = 0,
    #[doc = "1: In Backup Domain, the backup domain is always supplied by battery backup power"]
    FORCED = 1,
}
impl From<CONF_A> for bool {
    #[inline(always)]
    fn from(variant: CONF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CONF`"]
pub type CONF_R = crate::R<bool, CONF_A>;
impl CONF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CONF_A {
        match self.bits {
            false => CONF_A::BOD33,
            true => CONF_A::FORCED,
        }
    }
    #[doc = "Checks if the value of the field is `BOD33`"]
    #[inline(always)]
    pub fn is_bod33(&self) -> bool {
        *self == CONF_A::BOD33
    }
    #[doc = "Checks if the value of the field is `FORCED`"]
    #[inline(always)]
    pub fn is_forced(&self) -> bool {
        *self == CONF_A::FORCED
    }
}
#[doc = "Write proxy for field `CONF`"]
pub struct CONF_W<'a> {
    w: &'a mut W,
}
impl<'a> CONF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CONF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The power switch is handled by the BOD33"]
    #[inline(always)]
    pub fn bod33(self) -> &'a mut W {
        self.variant(CONF_A::BOD33)
    }
    #[doc = "In Backup Domain, the backup domain is always supplied by battery backup power"]
    #[inline(always)]
    pub fn forced(self) -> &'a mut W {
        self.variant(CONF_A::FORCED)
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `WAKEEN`"]
pub type WAKEEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WAKEEN`"]
pub struct WAKEEN_W<'a> {
    w: &'a mut W,
}
impl<'a> WAKEEN_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Battery Backup Configuration"]
    #[inline(always)]
    pub fn conf(&self) -> CONF_R {
        CONF_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 2 - Wake Enable"]
    #[inline(always)]
    pub fn wakeen(&self) -> WAKEEN_R {
        WAKEEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Battery Backup Configuration"]
    #[inline(always)]
    pub fn conf(&mut self) -> CONF_W {
        CONF_W { w: self }
    }
    #[doc = "Bit 2 - Wake Enable"]
    #[inline(always)]
    pub fn wakeen(&mut self) -> WAKEEN_W {
        WAKEEN_W { w: self }
    }
}
