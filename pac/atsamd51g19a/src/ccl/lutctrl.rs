#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::LUTCTRL {
    #[doc = r" Modifies the contents of the register"]
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        self.register.set(f(&R { bits }, &mut W { bits }).bits);
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
        self.register.set(
            f(&mut W {
                bits: Self::reset_value(),
            })
            .bits,
        );
    }
    #[doc = r" Reset value of the register"]
    #[inline]
    pub const fn reset_value() -> u32 {
        0
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.register.set(Self::reset_value())
    }
}
#[doc = r" Value of the field"]
pub struct ENABLER {
    bits: bool,
}
impl ENABLER {
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
#[doc = "Possible values of the field `FILTSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FILTSELR {
    #[doc = "Filter disabled"]
    DISABLE,
    #[doc = "Synchronizer enabled"]
    SYNCH,
    #[doc = "Filter enabled"]
    FILTER,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl FILTSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            FILTSELR::DISABLE => 0,
            FILTSELR::SYNCH => 0x01,
            FILTSELR::FILTER => 0x02,
            FILTSELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> FILTSELR {
        match value {
            0 => FILTSELR::DISABLE,
            1 => FILTSELR::SYNCH,
            2 => FILTSELR::FILTER,
            i => FILTSELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == FILTSELR::DISABLE
    }
    #[doc = "Checks if the value of the field is `SYNCH`"]
    #[inline]
    pub fn is_synch(&self) -> bool {
        *self == FILTSELR::SYNCH
    }
    #[doc = "Checks if the value of the field is `FILTER`"]
    #[inline]
    pub fn is_filter(&self) -> bool {
        *self == FILTSELR::FILTER
    }
}
#[doc = r" Value of the field"]
pub struct EDGESELR {
    bits: bool,
}
impl EDGESELR {
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
#[doc = "Possible values of the field `INSEL0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INSEL0R {
    #[doc = "Masked input"]
    MASK,
    #[doc = "Feedback input source"]
    FEEDBACK,
    #[doc = "Linked LUT input source"]
    LINK,
    #[doc = "Event input source"]
    EVENT,
    #[doc = "I/O pin input source"]
    IO,
    #[doc = "AC input source"]
    AC,
    #[doc = "TC input source"]
    TC,
    #[doc = "Alternate TC input source"]
    ALTTC,
    #[doc = "TCC input source"]
    TCC,
    #[doc = "SERCOM input source"]
    SERCOM,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl INSEL0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            INSEL0R::MASK => 0,
            INSEL0R::FEEDBACK => 0x01,
            INSEL0R::LINK => 0x02,
            INSEL0R::EVENT => 0x03,
            INSEL0R::IO => 0x04,
            INSEL0R::AC => 0x05,
            INSEL0R::TC => 0x06,
            INSEL0R::ALTTC => 0x07,
            INSEL0R::TCC => 0x08,
            INSEL0R::SERCOM => 0x09,
            INSEL0R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> INSEL0R {
        match value {
            0 => INSEL0R::MASK,
            1 => INSEL0R::FEEDBACK,
            2 => INSEL0R::LINK,
            3 => INSEL0R::EVENT,
            4 => INSEL0R::IO,
            5 => INSEL0R::AC,
            6 => INSEL0R::TC,
            7 => INSEL0R::ALTTC,
            8 => INSEL0R::TCC,
            9 => INSEL0R::SERCOM,
            i => INSEL0R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `MASK`"]
    #[inline]
    pub fn is_mask(&self) -> bool {
        *self == INSEL0R::MASK
    }
    #[doc = "Checks if the value of the field is `FEEDBACK`"]
    #[inline]
    pub fn is_feedback(&self) -> bool {
        *self == INSEL0R::FEEDBACK
    }
    #[doc = "Checks if the value of the field is `LINK`"]
    #[inline]
    pub fn is_link(&self) -> bool {
        *self == INSEL0R::LINK
    }
    #[doc = "Checks if the value of the field is `EVENT`"]
    #[inline]
    pub fn is_event(&self) -> bool {
        *self == INSEL0R::EVENT
    }
    #[doc = "Checks if the value of the field is `IO`"]
    #[inline]
    pub fn is_io(&self) -> bool {
        *self == INSEL0R::IO
    }
    #[doc = "Checks if the value of the field is `AC`"]
    #[inline]
    pub fn is_ac(&self) -> bool {
        *self == INSEL0R::AC
    }
    #[doc = "Checks if the value of the field is `TC`"]
    #[inline]
    pub fn is_tc(&self) -> bool {
        *self == INSEL0R::TC
    }
    #[doc = "Checks if the value of the field is `ALTTC`"]
    #[inline]
    pub fn is_alttc(&self) -> bool {
        *self == INSEL0R::ALTTC
    }
    #[doc = "Checks if the value of the field is `TCC`"]
    #[inline]
    pub fn is_tcc(&self) -> bool {
        *self == INSEL0R::TCC
    }
    #[doc = "Checks if the value of the field is `SERCOM`"]
    #[inline]
    pub fn is_sercom(&self) -> bool {
        *self == INSEL0R::SERCOM
    }
}
#[doc = "Possible values of the field `INSEL1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INSEL1R {
    #[doc = "Masked input"]
    MASK,
    #[doc = "Feedback input source"]
    FEEDBACK,
    #[doc = "Linked LUT input source"]
    LINK,
    #[doc = "Event input source"]
    EVENT,
    #[doc = "I/O pin input source"]
    IO,
    #[doc = "AC input source"]
    AC,
    #[doc = "TC input source"]
    TC,
    #[doc = "Alternate TC input source"]
    ALTTC,
    #[doc = "TCC input source"]
    TCC,
    #[doc = "SERCOM input source"]
    SERCOM,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl INSEL1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            INSEL1R::MASK => 0,
            INSEL1R::FEEDBACK => 0x01,
            INSEL1R::LINK => 0x02,
            INSEL1R::EVENT => 0x03,
            INSEL1R::IO => 0x04,
            INSEL1R::AC => 0x05,
            INSEL1R::TC => 0x06,
            INSEL1R::ALTTC => 0x07,
            INSEL1R::TCC => 0x08,
            INSEL1R::SERCOM => 0x09,
            INSEL1R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> INSEL1R {
        match value {
            0 => INSEL1R::MASK,
            1 => INSEL1R::FEEDBACK,
            2 => INSEL1R::LINK,
            3 => INSEL1R::EVENT,
            4 => INSEL1R::IO,
            5 => INSEL1R::AC,
            6 => INSEL1R::TC,
            7 => INSEL1R::ALTTC,
            8 => INSEL1R::TCC,
            9 => INSEL1R::SERCOM,
            i => INSEL1R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `MASK`"]
    #[inline]
    pub fn is_mask(&self) -> bool {
        *self == INSEL1R::MASK
    }
    #[doc = "Checks if the value of the field is `FEEDBACK`"]
    #[inline]
    pub fn is_feedback(&self) -> bool {
        *self == INSEL1R::FEEDBACK
    }
    #[doc = "Checks if the value of the field is `LINK`"]
    #[inline]
    pub fn is_link(&self) -> bool {
        *self == INSEL1R::LINK
    }
    #[doc = "Checks if the value of the field is `EVENT`"]
    #[inline]
    pub fn is_event(&self) -> bool {
        *self == INSEL1R::EVENT
    }
    #[doc = "Checks if the value of the field is `IO`"]
    #[inline]
    pub fn is_io(&self) -> bool {
        *self == INSEL1R::IO
    }
    #[doc = "Checks if the value of the field is `AC`"]
    #[inline]
    pub fn is_ac(&self) -> bool {
        *self == INSEL1R::AC
    }
    #[doc = "Checks if the value of the field is `TC`"]
    #[inline]
    pub fn is_tc(&self) -> bool {
        *self == INSEL1R::TC
    }
    #[doc = "Checks if the value of the field is `ALTTC`"]
    #[inline]
    pub fn is_alttc(&self) -> bool {
        *self == INSEL1R::ALTTC
    }
    #[doc = "Checks if the value of the field is `TCC`"]
    #[inline]
    pub fn is_tcc(&self) -> bool {
        *self == INSEL1R::TCC
    }
    #[doc = "Checks if the value of the field is `SERCOM`"]
    #[inline]
    pub fn is_sercom(&self) -> bool {
        *self == INSEL1R::SERCOM
    }
}
#[doc = "Possible values of the field `INSEL2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INSEL2R {
    #[doc = "Masked input"]
    MASK,
    #[doc = "Feedback input source"]
    FEEDBACK,
    #[doc = "Linked LUT input source"]
    LINK,
    #[doc = "Event input source"]
    EVENT,
    #[doc = "I/O pin input source"]
    IO,
    #[doc = "AC input source"]
    AC,
    #[doc = "TC input source"]
    TC,
    #[doc = "Alternate TC input source"]
    ALTTC,
    #[doc = "TCC input source"]
    TCC,
    #[doc = "SERCOM input source"]
    SERCOM,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl INSEL2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            INSEL2R::MASK => 0,
            INSEL2R::FEEDBACK => 0x01,
            INSEL2R::LINK => 0x02,
            INSEL2R::EVENT => 0x03,
            INSEL2R::IO => 0x04,
            INSEL2R::AC => 0x05,
            INSEL2R::TC => 0x06,
            INSEL2R::ALTTC => 0x07,
            INSEL2R::TCC => 0x08,
            INSEL2R::SERCOM => 0x09,
            INSEL2R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> INSEL2R {
        match value {
            0 => INSEL2R::MASK,
            1 => INSEL2R::FEEDBACK,
            2 => INSEL2R::LINK,
            3 => INSEL2R::EVENT,
            4 => INSEL2R::IO,
            5 => INSEL2R::AC,
            6 => INSEL2R::TC,
            7 => INSEL2R::ALTTC,
            8 => INSEL2R::TCC,
            9 => INSEL2R::SERCOM,
            i => INSEL2R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `MASK`"]
    #[inline]
    pub fn is_mask(&self) -> bool {
        *self == INSEL2R::MASK
    }
    #[doc = "Checks if the value of the field is `FEEDBACK`"]
    #[inline]
    pub fn is_feedback(&self) -> bool {
        *self == INSEL2R::FEEDBACK
    }
    #[doc = "Checks if the value of the field is `LINK`"]
    #[inline]
    pub fn is_link(&self) -> bool {
        *self == INSEL2R::LINK
    }
    #[doc = "Checks if the value of the field is `EVENT`"]
    #[inline]
    pub fn is_event(&self) -> bool {
        *self == INSEL2R::EVENT
    }
    #[doc = "Checks if the value of the field is `IO`"]
    #[inline]
    pub fn is_io(&self) -> bool {
        *self == INSEL2R::IO
    }
    #[doc = "Checks if the value of the field is `AC`"]
    #[inline]
    pub fn is_ac(&self) -> bool {
        *self == INSEL2R::AC
    }
    #[doc = "Checks if the value of the field is `TC`"]
    #[inline]
    pub fn is_tc(&self) -> bool {
        *self == INSEL2R::TC
    }
    #[doc = "Checks if the value of the field is `ALTTC`"]
    #[inline]
    pub fn is_alttc(&self) -> bool {
        *self == INSEL2R::ALTTC
    }
    #[doc = "Checks if the value of the field is `TCC`"]
    #[inline]
    pub fn is_tcc(&self) -> bool {
        *self == INSEL2R::TCC
    }
    #[doc = "Checks if the value of the field is `SERCOM`"]
    #[inline]
    pub fn is_sercom(&self) -> bool {
        *self == INSEL2R::SERCOM
    }
}
#[doc = r" Value of the field"]
pub struct INVEIR {
    bits: bool,
}
impl INVEIR {
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
pub struct LUTEIR {
    bits: bool,
}
impl LUTEIR {
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
pub struct LUTEOR {
    bits: bool,
}
impl LUTEOR {
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
pub struct TRUTHR {
    bits: u8,
}
impl TRUTHR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _ENABLEW<'a> {
    w: &'a mut W,
}
impl<'a> _ENABLEW<'a> {
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
        self.w.bits &= !(0x01 << 1);
        self.w.bits |= ((value as u32) & 0x01) << 1;
        self.w
    }
}
#[doc = "Values that can be written to the field `FILTSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FILTSELW {
    #[doc = "Filter disabled"]
    DISABLE,
    #[doc = "Synchronizer enabled"]
    SYNCH,
    #[doc = "Filter enabled"]
    FILTER,
}
impl FILTSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            FILTSELW::DISABLE => 0,
            FILTSELW::SYNCH => 1,
            FILTSELW::FILTER => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FILTSELW<'a> {
    w: &'a mut W,
}
impl<'a> _FILTSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FILTSELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Filter disabled"]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(FILTSELW::DISABLE)
    }
    #[doc = "Synchronizer enabled"]
    #[inline]
    pub fn synch(self) -> &'a mut W {
        self.variant(FILTSELW::SYNCH)
    }
    #[doc = "Filter enabled"]
    #[inline]
    pub fn filter(self) -> &'a mut W {
        self.variant(FILTSELW::FILTER)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(0x03 << 4);
        self.w.bits |= ((value as u32) & 0x03) << 4;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _EDGESELW<'a> {
    w: &'a mut W,
}
impl<'a> _EDGESELW<'a> {
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
        self.w.bits &= !(0x01 << 7);
        self.w.bits |= ((value as u32) & 0x01) << 7;
        self.w
    }
}
#[doc = "Values that can be written to the field `INSEL0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INSEL0W {
    #[doc = "Masked input"]
    MASK,
    #[doc = "Feedback input source"]
    FEEDBACK,
    #[doc = "Linked LUT input source"]
    LINK,
    #[doc = "Event input source"]
    EVENT,
    #[doc = "I/O pin input source"]
    IO,
    #[doc = "AC input source"]
    AC,
    #[doc = "TC input source"]
    TC,
    #[doc = "Alternate TC input source"]
    ALTTC,
    #[doc = "TCC input source"]
    TCC,
    #[doc = "SERCOM input source"]
    SERCOM,
}
impl INSEL0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            INSEL0W::MASK => 0,
            INSEL0W::FEEDBACK => 1,
            INSEL0W::LINK => 2,
            INSEL0W::EVENT => 3,
            INSEL0W::IO => 4,
            INSEL0W::AC => 5,
            INSEL0W::TC => 6,
            INSEL0W::ALTTC => 7,
            INSEL0W::TCC => 8,
            INSEL0W::SERCOM => 9,
        }
    }
}
#[doc = r" Proxy"]
pub struct _INSEL0W<'a> {
    w: &'a mut W,
}
impl<'a> _INSEL0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: INSEL0W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Masked input"]
    #[inline]
    pub fn mask(self) -> &'a mut W {
        self.variant(INSEL0W::MASK)
    }
    #[doc = "Feedback input source"]
    #[inline]
    pub fn feedback(self) -> &'a mut W {
        self.variant(INSEL0W::FEEDBACK)
    }
    #[doc = "Linked LUT input source"]
    #[inline]
    pub fn link(self) -> &'a mut W {
        self.variant(INSEL0W::LINK)
    }
    #[doc = "Event input source"]
    #[inline]
    pub fn event(self) -> &'a mut W {
        self.variant(INSEL0W::EVENT)
    }
    #[doc = "I/O pin input source"]
    #[inline]
    pub fn io(self) -> &'a mut W {
        self.variant(INSEL0W::IO)
    }
    #[doc = "AC input source"]
    #[inline]
    pub fn ac(self) -> &'a mut W {
        self.variant(INSEL0W::AC)
    }
    #[doc = "TC input source"]
    #[inline]
    pub fn tc(self) -> &'a mut W {
        self.variant(INSEL0W::TC)
    }
    #[doc = "Alternate TC input source"]
    #[inline]
    pub fn alttc(self) -> &'a mut W {
        self.variant(INSEL0W::ALTTC)
    }
    #[doc = "TCC input source"]
    #[inline]
    pub fn tcc(self) -> &'a mut W {
        self.variant(INSEL0W::TCC)
    }
    #[doc = "SERCOM input source"]
    #[inline]
    pub fn sercom(self) -> &'a mut W {
        self.variant(INSEL0W::SERCOM)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(0x0f << 8);
        self.w.bits |= ((value as u32) & 0x0f) << 8;
        self.w
    }
}
#[doc = "Values that can be written to the field `INSEL1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INSEL1W {
    #[doc = "Masked input"]
    MASK,
    #[doc = "Feedback input source"]
    FEEDBACK,
    #[doc = "Linked LUT input source"]
    LINK,
    #[doc = "Event input source"]
    EVENT,
    #[doc = "I/O pin input source"]
    IO,
    #[doc = "AC input source"]
    AC,
    #[doc = "TC input source"]
    TC,
    #[doc = "Alternate TC input source"]
    ALTTC,
    #[doc = "TCC input source"]
    TCC,
    #[doc = "SERCOM input source"]
    SERCOM,
}
impl INSEL1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            INSEL1W::MASK => 0,
            INSEL1W::FEEDBACK => 1,
            INSEL1W::LINK => 2,
            INSEL1W::EVENT => 3,
            INSEL1W::IO => 4,
            INSEL1W::AC => 5,
            INSEL1W::TC => 6,
            INSEL1W::ALTTC => 7,
            INSEL1W::TCC => 8,
            INSEL1W::SERCOM => 9,
        }
    }
}
#[doc = r" Proxy"]
pub struct _INSEL1W<'a> {
    w: &'a mut W,
}
impl<'a> _INSEL1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: INSEL1W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Masked input"]
    #[inline]
    pub fn mask(self) -> &'a mut W {
        self.variant(INSEL1W::MASK)
    }
    #[doc = "Feedback input source"]
    #[inline]
    pub fn feedback(self) -> &'a mut W {
        self.variant(INSEL1W::FEEDBACK)
    }
    #[doc = "Linked LUT input source"]
    #[inline]
    pub fn link(self) -> &'a mut W {
        self.variant(INSEL1W::LINK)
    }
    #[doc = "Event input source"]
    #[inline]
    pub fn event(self) -> &'a mut W {
        self.variant(INSEL1W::EVENT)
    }
    #[doc = "I/O pin input source"]
    #[inline]
    pub fn io(self) -> &'a mut W {
        self.variant(INSEL1W::IO)
    }
    #[doc = "AC input source"]
    #[inline]
    pub fn ac(self) -> &'a mut W {
        self.variant(INSEL1W::AC)
    }
    #[doc = "TC input source"]
    #[inline]
    pub fn tc(self) -> &'a mut W {
        self.variant(INSEL1W::TC)
    }
    #[doc = "Alternate TC input source"]
    #[inline]
    pub fn alttc(self) -> &'a mut W {
        self.variant(INSEL1W::ALTTC)
    }
    #[doc = "TCC input source"]
    #[inline]
    pub fn tcc(self) -> &'a mut W {
        self.variant(INSEL1W::TCC)
    }
    #[doc = "SERCOM input source"]
    #[inline]
    pub fn sercom(self) -> &'a mut W {
        self.variant(INSEL1W::SERCOM)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(0x0f << 12);
        self.w.bits |= ((value as u32) & 0x0f) << 12;
        self.w
    }
}
#[doc = "Values that can be written to the field `INSEL2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INSEL2W {
    #[doc = "Masked input"]
    MASK,
    #[doc = "Feedback input source"]
    FEEDBACK,
    #[doc = "Linked LUT input source"]
    LINK,
    #[doc = "Event input source"]
    EVENT,
    #[doc = "I/O pin input source"]
    IO,
    #[doc = "AC input source"]
    AC,
    #[doc = "TC input source"]
    TC,
    #[doc = "Alternate TC input source"]
    ALTTC,
    #[doc = "TCC input source"]
    TCC,
    #[doc = "SERCOM input source"]
    SERCOM,
}
impl INSEL2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            INSEL2W::MASK => 0,
            INSEL2W::FEEDBACK => 1,
            INSEL2W::LINK => 2,
            INSEL2W::EVENT => 3,
            INSEL2W::IO => 4,
            INSEL2W::AC => 5,
            INSEL2W::TC => 6,
            INSEL2W::ALTTC => 7,
            INSEL2W::TCC => 8,
            INSEL2W::SERCOM => 9,
        }
    }
}
#[doc = r" Proxy"]
pub struct _INSEL2W<'a> {
    w: &'a mut W,
}
impl<'a> _INSEL2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: INSEL2W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Masked input"]
    #[inline]
    pub fn mask(self) -> &'a mut W {
        self.variant(INSEL2W::MASK)
    }
    #[doc = "Feedback input source"]
    #[inline]
    pub fn feedback(self) -> &'a mut W {
        self.variant(INSEL2W::FEEDBACK)
    }
    #[doc = "Linked LUT input source"]
    #[inline]
    pub fn link(self) -> &'a mut W {
        self.variant(INSEL2W::LINK)
    }
    #[doc = "Event input source"]
    #[inline]
    pub fn event(self) -> &'a mut W {
        self.variant(INSEL2W::EVENT)
    }
    #[doc = "I/O pin input source"]
    #[inline]
    pub fn io(self) -> &'a mut W {
        self.variant(INSEL2W::IO)
    }
    #[doc = "AC input source"]
    #[inline]
    pub fn ac(self) -> &'a mut W {
        self.variant(INSEL2W::AC)
    }
    #[doc = "TC input source"]
    #[inline]
    pub fn tc(self) -> &'a mut W {
        self.variant(INSEL2W::TC)
    }
    #[doc = "Alternate TC input source"]
    #[inline]
    pub fn alttc(self) -> &'a mut W {
        self.variant(INSEL2W::ALTTC)
    }
    #[doc = "TCC input source"]
    #[inline]
    pub fn tcc(self) -> &'a mut W {
        self.variant(INSEL2W::TCC)
    }
    #[doc = "SERCOM input source"]
    #[inline]
    pub fn sercom(self) -> &'a mut W {
        self.variant(INSEL2W::SERCOM)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(0x0f << 16);
        self.w.bits |= ((value as u32) & 0x0f) << 16;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _INVEIW<'a> {
    w: &'a mut W,
}
impl<'a> _INVEIW<'a> {
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
        self.w.bits &= !(0x01 << 20);
        self.w.bits |= ((value as u32) & 0x01) << 20;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _LUTEIW<'a> {
    w: &'a mut W,
}
impl<'a> _LUTEIW<'a> {
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
        self.w.bits &= !(0x01 << 21);
        self.w.bits |= ((value as u32) & 0x01) << 21;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _LUTEOW<'a> {
    w: &'a mut W,
}
impl<'a> _LUTEOW<'a> {
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
        self.w.bits &= !(0x01 << 22);
        self.w.bits |= ((value as u32) & 0x01) << 22;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TRUTHW<'a> {
    w: &'a mut W,
}
impl<'a> _TRUTHW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(0xff << 24);
        self.w.bits |= ((value as u32) & 0xff) << 24;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 1 - LUT Enable"]
    #[inline]
    pub fn enable(&self) -> ENABLER {
        let bits = ((self.bits >> 1) & 0x01) != 0;
        ENABLER { bits }
    }
    #[doc = "Bits 4:5 - Filter Selection"]
    #[inline]
    pub fn filtsel(&self) -> FILTSELR {
        FILTSELR::_from(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bit 7 - Edge Selection"]
    #[inline]
    pub fn edgesel(&self) -> EDGESELR {
        let bits = ((self.bits >> 7) & 0x01) != 0;
        EDGESELR { bits }
    }
    #[doc = "Bits 8:11 - Input Selection 0"]
    #[inline]
    pub fn insel0(&self) -> INSEL0R {
        INSEL0R::_from(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Input Selection 1"]
    #[inline]
    pub fn insel1(&self) -> INSEL1R {
        INSEL1R::_from(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Input Selection 2"]
    #[inline]
    pub fn insel2(&self) -> INSEL2R {
        INSEL2R::_from(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 20 - Inverted Event Input Enable"]
    #[inline]
    pub fn invei(&self) -> INVEIR {
        let bits = ((self.bits >> 20) & 0x01) != 0;
        INVEIR { bits }
    }
    #[doc = "Bit 21 - LUT Event Input Enable"]
    #[inline]
    pub fn lutei(&self) -> LUTEIR {
        let bits = ((self.bits >> 21) & 0x01) != 0;
        LUTEIR { bits }
    }
    #[doc = "Bit 22 - LUT Event Output Enable"]
    #[inline]
    pub fn luteo(&self) -> LUTEOR {
        let bits = ((self.bits >> 22) & 0x01) != 0;
        LUTEOR { bits }
    }
    #[doc = "Bits 24:31 - Truth Value"]
    #[inline]
    pub fn truth(&self) -> TRUTHR {
        let bits = ((self.bits >> 24) & 0xff) as u8;
        TRUTHR { bits }
    }
}
impl W {
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 1 - LUT Enable"]
    #[inline]
    pub fn enable(&mut self) -> _ENABLEW {
        _ENABLEW { w: self }
    }
    #[doc = "Bits 4:5 - Filter Selection"]
    #[inline]
    pub fn filtsel(&mut self) -> _FILTSELW {
        _FILTSELW { w: self }
    }
    #[doc = "Bit 7 - Edge Selection"]
    #[inline]
    pub fn edgesel(&mut self) -> _EDGESELW {
        _EDGESELW { w: self }
    }
    #[doc = "Bits 8:11 - Input Selection 0"]
    #[inline]
    pub fn insel0(&mut self) -> _INSEL0W {
        _INSEL0W { w: self }
    }
    #[doc = "Bits 12:15 - Input Selection 1"]
    #[inline]
    pub fn insel1(&mut self) -> _INSEL1W {
        _INSEL1W { w: self }
    }
    #[doc = "Bits 16:19 - Input Selection 2"]
    #[inline]
    pub fn insel2(&mut self) -> _INSEL2W {
        _INSEL2W { w: self }
    }
    #[doc = "Bit 20 - Inverted Event Input Enable"]
    #[inline]
    pub fn invei(&mut self) -> _INVEIW {
        _INVEIW { w: self }
    }
    #[doc = "Bit 21 - LUT Event Input Enable"]
    #[inline]
    pub fn lutei(&mut self) -> _LUTEIW {
        _LUTEIW { w: self }
    }
    #[doc = "Bit 22 - LUT Event Output Enable"]
    #[inline]
    pub fn luteo(&mut self) -> _LUTEOW {
        _LUTEOW { w: self }
    }
    #[doc = "Bits 24:31 - Truth Value"]
    #[inline]
    pub fn truth(&mut self) -> _TRUTHW {
        _TRUTHW { w: self }
    }
}
