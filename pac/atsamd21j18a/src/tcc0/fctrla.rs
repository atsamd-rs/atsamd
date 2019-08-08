#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::FCTRLA {
    #[doc = r" Modifies the contents of the register"]
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        let r = R { bits };
        let mut w = W { bits };
        f(&r, &mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
    #[doc = r" Writes to the register"]
    #[inline]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        let mut w = W::reset_value();
        f(&mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.write(|w| w)
    }
}
#[doc = "Possible values of the field `SRC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRCR {
    #[doc = "Fault input disabled"]
    DISABLE,
    #[doc = "MCEx (x=0,1) event input"]
    ENABLE,
    #[doc = "Inverted MCEx (x=0,1) event input"]
    INVERT,
    #[doc = "Alternate fault (A or B) state at the end of the previous period"]
    ALTFAULT,
}
impl SRCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SRCR::DISABLE => 0,
            SRCR::ENABLE => 1,
            SRCR::INVERT => 2,
            SRCR::ALTFAULT => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SRCR {
        match value {
            0 => SRCR::DISABLE,
            1 => SRCR::ENABLE,
            2 => SRCR::INVERT,
            3 => SRCR::ALTFAULT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == SRCR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == SRCR::ENABLE
    }
    #[doc = "Checks if the value of the field is `INVERT`"]
    #[inline]
    pub fn is_invert(&self) -> bool {
        *self == SRCR::INVERT
    }
    #[doc = "Checks if the value of the field is `ALTFAULT`"]
    #[inline]
    pub fn is_altfault(&self) -> bool {
        *self == SRCR::ALTFAULT
    }
}
#[doc = r" Value of the field"]
pub struct KEEPR {
    bits: bool,
}
impl KEEPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct QUALR {
    bits: bool,
}
impl QUALR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = "Possible values of the field `BLANK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BLANKR {
    #[doc = "No blanking applied"]
    NONE,
    #[doc = "Blanking applied from rising edge of the output waveform"]
    RISE,
    #[doc = "Blanking applied from falling edge of the output waveform"]
    FALL,
    #[doc = "Blanking applied from each toggle of the output waveform"]
    BOTH,
}
impl BLANKR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            BLANKR::NONE => 0,
            BLANKR::RISE => 1,
            BLANKR::FALL => 2,
            BLANKR::BOTH => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> BLANKR {
        match value {
            0 => BLANKR::NONE,
            1 => BLANKR::RISE,
            2 => BLANKR::FALL,
            3 => BLANKR::BOTH,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline]
    pub fn is_none(&self) -> bool {
        *self == BLANKR::NONE
    }
    #[doc = "Checks if the value of the field is `RISE`"]
    #[inline]
    pub fn is_rise(&self) -> bool {
        *self == BLANKR::RISE
    }
    #[doc = "Checks if the value of the field is `FALL`"]
    #[inline]
    pub fn is_fall(&self) -> bool {
        *self == BLANKR::FALL
    }
    #[doc = "Checks if the value of the field is `BOTH`"]
    #[inline]
    pub fn is_both(&self) -> bool {
        *self == BLANKR::BOTH
    }
}
#[doc = r" Value of the field"]
pub struct RESTARTR {
    bits: bool,
}
impl RESTARTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = "Possible values of the field `HALT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HALTR {
    #[doc = "Halt action disabled"]
    DISABLE,
    #[doc = "Hardware halt action"]
    HW,
    #[doc = "Software halt action"]
    SW,
    #[doc = "Non-recoverable fault"]
    NR,
}
impl HALTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            HALTR::DISABLE => 0,
            HALTR::HW => 1,
            HALTR::SW => 2,
            HALTR::NR => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> HALTR {
        match value {
            0 => HALTR::DISABLE,
            1 => HALTR::HW,
            2 => HALTR::SW,
            3 => HALTR::NR,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == HALTR::DISABLE
    }
    #[doc = "Checks if the value of the field is `HW`"]
    #[inline]
    pub fn is_hw(&self) -> bool {
        *self == HALTR::HW
    }
    #[doc = "Checks if the value of the field is `SW`"]
    #[inline]
    pub fn is_sw(&self) -> bool {
        *self == HALTR::SW
    }
    #[doc = "Checks if the value of the field is `NR`"]
    #[inline]
    pub fn is_nr(&self) -> bool {
        *self == HALTR::NR
    }
}
#[doc = "Possible values of the field `CHSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHSELR {
    #[doc = "Capture value stored in channel 0"]
    CC0,
    #[doc = "Capture value stored in channel 1"]
    CC1,
    #[doc = "Capture value stored in channel 2"]
    CC2,
    #[doc = "Capture value stored in channel 3"]
    CC3,
}
impl CHSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CHSELR::CC0 => 0,
            CHSELR::CC1 => 1,
            CHSELR::CC2 => 2,
            CHSELR::CC3 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CHSELR {
        match value {
            0 => CHSELR::CC0,
            1 => CHSELR::CC1,
            2 => CHSELR::CC2,
            3 => CHSELR::CC3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CC0`"]
    #[inline]
    pub fn is_cc0(&self) -> bool {
        *self == CHSELR::CC0
    }
    #[doc = "Checks if the value of the field is `CC1`"]
    #[inline]
    pub fn is_cc1(&self) -> bool {
        *self == CHSELR::CC1
    }
    #[doc = "Checks if the value of the field is `CC2`"]
    #[inline]
    pub fn is_cc2(&self) -> bool {
        *self == CHSELR::CC2
    }
    #[doc = "Checks if the value of the field is `CC3`"]
    #[inline]
    pub fn is_cc3(&self) -> bool {
        *self == CHSELR::CC3
    }
}
#[doc = "Possible values of the field `CAPTURE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAPTURER {
    #[doc = "No capture"]
    DISABLE,
    #[doc = "Capture on fault"]
    CAPT,
    #[doc = "Minimum capture"]
    CAPTMIN,
    #[doc = "Maximum capture"]
    CAPTMAX,
    #[doc = "Minimum local detection"]
    LOCMIN,
    #[doc = "Maximum local detection"]
    LOCMAX,
    #[doc = "Minimum and maximum local detection"]
    DERIV0,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl CAPTURER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CAPTURER::DISABLE => 0,
            CAPTURER::CAPT => 1,
            CAPTURER::CAPTMIN => 2,
            CAPTURER::CAPTMAX => 3,
            CAPTURER::LOCMIN => 4,
            CAPTURER::LOCMAX => 5,
            CAPTURER::DERIV0 => 6,
            CAPTURER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CAPTURER {
        match value {
            0 => CAPTURER::DISABLE,
            1 => CAPTURER::CAPT,
            2 => CAPTURER::CAPTMIN,
            3 => CAPTURER::CAPTMAX,
            4 => CAPTURER::LOCMIN,
            5 => CAPTURER::LOCMAX,
            6 => CAPTURER::DERIV0,
            i => CAPTURER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == CAPTURER::DISABLE
    }
    #[doc = "Checks if the value of the field is `CAPT`"]
    #[inline]
    pub fn is_capt(&self) -> bool {
        *self == CAPTURER::CAPT
    }
    #[doc = "Checks if the value of the field is `CAPTMIN`"]
    #[inline]
    pub fn is_captmin(&self) -> bool {
        *self == CAPTURER::CAPTMIN
    }
    #[doc = "Checks if the value of the field is `CAPTMAX`"]
    #[inline]
    pub fn is_captmax(&self) -> bool {
        *self == CAPTURER::CAPTMAX
    }
    #[doc = "Checks if the value of the field is `LOCMIN`"]
    #[inline]
    pub fn is_locmin(&self) -> bool {
        *self == CAPTURER::LOCMIN
    }
    #[doc = "Checks if the value of the field is `LOCMAX`"]
    #[inline]
    pub fn is_locmax(&self) -> bool {
        *self == CAPTURER::LOCMAX
    }
    #[doc = "Checks if the value of the field is `DERIV0`"]
    #[inline]
    pub fn is_deriv0(&self) -> bool {
        *self == CAPTURER::DERIV0
    }
}
#[doc = r" Value of the field"]
pub struct BLANKVALR {
    bits: u8,
}
impl BLANKVALR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct FILTERVALR {
    bits: u8,
}
impl FILTERVALR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Values that can be written to the field `SRC`"]
pub enum SRCW {
    #[doc = "Fault input disabled"]
    DISABLE,
    #[doc = "MCEx (x=0,1) event input"]
    ENABLE,
    #[doc = "Inverted MCEx (x=0,1) event input"]
    INVERT,
    #[doc = "Alternate fault (A or B) state at the end of the previous period"]
    ALTFAULT,
}
impl SRCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SRCW::DISABLE => 0,
            SRCW::ENABLE => 1,
            SRCW::INVERT => 2,
            SRCW::ALTFAULT => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SRCW<'a> {
    w: &'a mut W,
}
impl<'a> _SRCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SRCW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Fault input disabled"]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(SRCW::DISABLE)
    }
    #[doc = "MCEx (x=0,1) event input"]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(SRCW::ENABLE)
    }
    #[doc = "Inverted MCEx (x=0,1) event input"]
    #[inline]
    pub fn invert(self) -> &'a mut W {
        self.variant(SRCW::INVERT)
    }
    #[doc = "Alternate fault (A or B) state at the end of the previous period"]
    #[inline]
    pub fn altfault(self) -> &'a mut W {
        self.variant(SRCW::ALTFAULT)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _KEEPW<'a> {
    w: &'a mut W,
}
impl<'a> _KEEPW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _QUALW<'a> {
    w: &'a mut W,
}
impl<'a> _QUALW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `BLANK`"]
pub enum BLANKW {
    #[doc = "No blanking applied"]
    NONE,
    #[doc = "Blanking applied from rising edge of the output waveform"]
    RISE,
    #[doc = "Blanking applied from falling edge of the output waveform"]
    FALL,
    #[doc = "Blanking applied from each toggle of the output waveform"]
    BOTH,
}
impl BLANKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            BLANKW::NONE => 0,
            BLANKW::RISE => 1,
            BLANKW::FALL => 2,
            BLANKW::BOTH => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BLANKW<'a> {
    w: &'a mut W,
}
impl<'a> _BLANKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BLANKW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "No blanking applied"]
    #[inline]
    pub fn none(self) -> &'a mut W {
        self.variant(BLANKW::NONE)
    }
    #[doc = "Blanking applied from rising edge of the output waveform"]
    #[inline]
    pub fn rise(self) -> &'a mut W {
        self.variant(BLANKW::RISE)
    }
    #[doc = "Blanking applied from falling edge of the output waveform"]
    #[inline]
    pub fn fall(self) -> &'a mut W {
        self.variant(BLANKW::FALL)
    }
    #[doc = "Blanking applied from each toggle of the output waveform"]
    #[inline]
    pub fn both(self) -> &'a mut W {
        self.variant(BLANKW::BOTH)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RESTARTW<'a> {
    w: &'a mut W,
}
impl<'a> _RESTARTW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 7;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `HALT`"]
pub enum HALTW {
    #[doc = "Halt action disabled"]
    DISABLE,
    #[doc = "Hardware halt action"]
    HW,
    #[doc = "Software halt action"]
    SW,
    #[doc = "Non-recoverable fault"]
    NR,
}
impl HALTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            HALTW::DISABLE => 0,
            HALTW::HW => 1,
            HALTW::SW => 2,
            HALTW::NR => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HALTW<'a> {
    w: &'a mut W,
}
impl<'a> _HALTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HALTW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Halt action disabled"]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(HALTW::DISABLE)
    }
    #[doc = "Hardware halt action"]
    #[inline]
    pub fn hw(self) -> &'a mut W {
        self.variant(HALTW::HW)
    }
    #[doc = "Software halt action"]
    #[inline]
    pub fn sw(self) -> &'a mut W {
        self.variant(HALTW::SW)
    }
    #[doc = "Non-recoverable fault"]
    #[inline]
    pub fn nr(self) -> &'a mut W {
        self.variant(HALTW::NR)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CHSEL`"]
pub enum CHSELW {
    #[doc = "Capture value stored in channel 0"]
    CC0,
    #[doc = "Capture value stored in channel 1"]
    CC1,
    #[doc = "Capture value stored in channel 2"]
    CC2,
    #[doc = "Capture value stored in channel 3"]
    CC3,
}
impl CHSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CHSELW::CC0 => 0,
            CHSELW::CC1 => 1,
            CHSELW::CC2 => 2,
            CHSELW::CC3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CHSELW<'a> {
    w: &'a mut W,
}
impl<'a> _CHSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CHSELW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Capture value stored in channel 0"]
    #[inline]
    pub fn cc0(self) -> &'a mut W {
        self.variant(CHSELW::CC0)
    }
    #[doc = "Capture value stored in channel 1"]
    #[inline]
    pub fn cc1(self) -> &'a mut W {
        self.variant(CHSELW::CC1)
    }
    #[doc = "Capture value stored in channel 2"]
    #[inline]
    pub fn cc2(self) -> &'a mut W {
        self.variant(CHSELW::CC2)
    }
    #[doc = "Capture value stored in channel 3"]
    #[inline]
    pub fn cc3(self) -> &'a mut W {
        self.variant(CHSELW::CC3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CAPTURE`"]
