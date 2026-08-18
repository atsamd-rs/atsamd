#[doc = "Register `FCTRLA` reader"]
pub struct R(crate::R<FCTRLA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FCTRLA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FCTRLA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FCTRLA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FCTRLA` writer"]
pub struct W(crate::W<FCTRLA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FCTRLA_SPEC>;
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
impl From<crate::W<FCTRLA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FCTRLA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SRC` reader - Fault A Source"]
pub type SRC_R = crate::FieldReader<u8, SRCSELECT_A>;
#[doc = "Fault A Source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SRCSELECT_A {
    #[doc = "0: Fault input disabled"]
    DISABLE = 0,
    #[doc = "1: MCEx (x=0,1) event input"]
    ENABLE = 1,
    #[doc = "2: Inverted MCEx (x=0,1) event input"]
    INVERT = 2,
    #[doc = "3: Alternate fault (A or B) state at the end of the previous period"]
    ALTFAULT = 3,
}
impl From<SRCSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: SRCSELECT_A) -> Self {
        variant as _
    }
}
impl SRC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRCSELECT_A {
        match self.bits {
            0 => SRCSELECT_A::DISABLE,
            1 => SRCSELECT_A::ENABLE,
            2 => SRCSELECT_A::INVERT,
            3 => SRCSELECT_A::ALTFAULT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == SRCSELECT_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == SRCSELECT_A::ENABLE
    }
    #[doc = "Checks if the value of the field is `INVERT`"]
    #[inline(always)]
    pub fn is_invert(&self) -> bool {
        *self == SRCSELECT_A::INVERT
    }
    #[doc = "Checks if the value of the field is `ALTFAULT`"]
    #[inline(always)]
    pub fn is_altfault(&self) -> bool {
        *self == SRCSELECT_A::ALTFAULT
    }
}
#[doc = "Field `SRC` writer - Fault A Source"]
pub type SRC_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, FCTRLA_SPEC, u8, SRCSELECT_A, 2, O>;
impl<'a, const O: u8> SRC_W<'a, O> {
    #[doc = "Fault input disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(SRCSELECT_A::DISABLE)
    }
    #[doc = "MCEx (x=0,1) event input"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(SRCSELECT_A::ENABLE)
    }
    #[doc = "Inverted MCEx (x=0,1) event input"]
    #[inline(always)]
    pub fn invert(self) -> &'a mut W {
        self.variant(SRCSELECT_A::INVERT)
    }
    #[doc = "Alternate fault (A or B) state at the end of the previous period"]
    #[inline(always)]
    pub fn altfault(self) -> &'a mut W {
        self.variant(SRCSELECT_A::ALTFAULT)
    }
}
#[doc = "Field `KEEP` reader - Fault A Keeper"]
pub type KEEP_R = crate::BitReader<bool>;
#[doc = "Field `KEEP` writer - Fault A Keeper"]
pub type KEEP_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCTRLA_SPEC, bool, O>;
#[doc = "Field `QUAL` reader - Fault A Qualification"]
pub type QUAL_R = crate::BitReader<bool>;
#[doc = "Field `QUAL` writer - Fault A Qualification"]
pub type QUAL_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCTRLA_SPEC, bool, O>;
#[doc = "Field `BLANK` reader - Fault A Blanking Mode"]
pub type BLANK_R = crate::FieldReader<u8, BLANKSELECT_A>;
#[doc = "Fault A Blanking Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BLANKSELECT_A {
    #[doc = "0: Blanking applied from start of the ramp"]
    START = 0,
    #[doc = "1: Blanking applied from rising edge of the output waveform"]
    RISE = 1,
    #[doc = "2: Blanking applied from falling edge of the output waveform"]
    FALL = 2,
    #[doc = "3: Blanking applied from each toggle of the output waveform"]
    BOTH = 3,
}
impl From<BLANKSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: BLANKSELECT_A) -> Self {
        variant as _
    }
}
impl BLANK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BLANKSELECT_A {
        match self.bits {
            0 => BLANKSELECT_A::START,
            1 => BLANKSELECT_A::RISE,
            2 => BLANKSELECT_A::FALL,
            3 => BLANKSELECT_A::BOTH,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `START`"]
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        *self == BLANKSELECT_A::START
    }
    #[doc = "Checks if the value of the field is `RISE`"]
    #[inline(always)]
    pub fn is_rise(&self) -> bool {
        *self == BLANKSELECT_A::RISE
    }
    #[doc = "Checks if the value of the field is `FALL`"]
    #[inline(always)]
    pub fn is_fall(&self) -> bool {
        *self == BLANKSELECT_A::FALL
    }
    #[doc = "Checks if the value of the field is `BOTH`"]
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        *self == BLANKSELECT_A::BOTH
    }
}
#[doc = "Field `BLANK` writer - Fault A Blanking Mode"]
pub type BLANK_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, FCTRLA_SPEC, u8, BLANKSELECT_A, 2, O>;
impl<'a, const O: u8> BLANK_W<'a, O> {
    #[doc = "Blanking applied from start of the ramp"]
    #[inline(always)]
    pub fn start(self) -> &'a mut W {
        self.variant(BLANKSELECT_A::START)
    }
    #[doc = "Blanking applied from rising edge of the output waveform"]
    #[inline(always)]
    pub fn rise(self) -> &'a mut W {
        self.variant(BLANKSELECT_A::RISE)
    }
    #[doc = "Blanking applied from falling edge of the output waveform"]
    #[inline(always)]
    pub fn fall(self) -> &'a mut W {
        self.variant(BLANKSELECT_A::FALL)
    }
    #[doc = "Blanking applied from each toggle of the output waveform"]
    #[inline(always)]
    pub fn both(self) -> &'a mut W {
        self.variant(BLANKSELECT_A::BOTH)
    }
}
#[doc = "Field `RESTART` reader - Fault A Restart"]
pub type RESTART_R = crate::BitReader<bool>;
#[doc = "Field `RESTART` writer - Fault A Restart"]
pub type RESTART_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCTRLA_SPEC, bool, O>;
#[doc = "Field `HALT` reader - Fault A Halt Mode"]
pub type HALT_R = crate::FieldReader<u8, HALTSELECT_A>;
#[doc = "Fault A Halt Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum HALTSELECT_A {
    #[doc = "0: Halt action disabled"]
    DISABLE = 0,
    #[doc = "1: Hardware halt action"]
    HW = 1,
    #[doc = "2: Software halt action"]
    SW = 2,
    #[doc = "3: Non-recoverable fault"]
    NR = 3,
}
impl From<HALTSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: HALTSELECT_A) -> Self {
        variant as _
    }
}
impl HALT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HALTSELECT_A {
        match self.bits {
            0 => HALTSELECT_A::DISABLE,
            1 => HALTSELECT_A::HW,
            2 => HALTSELECT_A::SW,
            3 => HALTSELECT_A::NR,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == HALTSELECT_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `HW`"]
    #[inline(always)]
    pub fn is_hw(&self) -> bool {
        *self == HALTSELECT_A::HW
    }
    #[doc = "Checks if the value of the field is `SW`"]
    #[inline(always)]
    pub fn is_sw(&self) -> bool {
        *self == HALTSELECT_A::SW
    }
    #[doc = "Checks if the value of the field is `NR`"]
    #[inline(always)]
    pub fn is_nr(&self) -> bool {
        *self == HALTSELECT_A::NR
    }
}
#[doc = "Field `HALT` writer - Fault A Halt Mode"]
pub type HALT_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, FCTRLA_SPEC, u8, HALTSELECT_A, 2, O>;
impl<'a, const O: u8> HALT_W<'a, O> {
    #[doc = "Halt action disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(HALTSELECT_A::DISABLE)
    }
    #[doc = "Hardware halt action"]
    #[inline(always)]
    pub fn hw(self) -> &'a mut W {
        self.variant(HALTSELECT_A::HW)
    }
    #[doc = "Software halt action"]
    #[inline(always)]
    pub fn sw(self) -> &'a mut W {
        self.variant(HALTSELECT_A::SW)
    }
    #[doc = "Non-recoverable fault"]
    #[inline(always)]
    pub fn nr(self) -> &'a mut W {
        self.variant(HALTSELECT_A::NR)
    }
}
#[doc = "Field `CHSEL` reader - Fault A Capture Channel"]
pub type CHSEL_R = crate::FieldReader<u8, CHSELSELECT_A>;
#[doc = "Fault A Capture Channel\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CHSELSELECT_A {
    #[doc = "0: Capture value stored in channel 0"]
    CC0 = 0,
    #[doc = "1: Capture value stored in channel 1"]
    CC1 = 1,
    #[doc = "2: Capture value stored in channel 2"]
    CC2 = 2,
    #[doc = "3: Capture value stored in channel 3"]
    CC3 = 3,
}
impl From<CHSELSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: CHSELSELECT_A) -> Self {
        variant as _
    }
}
impl CHSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHSELSELECT_A {
        match self.bits {
            0 => CHSELSELECT_A::CC0,
            1 => CHSELSELECT_A::CC1,
            2 => CHSELSELECT_A::CC2,
            3 => CHSELSELECT_A::CC3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CC0`"]
    #[inline(always)]
    pub fn is_cc0(&self) -> bool {
        *self == CHSELSELECT_A::CC0
    }
    #[doc = "Checks if the value of the field is `CC1`"]
    #[inline(always)]
    pub fn is_cc1(&self) -> bool {
        *self == CHSELSELECT_A::CC1
    }
    #[doc = "Checks if the value of the field is `CC2`"]
    #[inline(always)]
    pub fn is_cc2(&self) -> bool {
        *self == CHSELSELECT_A::CC2
    }
    #[doc = "Checks if the value of the field is `CC3`"]
    #[inline(always)]
    pub fn is_cc3(&self) -> bool {
        *self == CHSELSELECT_A::CC3
    }
}
#[doc = "Field `CHSEL` writer - Fault A Capture Channel"]
pub type CHSEL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, FCTRLA_SPEC, u8, CHSELSELECT_A, 2, O>;
impl<'a, const O: u8> CHSEL_W<'a, O> {
    #[doc = "Capture value stored in channel 0"]
    #[inline(always)]
    pub fn cc0(self) -> &'a mut W {
        self.variant(CHSELSELECT_A::CC0)
    }
    #[doc = "Capture value stored in channel 1"]
    #[inline(always)]
    pub fn cc1(self) -> &'a mut W {
        self.variant(CHSELSELECT_A::CC1)
    }
    #[doc = "Capture value stored in channel 2"]
    #[inline(always)]
    pub fn cc2(self) -> &'a mut W {
        self.variant(CHSELSELECT_A::CC2)
    }
    #[doc = "Capture value stored in channel 3"]
    #[inline(always)]
    pub fn cc3(self) -> &'a mut W {
        self.variant(CHSELSELECT_A::CC3)
    }
}
#[doc = "Field `CAPTURE` reader - Fault A Capture Action"]
pub type CAPTURE_R = crate::FieldReader<u8, CAPTURESELECT_A>;
#[doc = "Fault A Capture Action\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CAPTURESELECT_A {
    #[doc = "0: No capture"]
    DISABLE = 0,
    #[doc = "1: Capture on fault"]
    CAPT = 1,
    #[doc = "2: Minimum capture"]
    CAPTMIN = 2,
    #[doc = "3: Maximum capture"]
    CAPTMAX = 3,
    #[doc = "4: Minimum local detection"]
    LOCMIN = 4,
    #[doc = "5: Maximum local detection"]
    LOCMAX = 5,
    #[doc = "6: Minimum and maximum local detection"]
    DERIV0 = 6,
    #[doc = "7: Capture with ramp index as MSB value"]
    CAPTMARK = 7,
}
impl From<CAPTURESELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: CAPTURESELECT_A) -> Self {
        variant as _
    }
}
impl CAPTURE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CAPTURESELECT_A {
        match self.bits {
            0 => CAPTURESELECT_A::DISABLE,
            1 => CAPTURESELECT_A::CAPT,
            2 => CAPTURESELECT_A::CAPTMIN,
            3 => CAPTURESELECT_A::CAPTMAX,
            4 => CAPTURESELECT_A::LOCMIN,
            5 => CAPTURESELECT_A::LOCMAX,
            6 => CAPTURESELECT_A::DERIV0,
            7 => CAPTURESELECT_A::CAPTMARK,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CAPTURESELECT_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `CAPT`"]
    #[inline(always)]
    pub fn is_capt(&self) -> bool {
        *self == CAPTURESELECT_A::CAPT
    }
    #[doc = "Checks if the value of the field is `CAPTMIN`"]
    #[inline(always)]
    pub fn is_captmin(&self) -> bool {
        *self == CAPTURESELECT_A::CAPTMIN
    }
    #[doc = "Checks if the value of the field is `CAPTMAX`"]
    #[inline(always)]
    pub fn is_captmax(&self) -> bool {
        *self == CAPTURESELECT_A::CAPTMAX
    }
    #[doc = "Checks if the value of the field is `LOCMIN`"]
    #[inline(always)]
    pub fn is_locmin(&self) -> bool {
        *self == CAPTURESELECT_A::LOCMIN
    }
    #[doc = "Checks if the value of the field is `LOCMAX`"]
    #[inline(always)]
    pub fn is_locmax(&self) -> bool {
        *self == CAPTURESELECT_A::LOCMAX
    }
    #[doc = "Checks if the value of the field is `DERIV0`"]
    #[inline(always)]
    pub fn is_deriv0(&self) -> bool {
        *self == CAPTURESELECT_A::DERIV0
    }
    #[doc = "Checks if the value of the field is `CAPTMARK`"]
    #[inline(always)]
    pub fn is_captmark(&self) -> bool {
        *self == CAPTURESELECT_A::CAPTMARK
    }
}
#[doc = "Field `CAPTURE` writer - Fault A Capture Action"]
pub type CAPTURE_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, FCTRLA_SPEC, u8, CAPTURESELECT_A, 3, O>;
impl<'a, const O: u8> CAPTURE_W<'a, O> {
    #[doc = "No capture"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CAPTURESELECT_A::DISABLE)
    }
    #[doc = "Capture on fault"]
    #[inline(always)]
    pub fn capt(self) -> &'a mut W {
        self.variant(CAPTURESELECT_A::CAPT)
    }
    #[doc = "Minimum capture"]
    #[inline(always)]
    pub fn captmin(self) -> &'a mut W {
        self.variant(CAPTURESELECT_A::CAPTMIN)
    }
    #[doc = "Maximum capture"]
    #[inline(always)]
    pub fn captmax(self) -> &'a mut W {
        self.variant(CAPTURESELECT_A::CAPTMAX)
    }
    #[doc = "Minimum local detection"]
    #[inline(always)]
    pub fn locmin(self) -> &'a mut W {
        self.variant(CAPTURESELECT_A::LOCMIN)
    }
    #[doc = "Maximum local detection"]
    #[inline(always)]
    pub fn locmax(self) -> &'a mut W {
        self.variant(CAPTURESELECT_A::LOCMAX)
    }
    #[doc = "Minimum and maximum local detection"]
    #[inline(always)]
    pub fn deriv0(self) -> &'a mut W {
        self.variant(CAPTURESELECT_A::DERIV0)
    }
    #[doc = "Capture with ramp index as MSB value"]
    #[inline(always)]
    pub fn captmark(self) -> &'a mut W {
        self.variant(CAPTURESELECT_A::CAPTMARK)
    }
}
#[doc = "Field `BLANKPRESC` reader - Fault A Blanking Prescaler"]
pub type BLANKPRESC_R = crate::BitReader<bool>;
#[doc = "Field `BLANKPRESC` writer - Fault A Blanking Prescaler"]
pub type BLANKPRESC_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCTRLA_SPEC, bool, O>;
#[doc = "Field `BLANKVAL` reader - Fault A Blanking Time"]
pub type BLANKVAL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BLANKVAL` writer - Fault A Blanking Time"]
pub type BLANKVAL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FCTRLA_SPEC, u8, u8, 8, O>;
#[doc = "Field `FILTERVAL` reader - Fault A Filter Value"]
pub type FILTERVAL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FILTERVAL` writer - Fault A Filter Value"]
pub type FILTERVAL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FCTRLA_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:1 - Fault A Source"]
    #[inline(always)]
    pub fn src(&self) -> SRC_R {
        SRC_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 3 - Fault A Keeper"]
    #[inline(always)]
    pub fn keep(&self) -> KEEP_R {
        KEEP_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Fault A Qualification"]
    #[inline(always)]
    pub fn qual(&self) -> QUAL_R {
        QUAL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - Fault A Blanking Mode"]
    #[inline(always)]
    pub fn blank(&self) -> BLANK_R {
        BLANK_R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - Fault A Restart"]
    #[inline(always)]
    pub fn restart(&self) -> RESTART_R {
        RESTART_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Fault A Halt Mode"]
    #[inline(always)]
    pub fn halt(&self) -> HALT_R {
        HALT_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Fault A Capture Channel"]
    #[inline(always)]
    pub fn chsel(&self) -> CHSEL_R {
        CHSEL_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:14 - Fault A Capture Action"]
    #[inline(always)]
    pub fn capture(&self) -> CAPTURE_R {
        CAPTURE_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - Fault A Blanking Prescaler"]
    #[inline(always)]
    pub fn blankpresc(&self) -> BLANKPRESC_R {
        BLANKPRESC_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:23 - Fault A Blanking Time"]
    #[inline(always)]
    pub fn blankval(&self) -> BLANKVAL_R {
        BLANKVAL_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:27 - Fault A Filter Value"]
    #[inline(always)]
    pub fn filterval(&self) -> FILTERVAL_R {
        FILTERVAL_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Fault A Source"]
    #[inline(always)]
    #[must_use]
    pub fn src(&mut self) -> SRC_W<0> {
        SRC_W::new(self)
    }
    #[doc = "Bit 3 - Fault A Keeper"]
    #[inline(always)]
    #[must_use]
    pub fn keep(&mut self) -> KEEP_W<3> {
        KEEP_W::new(self)
    }
    #[doc = "Bit 4 - Fault A Qualification"]
    #[inline(always)]
    #[must_use]
    pub fn qual(&mut self) -> QUAL_W<4> {
        QUAL_W::new(self)
    }
    #[doc = "Bits 5:6 - Fault A Blanking Mode"]
    #[inline(always)]
    #[must_use]
    pub fn blank(&mut self) -> BLANK_W<5> {
        BLANK_W::new(self)
    }
    #[doc = "Bit 7 - Fault A Restart"]
    #[inline(always)]
    #[must_use]
    pub fn restart(&mut self) -> RESTART_W<7> {
        RESTART_W::new(self)
    }
    #[doc = "Bits 8:9 - Fault A Halt Mode"]
    #[inline(always)]
    #[must_use]
    pub fn halt(&mut self) -> HALT_W<8> {
        HALT_W::new(self)
    }
    #[doc = "Bits 10:11 - Fault A Capture Channel"]
    #[inline(always)]
    #[must_use]
    pub fn chsel(&mut self) -> CHSEL_W<10> {
        CHSEL_W::new(self)
    }
    #[doc = "Bits 12:14 - Fault A Capture Action"]
    #[inline(always)]
    #[must_use]
    pub fn capture(&mut self) -> CAPTURE_W<12> {
        CAPTURE_W::new(self)
    }
    #[doc = "Bit 15 - Fault A Blanking Prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn blankpresc(&mut self) -> BLANKPRESC_W<15> {
        BLANKPRESC_W::new(self)
    }
    #[doc = "Bits 16:23 - Fault A Blanking Time"]
    #[inline(always)]
    #[must_use]
    pub fn blankval(&mut self) -> BLANKVAL_W<16> {
        BLANKVAL_W::new(self)
    }
    #[doc = "Bits 24:27 - Fault A Filter Value"]
    #[inline(always)]
    #[must_use]
    pub fn filterval(&mut self) -> FILTERVAL_W<24> {
        FILTERVAL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Recoverable Fault A Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fctrla](index.html) module"]
pub struct FCTRLA_SPEC;
impl crate::RegisterSpec for FCTRLA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fctrla::R](R) reader structure"]
impl crate::Readable for FCTRLA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fctrla::W](W) writer structure"]
impl crate::Writable for FCTRLA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FCTRLA to value 0"]
impl crate::Resettable for FCTRLA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
