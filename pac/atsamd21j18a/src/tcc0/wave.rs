#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::WAVE {
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
#[doc = "Possible values of the field `WAVEGEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WAVEGENR {
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
impl WAVEGENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            WAVEGENR::NFRQ => 0,
            WAVEGENR::MFRQ => 1,
            WAVEGENR::NPWM => 2,
            WAVEGENR::DSCRITICAL => 4,
            WAVEGENR::DSBOTTOM => 5,
            WAVEGENR::DSBOTH => 6,
            WAVEGENR::DSTOP => 7,
            WAVEGENR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> WAVEGENR {
        match value {
            0 => WAVEGENR::NFRQ,
            1 => WAVEGENR::MFRQ,
            2 => WAVEGENR::NPWM,
            4 => WAVEGENR::DSCRITICAL,
            5 => WAVEGENR::DSBOTTOM,
            6 => WAVEGENR::DSBOTH,
            7 => WAVEGENR::DSTOP,
            i => WAVEGENR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `NFRQ`"]
    #[inline]
    pub fn is_nfrq(&self) -> bool {
        *self == WAVEGENR::NFRQ
    }
    #[doc = "Checks if the value of the field is `MFRQ`"]
    #[inline]
    pub fn is_mfrq(&self) -> bool {
        *self == WAVEGENR::MFRQ
    }
    #[doc = "Checks if the value of the field is `NPWM`"]
    #[inline]
    pub fn is_npwm(&self) -> bool {
        *self == WAVEGENR::NPWM
    }
    #[doc = "Checks if the value of the field is `DSCRITICAL`"]
    #[inline]
    pub fn is_dscritical(&self) -> bool {
        *self == WAVEGENR::DSCRITICAL
    }
    #[doc = "Checks if the value of the field is `DSBOTTOM`"]
    #[inline]
    pub fn is_dsbottom(&self) -> bool {
        *self == WAVEGENR::DSBOTTOM
    }
    #[doc = "Checks if the value of the field is `DSBOTH`"]
    #[inline]
    pub fn is_dsboth(&self) -> bool {
        *self == WAVEGENR::DSBOTH
    }
    #[doc = "Checks if the value of the field is `DSTOP`"]
    #[inline]
    pub fn is_dstop(&self) -> bool {
        *self == WAVEGENR::DSTOP
    }
}
#[doc = "Possible values of the field `RAMP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RAMPR {
    #[doc = "RAMP1 operation"]
    RAMP1,
    #[doc = "Alternative RAMP2 operation"]
    RAMP2A,
    #[doc = "RAMP2 operation"]
    RAMP2,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl RAMPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            RAMPR::RAMP1 => 0,
            RAMPR::RAMP2A => 1,
            RAMPR::RAMP2 => 2,
            RAMPR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> RAMPR {
        match value {
            0 => RAMPR::RAMP1,
            1 => RAMPR::RAMP2A,
            2 => RAMPR::RAMP2,
            i => RAMPR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `RAMP1`"]
    #[inline]
    pub fn is_ramp1(&self) -> bool {
        *self == RAMPR::RAMP1
    }
    #[doc = "Checks if the value of the field is `RAMP2A`"]
    #[inline]
    pub fn is_ramp2a(&self) -> bool {
        *self == RAMPR::RAMP2A
    }
    #[doc = "Checks if the value of the field is `RAMP2`"]
    #[inline]
    pub fn is_ramp2(&self) -> bool {
        *self == RAMPR::RAMP2
    }
}
#[doc = r" Value of the field"]
pub struct CIPERENR {
    bits: bool,
}
impl CIPERENR {
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
pub struct CICCEN0R {
    bits: bool,
}
impl CICCEN0R {
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
pub struct CICCEN1R {
    bits: bool,
}
impl CICCEN1R {
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
pub struct CICCEN2R {
    bits: bool,
}
impl CICCEN2R {
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
pub struct CICCEN3R {
    bits: bool,
}
impl CICCEN3R {
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
pub struct POL0R {
    bits: bool,
}
impl POL0R {
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
pub struct POL1R {
    bits: bool,
}
impl POL1R {
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
pub struct POL2R {
    bits: bool,
}
impl POL2R {
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
pub struct POL3R {
    bits: bool,
}
impl POL3R {
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
pub struct SWAP0R {
    bits: bool,
}
impl SWAP0R {
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
pub struct SWAP1R {
    bits: bool,
}
impl SWAP1R {
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
pub struct SWAP2R {
    bits: bool,
}
impl SWAP2R {
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
pub struct SWAP3R {
    bits: bool,
}
impl SWAP3R {
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
#[doc = "Values that can be written to the field `WAVEGEN`"]
pub enum WAVEGENW {
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
impl WAVEGENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            WAVEGENW::NFRQ => 0,
            WAVEGENW::MFRQ => 1,
            WAVEGENW::NPWM => 2,
            WAVEGENW::DSCRITICAL => 4,
            WAVEGENW::DSBOTTOM => 5,
            WAVEGENW::DSBOTH => 6,
            WAVEGENW::DSTOP => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WAVEGENW<'a> {
    w: &'a mut W,
}
impl<'a> _WAVEGENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WAVEGENW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Normal frequency"]
    #[inline]
    pub fn nfrq(self) -> &'a mut W {
        self.variant(WAVEGENW::NFRQ)
    }
    #[doc = "Match frequency"]
    #[inline]
    pub fn mfrq(self) -> &'a mut W {
        self.variant(WAVEGENW::MFRQ)
    }
    #[doc = "Normal PWM"]
    #[inline]
    pub fn npwm(self) -> &'a mut W {
        self.variant(WAVEGENW::NPWM)
    }
    #[doc = "Dual-slope critical"]
    #[inline]
    pub fn dscritical(self) -> &'a mut W {
        self.variant(WAVEGENW::DSCRITICAL)
    }
    #[doc = "Dual-slope with interrupt/event condition when COUNT reaches ZERO"]
    #[inline]
    pub fn dsbottom(self) -> &'a mut W {
        self.variant(WAVEGENW::DSBOTTOM)
    }
    #[doc = "Dual-slope with interrupt/event condition when COUNT reaches ZERO or TOP"]
    #[inline]
    pub fn dsboth(self) -> &'a mut W {
        self.variant(WAVEGENW::DSBOTH)
    }
    #[doc = "Dual-slope with interrupt/event condition when COUNT reaches TOP"]
    #[inline]
    pub fn dstop(self) -> &'a mut W {
        self.variant(WAVEGENW::DSTOP)
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
#[doc = "Values that can be written to the field `RAMP`"]
pub enum RAMPW {
    #[doc = "RAMP1 operation"]
    RAMP1,
    #[doc = "Alternative RAMP2 operation"]
    RAMP2A,
    #[doc = "RAMP2 operation"]
    RAMP2,
}
impl RAMPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            RAMPW::RAMP1 => 0,
            RAMPW::RAMP2A => 1,
            RAMPW::RAMP2 => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RAMPW<'a> {
    w: &'a mut W,
}
impl<'a> _RAMPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RAMPW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "RAMP1 operation"]
    #[inline]
    pub fn ramp1(self) -> &'a mut W {
        self.variant(RAMPW::RAMP1)
    }
    #[doc = "Alternative RAMP2 operation"]
    #[inline]
    pub fn ramp2a(self) -> &'a mut W {
        self.variant(RAMPW::RAMP2A)
    }
    #[doc = "RAMP2 operation"]
    #[inline]
    pub fn ramp2(self) -> &'a mut W {
        self.variant(RAMPW::RAMP2)
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
pub struct _CIPERENW<'a> {
    w: &'a mut W,
}
impl<'a> _CIPERENW<'a> {
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
pub struct _CICCEN0W<'a> {
    w: &'a mut W,
}
impl<'a> _CICCEN0W<'a> {
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
pub struct _CICCEN1W<'a> {
    w: &'a mut W,
}
impl<'a> _CICCEN1W<'a> {
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
pub struct _CICCEN2W<'a> {
    w: &'a mut W,
}
impl<'a> _CICCEN2W<'a> {
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
pub struct _CICCEN3W<'a> {
    w: &'a mut W,
}
impl<'a> _CICCEN3W<'a> {
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
pub struct _POL0W<'a> {
    w: &'a mut W,
}
impl<'a> _POL0W<'a> {
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
pub struct _POL1W<'a> {
    w: &'a mut W,
}
impl<'a> _POL1W<'a> {
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
pub struct _POL2W<'a> {
    w: &'a mut W,
}
impl<'a> _POL2W<'a> {
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
pub struct _POL3W<'a> {
    w: &'a mut W,
}
impl<'a> _POL3W<'a> {
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
pub struct _SWAP0W<'a> {
    w: &'a mut W,
}
impl<'a> _SWAP0W<'a> {
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
pub struct _SWAP1W<'a> {
    w: &'a mut W,
}
impl<'a> _SWAP1W<'a> {
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
pub struct _SWAP2W<'a> {
    w: &'a mut W,
}
impl<'a> _SWAP2W<'a> {
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
pub struct _SWAP3W<'a> {
    w: &'a mut W,
}
impl<'a> _SWAP3W<'a> {
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
    #[doc = "Bits 0:2 - Waveform Generation"]
    #[inline]
    pub fn wavegen(&self) -> WAVEGENR {
        WAVEGENR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 4:5 - Ramp Mode"]
    #[inline]
    pub fn ramp(&self) -> RAMPR {
        RAMPR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 7 - Circular period Enable"]
    #[inline]
    pub fn ciperen(&self) -> CIPERENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CIPERENR { bits }
    }
    #[doc = "Bit 8 - Circular Channel 0 Enable"]
    #[inline]
    pub fn ciccen0(&self) -> CICCEN0R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CICCEN0R { bits }
    }
    #[doc = "Bit 9 - Circular Channel 1 Enable"]
    #[inline]
    pub fn ciccen1(&self) -> CICCEN1R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CICCEN1R { bits }
    }
    #[doc = "Bit 10 - Circular Channel 2 Enable"]
    #[inline]
    pub fn ciccen2(&self) -> CICCEN2R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CICCEN2R { bits }
    }
    #[doc = "Bit 11 - Circular Channel 3 Enable"]
    #[inline]
    pub fn ciccen3(&self) -> CICCEN3R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CICCEN3R { bits }
    }
    #[doc = "Bit 16 - Channel 0 Polarity"]
    #[inline]
    pub fn pol0(&self) -> POL0R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        POL0R { bits }
    }
    #[doc = "Bit 17 - Channel 1 Polarity"]
    #[inline]
    pub fn pol1(&self) -> POL1R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        POL1R { bits }
    }
    #[doc = "Bit 18 - Channel 2 Polarity"]
    #[inline]
    pub fn pol2(&self) -> POL2R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        POL2R { bits }
    }
    #[doc = "Bit 19 - Channel 3 Polarity"]
    #[inline]
    pub fn pol3(&self) -> POL3R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        POL3R { bits }
    }
    #[doc = "Bit 24 - Swap DTI Output Pair 0"]
    #[inline]
    pub fn swap0(&self) -> SWAP0R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SWAP0R { bits }
    }
    #[doc = "Bit 25 - Swap DTI Output Pair 1"]
    #[inline]
    pub fn swap1(&self) -> SWAP1R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SWAP1R { bits }
    }
    #[doc = "Bit 26 - Swap DTI Output Pair 2"]
    #[inline]
    pub fn swap2(&self) -> SWAP2R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SWAP2R { bits }
    }
    #[doc = "Bit 27 - Swap DTI Output Pair 3"]
    #[inline]
    pub fn swap3(&self) -> SWAP3R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SWAP3R { bits }
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
    #[doc = "Bits 0:2 - Waveform Generation"]
    #[inline]
    pub fn wavegen(&mut self) -> _WAVEGENW {
        _WAVEGENW { w: self }
    }
    #[doc = "Bits 4:5 - Ramp Mode"]
    #[inline]
    pub fn ramp(&mut self) -> _RAMPW {
        _RAMPW { w: self }
    }
    #[doc = "Bit 7 - Circular period Enable"]
    #[inline]
    pub fn ciperen(&mut self) -> _CIPERENW {
        _CIPERENW { w: self }
    }
    #[doc = "Bit 8 - Circular Channel 0 Enable"]
    #[inline]
    pub fn ciccen0(&mut self) -> _CICCEN0W {
        _CICCEN0W { w: self }
    }
    #[doc = "Bit 9 - Circular Channel 1 Enable"]
    #[inline]
    pub fn ciccen1(&mut self) -> _CICCEN1W {
        _CICCEN1W { w: self }
    }
    #[doc = "Bit 10 - Circular Channel 2 Enable"]
    #[inline]
    pub fn ciccen2(&mut self) -> _CICCEN2W {
        _CICCEN2W { w: self }
    }
    #[doc = "Bit 11 - Circular Channel 3 Enable"]
    #[inline]
    pub fn ciccen3(&mut self) -> _CICCEN3W {
        _CICCEN3W { w: self }
    }
    #[doc = "Bit 16 - Channel 0 Polarity"]
    #[inline]
    pub fn pol0(&mut self) -> _POL0W {
        _POL0W { w: self }
    }
    #[doc = "Bit 17 - Channel 1 Polarity"]
    #[inline]
    pub fn pol1(&mut self) -> _POL1W {
        _POL1W { w: self }
    }
    #[doc = "Bit 18 - Channel 2 Polarity"]
    #[inline]
    pub fn pol2(&mut self) -> _POL2W {
        _POL2W { w: self }
    }
    #[doc = "Bit 19 - Channel 3 Polarity"]
    #[inline]
    pub fn pol3(&mut self) -> _POL3W {
        _POL3W { w: self }
    }
    #[doc = "Bit 24 - Swap DTI Output Pair 0"]
    #[inline]
    pub fn swap0(&mut self) -> _SWAP0W {
        _SWAP0W { w: self }
    }
    #[doc = "Bit 25 - Swap DTI Output Pair 1"]
    #[inline]
    pub fn swap1(&mut self) -> _SWAP1W {
        _SWAP1W { w: self }
    }
    #[doc = "Bit 26 - Swap DTI Output Pair 2"]
    #[inline]
    pub fn swap2(&mut self) -> _SWAP2W {
        _SWAP2W { w: self }
    }
    #[doc = "Bit 27 - Swap DTI Output Pair 3"]
    #[inline]
    pub fn swap3(&mut self) -> _SWAP3W {
        _SWAP3W { w: self }
    }
}
