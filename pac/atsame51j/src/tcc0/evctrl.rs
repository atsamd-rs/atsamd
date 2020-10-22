#[doc = "Reader of register EVCTRL"]
pub type R = crate::R<u32, super::EVCTRL>;
#[doc = "Writer for register EVCTRL"]
pub type W = crate::W<u32, super::EVCTRL>;
#[doc = "Register EVCTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::EVCTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
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
    #[doc = "6: Stamp capture"]
    STAMP = 6,
    #[doc = "7: Non-recoverable fault"]
    FAULT = 7,
}
impl From<EVACT0_A> for u8 {
    #[inline(always)]
    fn from(variant: EVACT0_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `EVACT0`"]
pub type EVACT0_R = crate::R<u8, EVACT0_A>;
impl EVACT0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EVACT0_A {
        match self.bits {
            0 => EVACT0_A::OFF,
            1 => EVACT0_A::RETRIGGER,
            2 => EVACT0_A::COUNTEV,
            3 => EVACT0_A::START,
            4 => EVACT0_A::INC,
            5 => EVACT0_A::COUNT,
            6 => EVACT0_A::STAMP,
            7 => EVACT0_A::FAULT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == EVACT0_A::OFF
    }
    #[doc = "Checks if the value of the field is `RETRIGGER`"]
    #[inline(always)]
    pub fn is_retrigger(&self) -> bool {
        *self == EVACT0_A::RETRIGGER
    }
    #[doc = "Checks if the value of the field is `COUNTEV`"]
    #[inline(always)]
    pub fn is_countev(&self) -> bool {
        *self == EVACT0_A::COUNTEV
    }
    #[doc = "Checks if the value of the field is `START`"]
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        *self == EVACT0_A::START
    }
    #[doc = "Checks if the value of the field is `INC`"]
    #[inline(always)]
    pub fn is_inc(&self) -> bool {
        *self == EVACT0_A::INC
    }
    #[doc = "Checks if the value of the field is `COUNT`"]
    #[inline(always)]
    pub fn is_count(&self) -> bool {
        *self == EVACT0_A::COUNT
    }
    #[doc = "Checks if the value of the field is `STAMP`"]
    #[inline(always)]
    pub fn is_stamp(&self) -> bool {
        *self == EVACT0_A::STAMP
    }
    #[doc = "Checks if the value of the field is `FAULT`"]
    #[inline(always)]
    pub fn is_fault(&self) -> bool {
        *self == EVACT0_A::FAULT
    }
}
#[doc = "Write proxy for field `EVACT0`"]
pub struct EVACT0_W<'a> {
    w: &'a mut W,
}
impl<'a> EVACT0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EVACT0_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
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
    #[doc = "Stamp capture"]
    #[inline(always)]
    pub fn stamp(self) -> &'a mut W {
        self.variant(EVACT0_A::STAMP)
    }
    #[doc = "Non-recoverable fault"]
    #[inline(always)]
    pub fn fault(self) -> &'a mut W {
        self.variant(EVACT0_A::FAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
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
#[doc = "Reader of field `EVACT1`"]
pub type EVACT1_R = crate::R<u8, EVACT1_A>;
impl EVACT1_R {
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
        *self == EVACT1_A::OFF
    }
    #[doc = "Checks if the value of the field is `RETRIGGER`"]
    #[inline(always)]
    pub fn is_retrigger(&self) -> bool {
        *self == EVACT1_A::RETRIGGER
    }
    #[doc = "Checks if the value of the field is `DIR`"]
    #[inline(always)]
    pub fn is_dir(&self) -> bool {
        *self == EVACT1_A::DIR
    }
    #[doc = "Checks if the value of the field is `STOP`"]
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        *self == EVACT1_A::STOP
    }
    #[doc = "Checks if the value of the field is `DEC`"]
    #[inline(always)]
    pub fn is_dec(&self) -> bool {
        *self == EVACT1_A::DEC
    }
    #[doc = "Checks if the value of the field is `PPW`"]
    #[inline(always)]
    pub fn is_ppw(&self) -> bool {
        *self == EVACT1_A::PPW
    }
    #[doc = "Checks if the value of the field is `PWP`"]
    #[inline(always)]
    pub fn is_pwp(&self) -> bool {
        *self == EVACT1_A::PWP
    }
    #[doc = "Checks if the value of the field is `FAULT`"]
    #[inline(always)]
    pub fn is_fault(&self) -> bool {
        *self == EVACT1_A::FAULT
    }
}
#[doc = "Write proxy for field `EVACT1`"]
pub struct EVACT1_W<'a> {
    w: &'a mut W,
}
impl<'a> EVACT1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EVACT1_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x07 << 3)) | (((value as u32) & 0x07) << 3);
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
#[doc = "Reader of field `CNTSEL`"]
pub type CNTSEL_R = crate::R<u8, CNTSEL_A>;
impl CNTSEL_R {
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
        *self == CNTSEL_A::START
    }
    #[doc = "Checks if the value of the field is `END`"]
    #[inline(always)]
    pub fn is_end(&self) -> bool {
        *self == CNTSEL_A::END
    }
    #[doc = "Checks if the value of the field is `BETWEEN`"]
    #[inline(always)]
    pub fn is_between(&self) -> bool {
        *self == CNTSEL_A::BETWEEN
    }
    #[doc = "Checks if the value of the field is `BOUNDARY`"]
    #[inline(always)]
    pub fn is_boundary(&self) -> bool {
        *self == CNTSEL_A::BOUNDARY
    }
}
#[doc = "Write proxy for field `CNTSEL`"]
pub struct CNTSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CNTSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CNTSEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "Reader of field `OVFEO`"]
pub type OVFEO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OVFEO`"]
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `TRGEO`"]
pub type TRGEO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TRGEO`"]
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `CNTEO`"]
pub type CNTEO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CNTEO`"]
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `TCINV0`"]
pub type TCINV0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TCINV0`"]
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `TCINV1`"]
pub type TCINV1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TCINV1`"]
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Reader of field `TCEI0`"]
pub type TCEI0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TCEI0`"]
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Reader of field `TCEI1`"]
pub type TCEI1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TCEI1`"]
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Reader of field `MCEI0`"]
pub type MCEI0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MCEI0`"]
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `MCEI1`"]
pub type MCEI1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MCEI1`"]
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Reader of field `MCEI2`"]
pub type MCEI2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MCEI2`"]
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Reader of field `MCEI3`"]
pub type MCEI3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MCEI3`"]
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Reader of field `MCEI4`"]
pub type MCEI4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MCEI4`"]
pub struct MCEI4_W<'a> {
    w: &'a mut W,
}
impl<'a> MCEI4_W<'a> {
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
#[doc = "Reader of field `MCEI5`"]
pub type MCEI5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MCEI5`"]
pub struct MCEI5_W<'a> {
    w: &'a mut W,
}
impl<'a> MCEI5_W<'a> {
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
#[doc = "Reader of field `MCEO0`"]
pub type MCEO0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MCEO0`"]
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Reader of field `MCEO1`"]
pub type MCEO1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MCEO1`"]
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "Reader of field `MCEO2`"]
pub type MCEO2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MCEO2`"]
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "Reader of field `MCEO3`"]
pub type MCEO3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MCEO3`"]
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
#[doc = "Reader of field `MCEO4`"]
pub type MCEO4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MCEO4`"]
pub struct MCEO4_W<'a> {
    w: &'a mut W,
}
impl<'a> MCEO4_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "Reader of field `MCEO5`"]
pub type MCEO5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MCEO5`"]
pub struct MCEO5_W<'a> {
    w: &'a mut W,
}
impl<'a> MCEO5_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
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
    #[doc = "Bit 20 - Match or Capture Channel 4 Event Input Enable"]
    #[inline(always)]
    pub fn mcei4(&self) -> MCEI4_R {
        MCEI4_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Match or Capture Channel 5 Event Input Enable"]
    #[inline(always)]
    pub fn mcei5(&self) -> MCEI5_R {
        MCEI5_R::new(((self.bits >> 21) & 0x01) != 0)
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
    #[doc = "Bit 28 - Match or Capture Channel 4 Event Output Enable"]
    #[inline(always)]
    pub fn mceo4(&self) -> MCEO4_R {
        MCEO4_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Match or Capture Channel 5 Event Output Enable"]
    #[inline(always)]
    pub fn mceo5(&self) -> MCEO5_R {
        MCEO5_R::new(((self.bits >> 29) & 0x01) != 0)
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
    #[doc = "Bit 20 - Match or Capture Channel 4 Event Input Enable"]
    #[inline(always)]
    pub fn mcei4(&mut self) -> MCEI4_W {
        MCEI4_W { w: self }
    }
    #[doc = "Bit 21 - Match or Capture Channel 5 Event Input Enable"]
    #[inline(always)]
    pub fn mcei5(&mut self) -> MCEI5_W {
        MCEI5_W { w: self }
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
    #[doc = "Bit 28 - Match or Capture Channel 4 Event Output Enable"]
    #[inline(always)]
    pub fn mceo4(&mut self) -> MCEO4_W {
        MCEO4_W { w: self }
    }
    #[doc = "Bit 29 - Match or Capture Channel 5 Event Output Enable"]
    #[inline(always)]
    pub fn mceo5(&mut self) -> MCEO5_W {
        MCEO5_W { w: self }
    }
}
