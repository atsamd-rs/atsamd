#[doc = "Reader of register IRQMON%s"]
pub type R = crate::R<u16, super::IRQMON>;
#[doc = "Writer for register IRQMON%s"]
pub type W = crate::W<u16, super::IRQMON>;
#[doc = "Register IRQMON%s `reset()`'s with value 0"]
impl crate::ResetValue for super::IRQMON {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Extended Interrupt Request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EXTEND_A {
    #[doc = "0: Event is Interrupt Request signal"]
    NO = 0,
    #[doc = "1: Event is Interrupt Request signal extended until end of Interrupt Handler"]
    YES = 1,
}
impl From<EXTEND_A> for bool {
    #[inline(always)]
    fn from(variant: EXTEND_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EXTEND`"]
pub type EXTEND_R = crate::R<bool, EXTEND_A>;
impl EXTEND_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EXTEND_A {
        match self.bits {
            false => EXTEND_A::NO,
            true => EXTEND_A::YES,
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == EXTEND_A::NO
    }
    #[doc = "Checks if the value of the field is `YES`"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        *self == EXTEND_A::YES
    }
}
#[doc = "Write proxy for field `EXTEND`"]
pub struct EXTEND_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTEND_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EXTEND_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Event is Interrupt Request signal"]
    #[inline(always)]
    pub fn no(self) -> &'a mut W {
        self.variant(EXTEND_A::NO)
    }
    #[doc = "Event is Interrupt Request signal extended until end of Interrupt Handler"]
    #[inline(always)]
    pub fn yes(self) -> &'a mut W {
        self.variant(EXTEND_A::YES)
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u16) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `DROP`"]
pub type DROP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DROP`"]
pub struct DROP_W<'a> {
    w: &'a mut W,
}
impl<'a> DROP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u16) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `CPUID`"]
pub type CPUID_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CPUID`"]
pub struct CPUID_W<'a> {
    w: &'a mut W,
}
impl<'a> CPUID_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u16) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `IRQNUM`"]
pub type IRQNUM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IRQNUM`"]
pub struct IRQNUM_W<'a> {
    w: &'a mut W,
}
impl<'a> IRQNUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u16) & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Extended Interrupt Request"]
    #[inline(always)]
    pub fn extend(&self) -> EXTEND_R {
        EXTEND_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Drop Shortened Events"]
    #[inline(always)]
    pub fn drop(&self) -> DROP_R {
        DROP_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - ID of CPU currently servicing this IRQ"]
    #[inline(always)]
    pub fn cpuid(&self) -> CPUID_R {
        CPUID_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 8:15 - Interrupt Request Number"]
    #[inline(always)]
    pub fn irqnum(&self) -> IRQNUM_R {
        IRQNUM_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Extended Interrupt Request"]
    #[inline(always)]
    pub fn extend(&mut self) -> EXTEND_W {
        EXTEND_W { w: self }
    }
    #[doc = "Bit 1 - Drop Shortened Events"]
    #[inline(always)]
    pub fn drop(&mut self) -> DROP_W {
        DROP_W { w: self }
    }
    #[doc = "Bit 2 - ID of CPU currently servicing this IRQ"]
    #[inline(always)]
    pub fn cpuid(&mut self) -> CPUID_W {
        CPUID_W { w: self }
    }
    #[doc = "Bits 8:15 - Interrupt Request Number"]
    #[inline(always)]
    pub fn irqnum(&mut self) -> IRQNUM_W {
        IRQNUM_W { w: self }
    }
}
