#[doc = "Reader of register GENCTRL"]
pub type R = crate::R<u32, super::GENCTRL>;
#[doc = "Writer for register GENCTRL"]
pub type W = crate::W<u32, super::GENCTRL>;
#[doc = "Register GENCTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::GENCTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ID`"]
pub type ID_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ID`"]
pub struct ID_W<'a> {
    w: &'a mut W,
}
impl<'a> ID_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Source Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SRC_A {
    #[doc = "0: XOSC oscillator output"]
    XOSC = 0,
    #[doc = "1: Generator input pad"]
    GCLKIN = 1,
    #[doc = "2: Generic clock generator 1 output"]
    GCLKGEN1 = 2,
    #[doc = "3: OSCULP32K oscillator output"]
    OSCULP32K = 3,
    #[doc = "4: OSC32K oscillator output"]
    OSC32K = 4,
    #[doc = "5: XOSC32K oscillator output"]
    XOSC32K = 5,
    #[doc = "6: OSC8M oscillator output"]
    OSC8M = 6,
    #[doc = "7: DFLL48M output"]
    DFLL48M = 7,
    #[doc = "8: DPLL96M output"]
    DPLL96M = 8,
}
impl From<SRC_A> for u8 {
    #[inline(always)]
    fn from(variant: SRC_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SRC`"]
