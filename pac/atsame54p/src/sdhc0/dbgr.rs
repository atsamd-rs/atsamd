#[doc = "Reader of register DBGR"]
pub type R = crate::R<u8, super::DBGR>;
#[doc = "Writer for register DBGR"]
pub type W = crate::W<u8, super::DBGR>;
#[doc = "Register DBGR `reset()`'s with value 0"]
impl crate::ResetValue for super::DBGR {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Non-intrusive debug enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NIDBG_A {
    #[doc = "0: Debugging is intrusive (reads of BDPR from debugger are considered and increment the internal buffer pointer)"]
    IDBG = 0,
    #[doc = "1: Debugging is not intrusive (reads of BDPR from debugger are discarded and do not increment the internal buffer pointer)"]
    NIDBG = 1,
}
impl From<NIDBG_A> for bool {
    #[inline(always)]
    fn from(variant: NIDBG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `NIDBG`"]
pub type NIDBG_R = crate::R<bool, NIDBG_A>;
impl NIDBG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NIDBG_A {
        match self.bits {
            false => NIDBG_A::IDBG,
            true => NIDBG_A::NIDBG,
        }
    }
    #[doc = "Checks if the value of the field is `IDBG`"]
    #[inline(always)]
    pub fn is_idbg(&self) -> bool {
        *self == NIDBG_A::IDBG
    }
    #[doc = "Checks if the value of the field is `NIDBG`"]
    #[inline(always)]
    pub fn is_nidbg(&self) -> bool {
        *self == NIDBG_A::NIDBG
    }
}
#[doc = "Write proxy for field `NIDBG`"]
pub struct NIDBG_W<'a> {
    w: &'a mut W,
}
impl<'a> NIDBG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: NIDBG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Debugging is intrusive (reads of BDPR from debugger are considered and increment the internal buffer pointer)"]
    #[inline(always)]
    pub fn idbg(self) -> &'a mut W {
        self.variant(NIDBG_A::IDBG)
    }
    #[doc = "Debugging is not intrusive (reads of BDPR from debugger are discarded and do not increment the internal buffer pointer)"]
    #[inline(always)]
    pub fn nidbg(self) -> &'a mut W {
        self.variant(NIDBG_A::NIDBG)
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
impl R {
    #[doc = "Bit 0 - Non-intrusive debug enable"]
    #[inline(always)]
    pub fn nidbg(&self) -> NIDBG_R {
        NIDBG_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Non-intrusive debug enable"]
    #[inline(always)]
    pub fn nidbg(&mut self) -> NIDBG_W {
        NIDBG_W { w: self }
    }
}
