#[doc = "Register `EVCTRL` reader"]
pub struct R(crate::R<EVCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EVCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EVCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EVCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EVCTRL` writer"]
pub struct W(crate::W<EVCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EVCTRL_SPEC>;
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
impl From<crate::W<EVCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EVCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Timer/counter Input Event0 Action\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EVACT0_A {
    #[doc = "0: Event action disabled"]
    OFF = 0,
    #[doc = "1: Start, restart or re-trigger counter on event"]
    RETRIGGER = 1,
    #[doc = "2: Count on event"]
    COUNTEV = 2,
    #[doc = "3: Start counter on event"]
    START = 3,
    #[doc = "4: Increment counter on event"]
    INC = 4,
    #[doc = "5: Count on active state of asynchronous event"]
    COUNT = 5,
    #[doc = "7: Non-recoverable fault"]
    FAULT = 7,
}
impl From<EVACT0_A> for u8 {
    #[inline(always)]
    fn from(variant: EVACT0_A) -> Self {
        variant as _
    }
}
#[doc = "Field `EVACT0` reader - Timer/counter Input Event0 Action"]
pub struct EVACT0_R(crate::FieldReader<u8, EVACT0_A>);
impl EVACT0_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        EVACT0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<EVACT0_A> {
        match self.bits {
            0 => Some(EVACT0_A::OFF),
            1 => Some(EVACT0_A::RETRIGGER),
            2 => Some(EVACT0_A::COUNTEV),
            3 => Some(EVACT0_A::START),
            4 => Some(EVACT0_A::INC),
            5 => Some(EVACT0_A::COUNT),
            7 => Some(EVACT0_A::FAULT),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        **self == EVACT0_A::OFF
    }
    #[doc = "Checks if the value of the field is `RETRIGGER`"]
    #[inline(always)]
    pub fn is_retrigger(&self) -> bool {
        **self == EVACT0_A::RETRIGGER
    }
    #[doc = "Checks if the value of the field is `COUNTEV`"]
    #[inline(always)]
    pub fn is_countev(&self) -> bool {
        **self == EVACT0_A::COUNTEV
    }
    #[doc = "Checks if the value of the field is `START`"]
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        **self == EVACT0_A::START
    }
    #[doc = "Checks if the value of the field is `INC`"]
    #[inline(always)]
    pub fn is_inc(&self) -> bool {
        **self == EVACT0_A::INC
    }
    #[doc = "Checks if the value of the field is `COUNT`"]
    #[inline(always)]
    pub fn is_count(&self) -> bool {
        **self == EVACT0_A::COUNT
    }
    #[doc = "Checks if the value of the field is `FAULT`"]
    #[inline(always)]
    pub fn is_fault(&self) -> bool {
        **self == EVACT0_A::FAULT
    }
}
impl core::ops::Deref for EVACT0_R {
    type Target = crate::FieldReader<u8, EVACT0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EVACT0` writer - Timer/counter Input Event0 Action"]
pub struct EVACT0_W<'a> {
    w: &'a mut W,
}
impl<'a> EVACT0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EVACT0_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Event action disabled"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(EVACT0_A::OFF)
    }
    #[doc = "Start, restart or re-trigger counter on event"]
    #[inline(always)]
    pub fn retrigger(self) -> &'a mut W {
        self.variant(EVACT0_A::RETRIGGER)
    }
    #[doc = "Count on event"]
    #[inline(always)]
    pub fn countev(self) -> &'a mut W {
        self.variant(EVACT0_A::COUNTEV)
    }
    #[doc = "Start counter on event"]
    #[inline(always)]
    pub fn start(self) -> &'a mut W {
        self.variant(EVACT0_A::START)
    }
    #[doc = "Increment counter on event"]
    #[inline(always)]
    pub fn inc(self) -> &'a mut W {
        self.variant(EVACT0_A::INC)
    }
    #[doc = "Count on active state of asynchronous event"]
    #[inline(always)]
    pub fn count(self) -> &'a mut W {
        self.variant(EVACT0_A::COUNT)
    }
    #[doc = "Non-recoverable fault"]
    #[inline(always)]
    pub fn fault(self) -> &'a mut W {
        self.variant(EVACT0_A::FAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
#[doc = "Timer/counter Input Event1 Action\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EVACT1_A {
    #[doc = "0: Event action disabled"]
    OFF = 0,
    #[doc = "1: Re-trigger counter on event"]
    RETRIGGER = 1,
    #[doc = "2: Direction control"]
    DIR = 2,
    #[doc = "3: Stop counter on event"]
    STOP = 3,
    #[doc = "4: Decrement counter on event"]
    DEC = 4,
    #[doc = "5: Period capture value in CC0 register, pulse width capture value in CC1 register"]
    PPW = 5,
    #[doc = "6: Period capture value in CC1 register, pulse width capture value in CC0 register"]
    PWP = 6,
    #[doc = "7: Non-recoverable fault"]
    FAULT = 7,
}
impl From<EVACT1_A> for u8 {
    #[inline(always)]
    fn from(variant: EVACT1_A) -> Self {
        variant as _
    }
}
#[doc = "Field `EVACT1` reader - Timer/counter Input Event1 Action"]
pub struct EVACT1_R(crate::FieldReader<u8, EVACT1_A>);
impl EVACT1_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        EVACT1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EVACT1_A {
        match self.bits {
            0 => EVACT1_A::OFF,
            1 => EVACT1_A::RETRIGGER,
            2 => EVACT1_A::DIR,
            3 => EVACT1_A::STOP,
            4 => EVACT1_A::DEC,
            5 => EVACT1_A::PPW,
            6 => EVACT1_A::PWP,
            7 => EVACT1_A::FAULT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        **self == EVACT1_A::OFF
    }
    #[doc = "Checks if the value of the field is `RETRIGGER`"]
    #[inline(always)]
    pub fn is_retrigger(&self) -> bool {
        **self == EVACT1_A::RETRIGGER
    }
    #[doc = "Checks if the value of the field is `DIR`"]
    #[inline(always)]
    pub fn is_dir(&self) -> bool {
        **self == EVACT1_A::DIR
    }
    #[doc = "Checks if the value of the field is `STOP`"]
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        **self == EVACT1_A::STOP
    }
    #[doc = "Checks if the value of the field is `DEC`"]
    #[inline(always)]
    pub fn is_dec(&self) -> bool {
        **self == EVACT1_A::DEC
    }
    #[doc = "Checks if the value of the field is `PPW`"]
    #[inline(always)]
    pub fn is_ppw(&self) -> bool {
        **self == EVACT1_A::PPW
    }
    #[doc = "Checks if the value of the field is `PWP`"]
    #[inline(always)]
    pub fn is_pwp(&self) -> bool {
        **self == EVACT1_A::PWP
    }
    #[doc = "Checks if the value of the field is `FAULT`"]
    #[inline(always)]
    pub fn is_fault(&self) -> bool {
        **self == EVACT1_A::FAULT
    }
}
impl core::ops::Deref for EVACT1_R {
    type Target = crate::FieldReader<u8, EVACT1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EVACT1` writer - Timer/counter Input Event1 Action"]
pub struct EVACT1_W<'a> {
    w: &'a mut W,
}
impl<'a> EVACT1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EVACT1_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Event action disabled"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(EVACT1_A::OFF)
    }
    #[doc = "Re-trigger counter on event"]
    #[inline(always)]
    pub fn retrigger(self) -> &'a mut W {
        self.variant(EVACT1_A::RETRIGGER)
    }
    #[doc = "Direction control"]
    #[inline(always)]
    pub fn dir(self) -> &'a mut W {
        self.variant(EVACT1_A::DIR)
    }
    #[doc = "Stop counter on event"]
    #[inline(always)]
    pub fn stop(self) -> &'a mut W {
        self.variant(EVACT1_A::STOP)
    }
    #[doc = "Decrement counter on event"]
    #[inline(always)]
    pub fn dec(self) -> &'a mut W {
        self.variant(EVACT1_A::DEC)
    }
    #[doc = "Period capture value in CC0 register, pulse width capture value in CC1 register"]
    #[inline(always)]
    pub fn ppw(self) -> &'a mut W {
        self.variant(EVACT1_A::PPW)
    }
    #[doc = "Period capture value in CC1 register, pulse width capture value in CC0 register"]
    #[inline(always)]
    pub fn pwp(self) -> &'a mut W {
        self.variant(EVACT1_A::PWP)
    }
    #[doc = "Non-recoverable fault"]
    #[inline(always)]
    pub fn fault(self) -> &'a mut W {
        self.variant(EVACT1_A::FAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 3)) | ((value as u32 & 0x07) << 3);
        self.w
    }
}
#[doc = "Timer/counter Output Event Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CNTSEL_A {
    #[doc = "0: An interrupt/event is generated when a new counter cycle starts"]
    START = 0,
    #[doc = "1: An interrupt/event is generated when a counter cycle ends"]
    END = 1,
    #[doc = "2: An interrupt/event is generated when a counter cycle ends, except for the first and last cycles"]
    BETWEEN = 2,
    #[doc = "3: An interrupt/event is generated when a new counter cycle starts or a counter cycle ends"]
    BOUNDARY = 3,
}
impl From<CNTSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: CNTSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CNTSEL` reader - Timer/counter Output Event Mode"]
pub struct CNTSEL_R(crate::FieldReader<u8, CNTSEL_A>);
impl CNTSEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CNTSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CNTSEL_A {
        match self.bits {
            0 => CNTSEL_A::START,
            1 => CNTSEL_A::END,
            2 => CNTSEL_A::BETWEEN,
            3 => CNTSEL_A::BOUNDARY,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `START`"]
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        **self == CNTSEL_A::START
    }
    #[doc = "Checks if the value of the field is `END`"]
    #[inline(always)]
    pub fn is_end(&self) -> bool {
        **self == CNTSEL_A::END
    }
    #[doc = "Checks if the value of the field is `BETWEEN`"]
    #[inline(always)]
    pub fn is_between(&self) -> bool {
        **self == CNTSEL_A::BETWEEN
    }
    #[doc = "Checks if the value of the field is `BOUNDARY`"]
    #[inline(always)]
    pub fn is_boundary(&self) -> bool {
        **self == CNTSEL_A::BOUNDARY
    }
}
impl core::ops::Deref for CNTSEL_R {
    type Target = crate::FieldReader<u8, CNTSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CNTSEL` writer - Timer/counter Output Event Mode"]
pub struct CNTSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CNTSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CNTSEL_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "An interrupt/event is generated when a new counter cycle starts"]
    #[inline(always)]
    pub fn start(self) -> &'a mut W {
        self.variant(CNTSEL_A::START)
    }
    #[doc = "An interrupt/event is generated when a counter cycle ends"]
    #[inline(always)]
    pub fn end(self) -> &'a mut W {
        self.variant(CNTSEL_A::END)
    }
    #[doc = "An interrupt/event is generated when a counter cycle ends, except for the first and last cycles"]
    #[inline(always)]
    pub fn between(self) -> &'a mut W {
        self.variant(CNTSEL_A::BETWEEN)
    }
    #[doc = "An interrupt/event is generated when a new counter cycle starts or a counter cycle ends"]
    #[inline(always)]
    pub fn boundary(self) -> &'a mut W {
        self.variant(CNTSEL_A::BOUNDARY)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | ((value as u32 & 0x03) << 6);
        self.w
    }
}
#[doc = "Field `OVFEO` reader - Overflow/Underflow Output Event Enable"]
pub struct OVFEO_R(crate::FieldReader<bool, bool>);
impl OVFEO_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OVFEO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVFEO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVFEO` writer - Overflow/Underflow Output Event Enable"]
pub struct OVFEO_W<'a> {
    w: &'a mut W,
}
impl<'a> OVFEO_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `TRGEO` reader - Retrigger Output Event Enable"]
pub struct TRGEO_R(crate::FieldReader<bool, bool>);
impl TRGEO_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TRGEO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRGEO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRGEO` writer - Retrigger Output Event Enable"]
pub struct TRGEO_W<'a> {
    w: &'a mut W,
}
impl<'a> TRGEO_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Field `CNTEO` reader - Timer/counter Output Event Enable"]
pub struct CNTEO_R(crate::FieldReader<bool, bool>);
impl CNTEO_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CNTEO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CNTEO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CNTEO` writer - Timer/counter Output Event Enable"]
pub struct CNTEO_W<'a> {
    w: &'a mut W,
}
impl<'a> CNTEO_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "Field `TCINV0` reader - Inverted Event 0 Input Enable"]
pub struct TCINV0_R(crate::FieldReader<bool, bool>);
impl TCINV0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TCINV0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TCINV0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TCINV0` writer - Inverted Event 0 Input Enable"]
pub struct TCINV0_W<'a> {
    w: &'a mut W,
}
impl<'a> TCINV0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "Field `TCINV1` reader - Inverted Event 1 Input Enable"]
pub struct TCINV1_R(crate::FieldReader<bool, bool>);
impl TCINV1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TCINV1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TCINV1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TCINV1` writer - Inverted Event 1 Input Enable"]
pub struct TCINV1_W<'a> {
    w: &'a mut W,
}
impl<'a> TCINV1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "Field `TCEI0` reader - Timer/counter Event 0 Input Enable"]
pub struct TCEI0_R(crate::FieldReader<bool, bool>);
impl TCEI0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TCEI0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TCEI0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TCEI0` writer - Timer/counter Event 0 Input Enable"]
pub struct TCEI0_W<'a> {
    w: &'a mut W,
}
impl<'a> TCEI0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
#[doc = "Field `TCEI1` reader - Timer/counter Event 1 Input Enable"]
pub struct TCEI1_R(crate::FieldReader<bool, bool>);
impl TCEI1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TCEI1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TCEI1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TCEI1` writer - Timer/counter Event 1 Input Enable"]
pub struct TCEI1_W<'a> {
    w: &'a mut W,
}
impl<'a> TCEI1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
#[doc = "Field `MCEI0` reader - Match or Capture Channel 0 Event Input Enable"]
pub struct MCEI0_R(crate::FieldReader<bool, bool>);
impl MCEI0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MCEI0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MCEI0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MCEI0` writer - Match or Capture Channel 0 Event Input Enable"]
pub struct MCEI0_W<'a> {
    w: &'a mut W,
}
impl<'a> MCEI0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Field `MCEI1` reader - Match or Capture Channel 1 Event Input Enable"]
pub struct MCEI1_R(crate::FieldReader<bool, bool>);
impl MCEI1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MCEI1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MCEI1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MCEI1` writer - Match or Capture Channel 1 Event Input Enable"]
pub struct MCEI1_W<'a> {
    w: &'a mut W,
}
impl<'a> MCEI1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "Field `MCEI2` reader - Match or Capture Channel 2 Event Input Enable"]
pub struct MCEI2_R(crate::FieldReader<bool, bool>);
impl MCEI2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MCEI2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MCEI2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MCEI2` writer - Match or Capture Channel 2 Event Input Enable"]
pub struct MCEI2_W<'a> {
    w: &'a mut W,
}
impl<'a> MCEI2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
#[doc = "Field `MCEI3` reader - Match or Capture Channel 3 Event Input Enable"]
pub struct MCEI3_R(crate::FieldReader<bool, bool>);
impl MCEI3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MCEI3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MCEI3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MCEI3` writer - Match or Capture Channel 3 Event Input Enable"]
pub struct MCEI3_W<'a> {
    w: &'a mut W,
}
impl<'a> MCEI3_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
#[doc = "Field `MCEO0` reader - Match or Capture Channel 0 Event Output Enable"]
pub struct MCEO0_R(crate::FieldReader<bool, bool>);
impl MCEO0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MCEO0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MCEO0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MCEO0` writer - Match or Capture Channel 0 Event Output Enable"]
pub struct MCEO0_W<'a> {
    w: &'a mut W,
}
impl<'a> MCEO0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
#[doc = "Field `MCEO1` reader - Match or Capture Channel 1 Event Output Enable"]
pub struct MCEO1_R(crate::FieldReader<bool, bool>);
impl MCEO1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MCEO1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MCEO1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MCEO1` writer - Match or Capture Channel 1 Event Output Enable"]
pub struct MCEO1_W<'a> {
    w: &'a mut W,
}
impl<'a> MCEO1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | ((value as u32 & 0x01) << 25);
        self.w
    }
}
#[doc = "Field `MCEO2` reader - Match or Capture Channel 2 Event Output Enable"]
pub struct MCEO2_R(crate::FieldReader<bool, bool>);
impl MCEO2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MCEO2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MCEO2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MCEO2` writer - Match or Capture Channel 2 Event Output Enable"]
pub struct MCEO2_W<'a> {
    w: &'a mut W,
}
impl<'a> MCEO2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | ((value as u32 & 0x01) << 26);
        self.w
    }
}
#[doc = "Field `MCEO3` reader - Match or Capture Channel 3 Event Output Enable"]
pub struct MCEO3_R(crate::FieldReader<bool, bool>);
impl MCEO3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MCEO3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MCEO3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MCEO3` writer - Match or Capture Channel 3 Event Output Enable"]
pub struct MCEO3_W<'a> {
    w: &'a mut W,
}
impl<'a> MCEO3_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | ((value as u32 & 0x01) << 27);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Timer/counter Input Event0 Action"]
    #[inline(always)]
    pub fn evact0(&self) -> EVACT0_R {
        EVACT0_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 3:5 - Timer/counter Input Event1 Action"]
    #[inline(always)]
    pub fn evact1(&self) -> EVACT1_R {
        EVACT1_R::new(((self.bits >> 3) & 0x07) as u8)
    }
    #[doc = "Bits 6:7 - Timer/counter Output Event Mode"]
    #[inline(always)]
    pub fn cntsel(&self) -> CNTSEL_R {
        CNTSEL_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bit 8 - Overflow/Underflow Output Event Enable"]
    #[inline(always)]
    pub fn ovfeo(&self) -> OVFEO_R {
        OVFEO_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Retrigger Output Event Enable"]
    #[inline(always)]
    pub fn trgeo(&self) -> TRGEO_R {
        TRGEO_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Timer/counter Output Event Enable"]
    #[inline(always)]
    pub fn cnteo(&self) -> CNTEO_R {
        CNTEO_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Inverted Event 0 Input Enable"]
    #[inline(always)]
    pub fn tcinv0(&self) -> TCINV0_R {
        TCINV0_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Inverted Event 1 Input Enable"]
    #[inline(always)]
    pub fn tcinv1(&self) -> TCINV1_R {
        TCINV1_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Timer/counter Event 0 Input Enable"]
    #[inline(always)]
    pub fn tcei0(&self) -> TCEI0_R {
        TCEI0_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Timer/counter Event 1 Input Enable"]
    #[inline(always)]
    pub fn tcei1(&self) -> TCEI1_R {
        TCEI1_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Match or Capture Channel 0 Event Input Enable"]
    #[inline(always)]
    pub fn mcei0(&self) -> MCEI0_R {
        MCEI0_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Match or Capture Channel 1 Event Input Enable"]
    #[inline(always)]
    pub fn mcei1(&self) -> MCEI1_R {
        MCEI1_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Match or Capture Channel 2 Event Input Enable"]
    #[inline(always)]
    pub fn mcei2(&self) -> MCEI2_R {
        MCEI2_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Match or Capture Channel 3 Event Input Enable"]
    #[inline(always)]
    pub fn mcei3(&self) -> MCEI3_R {
        MCEI3_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Match or Capture Channel 0 Event Output Enable"]
    #[inline(always)]
    pub fn mceo0(&self) -> MCEO0_R {
        MCEO0_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Match or Capture Channel 1 Event Output Enable"]
    #[inline(always)]
    pub fn mceo1(&self) -> MCEO1_R {
        MCEO1_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Match or Capture Channel 2 Event Output Enable"]
    #[inline(always)]
    pub fn mceo2(&self) -> MCEO2_R {
        MCEO2_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Match or Capture Channel 3 Event Output Enable"]
    #[inline(always)]
    pub fn mceo3(&self) -> MCEO3_R {
        MCEO3_R::new(((self.bits >> 27) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Timer/counter Input Event0 Action"]
    #[inline(always)]
    pub fn evact0(&mut self) -> EVACT0_W {
        EVACT0_W { w: self }
    }
    #[doc = "Bits 3:5 - Timer/counter Input Event1 Action"]
    #[inline(always)]
    pub fn evact1(&mut self) -> EVACT1_W {
        EVACT1_W { w: self }
    }
    #[doc = "Bits 6:7 - Timer/counter Output Event Mode"]
    #[inline(always)]
    pub fn cntsel(&mut self) -> CNTSEL_W {
        CNTSEL_W { w: self }
    }
    #[doc = "Bit 8 - Overflow/Underflow Output Event Enable"]
    #[inline(always)]
    pub fn ovfeo(&mut self) -> OVFEO_W {
        OVFEO_W { w: self }
    }
    #[doc = "Bit 9 - Retrigger Output Event Enable"]
    #[inline(always)]
    pub fn trgeo(&mut self) -> TRGEO_W {
        TRGEO_W { w: self }
    }
    #[doc = "Bit 10 - Timer/counter Output Event Enable"]
    #[inline(always)]
    pub fn cnteo(&mut self) -> CNTEO_W {
        CNTEO_W { w: self }
    }
    #[doc = "Bit 12 - Inverted Event 0 Input Enable"]
    #[inline(always)]
    pub fn tcinv0(&mut self) -> TCINV0_W {
        TCINV0_W { w: self }
    }
    #[doc = "Bit 13 - Inverted Event 1 Input Enable"]
    #[inline(always)]
    pub fn tcinv1(&mut self) -> TCINV1_W {
        TCINV1_W { w: self }
    }
    #[doc = "Bit 14 - Timer/counter Event 0 Input Enable"]
    #[inline(always)]
    pub fn tcei0(&mut self) -> TCEI0_W {
        TCEI0_W { w: self }
    }
    #[doc = "Bit 15 - Timer/counter Event 1 Input Enable"]
    #[inline(always)]
    pub fn tcei1(&mut self) -> TCEI1_W {
        TCEI1_W { w: self }
    }
    #[doc = "Bit 16 - Match or Capture Channel 0 Event Input Enable"]
    #[inline(always)]
    pub fn mcei0(&mut self) -> MCEI0_W {
        MCEI0_W { w: self }
    }
    #[doc = "Bit 17 - Match or Capture Channel 1 Event Input Enable"]
    #[inline(always)]
    pub fn mcei1(&mut self) -> MCEI1_W {
        MCEI1_W { w: self }
    }
    #[doc = "Bit 18 - Match or Capture Channel 2 Event Input Enable"]
    #[inline(always)]
    pub fn mcei2(&mut self) -> MCEI2_W {
        MCEI2_W { w: self }
    }
    #[doc = "Bit 19 - Match or Capture Channel 3 Event Input Enable"]
    #[inline(always)]
    pub fn mcei3(&mut self) -> MCEI3_W {
        MCEI3_W { w: self }
    }
    #[doc = "Bit 24 - Match or Capture Channel 0 Event Output Enable"]
    #[inline(always)]
    pub fn mceo0(&mut self) -> MCEO0_W {
        MCEO0_W { w: self }
    }
    #[doc = "Bit 25 - Match or Capture Channel 1 Event Output Enable"]
    #[inline(always)]
    pub fn mceo1(&mut self) -> MCEO1_W {
        MCEO1_W { w: self }
    }
    #[doc = "Bit 26 - Match or Capture Channel 2 Event Output Enable"]
    #[inline(always)]
    pub fn mceo2(&mut self) -> MCEO2_W {
        MCEO2_W { w: self }
    }
    #[doc = "Bit 27 - Match or Capture Channel 3 Event Output Enable"]
    #[inline(always)]
    pub fn mceo3(&mut self) -> MCEO3_W {
        MCEO3_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Event Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [evctrl](index.html) module"]
pub struct EVCTRL_SPEC;
impl crate::RegisterSpec for EVCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [evctrl::R](R) reader structure"]
impl crate::Readable for EVCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [evctrl::W](W) writer structure"]
impl crate::Writable for EVCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EVCTRL to value 0"]
impl crate::Resettable for EVCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
