#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::GENCTRL {
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
pub struct IDR {
    bits: u8,
}
impl IDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `SRC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRCR {
    #[doc = "XOSC oscillator output"]
    XOSC,
    #[doc = "Generator input pad"]
    GCLKIN,
    #[doc = "Generic clock generator 1 output"]
    GCLKGEN1,
    #[doc = "OSCULP32K oscillator output"]
    OSCULP32K,
    #[doc = "OSC32K oscillator output"]
    OSC32K,
    #[doc = "XOSC32K oscillator output"]
    XOSC32K,
    #[doc = "OSC8M oscillator output"]
    OSC8M,
    #[doc = "DFLL48M output"]
    DFLL48M,
    #[doc = "DPLL96M output"]
    DPLL96M,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SRCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SRCR::XOSC => 0,
            SRCR::GCLKIN => 0x01,
            SRCR::GCLKGEN1 => 0x02,
            SRCR::OSCULP32K => 0x03,
            SRCR::OSC32K => 0x04,
            SRCR::XOSC32K => 0x05,
            SRCR::OSC8M => 0x06,
            SRCR::DFLL48M => 0x07,
            SRCR::DPLL96M => 0x08,
            SRCR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SRCR {
        match value {
            0 => SRCR::XOSC,
            1 => SRCR::GCLKIN,
            2 => SRCR::GCLKGEN1,
            3 => SRCR::OSCULP32K,
            4 => SRCR::OSC32K,
            5 => SRCR::XOSC32K,
            6 => SRCR::OSC8M,
            7 => SRCR::DFLL48M,
            8 => SRCR::DPLL96M,
            i => SRCR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `XOSC`"]
    #[inline]
    pub fn is_xosc(&self) -> bool {
        *self == SRCR::XOSC
    }
    #[doc = "Checks if the value of the field is `GCLKIN`"]
    #[inline]
    pub fn is_gclkin(&self) -> bool {
        *self == SRCR::GCLKIN
    }
    #[doc = "Checks if the value of the field is `GCLKGEN1`"]
    #[inline]
    pub fn is_gclkgen1(&self) -> bool {
        *self == SRCR::GCLKGEN1
    }
    #[doc = "Checks if the value of the field is `OSCULP32K`"]
    #[inline]
    pub fn is_osculp32k(&self) -> bool {
        *self == SRCR::OSCULP32K
    }
    #[doc = "Checks if the value of the field is `OSC32K`"]
    #[inline]
    pub fn is_osc32k(&self) -> bool {
        *self == SRCR::OSC32K
    }
    #[doc = "Checks if the value of the field is `XOSC32K`"]
    #[inline]
    pub fn is_xosc32k(&self) -> bool {
        *self == SRCR::XOSC32K
    }
    #[doc = "Checks if the value of the field is `OSC8M`"]
    #[inline]
    pub fn is_osc8m(&self) -> bool {
        *self == SRCR::OSC8M
    }
    #[doc = "Checks if the value of the field is `DFLL48M`"]
    #[inline]
    pub fn is_dfll48m(&self) -> bool {
        *self == SRCR::DFLL48M
    }
    #[doc = "Checks if the value of the field is `DPLL96M`"]
    #[inline]
    pub fn is_dpll96m(&self) -> bool {
        *self == SRCR::DPLL96M
    }
}
#[doc = r" Value of the field"]
pub struct GENENR {
    bits: bool,
}
impl GENENR {
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
pub struct IDCR {
    bits: bool,
}
impl IDCR {
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
pub struct OOVR {
    bits: bool,
}
impl OOVR {
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
pub struct OER {
    bits: bool,
}
impl OER {
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
pub struct DIVSELR {
    bits: bool,
}
impl DIVSELR {
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
#[doc = r" Proxy"]
pub struct _IDW<'a> {
    w: &'a mut W,
}
impl<'a> _IDW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(0x0f << 0);
        self.w.bits |= ((value as u32) & 0x0f) << 0;
        self.w
    }
}
#[doc = "Values that can be written to the field `SRC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRCW {
    #[doc = "XOSC oscillator output"]
    XOSC,
    #[doc = "Generator input pad"]
    GCLKIN,
    #[doc = "Generic clock generator 1 output"]
    GCLKGEN1,
    #[doc = "OSCULP32K oscillator output"]
    OSCULP32K,
    #[doc = "OSC32K oscillator output"]
    OSC32K,
    #[doc = "XOSC32K oscillator output"]
    XOSC32K,
    #[doc = "OSC8M oscillator output"]
    OSC8M,
    #[doc = "DFLL48M output"]
    DFLL48M,
    #[doc = "DPLL96M output"]
    DPLL96M,
}
impl SRCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SRCW::XOSC => 0,
            SRCW::GCLKIN => 1,
            SRCW::GCLKGEN1 => 2,
            SRCW::OSCULP32K => 3,
            SRCW::OSC32K => 4,
            SRCW::XOSC32K => 5,
            SRCW::OSC8M => 6,
            SRCW::DFLL48M => 7,
            SRCW::DPLL96M => 8,
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
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "XOSC oscillator output"]
    #[inline]
    pub fn xosc(self) -> &'a mut W {
        self.variant(SRCW::XOSC)
    }
    #[doc = "Generator input pad"]
    #[inline]
    pub fn gclkin(self) -> &'a mut W {
        self.variant(SRCW::GCLKIN)
    }
    #[doc = "Generic clock generator 1 output"]
    #[inline]
    pub fn gclkgen1(self) -> &'a mut W {
        self.variant(SRCW::GCLKGEN1)
    }
    #[doc = "OSCULP32K oscillator output"]
    #[inline]
    pub fn osculp32k(self) -> &'a mut W {
        self.variant(SRCW::OSCULP32K)
    }
    #[doc = "OSC32K oscillator output"]
    #[inline]
    pub fn osc32k(self) -> &'a mut W {
        self.variant(SRCW::OSC32K)
    }
    #[doc = "XOSC32K oscillator output"]
    #[inline]
    pub fn xosc32k(self) -> &'a mut W {
        self.variant(SRCW::XOSC32K)
    }
    #[doc = "OSC8M oscillator output"]
    #[inline]
    pub fn osc8m(self) -> &'a mut W {
        self.variant(SRCW::OSC8M)
    }
    #[doc = "DFLL48M output"]
    #[inline]
    pub fn dfll48m(self) -> &'a mut W {
        self.variant(SRCW::DFLL48M)
    }
    #[doc = "DPLL96M output"]
    #[inline]
    pub fn dpll96m(self) -> &'a mut W {
        self.variant(SRCW::DPLL96M)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(0x1f << 8);
        self.w.bits |= ((value as u32) & 0x1f) << 8;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _GENENW<'a> {
    w: &'a mut W,
}
impl<'a> _GENENW<'a> {
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
        self.w.bits &= !(0x01 << 16);
        self.w.bits |= ((value as u32) & 0x01) << 16;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _IDCW<'a> {
    w: &'a mut W,
}
impl<'a> _IDCW<'a> {
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
        self.w.bits &= !(0x01 << 17);
        self.w.bits |= ((value as u32) & 0x01) << 17;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _OOVW<'a> {
    w: &'a mut W,
}
impl<'a> _OOVW<'a> {
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
        self.w.bits &= !(0x01 << 18);
        self.w.bits |= ((value as u32) & 0x01) << 18;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _OEW<'a> {
    w: &'a mut W,
}
impl<'a> _OEW<'a> {
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
        self.w.bits &= !(0x01 << 19);
        self.w.bits |= ((value as u32) & 0x01) << 19;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DIVSELW<'a> {
    w: &'a mut W,
}
impl<'a> _DIVSELW<'a> {
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
        self.w.bits &= !(0x01 << 21);
        self.w.bits |= ((value as u32) & 0x01) << 21;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:3 - Generic Clock Generator Selection"]
    #[inline]
    pub fn id(&self) -> IDR {
        let bits = ((self.bits >> 0) & 0x0f) as u8;
        IDR { bits }
    }
    #[doc = "Bits 8:12 - Source Select"]
    #[inline]
    pub fn src(&self) -> SRCR {
        SRCR::_from(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bit 16 - Generic Clock Generator Enable"]
    #[inline]
    pub fn genen(&self) -> GENENR {
        let bits = ((self.bits >> 16) & 0x01) != 0;
        GENENR { bits }
    }
    #[doc = "Bit 17 - Improve Duty Cycle"]
    #[inline]
    pub fn idc(&self) -> IDCR {
        let bits = ((self.bits >> 17) & 0x01) != 0;
        IDCR { bits }
    }
    #[doc = "Bit 18 - Output Off Value"]
    #[inline]
    pub fn oov(&self) -> OOVR {
        let bits = ((self.bits >> 18) & 0x01) != 0;
        OOVR { bits }
    }
    #[doc = "Bit 19 - Output Enable"]
    #[inline]
    pub fn oe(&self) -> OER {
        let bits = ((self.bits >> 19) & 0x01) != 0;
        OER { bits }
    }
    #[doc = "Bit 20 - Divide Selection"]
    #[inline]
    pub fn divsel(&self) -> DIVSELR {
        let bits = ((self.bits >> 20) & 0x01) != 0;
        DIVSELR { bits }
    }
    #[doc = "Bit 21 - Run in Standby"]
    #[inline]
    pub fn runstdby(&self) -> RUNSTDBYR {
        let bits = ((self.bits >> 21) & 0x01) != 0;
        RUNSTDBYR { bits }
    }
}
impl W {
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:3 - Generic Clock Generator Selection"]
    #[inline]
    pub fn id(&mut self) -> _IDW {
        _IDW { w: self }
    }
    #[doc = "Bits 8:12 - Source Select"]
    #[inline]
    pub fn src(&mut self) -> _SRCW {
        _SRCW { w: self }
    }
    #[doc = "Bit 16 - Generic Clock Generator Enable"]
    #[inline]
    pub fn genen(&mut self) -> _GENENW {
        _GENENW { w: self }
    }
    #[doc = "Bit 17 - Improve Duty Cycle"]
    #[inline]
    pub fn idc(&mut self) -> _IDCW {
        _IDCW { w: self }
    }
    #[doc = "Bit 18 - Output Off Value"]
    #[inline]
    pub fn oov(&mut self) -> _OOVW {
        _OOVW { w: self }
    }
    #[doc = "Bit 19 - Output Enable"]
    #[inline]
    pub fn oe(&mut self) -> _OEW {
        _OEW { w: self }
    }
    #[doc = "Bit 20 - Divide Selection"]
    #[inline]
    pub fn divsel(&mut self) -> _DIVSELW {
        _DIVSELW { w: self }
    }
    #[doc = "Bit 21 - Run in Standby"]
    #[inline]
    pub fn runstdby(&mut self) -> _RUNSTDBYW {
        _RUNSTDBYW { w: self }
    }
}
