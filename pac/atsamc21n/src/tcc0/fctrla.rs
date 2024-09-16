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
#[doc = "Fault A Source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SRC_A {
    #[doc = "0: Fault input disabled"]
    DISABLE = 0,
    #[doc = "1: MCEx (x=0,1) event input"]
    ENABLE = 1,
    #[doc = "2: Inverted MCEx (x=0,1) event input"]
    INVERT = 2,
    #[doc = "3: Alternate fault (A or B) state at the end of the previous period"]
    ALTFAULT = 3,
}
impl From<SRC_A> for u8 {
    #[inline(always)]
    fn from(variant: SRC_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SRC` reader - Fault A Source"]
pub struct SRC_R(crate::FieldReader<u8, SRC_A>);
impl SRC_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SRC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRC_A {
        match self.bits {
            0 => SRC_A::DISABLE,
            1 => SRC_A::ENABLE,
            2 => SRC_A::INVERT,
            3 => SRC_A::ALTFAULT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == SRC_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == SRC_A::ENABLE
    }
    #[doc = "Checks if the value of the field is `INVERT`"]
    #[inline(always)]
    pub fn is_invert(&self) -> bool {
        **self == SRC_A::INVERT
    }
    #[doc = "Checks if the value of the field is `ALTFAULT`"]
    #[inline(always)]
    pub fn is_altfault(&self) -> bool {
        **self == SRC_A::ALTFAULT
    }
}
impl core::ops::Deref for SRC_R {
    type Target = crate::FieldReader<u8, SRC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SRC` writer - Fault A Source"]
pub struct SRC_W<'a> {
    w: &'a mut W,
}
impl<'a> SRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SRC_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Fault input disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(SRC_A::DISABLE)
    }
    #[doc = "MCEx (x=0,1) event input"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(SRC_A::ENABLE)
    }
    #[doc = "Inverted MCEx (x=0,1) event input"]
    #[inline(always)]
    pub fn invert(self) -> &'a mut W {
        self.variant(SRC_A::INVERT)
    }
    #[doc = "Alternate fault (A or B) state at the end of the previous period"]
    #[inline(always)]
    pub fn altfault(self) -> &'a mut W {
        self.variant(SRC_A::ALTFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "Field `KEEP` reader - Fault A Keeper"]