pub type SRC_R = crate::R<u8, SRC_A>;
impl SRC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SRC_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SRC_A::XOSC),
            1 => Val(SRC_A::GCLKIN),
            2 => Val(SRC_A::GCLKGEN1),
            3 => Val(SRC_A::OSCULP32K),
            4 => Val(SRC_A::OSC32K),
            5 => Val(SRC_A::XOSC32K),
            6 => Val(SRC_A::OSC8M),
            7 => Val(SRC_A::DFLL48M),
            8 => Val(SRC_A::DPLL96M),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `XOSC`"]
    #[inline(always)]
    pub fn is_xosc(&self) -> bool {
        *self == SRC_A::XOSC
    }
    #[doc = "Checks if the value of the field is `GCLKIN`"]
    #[inline(always)]
    pub fn is_gclkin(&self) -> bool {
        *self == SRC_A::GCLKIN
    }
    #[doc = "Checks if the value of the field is `GCLKGEN1`"]
    #[inline(always)]
    pub fn is_gclkgen1(&self) -> bool {
        *self == SRC_A::GCLKGEN1
    }
    #[doc = "Checks if the value of the field is `OSCULP32K`"]
    #[inline(always)]
    pub fn is_osculp32k(&self) -> bool {
        *self == SRC_A::OSCULP32K
    }
    #[doc = "Checks if the value of the field is `OSC32K`"]
    #[inline(always)]
    pub fn is_osc32k(&self) -> bool {
        *self == SRC_A::OSC32K
    }
    #[doc = "Checks if the value of the field is `XOSC32K`"]
    #[inline(always)]
    pub fn is_xosc32k(&self) -> bool {
        *self == SRC_A::XOSC32K
    }
    #[doc = "Checks if the value of the field is `OSC8M`"]
    #[inline(always)]
    pub fn is_osc8m(&self) -> bool {
        *self == SRC_A::OSC8M
    }
    #[doc = "Checks if the value of the field is `DFLL48M`"]
    #[inline(always)]
    pub fn is_dfll48m(&self) -> bool {
        *self == SRC_A::DFLL48M
    }
    #[doc = "Checks if the value of the field is `DPLL96M`"]
    #[inline(always)]
    pub fn is_dpll96m(&self) -> bool {
        *self == SRC_A::DPLL96M
    }
}
#[doc = "Write proxy for field `SRC`"]
pub struct SRC_W<'a> {
    w: &'a mut W,
}
impl<'a> SRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SRC_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "XOSC oscillator output"]
    #[inline(always)]
    pub fn xosc(self) -> &'a mut W {
        self.variant(SRC_A::XOSC)
    }
    #[doc = "Generator input pad"]
    #[inline(always)]
    pub fn gclkin(self) -> &'a mut W {
        self.variant(SRC_A::GCLKIN)
    }
    #[doc = "Generic clock generator 1 output"]
    #[inline(always)]
    pub fn gclkgen1(self) -> &'a mut W {
        self.variant(SRC_A::GCLKGEN1)
    }
    #[doc = "OSCULP32K oscillator output"]
    #[inline(always)]
    pub fn osculp32k(self) -> &'a mut W {
        self.variant(SRC_A::OSCULP32K)
    }
    #[doc = "OSC32K oscillator output"]
    #[inline(always)]
    pub fn osc32k(self) -> &'a mut W {
        self.variant(SRC_A::OSC32K)
    }
    #[doc = "XOSC32K oscillator output"]
    #[inline(always)]
    pub fn xosc32k(self) -> &'a mut W {
        self.variant(SRC_A::XOSC32K)
    }
    #[doc = "OSC8M oscillator output"]
    #[inline(always)]
    pub fn osc8m(self) -> &'a mut W {
        self.variant(SRC_A::OSC8M)
    }
    #[doc = "DFLL48M output"]
    #[inline(always)]
    pub fn dfll48m(self) -> &'a mut W {
        self.variant(SRC_A::DFLL48M)
    }
    #[doc = "DPLL96M output"]
    #[inline(always)]
    pub fn dpll96m(self) -> &'a mut W {
        self.variant(SRC_A::DPLL96M)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 8)) | (((value as u32) & 0x1f) << 8);
        self.w
    }
}
#[doc = "Reader of field `GENEN`"]
pub type GENEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GENEN`"]
pub struct GENEN_W<'a> {
    w: &'a mut W,
}
impl<'a> GENEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `IDC`"]
pub type IDC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IDC`"]
pub struct IDC_W<'a> {
    w: &'a mut W,
}
impl<'a> IDC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Reader of field `OOV`"]
pub type OOV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OOV`"]
pub struct OOV_W<'a> {
    w: &'a mut W,
}
impl<'a> OOV_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Reader of field `OE`"]
pub type OE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OE`"]
pub struct OE_W<'a> {
    w: &'a mut W,
}
impl<'a> OE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Reader of field `DIVSEL`"]
pub type DIVSEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIVSEL`"]
pub struct DIVSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> DIVSEL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Reader of field `RUNSTDBY`"]
pub type RUNSTDBY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RUNSTDBY`"]
pub struct RUNSTDBY_W<'a> {
    w: &'a mut W,
}
impl<'a> RUNSTDBY_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Generic Clock Generator Selection"]
    #[inline(always)]
    pub fn id(&self) -> ID_R {
        ID_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:12 - Source Select"]
    #[inline(always)]
    pub fn src(&self) -> SRC_R {
        SRC_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bit 16 - Generic Clock Generator Enable"]
    #[inline(always)]
    pub fn genen(&self) -> GENEN_R {
        GENEN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Improve Duty Cycle"]
    #[inline(always)]
    pub fn idc(&self) -> IDC_R {
        IDC_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Output Off Value"]
    #[inline(always)]
    pub fn oov(&self) -> OOV_R {
        OOV_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Output Enable"]
    #[inline(always)]
    pub fn oe(&self) -> OE_R {
        OE_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Divide Selection"]
    #[inline(always)]
    pub fn divsel(&self) -> DIVSEL_R {
        DIVSEL_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Run in Standby"]
    #[inline(always)]
    pub fn runstdby(&self) -> RUNSTDBY_R {
        RUNSTDBY_R::new(((self.bits >> 21) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Generic Clock Generator Selection"]
    #[inline(always)]
    pub fn id(&mut self) -> ID_W {
        ID_W { w: self }
    }
    #[doc = "Bits 8:12 - Source Select"]
    #[inline(always)]
    pub fn src(&mut self) -> SRC_W {
        SRC_W { w: self }
    }
    #[doc = "Bit 16 - Generic Clock Generator Enable"]
    #[inline(always)]
    pub fn genen(&mut self) -> GENEN_W {
        GENEN_W { w: self }
    }
    #[doc = "Bit 17 - Improve Duty Cycle"]
    #[inline(always)]
    pub fn idc(&mut self) -> IDC_W {
        IDC_W { w: self }
    }
    #[doc = "Bit 18 - Output Off Value"]
    #[inline(always)]
    pub fn oov(&mut self) -> OOV_W {
        OOV_W { w: self }
    }
    #[doc = "Bit 19 - Output Enable"]
    #[inline(always)]
    pub fn oe(&mut self) -> OE_W {
        OE_W { w: self }
    }
    #[doc = "Bit 20 - Divide Selection"]
    #[inline(always)]
    pub fn divsel(&mut self) -> DIVSEL_W {
        DIVSEL_W { w: self }
    }
    #[doc = "Bit 21 - Run in Standby"]
    #[inline(always)]
    pub fn runstdby(&mut self) -> RUNSTDBY_W {
        RUNSTDBY_W { w: self }
    }
}
