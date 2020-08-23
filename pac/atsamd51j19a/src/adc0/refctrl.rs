#[doc = "Reader of register REFCTRL"]
pub type R = crate::R<u8, super::REFCTRL>;
#[doc = "Writer for register REFCTRL"]
pub type W = crate::W<u8, super::REFCTRL>;
#[doc = "Register REFCTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::REFCTRL {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reference Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum REFSEL_A {
    #[doc = "0: Internal Bandgap Reference"]
    INTREF = 0,
    #[doc = "2: 1/2 VDDANA"]
    INTVCC0 = 2,
    #[doc = "3: VDDANA"]
    INTVCC1 = 3,
    #[doc = "4: External Reference"]
    AREFA = 4,
    #[doc = "5: External Reference"]
    AREFB = 5,
    #[doc = "6: External Reference (only on ADC1)"]
    AREFC = 6,
}
impl From<REFSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: REFSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `REFSEL`"]
pub type REFSEL_R = crate::R<u8, REFSEL_A>;
impl REFSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, REFSEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(REFSEL_A::INTREF),
            2 => Val(REFSEL_A::INTVCC0),
            3 => Val(REFSEL_A::INTVCC1),
            4 => Val(REFSEL_A::AREFA),
            5 => Val(REFSEL_A::AREFB),
            6 => Val(REFSEL_A::AREFC),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `INTREF`"]
    #[inline(always)]
    pub fn is_intref(&self) -> bool {
        *self == REFSEL_A::INTREF
    }
    #[doc = "Checks if the value of the field is `INTVCC0`"]
    #[inline(always)]
    pub fn is_intvcc0(&self) -> bool {
        *self == REFSEL_A::INTVCC0
    }
    #[doc = "Checks if the value of the field is `INTVCC1`"]
    #[inline(always)]
    pub fn is_intvcc1(&self) -> bool {
        *self == REFSEL_A::INTVCC1
    }
    #[doc = "Checks if the value of the field is `AREFA`"]
    #[inline(always)]
    pub fn is_arefa(&self) -> bool {
        *self == REFSEL_A::AREFA
    }
    #[doc = "Checks if the value of the field is `AREFB`"]
    #[inline(always)]
    pub fn is_arefb(&self) -> bool {
        *self == REFSEL_A::AREFB
    }
    #[doc = "Checks if the value of the field is `AREFC`"]
    #[inline(always)]
    pub fn is_arefc(&self) -> bool {
        *self == REFSEL_A::AREFC
    }
}
#[doc = "Write proxy for field `REFSEL`"]
pub struct REFSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> REFSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REFSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Internal Bandgap Reference"]
    #[inline(always)]
    pub fn intref(self) -> &'a mut W {
        self.variant(REFSEL_A::INTREF)
    }
    #[doc = "1/2 VDDANA"]
    #[inline(always)]
    pub fn intvcc0(self) -> &'a mut W {
        self.variant(REFSEL_A::INTVCC0)
    }
    #[doc = "VDDANA"]
    #[inline(always)]
    pub fn intvcc1(self) -> &'a mut W {
        self.variant(REFSEL_A::INTVCC1)
    }
    #[doc = "External Reference"]
    #[inline(always)]
    pub fn arefa(self) -> &'a mut W {
        self.variant(REFSEL_A::AREFA)
    }
    #[doc = "External Reference"]
    #[inline(always)]
    pub fn arefb(self) -> &'a mut W {
        self.variant(REFSEL_A::AREFB)
    }
    #[doc = "External Reference (only on ADC1)"]
    #[inline(always)]
    pub fn arefc(self) -> &'a mut W {
        self.variant(REFSEL_A::AREFC)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u8) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `REFCOMP`"]
pub type REFCOMP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `REFCOMP`"]
pub struct REFCOMP_W<'a> {
    w: &'a mut W,
}
impl<'a> REFCOMP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u8) & 0x01) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Reference Selection"]
    #[inline(always)]
    pub fn refsel(&self) -> REFSEL_R {
        REFSEL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 7 - Reference Buffer Offset Compensation Enable"]
    #[inline(always)]
    pub fn refcomp(&self) -> REFCOMP_R {
        REFCOMP_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Reference Selection"]
    #[inline(always)]
    pub fn refsel(&mut self) -> REFSEL_W {
        REFSEL_W { w: self }
    }
    #[doc = "Bit 7 - Reference Buffer Offset Compensation Enable"]
    #[inline(always)]
    pub fn refcomp(&mut self) -> REFCOMP_W {
        REFCOMP_W { w: self }
    }
}
