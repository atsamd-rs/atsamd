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
#[doc = "Field `EVACT0` reader - Timer/counter Input Event0 Action"]
pub type EVACT0_R = crate::FieldReader<u8, EVACT0SELECT_A>;
#[doc = "Timer/counter Input Event0 Action\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EVACT0SELECT_A {
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
impl From<EVACT0SELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: EVACT0SELECT_A) -> Self {
        variant as _
    }
}
impl EVACT0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EVACT0SELECT_A {
        match self.bits {
            0 => EVACT0SELECT_A::OFF,
            1 => EVACT0SELECT_A::RETRIGGER,
            2 => EVACT0SELECT_A::COUNTEV,
            3 => EVACT0SELECT_A::START,
            4 => EVACT0SELECT_A::INC,
            5 => EVACT0SELECT_A::COUNT,
            6 => EVACT0SELECT_A::STAMP,
            7 => EVACT0SELECT_A::FAULT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == EVACT0SELECT_A::OFF
    }
    #[doc = "Checks if the value of the field is `RETRIGGER`"]
    #[inline(always)]
    pub fn is_retrigger(&self) -> bool {
        *self == EVACT0SELECT_A::RETRIGGER
    }
    #[doc = "Checks if the value of the field is `COUNTEV`"]
    #[inline(always)]
    pub fn is_countev(&self) -> bool {
        *self == EVACT0SELECT_A::COUNTEV
    }
    #[doc = "Checks if the value of the field is `START`"]
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        *self == EVACT0SELECT_A::START
    }
    #[doc = "Checks if the value of the field is `INC`"]
    #[inline(always)]
    pub fn is_inc(&self) -> bool {
        *self == EVACT0SELECT_A::INC
    }
    #[doc = "Checks if the value of the field is `COUNT`"]
    #[inline(always)]
    pub fn is_count(&self) -> bool {
        *self == EVACT0SELECT_A::COUNT
    }
    #[doc = "Checks if the value of the field is `STAMP`"]
    #[inline(always)]
    pub fn is_stamp(&self) -> bool {
        *self == EVACT0SELECT_A::STAMP
    }
    #[doc = "Checks if the value of the field is `FAULT`"]
    #[inline(always)]
    pub fn is_fault(&self) -> bool {
        *self == EVACT0SELECT_A::FAULT
    }
}
#[doc = "Field `EVACT0` writer - Timer/counter Input Event0 Action"]
pub type EVACT0_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, EVCTRL_SPEC, u8, EVACT0SELECT_A, 3, O>;
impl<'a, const O: u8> EVACT0_W<'a, O> {
    #[doc = "Event action disabled"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(EVACT0SELECT_A::OFF)
    }
    #[doc = "Start, restart or re-trigger counter on event"]
    #[inline(always)]
    pub fn retrigger(self) -> &'a mut W {
        self.variant(EVACT0SELECT_A::RETRIGGER)
    }
    #[doc = "Count on event"]
    #[inline(always)]
    pub fn countev(self) -> &'a mut W {
        self.variant(EVACT0SELECT_A::COUNTEV)
    }
    #[doc = "Start counter on event"]
    #[inline(always)]
    pub fn start(self) -> &'a mut W {
        self.variant(EVACT0SELECT_A::START)
    }
    #[doc = "Increment counter on event"]
    #[inline(always)]
    pub fn inc(self) -> &'a mut W {
        self.variant(EVACT0SELECT_A::INC)
    }
    #[doc = "Count on active state of asynchronous event"]
    #[inline(always)]
    pub fn count(self) -> &'a mut W {
        self.variant(EVACT0SELECT_A::COUNT)
    }
    #[doc = "Stamp capture"]
    #[inline(always)]
    pub fn stamp(self) -> &'a mut W {
        self.variant(EVACT0SELECT_A::STAMP)
    }
    #[doc = "Non-recoverable fault"]
    #[inline(always)]
    pub fn fault(self) -> &'a mut W {
        self.variant(EVACT0SELECT_A::FAULT)
    }
}
#[doc = "Field `EVACT1` reader - Timer/counter Input Event1 Action"]
pub type EVACT1_R = crate::FieldReader<u8, EVACT1SELECT_A>;
#[doc = "Timer/counter Input Event1 Action\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EVACT1SELECT_A {
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
impl From<EVACT1SELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: EVACT1SELECT_A) -> Self {
        variant as _
    }
}
impl EVACT1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EVACT1SELECT_A {
        match self.bits {
            0 => EVACT1SELECT_A::OFF,
            1 => EVACT1SELECT_A::RETRIGGER,
            2 => EVACT1SELECT_A::DIR,
            3 => EVACT1SELECT_A::STOP,
            4 => EVACT1SELECT_A::DEC,
            5 => EVACT1SELECT_A::PPW,
            6 => EVACT1SELECT_A::PWP,
            7 => EVACT1SELECT_A::FAULT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == EVACT1SELECT_A::OFF
    }
    #[doc = "Checks if the value of the field is `RETRIGGER`"]
    #[inline(always)]
    pub fn is_retrigger(&self) -> bool {
        *self == EVACT1SELECT_A::RETRIGGER
    }
    #[doc = "Checks if the value of the field is `DIR`"]
    #[inline(always)]
    pub fn is_dir(&self) -> bool {
        *self == EVACT1SELECT_A::DIR
    }
    #[doc = "Checks if the value of the field is `STOP`"]
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        *self == EVACT1SELECT_A::STOP
    }
    #[doc = "Checks if the value of the field is `DEC`"]
    #[inline(always)]
    pub fn is_dec(&self) -> bool {
        *self == EVACT1SELECT_A::DEC
    }
    #[doc = "Checks if the value of the field is `PPW`"]
    #[inline(always)]
    pub fn is_ppw(&self) -> bool {
        *self == EVACT1SELECT_A::PPW
    }
    #[doc = "Checks if the value of the field is `PWP`"]
    #[inline(always)]
    pub fn is_pwp(&self) -> bool {
        *self == EVACT1SELECT_A::PWP
    }
    #[doc = "Checks if the value of the field is `FAULT`"]
    #[inline(always)]
    pub fn is_fault(&self) -> bool {
        *self == EVACT1SELECT_A::FAULT
    }
}
#[doc = "Field `EVACT1` writer - Timer/counter Input Event1 Action"]
pub type EVACT1_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, EVCTRL_SPEC, u8, EVACT1SELECT_A, 3, O>;
impl<'a, const O: u8> EVACT1_W<'a, O> {
    #[doc = "Event action disabled"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(EVACT1SELECT_A::OFF)
    }
    #[doc = "Re-trigger counter on event"]
    #[inline(always)]
    pub fn retrigger(self) -> &'a mut W {
        self.variant(EVACT1SELECT_A::RETRIGGER)
    }
    #[doc = "Direction control"]
    #[inline(always)]
    pub fn dir(self) -> &'a mut W {
        self.variant(EVACT1SELECT_A::DIR)
    }
    #[doc = "Stop counter on event"]
    #[inline(always)]
    pub fn stop(self) -> &'a mut W {
        self.variant(EVACT1SELECT_A::STOP)
    }
    #[doc = "Decrement counter on event"]
    #[inline(always)]
    pub fn dec(self) -> &'a mut W {
        self.variant(EVACT1SELECT_A::DEC)
    }
    #[doc = "Period capture value in CC0 register, pulse width capture value in CC1 register"]
    #[inline(always)]
    pub fn ppw(self) -> &'a mut W {
        self.variant(EVACT1SELECT_A::PPW)
    }
    #[doc = "Period capture value in CC1 register, pulse width capture value in CC0 register"]
    #[inline(always)]
    pub fn pwp(self) -> &'a mut W {
        self.variant(EVACT1SELECT_A::PWP)
    }
    #[doc = "Non-recoverable fault"]
    #[inline(always)]
    pub fn fault(self) -> &'a mut W {
        self.variant(EVACT1SELECT_A::FAULT)
    }
}
#[doc = "Field `CNTSEL` reader - Timer/counter Output Event Mode"]
pub type CNTSEL_R = crate::FieldReader<u8, CNTSELSELECT_A>;
#[doc = "Timer/counter Output Event Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CNTSELSELECT_A {
    #[doc = "0: An interrupt/event is generated when a new counter cycle starts"]
    START = 0,
    #[doc = "1: An interrupt/event is generated when a counter cycle ends"]
    END = 1,
    #[doc = "2: An interrupt/event is generated when a counter cycle ends, except for the first and last cycles"]
    BETWEEN = 2,
    #[doc = "3: An interrupt/event is generated when a new counter cycle starts or a counter cycle ends"]
    BOUNDARY = 3,
}
impl From<CNTSELSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: CNTSELSELECT_A) -> Self {
        variant as _
    }
}
impl CNTSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CNTSELSELECT_A {
        match self.bits {
            0 => CNTSELSELECT_A::START,
            1 => CNTSELSELECT_A::END,
            2 => CNTSELSELECT_A::BETWEEN,
            3 => CNTSELSELECT_A::BOUNDARY,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `START`"]
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        *self == CNTSELSELECT_A::START
    }
    #[doc = "Checks if the value of the field is `END`"]
    #[inline(always)]
    pub fn is_end(&self) -> bool {
        *self == CNTSELSELECT_A::END
    }
    #[doc = "Checks if the value of the field is `BETWEEN`"]
    #[inline(always)]
    pub fn is_between(&self) -> bool {
        *self == CNTSELSELECT_A::BETWEEN
    }
    #[doc = "Checks if the value of the field is `BOUNDARY`"]
    #[inline(always)]
    pub fn is_boundary(&self) -> bool {
        *self == CNTSELSELECT_A::BOUNDARY
    }
}
#[doc = "Field `CNTSEL` writer - Timer/counter Output Event Mode"]
pub type CNTSEL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, EVCTRL_SPEC, u8, CNTSELSELECT_A, 2, O>;
impl<'a, const O: u8> CNTSEL_W<'a, O> {
    #[doc = "An interrupt/event is generated when a new counter cycle starts"]
    #[inline(always)]
    pub fn start(self) -> &'a mut W {
        self.variant(CNTSELSELECT_A::START)
    }
    #[doc = "An interrupt/event is generated when a counter cycle ends"]
    #[inline(always)]
    pub fn end(self) -> &'a mut W {
        self.variant(CNTSELSELECT_A::END)
    }
    #[doc = "An interrupt/event is generated when a counter cycle ends, except for the first and last cycles"]
    #[inline(always)]
    pub fn between(self) -> &'a mut W {
        self.variant(CNTSELSELECT_A::BETWEEN)
    }
    #[doc = "An interrupt/event is generated when a new counter cycle starts or a counter cycle ends"]
    #[inline(always)]
    pub fn boundary(self) -> &'a mut W {
        self.variant(CNTSELSELECT_A::BOUNDARY)
    }
}
#[doc = "Field `OVFEO` reader - Overflow/Underflow Output Event Enable"]
pub type OVFEO_R = crate::BitReader<bool>;
#[doc = "Field `OVFEO` writer - Overflow/Underflow Output Event Enable"]
pub type OVFEO_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVCTRL_SPEC, bool, O>;
#[doc = "Field `TRGEO` reader - Retrigger Output Event Enable"]
pub type TRGEO_R = crate::BitReader<bool>;
#[doc = "Field `TRGEO` writer - Retrigger Output Event Enable"]
pub type TRGEO_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVCTRL_SPEC, bool, O>;
#[doc = "Field `CNTEO` reader - Timer/counter Output Event Enable"]
pub type CNTEO_R = crate::BitReader<bool>;
#[doc = "Field `CNTEO` writer - Timer/counter Output Event Enable"]
pub type CNTEO_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVCTRL_SPEC, bool, O>;
#[doc = "Field `TCINV0` reader - Inverted Event 0 Input Enable"]
pub type TCINV0_R = crate::BitReader<bool>;
#[doc = "Field `TCINV0` writer - Inverted Event 0 Input Enable"]
pub type TCINV0_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVCTRL_SPEC, bool, O>;
#[doc = "Field `TCINV1` reader - Inverted Event 1 Input Enable"]
pub type TCINV1_R = crate::BitReader<bool>;
#[doc = "Field `TCINV1` writer - Inverted Event 1 Input Enable"]
pub type TCINV1_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVCTRL_SPEC, bool, O>;
#[doc = "Field `TCEI0` reader - Timer/counter Event 0 Input Enable"]
pub type TCEI0_R = crate::BitReader<bool>;
#[doc = "Field `TCEI0` writer - Timer/counter Event 0 Input Enable"]
pub type TCEI0_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVCTRL_SPEC, bool, O>;
#[doc = "Field `TCEI1` reader - Timer/counter Event 1 Input Enable"]
pub type TCEI1_R = crate::BitReader<bool>;
#[doc = "Field `TCEI1` writer - Timer/counter Event 1 Input Enable"]
pub type TCEI1_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVCTRL_SPEC, bool, O>;
#[doc = "Field `MCEI0` reader - Match or Capture Channel 0 Event Input Enable"]
pub type MCEI0_R = crate::BitReader<bool>;
#[doc = "Field `MCEI0` writer - Match or Capture Channel 0 Event Input Enable"]
pub type MCEI0_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVCTRL_SPEC, bool, O>;
#[doc = "Field `MCEI1` reader - Match or Capture Channel 1 Event Input Enable"]
pub type MCEI1_R = crate::BitReader<bool>;
#[doc = "Field `MCEI1` writer - Match or Capture Channel 1 Event Input Enable"]
pub type MCEI1_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVCTRL_SPEC, bool, O>;
#[doc = "Field `MCEI2` reader - Match or Capture Channel 2 Event Input Enable"]
pub type MCEI2_R = crate::BitReader<bool>;
#[doc = "Field `MCEI2` writer - Match or Capture Channel 2 Event Input Enable"]
pub type MCEI2_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVCTRL_SPEC, bool, O>;
#[doc = "Field `MCEI3` reader - Match or Capture Channel 3 Event Input Enable"]
pub type MCEI3_R = crate::BitReader<bool>;
#[doc = "Field `MCEI3` writer - Match or Capture Channel 3 Event Input Enable"]
pub type MCEI3_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVCTRL_SPEC, bool, O>;
#[doc = "Field `MCEO0` reader - Match or Capture Channel 0 Event Output Enable"]
pub type MCEO0_R = crate::BitReader<bool>;
#[doc = "Field `MCEO0` writer - Match or Capture Channel 0 Event Output Enable"]
pub type MCEO0_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVCTRL_SPEC, bool, O>;
#[doc = "Field `MCEO1` reader - Match or Capture Channel 1 Event Output Enable"]
pub type MCEO1_R = crate::BitReader<bool>;
#[doc = "Field `MCEO1` writer - Match or Capture Channel 1 Event Output Enable"]
pub type MCEO1_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVCTRL_SPEC, bool, O>;
#[doc = "Field `MCEO2` reader - Match or Capture Channel 2 Event Output Enable"]
pub type MCEO2_R = crate::BitReader<bool>;
#[doc = "Field `MCEO2` writer - Match or Capture Channel 2 Event Output Enable"]
pub type MCEO2_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVCTRL_SPEC, bool, O>;
#[doc = "Field `MCEO3` reader - Match or Capture Channel 3 Event Output Enable"]
pub type MCEO3_R = crate::BitReader<bool>;
#[doc = "Field `MCEO3` writer - Match or Capture Channel 3 Event Output Enable"]
pub type MCEO3_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVCTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:2 - Timer/counter Input Event0 Action"]
    #[inline(always)]
    pub fn evact0(&self) -> EVACT0_R {
        EVACT0_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - Timer/counter Input Event1 Action"]
    #[inline(always)]
    pub fn evact1(&self) -> EVACT1_R {
        EVACT1_R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:7 - Timer/counter Output Event Mode"]
    #[inline(always)]
    pub fn cntsel(&self) -> CNTSEL_R {
        CNTSEL_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - Overflow/Underflow Output Event Enable"]
    #[inline(always)]
    pub fn ovfeo(&self) -> OVFEO_R {
        OVFEO_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Retrigger Output Event Enable"]
    #[inline(always)]
    pub fn trgeo(&self) -> TRGEO_R {
        TRGEO_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Timer/counter Output Event Enable"]
    #[inline(always)]
    pub fn cnteo(&self) -> CNTEO_R {
        CNTEO_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12 - Inverted Event 0 Input Enable"]
    #[inline(always)]
    pub fn tcinv0(&self) -> TCINV0_R {
        TCINV0_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Inverted Event 1 Input Enable"]
    #[inline(always)]
    pub fn tcinv1(&self) -> TCINV1_R {
        TCINV1_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Timer/counter Event 0 Input Enable"]
    #[inline(always)]
    pub fn tcei0(&self) -> TCEI0_R {
        TCEI0_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Timer/counter Event 1 Input Enable"]
    #[inline(always)]
    pub fn tcei1(&self) -> TCEI1_R {
        TCEI1_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Match or Capture Channel 0 Event Input Enable"]
    #[inline(always)]
    pub fn mcei0(&self) -> MCEI0_R {
        MCEI0_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Match or Capture Channel 1 Event Input Enable"]
    #[inline(always)]
    pub fn mcei1(&self) -> MCEI1_R {
        MCEI1_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Match or Capture Channel 2 Event Input Enable"]
    #[inline(always)]
    pub fn mcei2(&self) -> MCEI2_R {
        MCEI2_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Match or Capture Channel 3 Event Input Enable"]
    #[inline(always)]
    pub fn mcei3(&self) -> MCEI3_R {
        MCEI3_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 24 - Match or Capture Channel 0 Event Output Enable"]
    #[inline(always)]
    pub fn mceo0(&self) -> MCEO0_R {
        MCEO0_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Match or Capture Channel 1 Event Output Enable"]
    #[inline(always)]
    pub fn mceo1(&self) -> MCEO1_R {
        MCEO1_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Match or Capture Channel 2 Event Output Enable"]
    #[inline(always)]
    pub fn mceo2(&self) -> MCEO2_R {
        MCEO2_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Match or Capture Channel 3 Event Output Enable"]
    #[inline(always)]
    pub fn mceo3(&self) -> MCEO3_R {
        MCEO3_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Timer/counter Input Event0 Action"]
    #[inline(always)]
    #[must_use]
    pub fn evact0(&mut self) -> EVACT0_W<0> {
        EVACT0_W::new(self)
    }
    #[doc = "Bits 3:5 - Timer/counter Input Event1 Action"]
    #[inline(always)]
    #[must_use]
    pub fn evact1(&mut self) -> EVACT1_W<3> {
        EVACT1_W::new(self)
    }
    #[doc = "Bits 6:7 - Timer/counter Output Event Mode"]
    #[inline(always)]
    #[must_use]
    pub fn cntsel(&mut self) -> CNTSEL_W<6> {
        CNTSEL_W::new(self)
    }
    #[doc = "Bit 8 - Overflow/Underflow Output Event Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ovfeo(&mut self) -> OVFEO_W<8> {
        OVFEO_W::new(self)
    }
    #[doc = "Bit 9 - Retrigger Output Event Enable"]
    #[inline(always)]
    #[must_use]
    pub fn trgeo(&mut self) -> TRGEO_W<9> {
        TRGEO_W::new(self)
    }
    #[doc = "Bit 10 - Timer/counter Output Event Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cnteo(&mut self) -> CNTEO_W<10> {
        CNTEO_W::new(self)
    }
    #[doc = "Bit 12 - Inverted Event 0 Input Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tcinv0(&mut self) -> TCINV0_W<12> {
        TCINV0_W::new(self)
    }
    #[doc = "Bit 13 - Inverted Event 1 Input Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tcinv1(&mut self) -> TCINV1_W<13> {
        TCINV1_W::new(self)
    }
    #[doc = "Bit 14 - Timer/counter Event 0 Input Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tcei0(&mut self) -> TCEI0_W<14> {
        TCEI0_W::new(self)
    }
    #[doc = "Bit 15 - Timer/counter Event 1 Input Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tcei1(&mut self) -> TCEI1_W<15> {
        TCEI1_W::new(self)
    }
    #[doc = "Bit 16 - Match or Capture Channel 0 Event Input Enable"]
    #[inline(always)]
    #[must_use]
    pub fn mcei0(&mut self) -> MCEI0_W<16> {
        MCEI0_W::new(self)
    }
    #[doc = "Bit 17 - Match or Capture Channel 1 Event Input Enable"]
    #[inline(always)]
    #[must_use]
    pub fn mcei1(&mut self) -> MCEI1_W<17> {
        MCEI1_W::new(self)
    }
    #[doc = "Bit 18 - Match or Capture Channel 2 Event Input Enable"]
    #[inline(always)]
    #[must_use]
    pub fn mcei2(&mut self) -> MCEI2_W<18> {
        MCEI2_W::new(self)
    }
    #[doc = "Bit 19 - Match or Capture Channel 3 Event Input Enable"]
    #[inline(always)]
    #[must_use]
    pub fn mcei3(&mut self) -> MCEI3_W<19> {
        MCEI3_W::new(self)
    }
    #[doc = "Bit 24 - Match or Capture Channel 0 Event Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn mceo0(&mut self) -> MCEO0_W<24> {
        MCEO0_W::new(self)
    }
    #[doc = "Bit 25 - Match or Capture Channel 1 Event Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn mceo1(&mut self) -> MCEO1_W<25> {
        MCEO1_W::new(self)
    }
    #[doc = "Bit 26 - Match or Capture Channel 2 Event Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn mceo2(&mut self) -> MCEO2_W<26> {
        MCEO2_W::new(self)
    }
    #[doc = "Bit 27 - Match or Capture Channel 3 Event Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn mceo3(&mut self) -> MCEO3_W<27> {
        MCEO3_W::new(self)
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EVCTRL to value 0"]
impl crate::Resettable for EVCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
