#[doc = "Reader of register CC2R"]
pub type R = crate::R<u32, super::CC2R>;
#[doc = "Writer for register CC2R"]
pub type W = crate::W<u32, super::CC2R>;
#[doc = "Register CC2R `reset()`'s with value 0"]
impl crate::ResetValue for super::CC2R {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Force SDCK Disabled\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FSDCLKD_A {
    #[doc = "0: No effect"]
    NOEFFECT = 0,
    #[doc = "1: SDCLK can be stopped at any time after DATA transfer.SDCLK enable forcing for 8 SDCLK cycles is disabled"]
    DISABLE = 1,
}
impl From<FSDCLKD_A> for bool {
    #[inline(always)]
    fn from(variant: FSDCLKD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FSDCLKD`"]
pub type FSDCLKD_R = crate::R<bool, FSDCLKD_A>;
impl FSDCLKD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FSDCLKD_A {
        match self.bits {
            false => FSDCLKD_A::NOEFFECT,
            true => FSDCLKD_A::DISABLE,
        }
    }
    #[doc = "Checks if the value of the field is `NOEFFECT`"]
    #[inline(always)]
    pub fn is_noeffect(&self) -> bool {
        *self == FSDCLKD_A::NOEFFECT
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == FSDCLKD_A::DISABLE
    }
}
#[doc = "Write proxy for field `FSDCLKD`"]
pub struct FSDCLKD_W<'a> {
    w: &'a mut W,
}
impl<'a> FSDCLKD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FSDCLKD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn noeffect(self) -> &'a mut W {
        self.variant(FSDCLKD_A::NOEFFECT)
    }
    #[doc = "SDCLK can be stopped at any time after DATA transfer.SDCLK enable forcing for 8 SDCLK cycles is disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(FSDCLKD_A::DISABLE)
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
impl R {
    #[doc = "Bit 0 - Force SDCK Disabled"]
    #[inline(always)]
    pub fn fsdclkd(&self) -> FSDCLKD_R {
        FSDCLKD_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Force SDCK Disabled"]
    #[inline(always)]
    pub fn fsdclkd(&mut self) -> FSDCLKD_W {
        FSDCLKD_W { w: self }
    }
}