pub struct KEEP_R(crate::FieldReader<bool, bool>);
impl KEEP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        KEEP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for KEEP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `KEEP` writer - Fault A Keeper"]
pub struct KEEP_W<'a> {
    w: &'a mut W,
}
impl<'a> KEEP_W<'a> {
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
#[doc = "Field `QUAL` reader - Fault A Qualification"]
pub struct QUAL_R(crate::FieldReader<bool, bool>);
impl QUAL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        QUAL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for QUAL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `QUAL` writer - Fault A Qualification"]
pub struct QUAL_W<'a> {
    w: &'a mut W,
}
impl<'a> QUAL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Fault A Blanking Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum BLANK_A {
    #[doc = "0: Blanking applied from start of the ramp"]
    START = 0,
    #[doc = "1: Blanking applied from rising edge of the output waveform"]
    RISE = 1,
    #[doc = "2: Blanking applied from falling edge of the output waveform"]
    FALL = 2,
    #[doc = "3: Blanking applied from each toggle of the output waveform"]
    BOTH = 3,
}
impl From<BLANK_A> for u8 {
    #[inline(always)]
    fn from(variant: BLANK_A) -> Self {
        variant as _
    }
}
#[doc = "Field `BLANK` reader - Fault A Blanking Mode"]
pub struct BLANK_R(crate::FieldReader<u8, BLANK_A>);
impl BLANK_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        BLANK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BLANK_A {
        match self.bits {
            0 => BLANK_A::START,
            1 => BLANK_A::RISE,
            2 => BLANK_A::FALL,
            3 => BLANK_A::BOTH,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `START`"]
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        **self == BLANK_A::START
    }
    #[doc = "Checks if the value of the field is `RISE`"]
    #[inline(always)]
    pub fn is_rise(&self) -> bool {
        **self == BLANK_A::RISE
    }
    #[doc = "Checks if the value of the field is `FALL`"]
    #[inline(always)]
    pub fn is_fall(&self) -> bool {
        **self == BLANK_A::FALL
    }
    #[doc = "Checks if the value of the field is `BOTH`"]
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        **self == BLANK_A::BOTH
    }
}
impl core::ops::Deref for BLANK_R {
    type Target = crate::FieldReader<u8, BLANK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BLANK` writer - Fault A Blanking Mode"]
pub struct BLANK_W<'a> {
    w: &'a mut W,
}
impl<'a> BLANK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BLANK_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Blanking applied from start of the ramp"]
    #[inline(always)]
    pub fn start(self) -> &'a mut W {
        self.variant(BLANK_A::START)
    }
    #[doc = "Blanking applied from rising edge of the output waveform"]
    #[inline(always)]
    pub fn rise(self) -> &'a mut W {
        self.variant(BLANK_A::RISE)
    }
    #[doc = "Blanking applied from falling edge of the output waveform"]
    #[inline(always)]
    pub fn fall(self) -> &'a mut W {
        self.variant(BLANK_A::FALL)
    }
    #[doc = "Blanking applied from each toggle of the output waveform"]
    #[inline(always)]
    pub fn both(self) -> &'a mut W {
        self.variant(BLANK_A::BOTH)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 5)) | ((value as u32 & 0x03) << 5);
        self.w
    }
}
#[doc = "Field `RESTART` reader - Fault A Restart"]
pub struct RESTART_R(crate::FieldReader<bool, bool>);
impl RESTART_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RESTART_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESTART_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESTART` writer - Fault A Restart"]
pub struct RESTART_W<'a> {
    w: &'a mut W,
}
impl<'a> RESTART_W<'a> {
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
#[doc = "Fault A Halt Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum HALT_A {
    #[doc = "0: Halt action disabled"]
    DISABLE = 0,
    #[doc = "1: Hardware halt action"]
    HW = 1,
    #[doc = "2: Software halt action"]
    SW = 2,
    #[doc = "3: Non-recoverable fault"]
    NR = 3,
}
impl From<HALT_A> for u8 {
    #[inline(always)]
    fn from(variant: HALT_A) -> Self {
        variant as _
    }
}
#[doc = "Field `HALT` reader - Fault A Halt Mode"]
pub struct HALT_R(crate::FieldReader<u8, HALT_A>);
impl HALT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        HALT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HALT_A {
        match self.bits {
            0 => HALT_A::DISABLE,
            1 => HALT_A::HW,
            2 => HALT_A::SW,
            3 => HALT_A::NR,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == HALT_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `HW`"]
    #[inline(always)]
    pub fn is_hw(&self) -> bool {
        **self == HALT_A::HW
    }
    #[doc = "Checks if the value of the field is `SW`"]
    #[inline(always)]
    pub fn is_sw(&self) -> bool {
        **self == HALT_A::SW
    }
    #[doc = "Checks if the value of the field is `NR`"]
    #[inline(always)]
    pub fn is_nr(&self) -> bool {
        **self == HALT_A::NR
    }
}
impl core::ops::Deref for HALT_R {
    type Target = crate::FieldReader<u8, HALT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HALT` writer - Fault A Halt Mode"]
pub struct HALT_W<'a> {
    w: &'a mut W,
}
impl<'a> HALT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HALT_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Halt action disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(HALT_A::DISABLE)
    }
    #[doc = "Hardware halt action"]
    #[inline(always)]
    pub fn hw(self) -> &'a mut W {
        self.variant(HALT_A::HW)
    }
    #[doc = "Software halt action"]
    #[inline(always)]
    pub fn sw(self) -> &'a mut W {
        self.variant(HALT_A::SW)
    }
    #[doc = "Non-recoverable fault"]
    #[inline(always)]
    pub fn nr(self) -> &'a mut W {
        self.variant(HALT_A::NR)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | ((value as u32 & 0x03) << 8);
        self.w
    }
}
#[doc = "Fault A Capture Channel\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CHSEL_A {
    #[doc = "0: Capture value stored in channel 0"]
    CC0 = 0,
    #[doc = "1: Capture value stored in channel 1"]
    CC1 = 1,
    #[doc = "2: Capture value stored in channel 2"]
    CC2 = 2,
    #[doc = "3: Capture value stored in channel 3"]
    CC3 = 3,
}
impl From<CHSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: CHSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CHSEL` reader - Fault A Capture Channel"]
pub struct CHSEL_R(crate::FieldReader<u8, CHSEL_A>);
impl CHSEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CHSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHSEL_A {
        match self.bits {
            0 => CHSEL_A::CC0,
            1 => CHSEL_A::CC1,
            2 => CHSEL_A::CC2,
            3 => CHSEL_A::CC3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CC0`"]
    #[inline(always)]
    pub fn is_cc0(&self) -> bool {
        **self == CHSEL_A::CC0
    }
    #[doc = "Checks if the value of the field is `CC1`"]
    #[inline(always)]
    pub fn is_cc1(&self) -> bool {
        **self == CHSEL_A::CC1
    }
    #[doc = "Checks if the value of the field is `CC2`"]
    #[inline(always)]
    pub fn is_cc2(&self) -> bool {
        **self == CHSEL_A::CC2
    }
    #[doc = "Checks if the value of the field is `CC3`"]
    #[inline(always)]
    pub fn is_cc3(&self) -> bool {
        **self == CHSEL_A::CC3
    }
}
impl core::ops::Deref for CHSEL_R {
    type Target = crate::FieldReader<u8, CHSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHSEL` writer - Fault A Capture Channel"]
pub struct CHSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CHSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHSEL_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Capture value stored in channel 0"]
    #[inline(always)]
    pub fn cc0(self) -> &'a mut W {
        self.variant(CHSEL_A::CC0)
    }
    #[doc = "Capture value stored in channel 1"]
    #[inline(always)]
    pub fn cc1(self) -> &'a mut W {
        self.variant(CHSEL_A::CC1)
    }
    #[doc = "Capture value stored in channel 2"]
    #[inline(always)]
    pub fn cc2(self) -> &'a mut W {
        self.variant(CHSEL_A::CC2)
    }
    #[doc = "Capture value stored in channel 3"]
    #[inline(always)]
    pub fn cc3(self) -> &'a mut W {
        self.variant(CHSEL_A::CC3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | ((value as u32 & 0x03) << 10);
        self.w
    }
}
#[doc = "Fault A Capture Action\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CAPTURE_A {
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
impl From<CAPTURE_A> for u8 {
    #[inline(always)]
    fn from(variant: CAPTURE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CAPTURE` reader - Fault A Capture Action"]
pub struct CAPTURE_R(crate::FieldReader<u8, CAPTURE_A>);
impl CAPTURE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CAPTURE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CAPTURE_A {
        match self.bits {
            0 => CAPTURE_A::DISABLE,
            1 => CAPTURE_A::CAPT,
            2 => CAPTURE_A::CAPTMIN,
            3 => CAPTURE_A::CAPTMAX,
            4 => CAPTURE_A::LOCMIN,
            5 => CAPTURE_A::LOCMAX,
            6 => CAPTURE_A::DERIV0,
            7 => CAPTURE_A::CAPTMARK,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == CAPTURE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `CAPT`"]
    #[inline(always)]
    pub fn is_capt(&self) -> bool {
        **self == CAPTURE_A::CAPT
    }
    #[doc = "Checks if the value of the field is `CAPTMIN`"]
    #[inline(always)]
    pub fn is_captmin(&self) -> bool {
        **self == CAPTURE_A::CAPTMIN
    }
    #[doc = "Checks if the value of the field is `CAPTMAX`"]
    #[inline(always)]
    pub fn is_captmax(&self) -> bool {
        **self == CAPTURE_A::CAPTMAX
    }
    #[doc = "Checks if the value of the field is `LOCMIN`"]
    #[inline(always)]
    pub fn is_locmin(&self) -> bool {
        **self == CAPTURE_A::LOCMIN
    }
    #[doc = "Checks if the value of the field is `LOCMAX`"]
    #[inline(always)]
    pub fn is_locmax(&self) -> bool {
        **self == CAPTURE_A::LOCMAX
    }
    #[doc = "Checks if the value of the field is `DERIV0`"]
    #[inline(always)]
    pub fn is_deriv0(&self) -> bool {
        **self == CAPTURE_A::DERIV0
    }
    #[doc = "Checks if the value of the field is `CAPTMARK`"]
    #[inline(always)]
    pub fn is_captmark(&self) -> bool {
        **self == CAPTURE_A::CAPTMARK
    }
}
impl core::ops::Deref for CAPTURE_R {
    type Target = crate::FieldReader<u8, CAPTURE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAPTURE` writer - Fault A Capture Action"]
pub struct CAPTURE_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPTURE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CAPTURE_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "No capture"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CAPTURE_A::DISABLE)
    }
    #[doc = "Capture on fault"]
    #[inline(always)]
    pub fn capt(self) -> &'a mut W {
        self.variant(CAPTURE_A::CAPT)
    }
    #[doc = "Minimum capture"]
    #[inline(always)]
    pub fn captmin(self) -> &'a mut W {
        self.variant(CAPTURE_A::CAPTMIN)
    }
    #[doc = "Maximum capture"]
    #[inline(always)]
    pub fn captmax(self) -> &'a mut W {
        self.variant(CAPTURE_A::CAPTMAX)
    }
    #[doc = "Minimum local detection"]
    #[inline(always)]
    pub fn locmin(self) -> &'a mut W {
        self.variant(CAPTURE_A::LOCMIN)
    }
    #[doc = "Maximum local detection"]
    #[inline(always)]
    pub fn locmax(self) -> &'a mut W {
        self.variant(CAPTURE_A::LOCMAX)
    }
    #[doc = "Minimum and maximum local detection"]
    #[inline(always)]
    pub fn deriv0(self) -> &'a mut W {
        self.variant(CAPTURE_A::DERIV0)
    }
    #[doc = "Capture with ramp index as MSB value"]
    #[inline(always)]
    pub fn captmark(self) -> &'a mut W {
        self.variant(CAPTURE_A::CAPTMARK)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 12)) | ((value as u32 & 0x07) << 12);
        self.w
    }
}
#[doc = "Field `BLANKPRESC` reader - Fault A Blanking Prescaler"]
pub struct BLANKPRESC_R(crate::FieldReader<bool, bool>);
impl BLANKPRESC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BLANKPRESC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BLANKPRESC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BLANKPRESC` writer - Fault A Blanking Prescaler"]
pub struct BLANKPRESC_W<'a> {
    w: &'a mut W,
}
impl<'a> BLANKPRESC_W<'a> {
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
#[doc = "Field `BLANKVAL` reader - Fault A Blanking Time"]
pub struct BLANKVAL_R(crate::FieldReader<u8, u8>);
impl BLANKVAL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        BLANKVAL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BLANKVAL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BLANKVAL` writer - Fault A Blanking Time"]
pub struct BLANKVAL_W<'a> {
    w: &'a mut W,
}
impl<'a> BLANKVAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
#[doc = "Field `FILTERVAL` reader - Fault A Filter Value"]
pub struct FILTERVAL_R(crate::FieldReader<u8, u8>);
impl FILTERVAL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        FILTERVAL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FILTERVAL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FILTERVAL` writer - Fault A Filter Value"]
pub struct FILTERVAL_W<'a> {
    w: &'a mut W,
}
impl<'a> FILTERVAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | ((value as u32 & 0x0f) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Fault A Source"]
    #[inline(always)]
    pub fn src(&self) -> SRC_R {
        SRC_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 3 - Fault A Keeper"]
    #[inline(always)]
    pub fn keep(&self) -> KEEP_R {
        KEEP_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Fault A Qualification"]
    #[inline(always)]
    pub fn qual(&self) -> QUAL_R {
        QUAL_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 5:6 - Fault A Blanking Mode"]
    #[inline(always)]
    pub fn blank(&self) -> BLANK_R {
        BLANK_R::new(((self.bits >> 5) & 0x03) as u8)
    }
    #[doc = "Bit 7 - Fault A Restart"]
    #[inline(always)]
    pub fn restart(&self) -> RESTART_R {
        RESTART_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:9 - Fault A Halt Mode"]
    #[inline(always)]
    pub fn halt(&self) -> HALT_R {
        HALT_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 10:11 - Fault A Capture Channel"]
    #[inline(always)]
    pub fn chsel(&self) -> CHSEL_R {
        CHSEL_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 12:14 - Fault A Capture Action"]
    #[inline(always)]
    pub fn capture(&self) -> CAPTURE_R {
        CAPTURE_R::new(((self.bits >> 12) & 0x07) as u8)
    }
    #[doc = "Bit 15 - Fault A Blanking Prescaler"]
    #[inline(always)]
    pub fn blankpresc(&self) -> BLANKPRESC_R {
        BLANKPRESC_R::new(((self.bits >> 15) & 0x01) != 0)
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
    pub fn src(&mut self) -> SRC_W {
        SRC_W { w: self }
    }
    #[doc = "Bit 3 - Fault A Keeper"]
    #[inline(always)]
    pub fn keep(&mut self) -> KEEP_W {
        KEEP_W { w: self }
    }
    #[doc = "Bit 4 - Fault A Qualification"]
    #[inline(always)]
    pub fn qual(&mut self) -> QUAL_W {
        QUAL_W { w: self }
    }
    #[doc = "Bits 5:6 - Fault A Blanking Mode"]
    #[inline(always)]
    pub fn blank(&mut self) -> BLANK_W {
        BLANK_W { w: self }
    }
    #[doc = "Bit 7 - Fault A Restart"]
    #[inline(always)]
    pub fn restart(&mut self) -> RESTART_W {
        RESTART_W { w: self }
    }
    #[doc = "Bits 8:9 - Fault A Halt Mode"]
    #[inline(always)]
    pub fn halt(&mut self) -> HALT_W {
        HALT_W { w: self }
    }
    #[doc = "Bits 10:11 - Fault A Capture Channel"]
    #[inline(always)]
    pub fn chsel(&mut self) -> CHSEL_W {
        CHSEL_W { w: self }
    }
    #[doc = "Bits 12:14 - Fault A Capture Action"]
    #[inline(always)]
    pub fn capture(&mut self) -> CAPTURE_W {
        CAPTURE_W { w: self }
    }
    #[doc = "Bit 15 - Fault A Blanking Prescaler"]
    #[inline(always)]
    pub fn blankpresc(&mut self) -> BLANKPRESC_W {
        BLANKPRESC_W { w: self }
    }
    #[doc = "Bits 16:23 - Fault A Blanking Time"]
    #[inline(always)]
    pub fn blankval(&mut self) -> BLANKVAL_W {
        BLANKVAL_W { w: self }
    }
    #[doc = "Bits 24:27 - Fault A Filter Value"]
    #[inline(always)]
    pub fn filterval(&mut self) -> FILTERVAL_W {
        FILTERVAL_W { w: self }
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
}
#[doc = "`reset()` method sets FCTRLA to value 0"]
impl crate::Resettable for FCTRLA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
