#[doc = "Reader of register HC2R"]
pub type R = crate::R<u16, super::HC2R>;
#[doc = "Writer for register HC2R"]
pub type W = crate::W<u16, super::HC2R>;
#[doc = "Register HC2R `reset()`'s with value 0"]
impl crate::ResetValue for super::HC2R {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "UHS Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum UHSMS_A {
    #[doc = "0: SDR12"]
    SDR12 = 0,
    #[doc = "1: SDR25"]
    SDR25 = 1,
    #[doc = "2: SDR50"]
    SDR50 = 2,
    #[doc = "3: SDR104"]
    SDR104 = 3,
    #[doc = "4: DDR50"]
    DDR50 = 4,
}
impl From<UHSMS_A> for u8 {
    #[inline(always)]
    fn from(variant: UHSMS_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `UHSMS`"]
pub type UHSMS_R = crate::R<u8, UHSMS_A>;
impl UHSMS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, UHSMS_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(UHSMS_A::SDR12),
            1 => Val(UHSMS_A::SDR25),
            2 => Val(UHSMS_A::SDR50),
            3 => Val(UHSMS_A::SDR104),
            4 => Val(UHSMS_A::DDR50),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SDR12`"]
    #[inline(always)]
    pub fn is_sdr12(&self) -> bool {
        *self == UHSMS_A::SDR12
    }
    #[doc = "Checks if the value of the field is `SDR25`"]
    #[inline(always)]
    pub fn is_sdr25(&self) -> bool {
        *self == UHSMS_A::SDR25
    }
    #[doc = "Checks if the value of the field is `SDR50`"]
    #[inline(always)]
    pub fn is_sdr50(&self) -> bool {
        *self == UHSMS_A::SDR50
    }
    #[doc = "Checks if the value of the field is `SDR104`"]
    #[inline(always)]
    pub fn is_sdr104(&self) -> bool {
        *self == UHSMS_A::SDR104
    }
    #[doc = "Checks if the value of the field is `DDR50`"]
    #[inline(always)]
    pub fn is_ddr50(&self) -> bool {
        *self == UHSMS_A::DDR50
    }
}
#[doc = "Write proxy for field `UHSMS`"]
pub struct UHSMS_W<'a> {
    w: &'a mut W,
}
impl<'a> UHSMS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UHSMS_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "SDR12"]
    #[inline(always)]
    pub fn sdr12(self) -> &'a mut W {
        self.variant(UHSMS_A::SDR12)
    }
    #[doc = "SDR25"]
    #[inline(always)]
    pub fn sdr25(self) -> &'a mut W {
        self.variant(UHSMS_A::SDR25)
    }
    #[doc = "SDR50"]
    #[inline(always)]
    pub fn sdr50(self) -> &'a mut W {
        self.variant(UHSMS_A::SDR50)
    }
    #[doc = "SDR104"]
    #[inline(always)]
    pub fn sdr104(self) -> &'a mut W {
        self.variant(UHSMS_A::SDR104)
    }
    #[doc = "DDR50"]
    #[inline(always)]
    pub fn ddr50(self) -> &'a mut W {
        self.variant(UHSMS_A::DDR50)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u16) & 0x07);
        self.w
    }
}
#[doc = "1.8V Signaling Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VS18EN_A {
    #[doc = "0: 3.3V Signaling"]
    S33V = 0,
    #[doc = "1: 1.8V Signaling"]
    S18V = 1,
}
impl From<VS18EN_A> for bool {
    #[inline(always)]
    fn from(variant: VS18EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `VS18EN`"]
pub type VS18EN_R = crate::R<bool, VS18EN_A>;
impl VS18EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VS18EN_A {
        match self.bits {
            false => VS18EN_A::S33V,
            true => VS18EN_A::S18V,
        }
    }
    #[doc = "Checks if the value of the field is `S33V`"]
    #[inline(always)]
    pub fn is_s33v(&self) -> bool {
        *self == VS18EN_A::S33V
    }
    #[doc = "Checks if the value of the field is `S18V`"]
    #[inline(always)]
    pub fn is_s18v(&self) -> bool {
        *self == VS18EN_A::S18V
    }
}
#[doc = "Write proxy for field `VS18EN`"]
pub struct VS18EN_W<'a> {
    w: &'a mut W,
}
impl<'a> VS18EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VS18EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "3.3V Signaling"]
    #[inline(always)]
    pub fn s33v(self) -> &'a mut W {
        self.variant(VS18EN_A::S33V)
    }
    #[doc = "1.8V Signaling"]
    #[inline(always)]
    pub fn s18v(self) -> &'a mut W {
        self.variant(VS18EN_A::S18V)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u16) & 0x01) << 3);
        self.w
    }
}
#[doc = "Driver Strength Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DRVSEL_A {
    #[doc = "0: Driver Type B is Selected (Default)"]
    B = 0,
    #[doc = "1: Driver Type A is Selected"]
    A = 1,
    #[doc = "2: Driver Type C is Selected"]
    C = 2,
    #[doc = "3: Driver Type D is Selected"]
    D = 3,
}
impl From<DRVSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: DRVSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `DRVSEL`"]
pub type DRVSEL_R = crate::R<u8, DRVSEL_A>;
impl DRVSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DRVSEL_A {
        match self.bits {
            0 => DRVSEL_A::B,
            1 => DRVSEL_A::A,
            2 => DRVSEL_A::C,
            3 => DRVSEL_A::D,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `B`"]
    #[inline(always)]
    pub fn is_b(&self) -> bool {
        *self == DRVSEL_A::B
    }
    #[doc = "Checks if the value of the field is `A`"]
    #[inline(always)]
    pub fn is_a(&self) -> bool {
        *self == DRVSEL_A::A
    }
    #[doc = "Checks if the value of the field is `C`"]
    #[inline(always)]
    pub fn is_c(&self) -> bool {
        *self == DRVSEL_A::C
    }
    #[doc = "Checks if the value of the field is `D`"]
    #[inline(always)]
    pub fn is_d(&self) -> bool {
        *self == DRVSEL_A::D
    }
}
#[doc = "Write proxy for field `DRVSEL`"]
pub struct DRVSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> DRVSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DRVSEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Driver Type B is Selected (Default)"]
    #[inline(always)]
    pub fn b(self) -> &'a mut W {
        self.variant(DRVSEL_A::B)
    }
    #[doc = "Driver Type A is Selected"]
    #[inline(always)]
    pub fn a(self) -> &'a mut W {
        self.variant(DRVSEL_A::A)
    }
    #[doc = "Driver Type C is Selected"]
    #[inline(always)]
    pub fn c(self) -> &'a mut W {
        self.variant(DRVSEL_A::C)
    }
    #[doc = "Driver Type D is Selected"]
    #[inline(always)]
    pub fn d(self) -> &'a mut W {
        self.variant(DRVSEL_A::D)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u16) & 0x03) << 4);
        self.w
    }
}
#[doc = "Execute Tuning\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EXTUN_A {
    #[doc = "0: Not Tuned or Tuning Completed"]
    NO = 0,
    #[doc = "1: Execute Tuning"]
    REQUESTED = 1,
}
impl From<EXTUN_A> for bool {
    #[inline(always)]
    fn from(variant: EXTUN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EXTUN`"]
pub type EXTUN_R = crate::R<bool, EXTUN_A>;
impl EXTUN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EXTUN_A {
        match self.bits {
            false => EXTUN_A::NO,
            true => EXTUN_A::REQUESTED,
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == EXTUN_A::NO
    }
    #[doc = "Checks if the value of the field is `REQUESTED`"]
    #[inline(always)]
    pub fn is_requested(&self) -> bool {
        *self == EXTUN_A::REQUESTED
    }
}
#[doc = "Write proxy for field `EXTUN`"]
pub struct EXTUN_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTUN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EXTUN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Not Tuned or Tuning Completed"]
    #[inline(always)]
    pub fn no(self) -> &'a mut W {
        self.variant(EXTUN_A::NO)
    }
    #[doc = "Execute Tuning"]
    #[inline(always)]
    pub fn requested(self) -> &'a mut W {
        self.variant(EXTUN_A::REQUESTED)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u16) & 0x01) << 6);
        self.w
    }
}
#[doc = "Sampling Clock Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLCKSEL_A {
    #[doc = "0: Fixed clock is used to sample data"]
    FIXED = 0,
    #[doc = "1: Tuned clock is used to sample data"]
    TUNED = 1,
}
impl From<SLCKSEL_A> for bool {
    #[inline(always)]
    fn from(variant: SLCKSEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SLCKSEL`"]
pub type SLCKSEL_R = crate::R<bool, SLCKSEL_A>;
impl SLCKSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SLCKSEL_A {
        match self.bits {
            false => SLCKSEL_A::FIXED,
            true => SLCKSEL_A::TUNED,
        }
    }
    #[doc = "Checks if the value of the field is `FIXED`"]
    #[inline(always)]
    pub fn is_fixed(&self) -> bool {
        *self == SLCKSEL_A::FIXED
    }
    #[doc = "Checks if the value of the field is `TUNED`"]
    #[inline(always)]
    pub fn is_tuned(&self) -> bool {
        *self == SLCKSEL_A::TUNED
    }
}
#[doc = "Write proxy for field `SLCKSEL`"]
pub struct SLCKSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SLCKSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SLCKSEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Fixed clock is used to sample data"]
    #[inline(always)]
    pub fn fixed(self) -> &'a mut W {
        self.variant(SLCKSEL_A::FIXED)
    }
    #[doc = "Tuned clock is used to sample data"]
    #[inline(always)]
    pub fn tuned(self) -> &'a mut W {
        self.variant(SLCKSEL_A::TUNED)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u16) & 0x01) << 7);
        self.w
    }
}
#[doc = "Asynchronous Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ASINTEN_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<ASINTEN_A> for bool {
    #[inline(always)]
    fn from(variant: ASINTEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ASINTEN`"]
pub type ASINTEN_R = crate::R<bool, ASINTEN_A>;
impl ASINTEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ASINTEN_A {
        match self.bits {
            false => ASINTEN_A::DISABLED,
            true => ASINTEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ASINTEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ASINTEN_A::ENABLED
    }
}
#[doc = "Write proxy for field `ASINTEN`"]
pub struct ASINTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ASINTEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ASINTEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ASINTEN_A::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ASINTEN_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u16) & 0x01) << 14);
        self.w
    }
}
#[doc = "Preset Value Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PVALEN_A {
    #[doc = "0: SDCLK and Driver Strength are controlled by Host Controller"]
    HOST = 0,
    #[doc = "1: Automatic Selection by Preset Value is Enabled"]
    AUTO = 1,
}
impl From<PVALEN_A> for bool {
    #[inline(always)]
    fn from(variant: PVALEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PVALEN`"]
pub type PVALEN_R = crate::R<bool, PVALEN_A>;
impl PVALEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PVALEN_A {
        match self.bits {
            false => PVALEN_A::HOST,
            true => PVALEN_A::AUTO,
        }
    }
    #[doc = "Checks if the value of the field is `HOST`"]
    #[inline(always)]
    pub fn is_host(&self) -> bool {
        *self == PVALEN_A::HOST
    }
    #[doc = "Checks if the value of the field is `AUTO`"]
    #[inline(always)]
    pub fn is_auto(&self) -> bool {
        *self == PVALEN_A::AUTO
    }
}
#[doc = "Write proxy for field `PVALEN`"]
pub struct PVALEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PVALEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PVALEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "SDCLK and Driver Strength are controlled by Host Controller"]
    #[inline(always)]
    pub fn host(self) -> &'a mut W {
        self.variant(PVALEN_A::HOST)
    }
    #[doc = "Automatic Selection by Preset Value is Enabled"]
    #[inline(always)]
    pub fn auto(self) -> &'a mut W {
        self.variant(PVALEN_A::AUTO)
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u16) & 0x01) << 15);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - UHS Mode Select"]
    #[inline(always)]
    pub fn uhsms(&self) -> UHSMS_R {
        UHSMS_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bit 3 - 1.8V Signaling Enable"]
    #[inline(always)]
    pub fn vs18en(&self) -> VS18EN_R {
        VS18EN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 4:5 - Driver Strength Select"]
    #[inline(always)]
    pub fn drvsel(&self) -> DRVSEL_R {
        DRVSEL_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bit 6 - Execute Tuning"]
    #[inline(always)]
    pub fn extun(&self) -> EXTUN_R {
        EXTUN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Sampling Clock Select"]
    #[inline(always)]
    pub fn slcksel(&self) -> SLCKSEL_R {
        SLCKSEL_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Asynchronous Interrupt Enable"]
    #[inline(always)]
    pub fn asinten(&self) -> ASINTEN_R {
        ASINTEN_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Preset Value Enable"]
    #[inline(always)]
    pub fn pvalen(&self) -> PVALEN_R {
        PVALEN_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - UHS Mode Select"]
    #[inline(always)]
    pub fn uhsms(&mut self) -> UHSMS_W {
        UHSMS_W { w: self }
    }
    #[doc = "Bit 3 - 1.8V Signaling Enable"]
    #[inline(always)]
    pub fn vs18en(&mut self) -> VS18EN_W {
        VS18EN_W { w: self }
    }
    #[doc = "Bits 4:5 - Driver Strength Select"]
    #[inline(always)]
    pub fn drvsel(&mut self) -> DRVSEL_W {
        DRVSEL_W { w: self }
    }
    #[doc = "Bit 6 - Execute Tuning"]
    #[inline(always)]
    pub fn extun(&mut self) -> EXTUN_W {
        EXTUN_W { w: self }
    }
    #[doc = "Bit 7 - Sampling Clock Select"]
    #[inline(always)]
    pub fn slcksel(&mut self) -> SLCKSEL_W {
        SLCKSEL_W { w: self }
    }
    #[doc = "Bit 14 - Asynchronous Interrupt Enable"]
    #[inline(always)]
    pub fn asinten(&mut self) -> ASINTEN_W {
        ASINTEN_W { w: self }
    }
    #[doc = "Bit 15 - Preset Value Enable"]
    #[inline(always)]
    pub fn pvalen(&mut self) -> PVALEN_W {
        PVALEN_W { w: self }
    }
}
