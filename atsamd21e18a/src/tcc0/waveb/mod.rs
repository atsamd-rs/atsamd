#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::WAVEB {
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
#[doc = "Possible values of the field `WAVEGENB`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WAVEGENBR {
    #[doc = "Normal frequency"]
    NFRQ,
    #[doc = "Match frequency"]
    MFRQ,
    #[doc = "Normal PWM"]
    NPWM,
    #[doc = "Dual-slope critical"]
    DSCRITICAL,
    #[doc = "Dual-slope with interrupt/event condition when COUNT reaches ZERO"]
    DSBOTTOM,
    #[doc = "Dual-slope with interrupt/event condition when COUNT reaches ZERO or TOP"]
    DSBOTH,
    #[doc = "Dual-slope with interrupt/event condition when COUNT reaches TOP"]
    DSTOP,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl WAVEGENBR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            WAVEGENBR::NFRQ => 0,
            WAVEGENBR::MFRQ => 1,
            WAVEGENBR::NPWM => 2,
            WAVEGENBR::DSCRITICAL => 4,
            WAVEGENBR::DSBOTTOM => 5,
            WAVEGENBR::DSBOTH => 6,
            WAVEGENBR::DSTOP => 7,
            WAVEGENBR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> WAVEGENBR {
        match value {
            0 => WAVEGENBR::NFRQ,
            1 => WAVEGENBR::MFRQ,
            2 => WAVEGENBR::NPWM,
            4 => WAVEGENBR::DSCRITICAL,
            5 => WAVEGENBR::DSBOTTOM,
            6 => WAVEGENBR::DSBOTH,
            7 => WAVEGENBR::DSTOP,
            i => WAVEGENBR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `NFRQ`"]
    #[inline]
    pub fn is_nfrq(&self) -> bool {
        *self == WAVEGENBR::NFRQ
    }
    #[doc = "Checks if the value of the field is `MFRQ`"]
    #[inline]
    pub fn is_mfrq(&self) -> bool {
        *self == WAVEGENBR::MFRQ
    }
    #[doc = "Checks if the value of the field is `NPWM`"]
    #[inline]
    pub fn is_npwm(&self) -> bool {
        *self == WAVEGENBR::NPWM
    }
    #[doc = "Checks if the value of the field is `DSCRITICAL`"]
    #[inline]
    pub fn is_dscritical(&self) -> bool {
        *self == WAVEGENBR::DSCRITICAL
    }
    #[doc = "Checks if the value of the field is `DSBOTTOM`"]
    #[inline]
    pub fn is_dsbottom(&self) -> bool {
        *self == WAVEGENBR::DSBOTTOM
    }
    #[doc = "Checks if the value of the field is `DSBOTH`"]
    #[inline]
    pub fn is_dsboth(&self) -> bool {
        *self == WAVEGENBR::DSBOTH
    }
    #[doc = "Checks if the value of the field is `DSTOP`"]
    #[inline]
    pub fn is_dstop(&self) -> bool {
        *self == WAVEGENBR::DSTOP
    }
}
#[doc = "Possible values of the field `RAMPB`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RAMPBR {
    #[doc = "RAMP1 operation"]
    RAMP1,
    #[doc = "Alternative RAMP2 operation"]
    RAMP2A,
    #[doc = "RAMP2 operation"]
    RAMP2,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl RAMPBR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            RAMPBR::RAMP1 => 0,
            RAMPBR::RAMP2A => 1,
            RAMPBR::RAMP2 => 2,
            RAMPBR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> RAMPBR {
        match value {
            0 => RAMPBR::RAMP1,
            1 => RAMPBR::RAMP2A,
            2 => RAMPBR::RAMP2,
            i => RAMPBR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `RAMP1`"]
    #[inline]
    pub fn is_ramp1(&self) -> bool {
        *self == RAMPBR::RAMP1
    }
    #[doc = "Checks if the value of the field is `RAMP2A`"]
    #[inline]
    pub fn is_ramp2a(&self) -> bool {
        *self == RAMPBR::RAMP2A
    }
    #[doc = "Checks if the value of the field is `RAMP2`"]
    #[inline]
    pub fn is_ramp2(&self) -> bool {
        *self == RAMPBR::RAMP2
    }
}
#[doc = r" Value of the field"]
pub struct CIPERENBR {
    bits: bool,
}
impl CIPERENBR {
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
pub struct CICCENB0R {
    bits: bool,
}
impl CICCENB0R {
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
pub struct CICCENB1R {
    bits: bool,
}
impl CICCENB1R {
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
pub struct CICCENB2R {
    bits: bool,
}
impl CICCENB2R {
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
pub struct CICCENB3R {
    bits: bool,
}
impl CICCENB3R {
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
pub struct POLB0R {
    bits: bool,
}
impl POLB0R {
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
pub struct POLB1R {
    bits: bool,
}
impl POLB1R {
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
pub struct POLB2R {
    bits: bool,
}
impl POLB2R {
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
pub struct POLB3R {
    bits: bool,
}
impl POLB3R {
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
pub struct SWAPB0R {
    bits: bool,
}
impl SWAPB0R {
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
pub struct SWAPB1R {
    bits: bool,
}
impl SWAPB1R {
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
pub struct SWAPB2R {
    bits: bool,
}
impl SWAPB2R {
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
pub struct SWAPB3R {
    bits: bool,
}
impl SWAPB3R {
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
#[doc = "Values that can be written to the field `WAVEGENB`"]
pub enum WAVEGENBW {
    #[doc = "Normal frequency"]
    NFRQ,
    #[doc = "Match frequency"]
    MFRQ,
    #[doc = "Normal PWM"]
    NPWM,
    #[doc = "Dual-slope critical"]
    DSCRITICAL,
    #[doc = "Dual-slope with interrupt/event condition when COUNT reaches ZERO"]
    DSBOTTOM,
    #[doc = "Dual-slope with interrupt/event condition when COUNT reaches ZERO or TOP"]
    DSBOTH,
    #[doc = "Dual-slope with interrupt/event condition when COUNT reaches TOP"]
    DSTOP,
}
impl WAVEGENBW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            WAVEGENBW::NFRQ => 0,
            WAVEGENBW::MFRQ => 1,
            WAVEGENBW::NPWM => 2,
            WAVEGENBW::DSCRITICAL => 4,
            WAVEGENBW::DSBOTTOM => 5,
            WAVEGENBW::DSBOTH => 6,
            WAVEGENBW::DSTOP => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WAVEGENBW<'a> {
    w: &'a mut W,
}
impl<'a> _WAVEGENBW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WAVEGENBW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Normal frequency"]
    #[inline]
    pub fn nfrq(self) -> &'a mut W {
        self.variant(WAVEGENBW::NFRQ)
    }
    #[doc = "Match frequency"]
    #[inline]
    pub fn mfrq(self) -> &'a mut W {
        self.variant(WAVEGENBW::MFRQ)
    }
    #[doc = "Normal PWM"]
    #[inline]
    pub fn npwm(self) -> &'a mut W {
        self.variant(WAVEGENBW::NPWM)
    }
    #[doc = "Dual-slope critical"]
    #[inline]
    pub fn dscritical(self) -> &'a mut W {
        self.variant(WAVEGENBW::DSCRITICAL)
    }
    #[doc = "Dual-slope with interrupt/event condition when COUNT reaches ZERO"]
    #[inline]
    pub fn dsbottom(self) -> &'a mut W {
        self.variant(WAVEGENBW::DSBOTTOM)
    }
    #[doc = "Dual-slope with interrupt/event condition when COUNT reaches ZERO or TOP"]
    #[inline]
    pub fn dsboth(self) -> &'a mut W {
        self.variant(WAVEGENBW::DSBOTH)
    }
    #[doc = "Dual-slope with interrupt/event condition when COUNT reaches TOP"]
    #[inline]
    pub fn dstop(self) -> &'a mut W {
        self.variant(WAVEGENBW::DSTOP)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RAMPB`"]
pub enum RAMPBW {
    #[doc = "RAMP1 operation"]
    RAMP1,
    #[doc = "Alternative RAMP2 operation"]
    RAMP2A,
    #[doc = "RAMP2 operation"]
    RAMP2,
}
impl RAMPBW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            RAMPBW::RAMP1 => 0,
            RAMPBW::RAMP2A => 1,
            RAMPBW::RAMP2 => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RAMPBW<'a> {
    w: &'a mut W,
}
impl<'a> _RAMPBW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RAMPBW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "RAMP1 operation"]
    #[inline]
    pub fn ramp1(self) -> &'a mut W {
        self.variant(RAMPBW::RAMP1)
    }
    #[doc = "Alternative RAMP2 operation"]
    #[inline]
    pub fn ramp2a(self) -> &'a mut W {
        self.variant(RAMPBW::RAMP2A)
    }
    #[doc = "RAMP2 operation"]
    #[inline]
    pub fn ramp2(self) -> &'a mut W {
        self.variant(RAMPBW::RAMP2)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CIPERENBW<'a> {
    w: &'a mut W,
}
impl<'a> _CIPERENBW<'a> {
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
#[doc = r" Proxy"]
pub struct _CICCENB0W<'a> {
    w: &'a mut W,
}
impl<'a> _CICCENB0W<'a> {
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
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CICCENB1W<'a> {
    w: &'a mut W,
}
impl<'a> _CICCENB1W<'a> {
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
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CICCENB2W<'a> {
    w: &'a mut W,
}
impl<'a> _CICCENB2W<'a> {
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
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CICCENB3W<'a> {
    w: &'a mut W,
}
impl<'a> _CICCENB3W<'a> {
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
        const OFFSET: u8 = 11;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _POLB0W<'a> {
    w: &'a mut W,
}
impl<'a> _POLB0W<'a> {
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
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _POLB1W<'a> {
    w: &'a mut W,
}
impl<'a> _POLB1W<'a> {
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
        const OFFSET: u8 = 17;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _POLB2W<'a> {
    w: &'a mut W,
}
impl<'a> _POLB2W<'a> {
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
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _POLB3W<'a> {
    w: &'a mut W,
}
impl<'a> _POLB3W<'a> {
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
        const OFFSET: u8 = 19;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SWAPB0W<'a> {
    w: &'a mut W,
}
impl<'a> _SWAPB0W<'a> {
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
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SWAPB1W<'a> {
    w: &'a mut W,
}
impl<'a> _SWAPB1W<'a> {
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
        const OFFSET: u8 = 25;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SWAPB2W<'a> {
    w: &'a mut W,
}
impl<'a> _SWAPB2W<'a> {
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
        const OFFSET: u8 = 26;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SWAPB3W<'a> {
    w: &'a mut W,
}
impl<'a> _SWAPB3W<'a> {
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
        const OFFSET: u8 = 27;
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
    #[doc = "Bits 0:2 - Waveform Generation Buffer"]
    #[inline]
    pub fn wavegenb(&self) -> WAVEGENBR {
        WAVEGENBR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 4:5 - Ramp Mode Buffer"]
    #[inline]
    pub fn rampb(&self) -> RAMPBR {
        RAMPBR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 7 - Circular Period Enable Buffer"]
    #[inline]
    pub fn ciperenb(&self) -> CIPERENBR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CIPERENBR { bits }
    }
    #[doc = "Bit 8 - Circular Channel 0 Enable Buffer"]
    #[inline]
    pub fn ciccenb0(&self) -> CICCENB0R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CICCENB0R { bits }
    }
    #[doc = "Bit 9 - Circular Channel 1 Enable Buffer"]
    #[inline]
    pub fn ciccenb1(&self) -> CICCENB1R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CICCENB1R { bits }
    }
    #[doc = "Bit 10 - Circular Channel 2 Enable Buffer"]
    #[inline]
    pub fn ciccenb2(&self) -> CICCENB2R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CICCENB2R { bits }
    }
    #[doc = "Bit 11 - Circular Channel 3 Enable Buffer"]
    #[inline]
    pub fn ciccenb3(&self) -> CICCENB3R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CICCENB3R { bits }
    }
    #[doc = "Bit 16 - Channel 0 Polarity Buffer"]
    #[inline]
    pub fn polb0(&self) -> POLB0R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        POLB0R { bits }
    }
    #[doc = "Bit 17 - Channel 1 Polarity Buffer"]
    #[inline]
    pub fn polb1(&self) -> POLB1R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        POLB1R { bits }
    }
    #[doc = "Bit 18 - Channel 2 Polarity Buffer"]
    #[inline]
    pub fn polb2(&self) -> POLB2R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        POLB2R { bits }
    }
    #[doc = "Bit 19 - Channel 3 Polarity Buffer"]
    #[inline]
    pub fn polb3(&self) -> POLB3R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        POLB3R { bits }
    }
    #[doc = "Bit 24 - Swap DTI Output Pair 0 Buffer"]
    #[inline]
    pub fn swapb0(&self) -> SWAPB0R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SWAPB0R { bits }
    }
    #[doc = "Bit 25 - Swap DTI Output Pair 1 Buffer"]
    #[inline]
    pub fn swapb1(&self) -> SWAPB1R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SWAPB1R { bits }
    }
    #[doc = "Bit 26 - Swap DTI Output Pair 2 Buffer"]
    #[inline]
    pub fn swapb2(&self) -> SWAPB2R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SWAPB2R { bits }
    }
    #[doc = "Bit 27 - Swap DTI Output Pair 3 Buffer"]
    #[inline]
    pub fn swapb3(&self) -> SWAPB3R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SWAPB3R { bits }
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
    #[doc = "Bits 0:2 - Waveform Generation Buffer"]
    #[inline]
    pub fn wavegenb(&mut self) -> _WAVEGENBW {
        _WAVEGENBW { w: self }
    }
    #[doc = "Bits 4:5 - Ramp Mode Buffer"]
    #[inline]
    pub fn rampb(&mut self) -> _RAMPBW {
        _RAMPBW { w: self }
    }
    #[doc = "Bit 7 - Circular Period Enable Buffer"]
    #[inline]
    pub fn ciperenb(&mut self) -> _CIPERENBW {
        _CIPERENBW { w: self }
    }
    #[doc = "Bit 8 - Circular Channel 0 Enable Buffer"]
    #[inline]
    pub fn ciccenb0(&mut self) -> _CICCENB0W {
        _CICCENB0W { w: self }
    }
    #[doc = "Bit 9 - Circular Channel 1 Enable Buffer"]
    #[inline]
    pub fn ciccenb1(&mut self) -> _CICCENB1W {
        _CICCENB1W { w: self }
    }
    #[doc = "Bit 10 - Circular Channel 2 Enable Buffer"]
    #[inline]
    pub fn ciccenb2(&mut self) -> _CICCENB2W {
        _CICCENB2W { w: self }
    }
    #[doc = "Bit 11 - Circular Channel 3 Enable Buffer"]
    #[inline]
    pub fn ciccenb3(&mut self) -> _CICCENB3W {
        _CICCENB3W { w: self }
    }
    #[doc = "Bit 16 - Channel 0 Polarity Buffer"]
    #[inline]
    pub fn polb0(&mut self) -> _POLB0W {
        _POLB0W { w: self }
    }
    #[doc = "Bit 17 - Channel 1 Polarity Buffer"]
    #[inline]
    pub fn polb1(&mut self) -> _POLB1W {
        _POLB1W { w: self }
    }
    #[doc = "Bit 18 - Channel 2 Polarity Buffer"]
    #[inline]
    pub fn polb2(&mut self) -> _POLB2W {
        _POLB2W { w: self }
    }
    #[doc = "Bit 19 - Channel 3 Polarity Buffer"]
    #[inline]
    pub fn polb3(&mut self) -> _POLB3W {
        _POLB3W { w: self }
    }
    #[doc = "Bit 24 - Swap DTI Output Pair 0 Buffer"]
    #[inline]
    pub fn swapb0(&mut self) -> _SWAPB0W {
        _SWAPB0W { w: self }
    }
    #[doc = "Bit 25 - Swap DTI Output Pair 1 Buffer"]
    #[inline]
    pub fn swapb1(&mut self) -> _SWAPB1W {
        _SWAPB1W { w: self }
    }
    #[doc = "Bit 26 - Swap DTI Output Pair 2 Buffer"]
    #[inline]
    pub fn swapb2(&mut self) -> _SWAPB2W {
        _SWAPB2W { w: self }
    }
    #[doc = "Bit 27 - Swap DTI Output Pair 3 Buffer"]
    #[inline]
    pub fn swapb3(&mut self) -> _SWAPB3W {
        _SWAPB3W { w: self }
    }
}