pub enum CAPTUREW {
    #[doc = "No capture"]
    DISABLE,
    #[doc = "Capture on fault"]
    CAPT,
    #[doc = "Minimum capture"]
    CAPTMIN,
    #[doc = "Maximum capture"]
    CAPTMAX,
    #[doc = "Minimum local detection"]
    LOCMIN,
    #[doc = "Maximum local detection"]
    LOCMAX,
    #[doc = "Minimum and maximum local detection"]
    DERIV0,
}
impl CAPTUREW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CAPTUREW::DISABLE => 0,
            CAPTUREW::CAPT => 1,
            CAPTUREW::CAPTMIN => 2,
            CAPTUREW::CAPTMAX => 3,
            CAPTUREW::LOCMIN => 4,
            CAPTUREW::LOCMAX => 5,
            CAPTUREW::DERIV0 => 6,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CAPTUREW<'a> {
    w: &'a mut W,
}
impl<'a> _CAPTUREW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CAPTUREW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "No capture"]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(CAPTUREW::DISABLE)
    }
    #[doc = "Capture on fault"]
    #[inline]
    pub fn capt(self) -> &'a mut W {
        self.variant(CAPTUREW::CAPT)
    }
    #[doc = "Minimum capture"]
    #[inline]
    pub fn captmin(self) -> &'a mut W {
        self.variant(CAPTUREW::CAPTMIN)
    }
    #[doc = "Maximum capture"]
    #[inline]
    pub fn captmax(self) -> &'a mut W {
        self.variant(CAPTUREW::CAPTMAX)
    }
    #[doc = "Minimum local detection"]
    #[inline]
    pub fn locmin(self) -> &'a mut W {
        self.variant(CAPTUREW::LOCMIN)
    }
    #[doc = "Maximum local detection"]
    #[inline]
    pub fn locmax(self) -> &'a mut W {
        self.variant(CAPTUREW::LOCMAX)
    }
    #[doc = "Minimum and maximum local detection"]
    #[inline]
    pub fn deriv0(self) -> &'a mut W {
        self.variant(CAPTUREW::DERIV0)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _BLANKVALW<'a> {
    w: &'a mut W,
}
impl<'a> _BLANKVALW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _FILTERVALW<'a> {
    w: &'a mut W,
}
impl<'a> _FILTERVALW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:1 - Fault A Source"]
    #[inline]
    pub fn src(&self) -> SRCR {
        SRCR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 3 - Fault A Keeper"]
    #[inline]
    pub fn keep(&self) -> KEEPR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        KEEPR { bits }
    }
    #[doc = "Bit 4 - Fault A Qualification"]
    #[inline]
    pub fn qual(&self) -> QUALR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        QUALR { bits }
    }
    #[doc = "Bits 5:6 - Fault A Blanking Mode"]
    #[inline]
    pub fn blank(&self) -> BLANKR {
        BLANKR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 7 - Fault A Restart"]
    #[inline]
    pub fn restart(&self) -> RESTARTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RESTARTR { bits }
    }
    #[doc = "Bits 8:9 - Fault A Halt Mode"]
    #[inline]
    pub fn halt(&self) -> HALTR {
        HALTR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 10:11 - Fault A Capture Channel"]
    #[inline]
    pub fn chsel(&self) -> CHSELR {
        CHSELR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 12:14 - Fault A Capture Action"]
    #[inline]
    pub fn capture(&self) -> CAPTURER {
        CAPTURER::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:23 - Fault A Blanking Time"]
    #[inline]
    pub fn blankval(&self) -> BLANKVALR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        BLANKVALR { bits }
    }
    #[doc = "Bits 24:27 - Fault A Filter Value"]
    #[inline]
    pub fn filterval(&self) -> FILTERVALR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        FILTERVALR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:1 - Fault A Source"]
    #[inline]
    pub fn src(&mut self) -> _SRCW {
        _SRCW { w: self }
    }
    #[doc = "Bit 3 - Fault A Keeper"]
    #[inline]
    pub fn keep(&mut self) -> _KEEPW {
        _KEEPW { w: self }
    }
    #[doc = "Bit 4 - Fault A Qualification"]
    #[inline]
    pub fn qual(&mut self) -> _QUALW {
        _QUALW { w: self }
    }
    #[doc = "Bits 5:6 - Fault A Blanking Mode"]
    #[inline]
    pub fn blank(&mut self) -> _BLANKW {
        _BLANKW { w: self }
    }
    #[doc = "Bit 7 - Fault A Restart"]
    #[inline]
    pub fn restart(&mut self) -> _RESTARTW {
        _RESTARTW { w: self }
    }
    #[doc = "Bits 8:9 - Fault A Halt Mode"]
    #[inline]
    pub fn halt(&mut self) -> _HALTW {
        _HALTW { w: self }
    }
    #[doc = "Bits 10:11 - Fault A Capture Channel"]
    #[inline]
    pub fn chsel(&mut self) -> _CHSELW {
        _CHSELW { w: self }
    }
    #[doc = "Bits 12:14 - Fault A Capture Action"]
    #[inline]
    pub fn capture(&mut self) -> _CAPTUREW {
        _CAPTUREW { w: self }
    }
    #[doc = "Bits 16:23 - Fault A Blanking Time"]
    #[inline]
    pub fn blankval(&mut self) -> _BLANKVALW {
        _BLANKVALW { w: self }
    }
    #[doc = "Bits 24:27 - Fault A Filter Value"]
    #[inline]
    pub fn filterval(&mut self) -> _FILTERVALW {
        _FILTERVALW { w: self }
    }
}
