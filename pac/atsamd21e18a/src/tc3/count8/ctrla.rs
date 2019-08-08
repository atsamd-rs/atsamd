#[doc = r" Value read from the register"]
pub struct R {
    bits: u16,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u16,
}
impl super::CTRLA {
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
#[doc = "Possible values of the field `MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODER {
    #[doc = "Counter in 16-bit mode"]
    COUNT16,
    #[doc = "Counter in 8-bit mode"]
    COUNT8,
    #[doc = "Counter in 32-bit mode"]
    COUNT32,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl MODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            MODER::COUNT16 => 0,
            MODER::COUNT8 => 1,
            MODER::COUNT32 => 2,
            MODER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> MODER {
        match value {
            0 => MODER::COUNT16,
            1 => MODER::COUNT8,
            2 => MODER::COUNT32,
            i => MODER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `COUNT16`"]
    #[inline]
    pub fn is_count16(&self) -> bool {
        *self == MODER::COUNT16
    }
    #[doc = "Checks if the value of the field is `COUNT8`"]
    #[inline]
    pub fn is_count8(&self) -> bool {
        *self == MODER::COUNT8
    }
    #[doc = "Checks if the value of the field is `COUNT32`"]
    #[inline]
    pub fn is_count32(&self) -> bool {
        *self == MODER::COUNT32
    }
}
#[doc = "Possible values of the field `WAVEGEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WAVEGENR {
    #[doc = "undocumented"]
    NFRQ,
    #[doc = "undocumented"]
    MFRQ,
    #[doc = "undocumented"]
    NPWM,
    #[doc = "undocumented"]
    MPWM,
}
impl WAVEGENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            WAVEGENR::NFRQ => 0,
            WAVEGENR::MFRQ => 1,
            WAVEGENR::NPWM => 2,
            WAVEGENR::MPWM => 3,
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
            3 => WAVEGENR::MPWM,
            _ => unreachable!(),
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
    #[doc = "Checks if the value of the field is `MPWM`"]
    #[inline]
    pub fn is_mpwm(&self) -> bool {
        *self == WAVEGENR::MPWM
    }
}
#[doc = "Possible values of the field `PRESCALER`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRESCALERR {
    #[doc = "Prescaler: GCLK_TC"]
    DIV1,
    #[doc = "Prescaler: GCLK_TC/2"]
    DIV2,
    #[doc = "Prescaler: GCLK_TC/4"]
    DIV4,
    #[doc = "Prescaler: GCLK_TC/8"]
    DIV8,
    #[doc = "Prescaler: GCLK_TC/16"]
    DIV16,
    #[doc = "Prescaler: GCLK_TC/64"]
    DIV64,
    #[doc = "Prescaler: GCLK_TC/256"]
    DIV256,
    #[doc = "Prescaler: GCLK_TC/1024"]
    DIV1024,
}
impl PRESCALERR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PRESCALERR::DIV1 => 0,
            PRESCALERR::DIV2 => 1,
            PRESCALERR::DIV4 => 2,
            PRESCALERR::DIV8 => 3,
            PRESCALERR::DIV16 => 4,
            PRESCALERR::DIV64 => 5,
            PRESCALERR::DIV256 => 6,
            PRESCALERR::DIV1024 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PRESCALERR {
        match value {
            0 => PRESCALERR::DIV1,
            1 => PRESCALERR::DIV2,
            2 => PRESCALERR::DIV4,
            3 => PRESCALERR::DIV8,
            4 => PRESCALERR::DIV16,
            5 => PRESCALERR::DIV64,
            6 => PRESCALERR::DIV256,
            7 => PRESCALERR::DIV1024,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIV1`"]
    #[inline]
    pub fn is_div1(&self) -> bool {
        *self == PRESCALERR::DIV1
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline]
    pub fn is_div2(&self) -> bool {
        *self == PRESCALERR::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline]
    pub fn is_div4(&self) -> bool {
        *self == PRESCALERR::DIV4
    }
    #[doc = "Checks if the value of the field is `DIV8`"]
    #[inline]
    pub fn is_div8(&self) -> bool {
        *self == PRESCALERR::DIV8
    }
    #[doc = "Checks if the value of the field is `DIV16`"]
    #[inline]
    pub fn is_div16(&self) -> bool {
        *self == PRESCALERR::DIV16
    }
    #[doc = "Checks if the value of the field is `DIV64`"]
    #[inline]
    pub fn is_div64(&self) -> bool {
        *self == PRESCALERR::DIV64
    }
    #[doc = "Checks if the value of the field is `DIV256`"]
    #[inline]
    pub fn is_div256(&self) -> bool {
        *self == PRESCALERR::DIV256
    }
    #[doc = "Checks if the value of the field is `DIV1024`"]
    #[inline]
    pub fn is_div1024(&self) -> bool {
        *self == PRESCALERR::DIV1024
    }
}
#[doc = r" Value of the field"]
pub struct RUNSTDBYR {
    bits: bool,
}
impl RUNSTDBYR {
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
#[doc = "Possible values of the field `PRESCSYNC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRESCSYNCR {
    #[doc = "Reload or reset the counter on next generic clock"]
    GCLK,
    #[doc = "Reload or reset the counter on next prescaler clock"]
    PRESC,
    #[doc = "Reload or reset the counter on next generic clock. Reset the prescaler counter"]
    RESYNC,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PRESCSYNCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PRESCSYNCR::GCLK => 0,
            PRESCSYNCR::PRESC => 1,
            PRESCSYNCR::RESYNC => 2,
            PRESCSYNCR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PRESCSYNCR {
        match value {
            0 => PRESCSYNCR::GCLK,
            1 => PRESCSYNCR::PRESC,
            2 => PRESCSYNCR::RESYNC,
            i => PRESCSYNCR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `GCLK`"]
    #[inline]
    pub fn is_gclk(&self) -> bool {
        *self == PRESCSYNCR::GCLK
    }
    #[doc = "Checks if the value of the field is `PRESC`"]
    #[inline]
    pub fn is_presc(&self) -> bool {
        *self == PRESCSYNCR::PRESC
    }
    #[doc = "Checks if the value of the field is `RESYNC`"]
    #[inline]
    pub fn is_resync(&self) -> bool {
        *self == PRESCSYNCR::RESYNC
    }
}
#[doc = r" Proxy"]
pub struct _SWRSTW<'a> {
    w: &'a mut W,
}
impl<'a> _SWRSTW<'a> {
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
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
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
        const MASK: bool = true;
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `MODE`"]
pub enum MODEW {
    #[doc = "Counter in 16-bit mode"]
    COUNT16,
    #[doc = "Counter in 8-bit mode"]
    COUNT8,
    #[doc = "Counter in 32-bit mode"]
    COUNT32,
}
impl MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            MODEW::COUNT16 => 0,
            MODEW::COUNT8 => 1,
            MODEW::COUNT32 => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _MODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MODEW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Counter in 16-bit mode"]
    #[inline]
    pub fn count16(self) -> &'a mut W {
        self.variant(MODEW::COUNT16)
    }
    #[doc = "Counter in 8-bit mode"]
    #[inline]
    pub fn count8(self) -> &'a mut W {
        self.variant(MODEW::COUNT8)
    }
    #[doc = "Counter in 32-bit mode"]
    #[inline]
    pub fn count32(self) -> &'a mut W {
        self.variant(MODEW::COUNT32)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `WAVEGEN`"]
pub enum WAVEGENW {
    #[doc = "`0`"]
    NFRQ,
    #[doc = "`1`"]
    MFRQ,
    #[doc = "`10`"]
    NPWM,
    #[doc = "`11`"]
    MPWM,
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
            WAVEGENW::MPWM => 3,
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
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "`0`"]
    #[inline]
    pub fn nfrq(self) -> &'a mut W {
        self.variant(WAVEGENW::NFRQ)
    }
    #[doc = "`1`"]
    #[inline]
    pub fn mfrq(self) -> &'a mut W {
        self.variant(WAVEGENW::MFRQ)
    }
    #[doc = "`10`"]
    #[inline]
    pub fn npwm(self) -> &'a mut W {
        self.variant(WAVEGENW::NPWM)
    }
    #[doc = "`11`"]
    #[inline]
    pub fn mpwm(self) -> &'a mut W {
        self.variant(WAVEGENW::MPWM)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PRESCALER`"]
pub enum PRESCALERW {
    #[doc = "Prescaler: GCLK_TC"]
    DIV1,
    #[doc = "Prescaler: GCLK_TC/2"]
    DIV2,
    #[doc = "Prescaler: GCLK_TC/4"]
    DIV4,
    #[doc = "Prescaler: GCLK_TC/8"]
    DIV8,
    #[doc = "Prescaler: GCLK_TC/16"]
    DIV16,
    #[doc = "Prescaler: GCLK_TC/64"]
    DIV64,
    #[doc = "Prescaler: GCLK_TC/256"]
    DIV256,
    #[doc = "Prescaler: GCLK_TC/1024"]
    DIV1024,
}
impl PRESCALERW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PRESCALERW::DIV1 => 0,
            PRESCALERW::DIV2 => 1,
            PRESCALERW::DIV4 => 2,
            PRESCALERW::DIV8 => 3,
            PRESCALERW::DIV16 => 4,
            PRESCALERW::DIV64 => 5,
            PRESCALERW::DIV256 => 6,
            PRESCALERW::DIV1024 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PRESCALERW<'a> {
    w: &'a mut W,
}
impl<'a> _PRESCALERW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PRESCALERW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Prescaler: GCLK_TC"]
    #[inline]
    pub fn div1(self) -> &'a mut W {
        self.variant(PRESCALERW::DIV1)
    }
    #[doc = "Prescaler: GCLK_TC/2"]
    #[inline]
    pub fn div2(self) -> &'a mut W {
        self.variant(PRESCALERW::DIV2)
    }
    #[doc = "Prescaler: GCLK_TC/4"]
    #[inline]
    pub fn div4(self) -> &'a mut W {
        self.variant(PRESCALERW::DIV4)
    }
    #[doc = "Prescaler: GCLK_TC/8"]
    #[inline]
    pub fn div8(self) -> &'a mut W {
        self.variant(PRESCALERW::DIV8)
    }
    #[doc = "Prescaler: GCLK_TC/16"]
    #[inline]
    pub fn div16(self) -> &'a mut W {
        self.variant(PRESCALERW::DIV16)
    }
    #[doc = "Prescaler: GCLK_TC/64"]
    #[inline]
    pub fn div64(self) -> &'a mut W {
        self.variant(PRESCALERW::DIV64)
    }
    #[doc = "Prescaler: GCLK_TC/256"]
    #[inline]
    pub fn div256(self) -> &'a mut W {
        self.variant(PRESCALERW::DIV256)
    }
    #[doc = "Prescaler: GCLK_TC/1024"]
    #[inline]
    pub fn div1024(self) -> &'a mut W {
        self.variant(PRESCALERW::DIV1024)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RUNSTDBYW<'a> {
    w: &'a mut W,
}
impl<'a> _RUNSTDBYW<'a> {
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
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PRESCSYNC`"]
pub enum PRESCSYNCW {
    #[doc = "Reload or reset the counter on next generic clock"]
    GCLK,
    #[doc = "Reload or reset the counter on next prescaler clock"]
    PRESC,
    #[doc = "Reload or reset the counter on next generic clock. Reset the prescaler counter"]
    RESYNC,
}
impl PRESCSYNCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PRESCSYNCW::GCLK => 0,
            PRESCSYNCW::PRESC => 1,
            PRESCSYNCW::RESYNC => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PRESCSYNCW<'a> {
    w: &'a mut W,
}
impl<'a> _PRESCSYNCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PRESCSYNCW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Reload or reset the counter on next generic clock"]
    #[inline]
    pub fn gclk(self) -> &'a mut W {
        self.variant(PRESCSYNCW::GCLK)
    }
    #[doc = "Reload or reset the counter on next prescaler clock"]
    #[inline]
    pub fn presc(self) -> &'a mut W {
        self.variant(PRESCSYNCW::PRESC)
    }
    #[doc = "Reload or reset the counter on next generic clock. Reset the prescaler counter"]
    #[inline]
    pub fn resync(self) -> &'a mut W {
        self.variant(PRESCSYNCW::RESYNC)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
    #[doc = "Bit 1 - Enable"]
    #[inline]
    pub fn enable(&self) -> ENABLER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        };
        ENABLER { bits }
    }
    #[doc = "Bits 2:3 - TC Mode"]
    #[inline]
    pub fn mode(&self) -> MODER {
        MODER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u16) as u8
        })
    }
    #[doc = "Bits 5:6 - Waveform Generation Operation"]
    #[inline]
    pub fn wavegen(&self) -> WAVEGENR {
        WAVEGENR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u16) as u8
        })
    }
    #[doc = "Bits 8:10 - Prescaler"]
    #[inline]
    pub fn prescaler(&self) -> PRESCALERR {
        PRESCALERR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u16) as u8
        })
    }
    #[doc = "Bit 11 - Run in Standby"]
    #[inline]
    pub fn runstdby(&self) -> RUNSTDBYR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        };
        RUNSTDBYR { bits }
    }
    #[doc = "Bits 12:13 - Prescaler and Counter Synchronization"]
    #[inline]
    pub fn prescsync(&self) -> PRESCSYNCR {
        PRESCSYNCR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u16) as u8
        })
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
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Software Reset"]
    #[inline]
    pub fn swrst(&mut self) -> _SWRSTW {
        _SWRSTW { w: self }
    }
    #[doc = "Bit 1 - Enable"]
    #[inline]
    pub fn enable(&mut self) -> _ENABLEW {
        _ENABLEW { w: self }
    }
    #[doc = "Bits 2:3 - TC Mode"]
    #[inline]
    pub fn mode(&mut self) -> _MODEW {
        _MODEW { w: self }
    }
    #[doc = "Bits 5:6 - Waveform Generation Operation"]
    #[inline]
    pub fn wavegen(&mut self) -> _WAVEGENW {
        _WAVEGENW { w: self }
    }
    #[doc = "Bits 8:10 - Prescaler"]
    #[inline]
    pub fn prescaler(&mut self) -> _PRESCALERW {
        _PRESCALERW { w: self }
    }
    #[doc = "Bit 11 - Run in Standby"]
    #[inline]
    pub fn runstdby(&mut self) -> _RUNSTDBYW {
        _RUNSTDBYW { w: self }
    }
    #[doc = "Bits 12:13 - Prescaler and Counter Synchronization"]
    #[inline]
    pub fn prescsync(&mut self) -> _PRESCSYNCW {
        _PRESCSYNCW { w: self }
    }
}
