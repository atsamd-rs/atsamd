#[doc = "Register `SUPC_WUMR` reader"]
pub struct R(crate::R<SUPC_WUMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SUPC_WUMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SUPC_WUMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SUPC_WUMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SUPC_WUMR` writer"]
pub struct W(crate::W<SUPC_WUMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SUPC_WUMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<SUPC_WUMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SUPC_WUMR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Supply Monitor Wake-up Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMEN_A {
    #[doc = "0: The supply monitor detection has no wake-up effect."]
    NOT_ENABLE = 0,
    #[doc = "1: The supply monitor detection forces the wake-up of the core power supply."]
    ENABLE = 1,
}
impl From<SMEN_A> for bool {
    #[inline(always)]
    fn from(variant: SMEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMEN` reader - Supply Monitor Wake-up Enable"]
pub struct SMEN_R(crate::FieldReader<bool, SMEN_A>);
impl SMEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SMEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SMEN_A {
        match self.bits {
            false => SMEN_A::NOT_ENABLE,
            true => SMEN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ENABLE`"]
    #[inline(always)]
    pub fn is_not_enable(&self) -> bool {
        **self == SMEN_A::NOT_ENABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == SMEN_A::ENABLE
    }
}
impl core::ops::Deref for SMEN_R {
    type Target = crate::FieldReader<bool, SMEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SMEN` writer - Supply Monitor Wake-up Enable"]
pub struct SMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SMEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The supply monitor detection has no wake-up effect."]
    #[inline(always)]
    pub fn not_enable(self) -> &'a mut W {
        self.variant(SMEN_A::NOT_ENABLE)
    }
    #[doc = "The supply monitor detection forces the wake-up of the core power supply."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(SMEN_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Real-time Timer Wake-up Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTTEN_A {
    #[doc = "0: The RTT alarm signal has no wake-up effect."]
    NOT_ENABLE = 0,
    #[doc = "1: The RTT alarm signal forces the wake-up of the core power supply."]
    ENABLE = 1,
}
impl From<RTTEN_A> for bool {
    #[inline(always)]
    fn from(variant: RTTEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTTEN` reader - Real-time Timer Wake-up Enable"]
pub struct RTTEN_R(crate::FieldReader<bool, RTTEN_A>);
impl RTTEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RTTEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTTEN_A {
        match self.bits {
            false => RTTEN_A::NOT_ENABLE,
            true => RTTEN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ENABLE`"]
    #[inline(always)]
    pub fn is_not_enable(&self) -> bool {
        **self == RTTEN_A::NOT_ENABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == RTTEN_A::ENABLE
    }
}
impl core::ops::Deref for RTTEN_R {
    type Target = crate::FieldReader<bool, RTTEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTTEN` writer - Real-time Timer Wake-up Enable"]
pub struct RTTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RTTEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RTTEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The RTT alarm signal has no wake-up effect."]
    #[inline(always)]
    pub fn not_enable(self) -> &'a mut W {
        self.variant(RTTEN_A::NOT_ENABLE)
    }
    #[doc = "The RTT alarm signal forces the wake-up of the core power supply."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(RTTEN_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Real-time Clock Wake-up Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTCEN_A {
    #[doc = "0: The RTC alarm signal has no wake-up effect."]
    NOT_ENABLE = 0,
    #[doc = "1: The RTC alarm signal forces the wake-up of the core power supply."]
    ENABLE = 1,
}
impl From<RTCEN_A> for bool {
    #[inline(always)]
    fn from(variant: RTCEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTCEN` reader - Real-time Clock Wake-up Enable"]
pub struct RTCEN_R(crate::FieldReader<bool, RTCEN_A>);
impl RTCEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RTCEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTCEN_A {
        match self.bits {
            false => RTCEN_A::NOT_ENABLE,
            true => RTCEN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ENABLE`"]
    #[inline(always)]
    pub fn is_not_enable(&self) -> bool {
        **self == RTCEN_A::NOT_ENABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == RTCEN_A::ENABLE
    }
}
impl core::ops::Deref for RTCEN_R {
    type Target = crate::FieldReader<bool, RTCEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTCEN` writer - Real-time Clock Wake-up Enable"]
pub struct RTCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RTCEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The RTC alarm signal has no wake-up effect."]
    #[inline(always)]
    pub fn not_enable(self) -> &'a mut W {
        self.variant(RTCEN_A::NOT_ENABLE)
    }
    #[doc = "The RTC alarm signal forces the wake-up of the core power supply."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(RTCEN_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Low-power Debouncer Enable WKUP0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPDBCEN0_A {
    #[doc = "0: The WKUP0 input pin is not connected to the low-power debouncer."]
    NOT_ENABLE = 0,
    #[doc = "1: The WKUP0 input pin is connected to the low-power debouncer and forces a system wake-up."]
    ENABLE = 1,
}
impl From<LPDBCEN0_A> for bool {
    #[inline(always)]
    fn from(variant: LPDBCEN0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LPDBCEN0` reader - Low-power Debouncer Enable WKUP0"]
pub struct LPDBCEN0_R(crate::FieldReader<bool, LPDBCEN0_A>);
impl LPDBCEN0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LPDBCEN0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPDBCEN0_A {
        match self.bits {
            false => LPDBCEN0_A::NOT_ENABLE,
            true => LPDBCEN0_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ENABLE`"]
    #[inline(always)]
    pub fn is_not_enable(&self) -> bool {
        **self == LPDBCEN0_A::NOT_ENABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == LPDBCEN0_A::ENABLE
    }
}
impl core::ops::Deref for LPDBCEN0_R {
    type Target = crate::FieldReader<bool, LPDBCEN0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LPDBCEN0` writer - Low-power Debouncer Enable WKUP0"]
pub struct LPDBCEN0_W<'a> {
    w: &'a mut W,
}
impl<'a> LPDBCEN0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPDBCEN0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The WKUP0 input pin is not connected to the low-power debouncer."]
    #[inline(always)]
    pub fn not_enable(self) -> &'a mut W {
        self.variant(LPDBCEN0_A::NOT_ENABLE)
    }
    #[doc = "The WKUP0 input pin is connected to the low-power debouncer and forces a system wake-up."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(LPDBCEN0_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Low-power Debouncer Enable WKUP1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPDBCEN1_A {
    #[doc = "0: The WKUP1 input pin is not connected to the low-power debouncer."]
    NOT_ENABLE = 0,
    #[doc = "1: The WKUP1 input pin is connected to the low-power debouncer and forces a system wake-up."]
    ENABLE = 1,
}
impl From<LPDBCEN1_A> for bool {
    #[inline(always)]
    fn from(variant: LPDBCEN1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LPDBCEN1` reader - Low-power Debouncer Enable WKUP1"]
pub struct LPDBCEN1_R(crate::FieldReader<bool, LPDBCEN1_A>);
impl LPDBCEN1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LPDBCEN1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPDBCEN1_A {
        match self.bits {
            false => LPDBCEN1_A::NOT_ENABLE,
            true => LPDBCEN1_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ENABLE`"]
    #[inline(always)]
    pub fn is_not_enable(&self) -> bool {
        **self == LPDBCEN1_A::NOT_ENABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == LPDBCEN1_A::ENABLE
    }
}
impl core::ops::Deref for LPDBCEN1_R {
    type Target = crate::FieldReader<bool, LPDBCEN1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LPDBCEN1` writer - Low-power Debouncer Enable WKUP1"]
pub struct LPDBCEN1_W<'a> {
    w: &'a mut W,
}
impl<'a> LPDBCEN1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPDBCEN1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The WKUP1 input pin is not connected to the low-power debouncer."]
    #[inline(always)]
    pub fn not_enable(self) -> &'a mut W {
        self.variant(LPDBCEN1_A::NOT_ENABLE)
    }
    #[doc = "The WKUP1 input pin is connected to the low-power debouncer and forces a system wake-up."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(LPDBCEN1_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Low-power Debouncer Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPDBCCLR_A {
    #[doc = "0: A low-power debounce event does not create an immediate clear on the first half of GPBR registers."]
    NOT_ENABLE = 0,
    #[doc = "1: A low-power debounce event on WKUP0 or WKUP1 generates an immediate clear on the first half of GPBR registers."]
    ENABLE = 1,
}
impl From<LPDBCCLR_A> for bool {
    #[inline(always)]
    fn from(variant: LPDBCCLR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LPDBCCLR` reader - Low-power Debouncer Clear"]
pub struct LPDBCCLR_R(crate::FieldReader<bool, LPDBCCLR_A>);
impl LPDBCCLR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LPDBCCLR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPDBCCLR_A {
        match self.bits {
            false => LPDBCCLR_A::NOT_ENABLE,
            true => LPDBCCLR_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ENABLE`"]
    #[inline(always)]
    pub fn is_not_enable(&self) -> bool {
        **self == LPDBCCLR_A::NOT_ENABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == LPDBCCLR_A::ENABLE
    }
}
impl core::ops::Deref for LPDBCCLR_R {
    type Target = crate::FieldReader<bool, LPDBCCLR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LPDBCCLR` writer - Low-power Debouncer Clear"]
pub struct LPDBCCLR_W<'a> {
    w: &'a mut W,
}
impl<'a> LPDBCCLR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPDBCCLR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "A low-power debounce event does not create an immediate clear on the first half of GPBR registers."]
    #[inline(always)]
    pub fn not_enable(self) -> &'a mut W {
        self.variant(LPDBCCLR_A::NOT_ENABLE)
    }
    #[doc = "A low-power debounce event on WKUP0 or WKUP1 generates an immediate clear on the first half of GPBR registers."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(LPDBCCLR_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Wake-up Inputs Debouncer Period\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum WKUPDBC_A {
    #[doc = "0: Immediate, no debouncing, detected active at least on one Slow Clock edge."]
    IMMEDIATE = 0,
    #[doc = "1: WKUPx shall be in its active state for at least 3 SLCK periods"]
    _3_SLCK = 1,
    #[doc = "2: WKUPx shall be in its active state for at least 32 SLCK periods"]
    _32_SLCK = 2,
    #[doc = "3: WKUPx shall be in its active state for at least 512 SLCK periods"]
    _512_SLCK = 3,
    #[doc = "4: WKUPx shall be in its active state for at least 4,096 SLCK periods"]
    _4096_SLCK = 4,
    #[doc = "5: WKUPx shall be in its active state for at least 32,768 SLCK periods"]
    _32768_SLCK = 5,
}
impl From<WKUPDBC_A> for u8 {
    #[inline(always)]
    fn from(variant: WKUPDBC_A) -> Self {
        variant as _
    }
}
#[doc = "Field `WKUPDBC` reader - Wake-up Inputs Debouncer Period"]
pub struct WKUPDBC_R(crate::FieldReader<u8, WKUPDBC_A>);
impl WKUPDBC_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        WKUPDBC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<WKUPDBC_A> {
        match self.bits {
            0 => Some(WKUPDBC_A::IMMEDIATE),
            1 => Some(WKUPDBC_A::_3_SLCK),
            2 => Some(WKUPDBC_A::_32_SLCK),
            3 => Some(WKUPDBC_A::_512_SLCK),
            4 => Some(WKUPDBC_A::_4096_SLCK),
            5 => Some(WKUPDBC_A::_32768_SLCK),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `IMMEDIATE`"]
    #[inline(always)]
    pub fn is_immediate(&self) -> bool {
        **self == WKUPDBC_A::IMMEDIATE
    }
    #[doc = "Checks if the value of the field is `_3_SLCK`"]
    #[inline(always)]
    pub fn is_3_slck(&self) -> bool {
        **self == WKUPDBC_A::_3_SLCK
    }
    #[doc = "Checks if the value of the field is `_32_SLCK`"]
    #[inline(always)]
    pub fn is_32_slck(&self) -> bool {
        **self == WKUPDBC_A::_32_SLCK
    }
    #[doc = "Checks if the value of the field is `_512_SLCK`"]
    #[inline(always)]
    pub fn is_512_slck(&self) -> bool {
        **self == WKUPDBC_A::_512_SLCK
    }
    #[doc = "Checks if the value of the field is `_4096_SLCK`"]
    #[inline(always)]
    pub fn is_4096_slck(&self) -> bool {
        **self == WKUPDBC_A::_4096_SLCK
    }
    #[doc = "Checks if the value of the field is `_32768_SLCK`"]
    #[inline(always)]
    pub fn is_32768_slck(&self) -> bool {
        **self == WKUPDBC_A::_32768_SLCK
    }
}
impl core::ops::Deref for WKUPDBC_R {
    type Target = crate::FieldReader<u8, WKUPDBC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WKUPDBC` writer - Wake-up Inputs Debouncer Period"]
pub struct WKUPDBC_W<'a> {
    w: &'a mut W,
}
impl<'a> WKUPDBC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WKUPDBC_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Immediate, no debouncing, detected active at least on one Slow Clock edge."]
    #[inline(always)]
    pub fn immediate(self) -> &'a mut W {
        self.variant(WKUPDBC_A::IMMEDIATE)
    }
    #[doc = "WKUPx shall be in its active state for at least 3 SLCK periods"]
    #[inline(always)]
    pub fn _3_slck(self) -> &'a mut W {
        self.variant(WKUPDBC_A::_3_SLCK)
    }
    #[doc = "WKUPx shall be in its active state for at least 32 SLCK periods"]
    #[inline(always)]
    pub fn _32_slck(self) -> &'a mut W {
        self.variant(WKUPDBC_A::_32_SLCK)
    }
    #[doc = "WKUPx shall be in its active state for at least 512 SLCK periods"]
    #[inline(always)]
    pub fn _512_slck(self) -> &'a mut W {
        self.variant(WKUPDBC_A::_512_SLCK)
    }
    #[doc = "WKUPx shall be in its active state for at least 4,096 SLCK periods"]
    #[inline(always)]
    pub fn _4096_slck(self) -> &'a mut W {
        self.variant(WKUPDBC_A::_4096_SLCK)
    }
    #[doc = "WKUPx shall be in its active state for at least 32,768 SLCK periods"]
    #[inline(always)]
    pub fn _32768_slck(self) -> &'a mut W {
        self.variant(WKUPDBC_A::_32768_SLCK)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 12)) | ((value as u32 & 0x07) << 12);
        self.w
    }
}
#[doc = "Low-power Debouncer Period\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LPDBC_A {
    #[doc = "0: Disable the low-power debouncers."]
    DISABLE = 0,
    #[doc = "1: WKUP0/1 in active state for at least 2 RTCOUTx clock periods"]
    _2_RTCOUT = 1,
    #[doc = "2: WKUP0/1 in active state for at least 3 RTCOUTx clock periods"]
    _3_RTCOUT = 2,
    #[doc = "3: WKUP0/1 in active state for at least 4 RTCOUTx clock periods"]
    _4_RTCOUT = 3,
    #[doc = "4: WKUP0/1 in active state for at least 5 RTCOUTx clock periods"]
    _5_RTCOUT = 4,
    #[doc = "5: WKUP0/1 in active state for at least 6 RTCOUTx clock periods"]
    _6_RTCOUT = 5,
    #[doc = "6: WKUP0/1 in active state for at least 7 RTCOUTx clock periods"]
    _7_RTCOUT = 6,
    #[doc = "7: WKUP0/1 in active state for at least 8 RTCOUTx clock periods"]
    _8_RTCOUT = 7,
}
impl From<LPDBC_A> for u8 {
    #[inline(always)]
    fn from(variant: LPDBC_A) -> Self {
        variant as _
    }
}
#[doc = "Field `LPDBC` reader - Low-power Debouncer Period"]
pub struct LPDBC_R(crate::FieldReader<u8, LPDBC_A>);
impl LPDBC_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        LPDBC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPDBC_A {
        match self.bits {
            0 => LPDBC_A::DISABLE,
            1 => LPDBC_A::_2_RTCOUT,
            2 => LPDBC_A::_3_RTCOUT,
            3 => LPDBC_A::_4_RTCOUT,
            4 => LPDBC_A::_5_RTCOUT,
            5 => LPDBC_A::_6_RTCOUT,
            6 => LPDBC_A::_7_RTCOUT,
            7 => LPDBC_A::_8_RTCOUT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == LPDBC_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `_2_RTCOUT`"]
    #[inline(always)]
    pub fn is_2_rtcout(&self) -> bool {
        **self == LPDBC_A::_2_RTCOUT
    }
    #[doc = "Checks if the value of the field is `_3_RTCOUT`"]
    #[inline(always)]
    pub fn is_3_rtcout(&self) -> bool {
        **self == LPDBC_A::_3_RTCOUT
    }
    #[doc = "Checks if the value of the field is `_4_RTCOUT`"]
    #[inline(always)]
    pub fn is_4_rtcout(&self) -> bool {
        **self == LPDBC_A::_4_RTCOUT
    }
    #[doc = "Checks if the value of the field is `_5_RTCOUT`"]
    #[inline(always)]
    pub fn is_5_rtcout(&self) -> bool {
        **self == LPDBC_A::_5_RTCOUT
    }
    #[doc = "Checks if the value of the field is `_6_RTCOUT`"]
    #[inline(always)]
    pub fn is_6_rtcout(&self) -> bool {
        **self == LPDBC_A::_6_RTCOUT
    }
    #[doc = "Checks if the value of the field is `_7_RTCOUT`"]
    #[inline(always)]
    pub fn is_7_rtcout(&self) -> bool {
        **self == LPDBC_A::_7_RTCOUT
    }
    #[doc = "Checks if the value of the field is `_8_RTCOUT`"]
    #[inline(always)]
    pub fn is_8_rtcout(&self) -> bool {
        **self == LPDBC_A::_8_RTCOUT
    }
}
impl core::ops::Deref for LPDBC_R {
    type Target = crate::FieldReader<u8, LPDBC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LPDBC` writer - Low-power Debouncer Period"]
pub struct LPDBC_W<'a> {
    w: &'a mut W,
}
impl<'a> LPDBC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPDBC_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Disable the low-power debouncers."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(LPDBC_A::DISABLE)
    }
    #[doc = "WKUP0/1 in active state for at least 2 RTCOUTx clock periods"]
    #[inline(always)]
    pub fn _2_rtcout(self) -> &'a mut W {
        self.variant(LPDBC_A::_2_RTCOUT)
    }
    #[doc = "WKUP0/1 in active state for at least 3 RTCOUTx clock periods"]
    #[inline(always)]
    pub fn _3_rtcout(self) -> &'a mut W {
        self.variant(LPDBC_A::_3_RTCOUT)
    }
    #[doc = "WKUP0/1 in active state for at least 4 RTCOUTx clock periods"]
    #[inline(always)]
    pub fn _4_rtcout(self) -> &'a mut W {
        self.variant(LPDBC_A::_4_RTCOUT)
    }
    #[doc = "WKUP0/1 in active state for at least 5 RTCOUTx clock periods"]
    #[inline(always)]
    pub fn _5_rtcout(self) -> &'a mut W {
        self.variant(LPDBC_A::_5_RTCOUT)
    }
    #[doc = "WKUP0/1 in active state for at least 6 RTCOUTx clock periods"]
    #[inline(always)]
    pub fn _6_rtcout(self) -> &'a mut W {
        self.variant(LPDBC_A::_6_RTCOUT)
    }
    #[doc = "WKUP0/1 in active state for at least 7 RTCOUTx clock periods"]
    #[inline(always)]
    pub fn _7_rtcout(self) -> &'a mut W {
        self.variant(LPDBC_A::_7_RTCOUT)
    }
    #[doc = "WKUP0/1 in active state for at least 8 RTCOUTx clock periods"]
    #[inline(always)]
    pub fn _8_rtcout(self) -> &'a mut W {
        self.variant(LPDBC_A::_8_RTCOUT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | ((value as u32 & 0x07) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bit 1 - Supply Monitor Wake-up Enable"]
    #[inline(always)]
    pub fn smen(&self) -> SMEN_R {
        SMEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Real-time Timer Wake-up Enable"]
    #[inline(always)]
    pub fn rtten(&self) -> RTTEN_R {
        RTTEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Real-time Clock Wake-up Enable"]
    #[inline(always)]
    pub fn rtcen(&self) -> RTCEN_R {
        RTCEN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Low-power Debouncer Enable WKUP0"]
    #[inline(always)]
    pub fn lpdbcen0(&self) -> LPDBCEN0_R {
        LPDBCEN0_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Low-power Debouncer Enable WKUP1"]
    #[inline(always)]
    pub fn lpdbcen1(&self) -> LPDBCEN1_R {
        LPDBCEN1_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Low-power Debouncer Clear"]
    #[inline(always)]
    pub fn lpdbcclr(&self) -> LPDBCCLR_R {
        LPDBCCLR_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 12:14 - Wake-up Inputs Debouncer Period"]
    #[inline(always)]
    pub fn wkupdbc(&self) -> WKUPDBC_R {
        WKUPDBC_R::new(((self.bits >> 12) & 0x07) as u8)
    }
    #[doc = "Bits 16:18 - Low-power Debouncer Period"]
    #[inline(always)]
    pub fn lpdbc(&self) -> LPDBC_R {
        LPDBC_R::new(((self.bits >> 16) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bit 1 - Supply Monitor Wake-up Enable"]
    #[inline(always)]
    pub fn smen(&mut self) -> SMEN_W {
        SMEN_W { w: self }
    }
    #[doc = "Bit 2 - Real-time Timer Wake-up Enable"]
    #[inline(always)]
    pub fn rtten(&mut self) -> RTTEN_W {
        RTTEN_W { w: self }
    }
    #[doc = "Bit 3 - Real-time Clock Wake-up Enable"]
    #[inline(always)]
    pub fn rtcen(&mut self) -> RTCEN_W {
        RTCEN_W { w: self }
    }
    #[doc = "Bit 5 - Low-power Debouncer Enable WKUP0"]
    #[inline(always)]
    pub fn lpdbcen0(&mut self) -> LPDBCEN0_W {
        LPDBCEN0_W { w: self }
    }
    #[doc = "Bit 6 - Low-power Debouncer Enable WKUP1"]
    #[inline(always)]
    pub fn lpdbcen1(&mut self) -> LPDBCEN1_W {
        LPDBCEN1_W { w: self }
    }
    #[doc = "Bit 7 - Low-power Debouncer Clear"]
    #[inline(always)]
    pub fn lpdbcclr(&mut self) -> LPDBCCLR_W {
        LPDBCCLR_W { w: self }
    }
    #[doc = "Bits 12:14 - Wake-up Inputs Debouncer Period"]
    #[inline(always)]
    pub fn wkupdbc(&mut self) -> WKUPDBC_W {
        WKUPDBC_W { w: self }
    }
    #[doc = "Bits 16:18 - Low-power Debouncer Period"]
    #[inline(always)]
    pub fn lpdbc(&mut self) -> LPDBC_W {
        LPDBC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Supply Controller Wake-up Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [supc_wumr](index.html) module"]
pub struct SUPC_WUMR_SPEC;
impl crate::RegisterSpec for SUPC_WUMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [supc_wumr::R](R) reader structure"]
impl crate::Readable for SUPC_WUMR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [supc_wumr::W](W) writer structure"]
impl crate::Writable for SUPC_WUMR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SUPC_WUMR to value 0"]
impl crate::Resettable for SUPC_WUMR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
