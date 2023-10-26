#[doc = "Register `FCTRLB` reader"]
pub type R = crate::R<FCTRLB_SPEC>;
#[doc = "Register `FCTRLB` writer"]
pub type W = crate::W<FCTRLB_SPEC>;
#[doc = "Field `SRC` reader - Fault B Source"]
pub type SRC_R = crate::FieldReader<SRCSELECT_A>;
#[doc = "Fault B Source\n\nValue on reset: 0"]
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
impl crate::FieldSpec for SRCSELECT_A {
    type Ux = u8;
}
impl SRC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SRCSELECT_A {
        match self.bits {
            0 => SRCSELECT_A::DISABLE,
            1 => SRCSELECT_A::ENABLE,
            2 => SRCSELECT_A::INVERT,
            3 => SRCSELECT_A::ALTFAULT,
            _ => unreachable!(),
        }
    }
    #[doc = "Fault input disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == SRCSELECT_A::DISABLE
    }
    #[doc = "MCEx (x=0,1) event input"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == SRCSELECT_A::ENABLE
    }
    #[doc = "Inverted MCEx (x=0,1) event input"]
    #[inline(always)]
    pub fn is_invert(&self) -> bool {
        *self == SRCSELECT_A::INVERT
    }
    #[doc = "Alternate fault (A or B) state at the end of the previous period"]
    #[inline(always)]
    pub fn is_altfault(&self) -> bool {
        *self == SRCSELECT_A::ALTFAULT
    }
}
#[doc = "Field `SRC` writer - Fault B Source"]
pub type SRC_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, SRCSELECT_A>;
impl<'a, REG, const O: u8> SRC_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Fault input disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(SRCSELECT_A::DISABLE)
    }
    #[doc = "MCEx (x=0,1) event input"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(SRCSELECT_A::ENABLE)
    }
    #[doc = "Inverted MCEx (x=0,1) event input"]
    #[inline(always)]
    pub fn invert(self) -> &'a mut crate::W<REG> {
        self.variant(SRCSELECT_A::INVERT)
    }
    #[doc = "Alternate fault (A or B) state at the end of the previous period"]
    #[inline(always)]
    pub fn altfault(self) -> &'a mut crate::W<REG> {
        self.variant(SRCSELECT_A::ALTFAULT)
    }
}
#[doc = "Field `KEEP` reader - Fault B Keeper"]
pub type KEEP_R = crate::BitReader;
#[doc = "Field `KEEP` writer - Fault B Keeper"]
pub type KEEP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `QUAL` reader - Fault B Qualification"]
pub type QUAL_R = crate::BitReader;
#[doc = "Field `QUAL` writer - Fault B Qualification"]
pub type QUAL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BLANK` reader - Fault B Blanking Mode"]
pub type BLANK_R = crate::FieldReader<BLANKSELECT_A>;
#[doc = "Fault B Blanking Mode\n\nValue on reset: 0"]
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
impl crate::FieldSpec for BLANKSELECT_A {
    type Ux = u8;
}
impl BLANK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BLANKSELECT_A {
        match self.bits {
            0 => BLANKSELECT_A::START,
            1 => BLANKSELECT_A::RISE,
            2 => BLANKSELECT_A::FALL,
            3 => BLANKSELECT_A::BOTH,
            _ => unreachable!(),
        }
    }
    #[doc = "Blanking applied from start of the ramp"]
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        *self == BLANKSELECT_A::START
    }
    #[doc = "Blanking applied from rising edge of the output waveform"]
    #[inline(always)]
    pub fn is_rise(&self) -> bool {
        *self == BLANKSELECT_A::RISE
    }
    #[doc = "Blanking applied from falling edge of the output waveform"]
    #[inline(always)]
    pub fn is_fall(&self) -> bool {
        *self == BLANKSELECT_A::FALL
    }
    #[doc = "Blanking applied from each toggle of the output waveform"]
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        *self == BLANKSELECT_A::BOTH
    }
}
#[doc = "Field `BLANK` writer - Fault B Blanking Mode"]
pub type BLANK_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, BLANKSELECT_A>;
impl<'a, REG, const O: u8> BLANK_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Blanking applied from start of the ramp"]
    #[inline(always)]
    pub fn start(self) -> &'a mut crate::W<REG> {
        self.variant(BLANKSELECT_A::START)
    }
    #[doc = "Blanking applied from rising edge of the output waveform"]
    #[inline(always)]
    pub fn rise(self) -> &'a mut crate::W<REG> {
        self.variant(BLANKSELECT_A::RISE)
    }
    #[doc = "Blanking applied from falling edge of the output waveform"]
    #[inline(always)]
    pub fn fall(self) -> &'a mut crate::W<REG> {
        self.variant(BLANKSELECT_A::FALL)
    }
    #[doc = "Blanking applied from each toggle of the output waveform"]
    #[inline(always)]
    pub fn both(self) -> &'a mut crate::W<REG> {
        self.variant(BLANKSELECT_A::BOTH)
    }
}
#[doc = "Field `RESTART` reader - Fault B Restart"]
pub type RESTART_R = crate::BitReader;
#[doc = "Field `RESTART` writer - Fault B Restart"]
pub type RESTART_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `HALT` reader - Fault B Halt Mode"]
pub type HALT_R = crate::FieldReader<HALTSELECT_A>;
#[doc = "Fault B Halt Mode\n\nValue on reset: 0"]
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
impl crate::FieldSpec for HALTSELECT_A {
    type Ux = u8;
}
impl HALT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HALTSELECT_A {
        match self.bits {
            0 => HALTSELECT_A::DISABLE,
            1 => HALTSELECT_A::HW,
            2 => HALTSELECT_A::SW,
            3 => HALTSELECT_A::NR,
            _ => unreachable!(),
        }
    }
    #[doc = "Halt action disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == HALTSELECT_A::DISABLE
    }
    #[doc = "Hardware halt action"]
    #[inline(always)]
    pub fn is_hw(&self) -> bool {
        *self == HALTSELECT_A::HW
    }
    #[doc = "Software halt action"]
    #[inline(always)]
    pub fn is_sw(&self) -> bool {
        *self == HALTSELECT_A::SW
    }
    #[doc = "Non-recoverable fault"]
    #[inline(always)]
    pub fn is_nr(&self) -> bool {
        *self == HALTSELECT_A::NR
    }
}
#[doc = "Field `HALT` writer - Fault B Halt Mode"]
pub type HALT_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, HALTSELECT_A>;
impl<'a, REG, const O: u8> HALT_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Halt action disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(HALTSELECT_A::DISABLE)
    }
    #[doc = "Hardware halt action"]
    #[inline(always)]
    pub fn hw(self) -> &'a mut crate::W<REG> {
        self.variant(HALTSELECT_A::HW)
    }
    #[doc = "Software halt action"]
    #[inline(always)]
    pub fn sw(self) -> &'a mut crate::W<REG> {
        self.variant(HALTSELECT_A::SW)
    }
    #[doc = "Non-recoverable fault"]
    #[inline(always)]
    pub fn nr(self) -> &'a mut crate::W<REG> {
        self.variant(HALTSELECT_A::NR)
    }
}
#[doc = "Field `CHSEL` reader - Fault B Capture Channel"]
pub type CHSEL_R = crate::FieldReader<CHSELSELECT_A>;
#[doc = "Fault B Capture Channel\n\nValue on reset: 0"]
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
impl crate::FieldSpec for CHSELSELECT_A {
    type Ux = u8;
}
impl CHSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CHSELSELECT_A {
        match self.bits {
            0 => CHSELSELECT_A::CC0,
            1 => CHSELSELECT_A::CC1,
            2 => CHSELSELECT_A::CC2,
            3 => CHSELSELECT_A::CC3,
            _ => unreachable!(),
        }
    }
    #[doc = "Capture value stored in channel 0"]
    #[inline(always)]
    pub fn is_cc0(&self) -> bool {
        *self == CHSELSELECT_A::CC0
    }
    #[doc = "Capture value stored in channel 1"]
    #[inline(always)]
    pub fn is_cc1(&self) -> bool {
        *self == CHSELSELECT_A::CC1
    }
    #[doc = "Capture value stored in channel 2"]
    #[inline(always)]
    pub fn is_cc2(&self) -> bool {
        *self == CHSELSELECT_A::CC2
    }
    #[doc = "Capture value stored in channel 3"]
    #[inline(always)]
    pub fn is_cc3(&self) -> bool {
        *self == CHSELSELECT_A::CC3
    }
}
#[doc = "Field `CHSEL` writer - Fault B Capture Channel"]
pub type CHSEL_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, CHSELSELECT_A>;
impl<'a, REG, const O: u8> CHSEL_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Capture value stored in channel 0"]
    #[inline(always)]
    pub fn cc0(self) -> &'a mut crate::W<REG> {
        self.variant(CHSELSELECT_A::CC0)
    }
    #[doc = "Capture value stored in channel 1"]
    #[inline(always)]
    pub fn cc1(self) -> &'a mut crate::W<REG> {
        self.variant(CHSELSELECT_A::CC1)
    }
    #[doc = "Capture value stored in channel 2"]
    #[inline(always)]
    pub fn cc2(self) -> &'a mut crate::W<REG> {
        self.variant(CHSELSELECT_A::CC2)
    }
    #[doc = "Capture value stored in channel 3"]
    #[inline(always)]
    pub fn cc3(self) -> &'a mut crate::W<REG> {
        self.variant(CHSELSELECT_A::CC3)
    }
}
#[doc = "Field `CAPTURE` reader - Fault B Capture Action"]
pub type CAPTURE_R = crate::FieldReader<CAPTURESELECT_A>;
#[doc = "Fault B Capture Action\n\nValue on reset: 0"]
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
impl crate::FieldSpec for CAPTURESELECT_A {
    type Ux = u8;
}
impl CAPTURE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CAPTURESELECT_A {
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
    #[doc = "No capture"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CAPTURESELECT_A::DISABLE
    }
    #[doc = "Capture on fault"]
    #[inline(always)]
    pub fn is_capt(&self) -> bool {
        *self == CAPTURESELECT_A::CAPT
    }
    #[doc = "Minimum capture"]
    #[inline(always)]
    pub fn is_captmin(&self) -> bool {
        *self == CAPTURESELECT_A::CAPTMIN
    }
    #[doc = "Maximum capture"]
    #[inline(always)]
    pub fn is_captmax(&self) -> bool {
        *self == CAPTURESELECT_A::CAPTMAX
    }
    #[doc = "Minimum local detection"]
    #[inline(always)]
    pub fn is_locmin(&self) -> bool {
        *self == CAPTURESELECT_A::LOCMIN
    }
    #[doc = "Maximum local detection"]
    #[inline(always)]
    pub fn is_locmax(&self) -> bool {
        *self == CAPTURESELECT_A::LOCMAX
    }
    #[doc = "Minimum and maximum local detection"]
    #[inline(always)]
    pub fn is_deriv0(&self) -> bool {
        *self == CAPTURESELECT_A::DERIV0
    }
    #[doc = "Capture with ramp index as MSB value"]
    #[inline(always)]
    pub fn is_captmark(&self) -> bool {
        *self == CAPTURESELECT_A::CAPTMARK
    }
}
#[doc = "Field `CAPTURE` writer - Fault B Capture Action"]
pub type CAPTURE_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 3, O, CAPTURESELECT_A>;
impl<'a, REG, const O: u8> CAPTURE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No capture"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(CAPTURESELECT_A::DISABLE)
    }
    #[doc = "Capture on fault"]
    #[inline(always)]
    pub fn capt(self) -> &'a mut crate::W<REG> {
        self.variant(CAPTURESELECT_A::CAPT)
    }
    #[doc = "Minimum capture"]
    #[inline(always)]
    pub fn captmin(self) -> &'a mut crate::W<REG> {
        self.variant(CAPTURESELECT_A::CAPTMIN)
    }
    #[doc = "Maximum capture"]
    #[inline(always)]
    pub fn captmax(self) -> &'a mut crate::W<REG> {
        self.variant(CAPTURESELECT_A::CAPTMAX)
    }
    #[doc = "Minimum local detection"]
    #[inline(always)]
    pub fn locmin(self) -> &'a mut crate::W<REG> {
        self.variant(CAPTURESELECT_A::LOCMIN)
    }
    #[doc = "Maximum local detection"]
    #[inline(always)]
    pub fn locmax(self) -> &'a mut crate::W<REG> {
        self.variant(CAPTURESELECT_A::LOCMAX)
    }
    #[doc = "Minimum and maximum local detection"]
    #[inline(always)]
    pub fn deriv0(self) -> &'a mut crate::W<REG> {
        self.variant(CAPTURESELECT_A::DERIV0)
    }
    #[doc = "Capture with ramp index as MSB value"]
    #[inline(always)]
    pub fn captmark(self) -> &'a mut crate::W<REG> {
        self.variant(CAPTURESELECT_A::CAPTMARK)
    }
}
#[doc = "Field `BLANKPRESC` reader - Fault B Blanking Prescaler"]
pub type BLANKPRESC_R = crate::BitReader;
#[doc = "Field `BLANKPRESC` writer - Fault B Blanking Prescaler"]
pub type BLANKPRESC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BLANKVAL` reader - Fault B Blanking Time"]
pub type BLANKVAL_R = crate::FieldReader;
#[doc = "Field `BLANKVAL` writer - Fault B Blanking Time"]
pub type BLANKVAL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `FILTERVAL` reader - Fault B Filter Value"]
pub type FILTERVAL_R = crate::FieldReader;
#[doc = "Field `FILTERVAL` writer - Fault B Filter Value"]
pub type FILTERVAL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
impl R {
    #[doc = "Bits 0:1 - Fault B Source"]
    #[inline(always)]
    pub fn src(&self) -> SRC_R {
        SRC_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 3 - Fault B Keeper"]
    #[inline(always)]
    pub fn keep(&self) -> KEEP_R {
        KEEP_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Fault B Qualification"]
    #[inline(always)]
    pub fn qual(&self) -> QUAL_R {
        QUAL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - Fault B Blanking Mode"]
    #[inline(always)]
    pub fn blank(&self) -> BLANK_R {
        BLANK_R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - Fault B Restart"]
    #[inline(always)]
    pub fn restart(&self) -> RESTART_R {
        RESTART_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Fault B Halt Mode"]
    #[inline(always)]
    pub fn halt(&self) -> HALT_R {
        HALT_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Fault B Capture Channel"]
    #[inline(always)]
    pub fn chsel(&self) -> CHSEL_R {
        CHSEL_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:14 - Fault B Capture Action"]
    #[inline(always)]
    pub fn capture(&self) -> CAPTURE_R {
        CAPTURE_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - Fault B Blanking Prescaler"]
    #[inline(always)]
    pub fn blankpresc(&self) -> BLANKPRESC_R {
        BLANKPRESC_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:23 - Fault B Blanking Time"]
    #[inline(always)]
    pub fn blankval(&self) -> BLANKVAL_R {
        BLANKVAL_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:27 - Fault B Filter Value"]
    #[inline(always)]
    pub fn filterval(&self) -> FILTERVAL_R {
        FILTERVAL_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Fault B Source"]
    #[inline(always)]
    #[must_use]
    pub fn src(&mut self) -> SRC_W<FCTRLB_SPEC, 0> {
        SRC_W::new(self)
    }
    #[doc = "Bit 3 - Fault B Keeper"]
    #[inline(always)]
    #[must_use]
    pub fn keep(&mut self) -> KEEP_W<FCTRLB_SPEC, 3> {
        KEEP_W::new(self)
    }
    #[doc = "Bit 4 - Fault B Qualification"]
    #[inline(always)]
    #[must_use]
    pub fn qual(&mut self) -> QUAL_W<FCTRLB_SPEC, 4> {
        QUAL_W::new(self)
    }
    #[doc = "Bits 5:6 - Fault B Blanking Mode"]
    #[inline(always)]
    #[must_use]
    pub fn blank(&mut self) -> BLANK_W<FCTRLB_SPEC, 5> {
        BLANK_W::new(self)
    }
    #[doc = "Bit 7 - Fault B Restart"]
    #[inline(always)]
    #[must_use]
    pub fn restart(&mut self) -> RESTART_W<FCTRLB_SPEC, 7> {
        RESTART_W::new(self)
    }
    #[doc = "Bits 8:9 - Fault B Halt Mode"]
    #[inline(always)]
    #[must_use]
    pub fn halt(&mut self) -> HALT_W<FCTRLB_SPEC, 8> {
        HALT_W::new(self)
    }
    #[doc = "Bits 10:11 - Fault B Capture Channel"]
    #[inline(always)]
    #[must_use]
    pub fn chsel(&mut self) -> CHSEL_W<FCTRLB_SPEC, 10> {
        CHSEL_W::new(self)
    }
    #[doc = "Bits 12:14 - Fault B Capture Action"]
    #[inline(always)]
    #[must_use]
    pub fn capture(&mut self) -> CAPTURE_W<FCTRLB_SPEC, 12> {
        CAPTURE_W::new(self)
    }
    #[doc = "Bit 15 - Fault B Blanking Prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn blankpresc(&mut self) -> BLANKPRESC_W<FCTRLB_SPEC, 15> {
        BLANKPRESC_W::new(self)
    }
    #[doc = "Bits 16:23 - Fault B Blanking Time"]
    #[inline(always)]
    #[must_use]
    pub fn blankval(&mut self) -> BLANKVAL_W<FCTRLB_SPEC, 16> {
        BLANKVAL_W::new(self)
    }
    #[doc = "Bits 24:27 - Fault B Filter Value"]
    #[inline(always)]
    #[must_use]
    pub fn filterval(&mut self) -> FILTERVAL_W<FCTRLB_SPEC, 24> {
        FILTERVAL_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Recoverable Fault B Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fctrlb::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fctrlb::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FCTRLB_SPEC;
impl crate::RegisterSpec for FCTRLB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fctrlb::R`](R) reader structure"]
impl crate::Readable for FCTRLB_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fctrlb::W`](W) writer structure"]
impl crate::Writable for FCTRLB_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FCTRLB to value 0"]
impl crate::Resettable for FCTRLB_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
