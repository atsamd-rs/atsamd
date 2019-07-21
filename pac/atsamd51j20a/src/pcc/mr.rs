#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::MR {
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
pub struct PCENR {
    bits: bool,
}
impl PCENR {
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
pub struct DSIZER {
    bits: u8,
}
impl DSIZER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct SCALER {
    bits: bool,
}
impl SCALER {
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
pub struct ALWYSR {
    bits: bool,
}
impl ALWYSR {
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
pub struct HALFSR {
    bits: bool,
}
impl HALFSR {
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
pub struct FRSTSR {
    bits: bool,
}
impl FRSTSR {
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
pub struct ISIZER {
    bits: u8,
}
impl ISIZER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CIDR {
    bits: u8,
}
impl CIDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _PCENW<'a> {
    w: &'a mut W,
}
impl<'a> _PCENW<'a> {
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
        self.w.bits &= !(0x01 << 0);
        self.w.bits |= ((value as u32) & 0x01) << 0;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DSIZEW<'a> {
    w: &'a mut W,
}
impl<'a> _DSIZEW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(0x03 << 4);
        self.w.bits |= ((value as u32) & 0x03) << 4;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SCALEW<'a> {
    w: &'a mut W,
}
impl<'a> _SCALEW<'a> {
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
        self.w.bits |= ((value as u32) & 0x01) << 8;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ALWYSW<'a> {
    w: &'a mut W,
}
impl<'a> _ALWYSW<'a> {
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
        self.w.bits |= ((value as u32) & 0x01) << 9;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _HALFSW<'a> {
    w: &'a mut W,
}
impl<'a> _HALFSW<'a> {
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
        self.w.bits |= ((value as u32) & 0x01) << 10;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _FRSTSW<'a> {
    w: &'a mut W,
}
impl<'a> _FRSTSW<'a> {
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
        self.w.bits |= ((value as u32) & 0x01) << 11;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ISIZEW<'a> {
    w: &'a mut W,
}
impl<'a> _ISIZEW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(0x07 << 16);
        self.w.bits |= ((value as u32) & 0x07) << 16;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CIDW<'a> {
    w: &'a mut W,
}
impl<'a> _CIDW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(0x03 << 30);
        self.w.bits |= ((value as u32) & 0x03) << 30;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Parallel Capture Enable"]
    #[inline]
    pub fn pcen(&self) -> PCENR {
        let bits = ((self.bits >> 0) & 0x01) != 0;
        PCENR { bits }
    }
    #[doc = "Bits 4:5 - Data size"]
    #[inline]
    pub fn dsize(&self) -> DSIZER {
        let bits = ((self.bits >> 4) & 0x03) as u8;
        DSIZER { bits }
    }
    #[doc = "Bit 8 - Scale data"]
    #[inline]
    pub fn scale(&self) -> SCALER {
        let bits = ((self.bits >> 8) & 0x01) != 0;
        SCALER { bits }
    }
    #[doc = "Bit 9 - Always Sampling"]
    #[inline]
    pub fn alwys(&self) -> ALWYSR {
        let bits = ((self.bits >> 9) & 0x01) != 0;
        ALWYSR { bits }
    }
    #[doc = "Bit 10 - Half Sampling"]
    #[inline]
    pub fn halfs(&self) -> HALFSR {
        let bits = ((self.bits >> 10) & 0x01) != 0;
        HALFSR { bits }
    }
    #[doc = "Bit 11 - First sample"]
    #[inline]
    pub fn frsts(&self) -> FRSTSR {
        let bits = ((self.bits >> 11) & 0x01) != 0;
        FRSTSR { bits }
    }
    #[doc = "Bits 16:18 - Input Data Size"]
    #[inline]
    pub fn isize(&self) -> ISIZER {
        let bits = ((self.bits >> 16) & 0x07) as u8;
        ISIZER { bits }
    }
    #[doc = "Bits 30:31 - Clear If Disabled"]
    #[inline]
    pub fn cid(&self) -> CIDR {
        let bits = ((self.bits >> 30) & 0x03) as u8;
        CIDR { bits }
    }
}
impl W {
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Parallel Capture Enable"]
    #[inline]
    pub fn pcen(&mut self) -> _PCENW {
        _PCENW { w: self }
    }
    #[doc = "Bits 4:5 - Data size"]
    #[inline]
    pub fn dsize(&mut self) -> _DSIZEW {
        _DSIZEW { w: self }
    }
    #[doc = "Bit 8 - Scale data"]
    #[inline]
    pub fn scale(&mut self) -> _SCALEW {
        _SCALEW { w: self }
    }
    #[doc = "Bit 9 - Always Sampling"]
    #[inline]
    pub fn alwys(&mut self) -> _ALWYSW {
        _ALWYSW { w: self }
    }
    #[doc = "Bit 10 - Half Sampling"]
    #[inline]
    pub fn halfs(&mut self) -> _HALFSW {
        _HALFSW { w: self }
    }
    #[doc = "Bit 11 - First sample"]
    #[inline]
    pub fn frsts(&mut self) -> _FRSTSW {
        _FRSTSW { w: self }
    }
    #[doc = "Bits 16:18 - Input Data Size"]
    #[inline]
    pub fn isize(&mut self) -> _ISIZEW {
        _ISIZEW { w: self }
    }
    #[doc = "Bits 30:31 - Clear If Disabled"]
    #[inline]
    pub fn cid(&mut self) -> _CIDW {
        _CIDW { w: self }
    }
}
