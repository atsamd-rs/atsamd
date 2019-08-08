#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CLKCTRL {
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
#[doc = "Possible values of the field `SLOTSIZE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLOTSIZER {
    #[doc = "8-bit Slot for Clock Unit n"]
    _8,
    #[doc = "16-bit Slot for Clock Unit n"]
    _16,
    #[doc = "24-bit Slot for Clock Unit n"]
    _24,
    #[doc = "32-bit Slot for Clock Unit n"]
    _32,
}
impl SLOTSIZER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SLOTSIZER::_8 => 0,
            SLOTSIZER::_16 => 1,
            SLOTSIZER::_24 => 2,
            SLOTSIZER::_32 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SLOTSIZER {
        match value {
            0 => SLOTSIZER::_8,
            1 => SLOTSIZER::_16,
            2 => SLOTSIZER::_24,
            3 => SLOTSIZER::_32,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_8`"]
    #[inline]
    pub fn is_8(&self) -> bool {
        *self == SLOTSIZER::_8
    }
    #[doc = "Checks if the value of the field is `_16`"]
    #[inline]
    pub fn is_16(&self) -> bool {
        *self == SLOTSIZER::_16
    }
    #[doc = "Checks if the value of the field is `_24`"]
    #[inline]
    pub fn is_24(&self) -> bool {
        *self == SLOTSIZER::_24
    }
    #[doc = "Checks if the value of the field is `_32`"]
    #[inline]
    pub fn is_32(&self) -> bool {
        *self == SLOTSIZER::_32
    }
}
#[doc = r" Value of the field"]
pub struct NBSLOTSR {
    bits: u8,
}
impl NBSLOTSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `FSWIDTH`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FSWIDTHR {
    #[doc = "Frame Sync Pulse is 1 Slot wide (default for I2S protocol)"]
    SLOT,
    #[doc = "Frame Sync Pulse is half a Frame wide"]
    HALF,
    #[doc = "Frame Sync Pulse is 1 Bit wide"]
    BIT,
    #[doc = "Clock Unit n operates in Burst mode, with a 1-bit wide Frame Sync pulse per Data sample, only when Data transfer is requested"]
    BURST,
}
impl FSWIDTHR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            FSWIDTHR::SLOT => 0,
            FSWIDTHR::HALF => 1,
            FSWIDTHR::BIT => 2,
            FSWIDTHR::BURST => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> FSWIDTHR {
        match value {
            0 => FSWIDTHR::SLOT,
            1 => FSWIDTHR::HALF,
            2 => FSWIDTHR::BIT,
            3 => FSWIDTHR::BURST,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SLOT`"]
    #[inline]
    pub fn is_slot(&self) -> bool {
        *self == FSWIDTHR::SLOT
    }
    #[doc = "Checks if the value of the field is `HALF`"]
    #[inline]
    pub fn is_half(&self) -> bool {
        *self == FSWIDTHR::HALF
    }
    #[doc = "Checks if the value of the field is `BIT`"]
    #[inline]
    pub fn is_bit_(&self) -> bool {
        *self == FSWIDTHR::BIT
    }
    #[doc = "Checks if the value of the field is `BURST`"]
    #[inline]
    pub fn is_burst(&self) -> bool {
        *self == FSWIDTHR::BURST
    }
}
#[doc = "Possible values of the field `BITDELAY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BITDELAYR {
    #[doc = "Left Justified (0 Bit Delay)"]
    LJ,
    #[doc = "I2S (1 Bit Delay)"]
    I2S,
}
impl BITDELAYR {
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
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            BITDELAYR::LJ => false,
            BITDELAYR::I2S => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BITDELAYR {
        match value {
            false => BITDELAYR::LJ,
            true => BITDELAYR::I2S,
        }
    }
    #[doc = "Checks if the value of the field is `LJ`"]
    #[inline]
    pub fn is_lj(&self) -> bool {
        *self == BITDELAYR::LJ
    }
    #[doc = "Checks if the value of the field is `I2S`"]
    #[inline]
    pub fn is_i2s(&self) -> bool {
        *self == BITDELAYR::I2S
    }
}
#[doc = "Possible values of the field `FSSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FSSELR {
    #[doc = "Divided Serial Clock n is used as Frame Sync n source"]
    SCKDIV,
    #[doc = "FSn input pin is used as Frame Sync n source"]
    FSPIN,
}
impl FSSELR {
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
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            FSSELR::SCKDIV => false,
            FSSELR::FSPIN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FSSELR {
        match value {
            false => FSSELR::SCKDIV,
            true => FSSELR::FSPIN,
        }
    }
    #[doc = "Checks if the value of the field is `SCKDIV`"]
    #[inline]
    pub fn is_sckdiv(&self) -> bool {
        *self == FSSELR::SCKDIV
    }
    #[doc = "Checks if the value of the field is `FSPIN`"]
    #[inline]
    pub fn is_fspin(&self) -> bool {
        *self == FSSELR::FSPIN
    }
}
#[doc = r" Value of the field"]
pub struct FSINVR {
    bits: bool,
}
impl FSINVR {
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
#[doc = "Possible values of the field `SCKSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SCKSELR {
    #[doc = "Divided Master Clock n is used as Serial Clock n source"]
    MCKDIV,
    #[doc = "SCKn input pin is used as Serial Clock n source"]
    SCKPIN,
}
impl SCKSELR {
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
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            SCKSELR::MCKDIV => false,
            SCKSELR::SCKPIN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SCKSELR {
        match value {
            false => SCKSELR::MCKDIV,
            true => SCKSELR::SCKPIN,
        }
    }
    #[doc = "Checks if the value of the field is `MCKDIV`"]
    #[inline]
    pub fn is_mckdiv(&self) -> bool {
        *self == SCKSELR::MCKDIV
    }
    #[doc = "Checks if the value of the field is `SCKPIN`"]
    #[inline]
    pub fn is_sckpin(&self) -> bool {
        *self == SCKSELR::SCKPIN
    }
}
#[doc = "Possible values of the field `MCKSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MCKSELR {
    #[doc = "GCLK_I2S_n is used as Master Clock n source"]
    GCLK,
    #[doc = "MCKn input pin is used as Master Clock n source"]
    MCKPIN,
}
impl MCKSELR {
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
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            MCKSELR::GCLK => false,
            MCKSELR::MCKPIN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MCKSELR {
        match value {
            false => MCKSELR::GCLK,
            true => MCKSELR::MCKPIN,
        }
    }
    #[doc = "Checks if the value of the field is `GCLK`"]
    #[inline]
    pub fn is_gclk(&self) -> bool {
        *self == MCKSELR::GCLK
    }
    #[doc = "Checks if the value of the field is `MCKPIN`"]
    #[inline]
    pub fn is_mckpin(&self) -> bool {
        *self == MCKSELR::MCKPIN
    }
}
#[doc = r" Value of the field"]
pub struct MCKENR {
    bits: bool,
}
impl MCKENR {
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
pub struct MCKDIVR {
    bits: u8,
}
impl MCKDIVR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct MCKOUTDIVR {
    bits: u8,
}
impl MCKOUTDIVR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct FSOUTINVR {
    bits: bool,
}
impl FSOUTINVR {
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
pub struct SCKOUTINVR {
    bits: bool,
}
impl SCKOUTINVR {
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
pub struct MCKOUTINVR {
    bits: bool,
}
impl MCKOUTINVR {
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
#[doc = "Values that can be written to the field `SLOTSIZE`"]
pub enum SLOTSIZEW {
    #[doc = "8-bit Slot for Clock Unit n"]
    _8,
    #[doc = "16-bit Slot for Clock Unit n"]
    _16,
    #[doc = "24-bit Slot for Clock Unit n"]
    _24,
    #[doc = "32-bit Slot for Clock Unit n"]
    _32,
}
impl SLOTSIZEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SLOTSIZEW::_8 => 0,
            SLOTSIZEW::_16 => 1,
            SLOTSIZEW::_24 => 2,
            SLOTSIZEW::_32 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SLOTSIZEW<'a> {
    w: &'a mut W,
}
impl<'a> _SLOTSIZEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SLOTSIZEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "8-bit Slot for Clock Unit n"]
    #[inline]
    pub fn _8(self) -> &'a mut W {
        self.variant(SLOTSIZEW::_8)
    }
    #[doc = "16-bit Slot for Clock Unit n"]
    #[inline]
    pub fn _16(self) -> &'a mut W {
        self.variant(SLOTSIZEW::_16)
    }
    #[doc = "24-bit Slot for Clock Unit n"]
    #[inline]
    pub fn _24(self) -> &'a mut W {
        self.variant(SLOTSIZEW::_24)
    }
    #[doc = "32-bit Slot for Clock Unit n"]
    #[inline]
    pub fn _32(self) -> &'a mut W {
        self.variant(SLOTSIZEW::_32)
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
pub struct _NBSLOTSW<'a> {
    w: &'a mut W,
}
impl<'a> _NBSLOTSW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `FSWIDTH`"]
pub enum FSWIDTHW {
    #[doc = "Frame Sync Pulse is 1 Slot wide (default for I2S protocol)"]
    SLOT,
    #[doc = "Frame Sync Pulse is half a Frame wide"]
    HALF,
    #[doc = "Frame Sync Pulse is 1 Bit wide"]
    BIT,
    #[doc = "Clock Unit n operates in Burst mode, with a 1-bit wide Frame Sync pulse per Data sample, only when Data transfer is requested"]
    BURST,
}
impl FSWIDTHW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            FSWIDTHW::SLOT => 0,
            FSWIDTHW::HALF => 1,
            FSWIDTHW::BIT => 2,
            FSWIDTHW::BURST => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FSWIDTHW<'a> {
    w: &'a mut W,
}
impl<'a> _FSWIDTHW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FSWIDTHW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Frame Sync Pulse is 1 Slot wide (default for I2S protocol)"]
    #[inline]
    pub fn slot(self) -> &'a mut W {
        self.variant(FSWIDTHW::SLOT)
    }
    #[doc = "Frame Sync Pulse is half a Frame wide"]
    #[inline]
    pub fn half(self) -> &'a mut W {
        self.variant(FSWIDTHW::HALF)
    }
    #[doc = "Frame Sync Pulse is 1 Bit wide"]
    #[inline]
    pub fn bit_(self) -> &'a mut W {
        self.variant(FSWIDTHW::BIT)
    }
    #[doc = "Clock Unit n operates in Burst mode, with a 1-bit wide Frame Sync pulse per Data sample, only when Data transfer is requested"]
    #[inline]
    pub fn burst(self) -> &'a mut W {
        self.variant(FSWIDTHW::BURST)
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
#[doc = "Values that can be written to the field `BITDELAY`"]
pub enum BITDELAYW {
    #[doc = "Left Justified (0 Bit Delay)"]
    LJ,
    #[doc = "I2S (1 Bit Delay)"]
    I2S,
}
impl BITDELAYW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BITDELAYW::LJ => false,
            BITDELAYW::I2S => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BITDELAYW<'a> {
    w: &'a mut W,
}
impl<'a> _BITDELAYW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BITDELAYW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Left Justified (0 Bit Delay)"]
    #[inline]
    pub fn lj(self) -> &'a mut W {
        self.variant(BITDELAYW::LJ)
    }
    #[doc = "I2S (1 Bit Delay)"]
    #[inline]
    pub fn i2s(self) -> &'a mut W {
        self.variant(BITDELAYW::I2S)
    }
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
#[doc = "Values that can be written to the field `FSSEL`"]
pub enum FSSELW {
    #[doc = "Divided Serial Clock n is used as Frame Sync n source"]
    SCKDIV,
    #[doc = "FSn input pin is used as Frame Sync n source"]
    FSPIN,
}
impl FSSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FSSELW::SCKDIV => false,
            FSSELW::FSPIN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FSSELW<'a> {
    w: &'a mut W,
}
impl<'a> _FSSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FSSELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Divided Serial Clock n is used as Frame Sync n source"]
    #[inline]
    pub fn sckdiv(self) -> &'a mut W {
        self.variant(FSSELW::SCKDIV)
    }
    #[doc = "FSn input pin is used as Frame Sync n source"]
    #[inline]
    pub fn fspin(self) -> &'a mut W {
        self.variant(FSSELW::FSPIN)
    }
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
pub struct _FSINVW<'a> {
    w: &'a mut W,
}
impl<'a> _FSINVW<'a> {
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
#[doc = "Values that can be written to the field `SCKSEL`"]
pub enum SCKSELW {
    #[doc = "Divided Master Clock n is used as Serial Clock n source"]
    MCKDIV,
    #[doc = "SCKn input pin is used as Serial Clock n source"]
    SCKPIN,
}
impl SCKSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SCKSELW::MCKDIV => false,
            SCKSELW::SCKPIN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SCKSELW<'a> {
    w: &'a mut W,
}
impl<'a> _SCKSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SCKSELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Divided Master Clock n is used as Serial Clock n source"]
    #[inline]
    pub fn mckdiv(self) -> &'a mut W {
        self.variant(SCKSELW::MCKDIV)
    }
    #[doc = "SCKn input pin is used as Serial Clock n source"]
    #[inline]
    pub fn sckpin(self) -> &'a mut W {
        self.variant(SCKSELW::SCKPIN)
    }
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
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `MCKSEL`"]
pub enum MCKSELW {
    #[doc = "GCLK_I2S_n is used as Master Clock n source"]
    GCLK,
    #[doc = "MCKn input pin is used as Master Clock n source"]
    MCKPIN,
}
impl MCKSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MCKSELW::GCLK => false,
            MCKSELW::MCKPIN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MCKSELW<'a> {
    w: &'a mut W,
}
impl<'a> _MCKSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MCKSELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "GCLK_I2S_n is used as Master Clock n source"]
    #[inline]
    pub fn gclk(self) -> &'a mut W {
        self.variant(MCKSELW::GCLK)
    }
    #[doc = "MCKn input pin is used as Master Clock n source"]
    #[inline]
    pub fn mckpin(self) -> &'a mut W {
        self.variant(MCKSELW::MCKPIN)
    }
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
pub struct _MCKENW<'a> {
    w: &'a mut W,
}
impl<'a> _MCKENW<'a> {
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
pub struct _MCKDIVW<'a> {
    w: &'a mut W,
}
impl<'a> _MCKDIVW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 19;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _MCKOUTDIVW<'a> {
    w: &'a mut W,
}
impl<'a> _MCKOUTDIVW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _FSOUTINVW<'a> {
    w: &'a mut W,
}
impl<'a> _FSOUTINVW<'a> {
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
        const OFFSET: u8 = 29;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SCKOUTINVW<'a> {
    w: &'a mut W,
}
impl<'a> _SCKOUTINVW<'a> {
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
        const OFFSET: u8 = 30;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _MCKOUTINVW<'a> {
    w: &'a mut W,
}
impl<'a> _MCKOUTINVW<'a> {
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
        const OFFSET: u8 = 31;
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
    #[doc = "Bits 0:1 - Slot Size"]
    #[inline]
    pub fn slotsize(&self) -> SLOTSIZER {
        SLOTSIZER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 2:4 - Number of Slots in Frame"]
    #[inline]
    pub fn nbslots(&self) -> NBSLOTSR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        NBSLOTSR { bits }
    }
    #[doc = "Bits 5:6 - Frame Sync Width"]
    #[inline]
    pub fn fswidth(&self) -> FSWIDTHR {
        FSWIDTHR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 7 - Data Delay from Frame Sync"]
    #[inline]
    pub fn bitdelay(&self) -> BITDELAYR {
        BITDELAYR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Frame Sync Select"]
    #[inline]
    pub fn fssel(&self) -> FSSELR {
        FSSELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - Frame Sync Invert"]
    #[inline]
    pub fn fsinv(&self) -> FSINVR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        FSINVR { bits }
    }
    #[doc = "Bit 12 - Serial Clock Select"]
    #[inline]
    pub fn scksel(&self) -> SCKSELR {
        SCKSELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - Master Clock Select"]
    #[inline]
    pub fn mcksel(&self) -> MCKSELR {
        MCKSELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 18 - Master Clock Enable"]
    #[inline]
    pub fn mcken(&self) -> MCKENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        MCKENR { bits }
    }
    #[doc = "Bits 19:23 - Master Clock Division Factor"]
    #[inline]
    pub fn mckdiv(&self) -> MCKDIVR {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        MCKDIVR { bits }
    }
    #[doc = "Bits 24:28 - Master Clock Output Division Factor"]
    #[inline]
    pub fn mckoutdiv(&self) -> MCKOUTDIVR {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        MCKOUTDIVR { bits }
    }
    #[doc = "Bit 29 - Frame Sync Output Invert"]
    #[inline]
    pub fn fsoutinv(&self) -> FSOUTINVR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        FSOUTINVR { bits }
    }
    #[doc = "Bit 30 - Serial Clock Output Invert"]
    #[inline]
    pub fn sckoutinv(&self) -> SCKOUTINVR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SCKOUTINVR { bits }
    }
    #[doc = "Bit 31 - Master Clock Output Invert"]
    #[inline]
    pub fn mckoutinv(&self) -> MCKOUTINVR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        MCKOUTINVR { bits }
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
    #[doc = "Bits 0:1 - Slot Size"]
    #[inline]
    pub fn slotsize(&mut self) -> _SLOTSIZEW {
        _SLOTSIZEW { w: self }
    }
    #[doc = "Bits 2:4 - Number of Slots in Frame"]
    #[inline]
    pub fn nbslots(&mut self) -> _NBSLOTSW {
        _NBSLOTSW { w: self }
    }
    #[doc = "Bits 5:6 - Frame Sync Width"]
    #[inline]
    pub fn fswidth(&mut self) -> _FSWIDTHW {
        _FSWIDTHW { w: self }
    }
    #[doc = "Bit 7 - Data Delay from Frame Sync"]
    #[inline]
    pub fn bitdelay(&mut self) -> _BITDELAYW {
        _BITDELAYW { w: self }
    }
    #[doc = "Bit 8 - Frame Sync Select"]
    #[inline]
    pub fn fssel(&mut self) -> _FSSELW {
        _FSSELW { w: self }
    }
    #[doc = "Bit 11 - Frame Sync Invert"]
    #[inline]
    pub fn fsinv(&mut self) -> _FSINVW {
        _FSINVW { w: self }
    }
    #[doc = "Bit 12 - Serial Clock Select"]
    #[inline]
    pub fn scksel(&mut self) -> _SCKSELW {
        _SCKSELW { w: self }
    }
    #[doc = "Bit 16 - Master Clock Select"]
    #[inline]
    pub fn mcksel(&mut self) -> _MCKSELW {
        _MCKSELW { w: self }
    }
    #[doc = "Bit 18 - Master Clock Enable"]
    #[inline]
    pub fn mcken(&mut self) -> _MCKENW {
        _MCKENW { w: self }
    }
    #[doc = "Bits 19:23 - Master Clock Division Factor"]
    #[inline]
    pub fn mckdiv(&mut self) -> _MCKDIVW {
        _MCKDIVW { w: self }
    }
    #[doc = "Bits 24:28 - Master Clock Output Division Factor"]
    #[inline]
    pub fn mckoutdiv(&mut self) -> _MCKOUTDIVW {
        _MCKOUTDIVW { w: self }
    }
    #[doc = "Bit 29 - Frame Sync Output Invert"]
    #[inline]
    pub fn fsoutinv(&mut self) -> _FSOUTINVW {
        _FSOUTINVW { w: self }
    }
    #[doc = "Bit 30 - Serial Clock Output Invert"]
    #[inline]
    pub fn sckoutinv(&mut self) -> _SCKOUTINVW {
        _SCKOUTINVW { w: self }
    }
    #[doc = "Bit 31 - Master Clock Output Invert"]
    #[inline]
    pub fn mckoutinv(&mut self) -> _MCKOUTINVW {
        _MCKOUTINVW { w: self }
    }
}
