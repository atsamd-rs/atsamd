#[doc = "Reader of register CTRLB"]
pub type R = crate::R<u8, super::CTRLB>;
#[doc = "Writer for register CTRLB"]
pub type W = crate::W<u8, super::CTRLB>;
#[doc = "Register CTRLB `reset()`'s with value 0x02"]
impl crate::ResetValue for super::CTRLB {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x02
    }
}
#[doc = "Reader of field `DIFF`"]
pub type DIFF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIFF`"]
pub struct DIFF_W<'a> {
    w: &'a mut W,
}
impl<'a> DIFF_W<'a> {
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
#[doc = "Reference Selection for DAC0/1\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum REFSEL_A {
    #[doc = "0: External reference unbuffered"]
    VREFPU = 0,
    #[doc = "1: Analog supply"]
    VDDANA = 1,
    #[doc = "2: External reference buffered"]
    VREFPB = 2,
    #[doc = "3: Internal bandgap reference"]
    INTREF = 3,
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
    pub fn variant(&self) -> REFSEL_A {
        match self.bits {
            0 => REFSEL_A::VREFPU,
            1 => REFSEL_A::VDDANA,
            2 => REFSEL_A::VREFPB,
            3 => REFSEL_A::INTREF,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VREFPU`"]
    #[inline(always)]
    pub fn is_vrefpu(&self) -> bool {
        *self == REFSEL_A::VREFPU
    }
    #[doc = "Checks if the value of the field is `VDDANA`"]
    #[inline(always)]
    pub fn is_vddana(&self) -> bool {
        *self == REFSEL_A::VDDANA
    }
    #[doc = "Checks if the value of the field is `VREFPB`"]
    #[inline(always)]
    pub fn is_vrefpb(&self) -> bool {
        *self == REFSEL_A::VREFPB
    }
    #[doc = "Checks if the value of the field is `INTREF`"]
    #[inline(always)]
    pub fn is_intref(&self) -> bool {
        *self == REFSEL_A::INTREF
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
        {
            self.bits(variant.into())
        }
    }
    #[doc = "External reference unbuffered"]
    #[inline(always)]
    pub fn vrefpu(self) -> &'a mut W {
        self.variant(REFSEL_A::VREFPU)
    }
    #[doc = "Analog supply"]
    #[inline(always)]
    pub fn vddana(self) -> &'a mut W {
        self.variant(REFSEL_A::VDDANA)
    }
    #[doc = "External reference buffered"]
    #[inline(always)]
    pub fn vrefpb(self) -> &'a mut W {
        self.variant(REFSEL_A::VREFPB)
    }
    #[doc = "Internal bandgap reference"]
    #[inline(always)]
    pub fn intref(self) -> &'a mut W {
        self.variant(REFSEL_A::INTREF)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 1)) | (((value as u8) & 0x03) << 1);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Differential mode enable"]
    #[inline(always)]
    pub fn diff(&self) -> DIFF_R {
        DIFF_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:2 - Reference Selection for DAC0/1"]
    #[inline(always)]
    pub fn refsel(&self) -> REFSEL_R {
        REFSEL_R::new(((self.bits >> 1) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Differential mode enable"]
    #[inline(always)]
    pub fn diff(&mut self) -> DIFF_W {
        DIFF_W { w: self }
    }
    #[doc = "Bits 1:2 - Reference Selection for DAC0/1"]
    #[inline(always)]
    pub fn refsel(&mut self) -> REFSEL_W {
        REFSEL_W { w: self }
    }
}
