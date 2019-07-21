#[doc = r" Value read from the register"]
pub struct R {
    bits: u16,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u16,
}
impl super::CTRLB {
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
    pub const fn reset_value() -> u16 {
        0
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.register.set(Self::reset_value())
    }
}
#[doc = r" Value of the field"]
pub struct RESUMER {
    bits: bool,
}
impl RESUMER {
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
#[doc = "Possible values of the field `SPDCONF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPDCONFR {
    #[doc = "Normal mode: the host starts in full-speed mode and performs a high-speed reset to switch to the high speed mode if the downstream peripheral is high-speed capable."]
    NORMAL,
    #[doc = "Full-speed: the host remains in full-speed mode whatever is the peripheral speed capability. Relevant in UTMI mode only."]
    FS,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SPDCONFR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SPDCONFR::NORMAL => 0,
            SPDCONFR::FS => 0x03,
            SPDCONFR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SPDCONFR {
        match value {
            0 => SPDCONFR::NORMAL,
            3 => SPDCONFR::FS,
            i => SPDCONFR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline]
    pub fn is_normal(&self) -> bool {
        *self == SPDCONFR::NORMAL
    }
    #[doc = "Checks if the value of the field is `FS`"]
    #[inline]
    pub fn is_fs(&self) -> bool {
        *self == SPDCONFR::FS
    }
}
#[doc = r" Value of the field"]
pub struct AUTORESUMER {
    bits: bool,
}
impl AUTORESUMER {
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
pub struct TSTJR {
    bits: bool,
}
impl TSTJR {
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
pub struct TSTKR {
    bits: bool,
}
impl TSTKR {
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
pub struct SOFER {
    bits: bool,
}
impl SOFER {
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
pub struct BUSRESETR {
    bits: bool,
}
impl BUSRESETR {
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
pub struct VBUSOKR {
    bits: bool,
}
impl VBUSOKR {
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
pub struct L1RESUMER {
    bits: bool,
}
impl L1RESUMER {
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
pub struct _RESUMEW<'a> {
    w: &'a mut W,
}
impl<'a> _RESUMEW<'a> {
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
        self.w.bits |= ((value as u16) & 0x01) << 1;
        self.w
    }
}
#[doc = "Values that can be written to the field `SPDCONF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPDCONFW {
    #[doc = "Normal mode: the host starts in full-speed mode and performs a high-speed reset to switch to the high speed mode if the downstream peripheral is high-speed capable."]
    NORMAL,
    #[doc = "Full-speed: the host remains in full-speed mode whatever is the peripheral speed capability. Relevant in UTMI mode only."]
    FS,
}
impl SPDCONFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SPDCONFW::NORMAL => 0,
            SPDCONFW::FS => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SPDCONFW<'a> {
    w: &'a mut W,
}
impl<'a> _SPDCONFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SPDCONFW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Normal mode: the host starts in full-speed mode and performs a high-speed reset to switch to the high speed mode if the downstream peripheral is high-speed capable."]
    #[inline]
    pub fn normal(self) -> &'a mut W {
        self.variant(SPDCONFW::NORMAL)
    }
    #[doc = "Full-speed: the host remains in full-speed mode whatever is the peripheral speed capability. Relevant in UTMI mode only."]
    #[inline]
    pub fn fs(self) -> &'a mut W {
        self.variant(SPDCONFW::FS)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(0x03 << 2);
        self.w.bits |= ((value as u16) & 0x03) << 2;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _AUTORESUMEW<'a> {
    w: &'a mut W,
}
impl<'a> _AUTORESUMEW<'a> {
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
        self.w.bits &= !(0x01 << 4);
        self.w.bits |= ((value as u16) & 0x01) << 4;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TSTJW<'a> {
    w: &'a mut W,
}
impl<'a> _TSTJW<'a> {
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
        self.w.bits &= !(0x01 << 5);
        self.w.bits |= ((value as u16) & 0x01) << 5;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TSTKW<'a> {
    w: &'a mut W,
}
impl<'a> _TSTKW<'a> {
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
        self.w.bits &= !(0x01 << 6);
        self.w.bits |= ((value as u16) & 0x01) << 6;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SOFEW<'a> {
    w: &'a mut W,
}
impl<'a> _SOFEW<'a> {
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
        self.w.bits &= !(0x01 << 8);
        self.w.bits |= ((value as u16) & 0x01) << 8;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _BUSRESETW<'a> {
    w: &'a mut W,
}
impl<'a> _BUSRESETW<'a> {
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
        self.w.bits &= !(0x01 << 9);
        self.w.bits |= ((value as u16) & 0x01) << 9;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _VBUSOKW<'a> {
    w: &'a mut W,
}
impl<'a> _VBUSOKW<'a> {
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
        self.w.bits &= !(0x01 << 10);
        self.w.bits |= ((value as u16) & 0x01) << 10;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _L1RESUMEW<'a> {
    w: &'a mut W,
}
impl<'a> _L1RESUMEW<'a> {
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
        self.w.bits &= !(0x01 << 11);
        self.w.bits |= ((value as u16) & 0x01) << 11;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
    #[doc = "Bit 1 - Send USB Resume"]
    #[inline]
    pub fn resume(&self) -> RESUMER {
        let bits = ((self.bits >> 1) & 0x01) != 0;
        RESUMER { bits }
    }
    #[doc = "Bits 2:3 - Speed Configuration for Host"]
    #[inline]
    pub fn spdconf(&self) -> SPDCONFR {
        SPDCONFR::_from(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bit 4 - Auto Resume Enable"]
    #[inline]
    pub fn autoresume(&self) -> AUTORESUMER {
        let bits = ((self.bits >> 4) & 0x01) != 0;
        AUTORESUMER { bits }
    }
    #[doc = "Bit 5 - Test mode J"]
    #[inline]
    pub fn tstj(&self) -> TSTJR {
        let bits = ((self.bits >> 5) & 0x01) != 0;
        TSTJR { bits }
    }
    #[doc = "Bit 6 - Test mode K"]
    #[inline]
    pub fn tstk(&self) -> TSTKR {
        let bits = ((self.bits >> 6) & 0x01) != 0;
        TSTKR { bits }
    }
    #[doc = "Bit 8 - Start of Frame Generation Enable"]
    #[inline]
    pub fn sofe(&self) -> SOFER {
        let bits = ((self.bits >> 8) & 0x01) != 0;
        SOFER { bits }
    }
    #[doc = "Bit 9 - Send USB Reset"]
    #[inline]
    pub fn busreset(&self) -> BUSRESETR {
        let bits = ((self.bits >> 9) & 0x01) != 0;
        BUSRESETR { bits }
    }
    #[doc = "Bit 10 - VBUS is OK"]
    #[inline]
    pub fn vbusok(&self) -> VBUSOKR {
        let bits = ((self.bits >> 10) & 0x01) != 0;
        VBUSOKR { bits }
    }
    #[doc = "Bit 11 - Send L1 Resume"]
    #[inline]
    pub fn l1resume(&self) -> L1RESUMER {
        let bits = ((self.bits >> 11) & 0x01) != 0;
        L1RESUMER { bits }
    }
}
impl W {
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 1 - Send USB Resume"]
    #[inline]
    pub fn resume(&mut self) -> _RESUMEW {
        _RESUMEW { w: self }
    }
    #[doc = "Bits 2:3 - Speed Configuration for Host"]
    #[inline]
    pub fn spdconf(&mut self) -> _SPDCONFW {
        _SPDCONFW { w: self }
    }
    #[doc = "Bit 4 - Auto Resume Enable"]
    #[inline]
    pub fn autoresume(&mut self) -> _AUTORESUMEW {
        _AUTORESUMEW { w: self }
    }
    #[doc = "Bit 5 - Test mode J"]
    #[inline]
    pub fn tstj(&mut self) -> _TSTJW {
        _TSTJW { w: self }
    }
    #[doc = "Bit 6 - Test mode K"]
    #[inline]
    pub fn tstk(&mut self) -> _TSTKW {
        _TSTKW { w: self }
    }
    #[doc = "Bit 8 - Start of Frame Generation Enable"]
    #[inline]
    pub fn sofe(&mut self) -> _SOFEW {
        _SOFEW { w: self }
    }
    #[doc = "Bit 9 - Send USB Reset"]
    #[inline]
    pub fn busreset(&mut self) -> _BUSRESETW {
        _BUSRESETW { w: self }
    }
    #[doc = "Bit 10 - VBUS is OK"]
    #[inline]
    pub fn vbusok(&mut self) -> _VBUSOKW {
        _VBUSOKW { w: self }
    }
    #[doc = "Bit 11 - Send L1 Resume"]
    #[inline]
    pub fn l1resume(&mut self) -> _L1RESUMEW {
        _L1RESUMEW { w: self }
    }
}
