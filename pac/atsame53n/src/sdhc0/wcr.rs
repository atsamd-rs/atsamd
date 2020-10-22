#[doc = "Reader of register WCR"]
pub type R = crate::R<u8, super::WCR>;
#[doc = "Writer for register WCR"]
pub type W = crate::W<u8, super::WCR>;
#[doc = "Register WCR `reset()`'s with value 0"]
impl crate::ResetValue for super::WCR {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Wakeup Event Enable on Card Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKENCINT_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Enable"]
    ENABLE = 1,
}
impl From<WKENCINT_A> for bool {
    #[inline(always)]
    fn from(variant: WKENCINT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `WKENCINT`"]
pub type WKENCINT_R = crate::R<bool, WKENCINT_A>;
impl WKENCINT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WKENCINT_A {
        match self.bits {
            false => WKENCINT_A::DISABLE,
            true => WKENCINT_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == WKENCINT_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == WKENCINT_A::ENABLE
    }
}
#[doc = "Write proxy for field `WKENCINT`"]
pub struct WKENCINT_W<'a> {
    w: &'a mut W,
}
impl<'a> WKENCINT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WKENCINT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(WKENCINT_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(WKENCINT_A::ENABLE)
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
#[doc = "Wakeup Event Enable on Card Insertion\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKENCINS_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Enable"]
    ENABLE = 1,
}
impl From<WKENCINS_A> for bool {
    #[inline(always)]
    fn from(variant: WKENCINS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `WKENCINS`"]
pub type WKENCINS_R = crate::R<bool, WKENCINS_A>;
impl WKENCINS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WKENCINS_A {
        match self.bits {
            false => WKENCINS_A::DISABLE,
            true => WKENCINS_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == WKENCINS_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == WKENCINS_A::ENABLE
    }
}
#[doc = "Write proxy for field `WKENCINS`"]
pub struct WKENCINS_W<'a> {
    w: &'a mut W,
}
impl<'a> WKENCINS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WKENCINS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(WKENCINS_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(WKENCINS_A::ENABLE)
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
#[doc = "Wakeup Event Enable on Card Removal\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKENCREM_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Enable"]
    ENABLE = 1,
}
impl From<WKENCREM_A> for bool {
    #[inline(always)]
    fn from(variant: WKENCREM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `WKENCREM`"]
pub type WKENCREM_R = crate::R<bool, WKENCREM_A>;
impl WKENCREM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WKENCREM_A {
        match self.bits {
            false => WKENCREM_A::DISABLE,
            true => WKENCREM_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == WKENCREM_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == WKENCREM_A::ENABLE
    }
}
#[doc = "Write proxy for field `WKENCREM`"]
pub struct WKENCREM_W<'a> {
    w: &'a mut W,
}
impl<'a> WKENCREM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WKENCREM_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(WKENCREM_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(WKENCREM_A::ENABLE)
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
    #[doc = "Bit 0 - Wakeup Event Enable on Card Interrupt"]
    #[inline(always)]
    pub fn wkencint(&self) -> WKENCINT_R {
        WKENCINT_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Wakeup Event Enable on Card Insertion"]
    #[inline(always)]
    pub fn wkencins(&self) -> WKENCINS_R {
        WKENCINS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Wakeup Event Enable on Card Removal"]
    #[inline(always)]
    pub fn wkencrem(&self) -> WKENCREM_R {
        WKENCREM_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Wakeup Event Enable on Card Interrupt"]
    #[inline(always)]
    pub fn wkencint(&mut self) -> WKENCINT_W {
        WKENCINT_W { w: self }
    }
    #[doc = "Bit 1 - Wakeup Event Enable on Card Insertion"]
    #[inline(always)]
    pub fn wkencins(&mut self) -> WKENCINS_W {
        WKENCINS_W { w: self }
    }
    #[doc = "Bit 2 - Wakeup Event Enable on Card Removal"]
    #[inline(always)]
    pub fn wkencrem(&mut self) -> WKENCREM_W {
        WKENCREM_W { w: self }
    }
}
