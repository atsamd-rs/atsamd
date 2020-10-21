#[doc = "Reader of register DPRESCALER"]
pub type R = crate::R<u32, super::DPRESCALER>;
#[doc = "Writer for register DPRESCALER"]
pub type W = crate::W<u32, super::DPRESCALER>;
#[doc = "Register DPRESCALER `reset()`'s with value 0"]
impl crate::ResetValue for super::DPRESCALER {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Debouncer Prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PRESCALER0_A {
    #[doc = "0: EIC clock divided by 2"]
    DIV2 = 0,
    #[doc = "1: EIC clock divided by 4"]
    DIV4 = 1,
    #[doc = "2: EIC clock divided by 8"]
    DIV8 = 2,
    #[doc = "3: EIC clock divided by 16"]
    DIV16 = 3,
    #[doc = "4: EIC clock divided by 32"]
    DIV32 = 4,
    #[doc = "5: EIC clock divided by 64"]
    DIV64 = 5,
    #[doc = "6: EIC clock divided by 128"]
    DIV128 = 6,
    #[doc = "7: EIC clock divided by 256"]
    DIV256 = 7,
}
impl From<PRESCALER0_A> for u8 {
    #[inline(always)]
    fn from(variant: PRESCALER0_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PRESCALER0`"]
pub type PRESCALER0_R = crate::R<u8, PRESCALER0_A>;
impl PRESCALER0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PRESCALER0_A {
        match self.bits {
            0 => PRESCALER0_A::DIV2,
            1 => PRESCALER0_A::DIV4,
            2 => PRESCALER0_A::DIV8,
            3 => PRESCALER0_A::DIV16,
            4 => PRESCALER0_A::DIV32,
            5 => PRESCALER0_A::DIV64,
            6 => PRESCALER0_A::DIV128,
            7 => PRESCALER0_A::DIV256,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == PRESCALER0_A::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == PRESCALER0_A::DIV4
    }
    #[doc = "Checks if the value of the field is `DIV8`"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == PRESCALER0_A::DIV8
    }
    #[doc = "Checks if the value of the field is `DIV16`"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == PRESCALER0_A::DIV16
    }
    #[doc = "Checks if the value of the field is `DIV32`"]
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == PRESCALER0_A::DIV32
    }
    #[doc = "Checks if the value of the field is `DIV64`"]
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == PRESCALER0_A::DIV64
    }
    #[doc = "Checks if the value of the field is `DIV128`"]
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        *self == PRESCALER0_A::DIV128
    }
    #[doc = "Checks if the value of the field is `DIV256`"]
    #[inline(always)]
    pub fn is_div256(&self) -> bool {
        *self == PRESCALER0_A::DIV256
    }
}
#[doc = "Write proxy for field `PRESCALER0`"]
pub struct PRESCALER0_W<'a> {
    w: &'a mut W,
}
impl<'a> PRESCALER0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PRESCALER0_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "EIC clock divided by 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(PRESCALER0_A::DIV2)
    }
    #[doc = "EIC clock divided by 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(PRESCALER0_A::DIV4)
    }
    #[doc = "EIC clock divided by 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(PRESCALER0_A::DIV8)
    }
    #[doc = "EIC clock divided by 16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut W {
        self.variant(PRESCALER0_A::DIV16)
    }
    #[doc = "EIC clock divided by 32"]
    #[inline(always)]
    pub fn div32(self) -> &'a mut W {
        self.variant(PRESCALER0_A::DIV32)
    }
    #[doc = "EIC clock divided by 64"]
    #[inline(always)]
    pub fn div64(self) -> &'a mut W {
        self.variant(PRESCALER0_A::DIV64)
    }
    #[doc = "EIC clock divided by 128"]
    #[inline(always)]
    pub fn div128(self) -> &'a mut W {
        self.variant(PRESCALER0_A::DIV128)
    }
    #[doc = "EIC clock divided by 256"]
    #[inline(always)]
    pub fn div256(self) -> &'a mut W {
        self.variant(PRESCALER0_A::DIV256)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "Debouncer number of states\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STATES0_A {
    #[doc = "0: 3 low frequency samples"]
    LFREQ3 = 0,
    #[doc = "1: 7 low frequency samples"]
    LFREQ7 = 1,
}
impl From<STATES0_A> for bool {
    #[inline(always)]
    fn from(variant: STATES0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `STATES0`"]
pub type STATES0_R = crate::R<bool, STATES0_A>;
impl STATES0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STATES0_A {
        match self.bits {
            false => STATES0_A::LFREQ3,
            true => STATES0_A::LFREQ7,
        }
    }
    #[doc = "Checks if the value of the field is `LFREQ3`"]
    #[inline(always)]
    pub fn is_lfreq3(&self) -> bool {
        *self == STATES0_A::LFREQ3
    }
    #[doc = "Checks if the value of the field is `LFREQ7`"]
    #[inline(always)]
    pub fn is_lfreq7(&self) -> bool {
        *self == STATES0_A::LFREQ7
    }
}
#[doc = "Write proxy for field `STATES0`"]
pub struct STATES0_W<'a> {
    w: &'a mut W,
}
impl<'a> STATES0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STATES0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "3 low frequency samples"]
    #[inline(always)]
    pub fn lfreq3(self) -> &'a mut W {
        self.variant(STATES0_A::LFREQ3)
    }
    #[doc = "7 low frequency samples"]
    #[inline(always)]
    pub fn lfreq7(self) -> &'a mut W {
        self.variant(STATES0_A::LFREQ7)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Debouncer Prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PRESCALER1_A {
    #[doc = "0: EIC clock divided by 2"]
    DIV2 = 0,
    #[doc = "1: EIC clock divided by 4"]
    DIV4 = 1,
    #[doc = "2: EIC clock divided by 8"]
    DIV8 = 2,
    #[doc = "3: EIC clock divided by 16"]
    DIV16 = 3,
    #[doc = "4: EIC clock divided by 32"]
    DIV32 = 4,
    #[doc = "5: EIC clock divided by 64"]
    DIV64 = 5,
    #[doc = "6: EIC clock divided by 128"]
    DIV128 = 6,
    #[doc = "7: EIC clock divided by 256"]
    DIV256 = 7,
}
impl From<PRESCALER1_A> for u8 {
    #[inline(always)]
    fn from(variant: PRESCALER1_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PRESCALER1`"]
pub type PRESCALER1_R = crate::R<u8, PRESCALER1_A>;
impl PRESCALER1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PRESCALER1_A {
        match self.bits {
            0 => PRESCALER1_A::DIV2,
            1 => PRESCALER1_A::DIV4,
            2 => PRESCALER1_A::DIV8,
            3 => PRESCALER1_A::DIV16,
            4 => PRESCALER1_A::DIV32,
            5 => PRESCALER1_A::DIV64,
            6 => PRESCALER1_A::DIV128,
            7 => PRESCALER1_A::DIV256,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == PRESCALER1_A::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == PRESCALER1_A::DIV4
    }
    #[doc = "Checks if the value of the field is `DIV8`"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == PRESCALER1_A::DIV8
    }
    #[doc = "Checks if the value of the field is `DIV16`"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == PRESCALER1_A::DIV16
    }
    #[doc = "Checks if the value of the field is `DIV32`"]
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == PRESCALER1_A::DIV32
    }
    #[doc = "Checks if the value of the field is `DIV64`"]
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == PRESCALER1_A::DIV64
    }
    #[doc = "Checks if the value of the field is `DIV128`"]
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        *self == PRESCALER1_A::DIV128
    }
    #[doc = "Checks if the value of the field is `DIV256`"]
    #[inline(always)]
    pub fn is_div256(&self) -> bool {
        *self == PRESCALER1_A::DIV256
    }
}
#[doc = "Write proxy for field `PRESCALER1`"]
pub struct PRESCALER1_W<'a> {
    w: &'a mut W,
}
impl<'a> PRESCALER1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PRESCALER1_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "EIC clock divided by 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(PRESCALER1_A::DIV2)
    }
    #[doc = "EIC clock divided by 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(PRESCALER1_A::DIV4)
    }
    #[doc = "EIC clock divided by 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(PRESCALER1_A::DIV8)
    }
    #[doc = "EIC clock divided by 16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut W {
        self.variant(PRESCALER1_A::DIV16)
    }
    #[doc = "EIC clock divided by 32"]
    #[inline(always)]
    pub fn div32(self) -> &'a mut W {
        self.variant(PRESCALER1_A::DIV32)
    }
    #[doc = "EIC clock divided by 64"]
    #[inline(always)]
    pub fn div64(self) -> &'a mut W {
        self.variant(PRESCALER1_A::DIV64)
    }
    #[doc = "EIC clock divided by 128"]
    #[inline(always)]
    pub fn div128(self) -> &'a mut W {
        self.variant(PRESCALER1_A::DIV128)
    }
    #[doc = "EIC clock divided by 256"]
    #[inline(always)]
    pub fn div256(self) -> &'a mut W {
        self.variant(PRESCALER1_A::DIV256)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | (((value as u32) & 0x07) << 4);
        self.w
    }
}
#[doc = "Debouncer number of states\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STATES1_A {
    #[doc = "0: 3 low frequency samples"]
    LFREQ3 = 0,
    #[doc = "1: 7 low frequency samples"]
    LFREQ7 = 1,
}
impl From<STATES1_A> for bool {
    #[inline(always)]
    fn from(variant: STATES1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `STATES1`"]
pub type STATES1_R = crate::R<bool, STATES1_A>;
impl STATES1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STATES1_A {
        match self.bits {
            false => STATES1_A::LFREQ3,
            true => STATES1_A::LFREQ7,
        }
    }
    #[doc = "Checks if the value of the field is `LFREQ3`"]
    #[inline(always)]
    pub fn is_lfreq3(&self) -> bool {
        *self == STATES1_A::LFREQ3
    }
    #[doc = "Checks if the value of the field is `LFREQ7`"]
    #[inline(always)]
    pub fn is_lfreq7(&self) -> bool {
        *self == STATES1_A::LFREQ7
    }
}
#[doc = "Write proxy for field `STATES1`"]
pub struct STATES1_W<'a> {
    w: &'a mut W,
}
impl<'a> STATES1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STATES1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "3 low frequency samples"]
    #[inline(always)]
    pub fn lfreq3(self) -> &'a mut W {
        self.variant(STATES1_A::LFREQ3)
    }
    #[doc = "7 low frequency samples"]
    #[inline(always)]
    pub fn lfreq7(self) -> &'a mut W {
        self.variant(STATES1_A::LFREQ7)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Pin Sampler frequency selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TICKON_A {
    #[doc = "0: Clocked by GCLK"]
    CLK_GCLK_EIC = 0,
    #[doc = "1: Clocked by Low Frequency Clock"]
    CLK_LFREQ = 1,
}
impl From<TICKON_A> for bool {
    #[inline(always)]
    fn from(variant: TICKON_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TICKON`"]
pub type TICKON_R = crate::R<bool, TICKON_A>;
impl TICKON_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TICKON_A {
        match self.bits {
            false => TICKON_A::CLK_GCLK_EIC,
            true => TICKON_A::CLK_LFREQ,
        }
    }
    #[doc = "Checks if the value of the field is `CLK_GCLK_EIC`"]
    #[inline(always)]
    pub fn is_clk_gclk_eic(&self) -> bool {
        *self == TICKON_A::CLK_GCLK_EIC
    }
    #[doc = "Checks if the value of the field is `CLK_LFREQ`"]
    #[inline(always)]
    pub fn is_clk_lfreq(&self) -> bool {
        *self == TICKON_A::CLK_LFREQ
    }
}
#[doc = "Write proxy for field `TICKON`"]
pub struct TICKON_W<'a> {
    w: &'a mut W,
}
impl<'a> TICKON_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TICKON_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clocked by GCLK"]
    #[inline(always)]
    pub fn clk_gclk_eic(self) -> &'a mut W {
        self.variant(TICKON_A::CLK_GCLK_EIC)
    }
    #[doc = "Clocked by Low Frequency Clock"]
    #[inline(always)]
    pub fn clk_lfreq(self) -> &'a mut W {
        self.variant(TICKON_A::CLK_LFREQ)
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Debouncer Prescaler"]
    #[inline(always)]
    pub fn prescaler0(&self) -> PRESCALER0_R {
        PRESCALER0_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bit 3 - Debouncer number of states"]
    #[inline(always)]
    pub fn states0(&self) -> STATES0_R {
        STATES0_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 4:6 - Debouncer Prescaler"]
    #[inline(always)]
    pub fn prescaler1(&self) -> PRESCALER1_R {
        PRESCALER1_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bit 7 - Debouncer number of states"]
    #[inline(always)]
    pub fn states1(&self) -> STATES1_R {
        STATES1_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Pin Sampler frequency selection"]
    #[inline(always)]
    pub fn tickon(&self) -> TICKON_R {
        TICKON_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Debouncer Prescaler"]
    #[inline(always)]
    pub fn prescaler0(&mut self) -> PRESCALER0_W {
        PRESCALER0_W { w: self }
    }
    #[doc = "Bit 3 - Debouncer number of states"]
    #[inline(always)]
    pub fn states0(&mut self) -> STATES0_W {
        STATES0_W { w: self }
    }
    #[doc = "Bits 4:6 - Debouncer Prescaler"]
    #[inline(always)]
    pub fn prescaler1(&mut self) -> PRESCALER1_W {
        PRESCALER1_W { w: self }
    }
    #[doc = "Bit 7 - Debouncer number of states"]
    #[inline(always)]
    pub fn states1(&mut self) -> STATES1_W {
        STATES1_W { w: self }
    }
    #[doc = "Bit 16 - Pin Sampler frequency selection"]
    #[inline(always)]
    pub fn tickon(&mut self) -> TICKON_W {
        TICKON_W { w: self }
    }
}
