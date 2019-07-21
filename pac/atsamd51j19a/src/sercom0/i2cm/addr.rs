#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::ADDR {
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
pub struct ADDRR {
    bits: u16,
}
impl ADDRR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct LENENR {
    bits: bool,
}
impl LENENR {
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
pub struct HSR {
    bits: bool,
}
impl HSR {
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
pub struct TENBITENR {
    bits: bool,
}
impl TENBITENR {
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
pub struct LENR {
    bits: u8,
}
impl LENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _ADDRW<'a> {
    w: &'a mut W,
}
impl<'a> _ADDRW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits &= !(0x07ff << 0);
        self.w.bits |= ((value as u32) & 0x07ff) << 0;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _LENENW<'a> {
    w: &'a mut W,
}
impl<'a> _LENENW<'a> {
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
        self.w.bits &= !(0x01 << 13);
        self.w.bits |= ((value as u32) & 0x01) << 13;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _HSW<'a> {
    w: &'a mut W,
}
impl<'a> _HSW<'a> {
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
        self.w.bits &= !(0x01 << 14);
        self.w.bits |= ((value as u32) & 0x01) << 14;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TENBITENW<'a> {
    w: &'a mut W,
}
impl<'a> _TENBITENW<'a> {
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
        self.w.bits &= !(0x01 << 15);
        self.w.bits |= ((value as u32) & 0x01) << 15;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _LENW<'a> {
    w: &'a mut W,
}
impl<'a> _LENW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(0xff << 16);
        self.w.bits |= ((value as u32) & 0xff) << 16;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:10 - Address Value"]
    #[inline]
    pub fn addr(&self) -> ADDRR {
        let bits = ((self.bits >> 0) & 0x07ff) as u16;
        ADDRR { bits }
    }
    #[doc = "Bit 13 - Length Enable"]
    #[inline]
    pub fn lenen(&self) -> LENENR {
        let bits = ((self.bits >> 13) & 0x01) != 0;
        LENENR { bits }
    }
    #[doc = "Bit 14 - High Speed Mode"]
    #[inline]
    pub fn hs(&self) -> HSR {
        let bits = ((self.bits >> 14) & 0x01) != 0;
        HSR { bits }
    }
    #[doc = "Bit 15 - Ten Bit Addressing Enable"]
    #[inline]
    pub fn tenbiten(&self) -> TENBITENR {
        let bits = ((self.bits >> 15) & 0x01) != 0;
        TENBITENR { bits }
    }
    #[doc = "Bits 16:23 - Length"]
    #[inline]
    pub fn len(&self) -> LENR {
        let bits = ((self.bits >> 16) & 0xff) as u8;
        LENR { bits }
    }
}
impl W {
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:10 - Address Value"]
    #[inline]
    pub fn addr(&mut self) -> _ADDRW {
        _ADDRW { w: self }
    }
    #[doc = "Bit 13 - Length Enable"]
    #[inline]
    pub fn lenen(&mut self) -> _LENENW {
        _LENENW { w: self }
    }
    #[doc = "Bit 14 - High Speed Mode"]
    #[inline]
    pub fn hs(&mut self) -> _HSW {
        _HSW { w: self }
    }
    #[doc = "Bit 15 - Ten Bit Addressing Enable"]
    #[inline]
    pub fn tenbiten(&mut self) -> _TENBITENW {
        _TENBITENW { w: self }
    }
    #[doc = "Bits 16:23 - Length"]
    #[inline]
    pub fn len(&mut self) -> _LENW {
        _LENW { w: self }
    }
}
