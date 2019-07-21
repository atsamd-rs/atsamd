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
pub struct GENCENR {
    bits: bool,
}
impl GENCENR {
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
pub struct ADDRMASKR {
    bits: u16,
}
impl ADDRMASKR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _GENCENW<'a> {
    w: &'a mut W,
}
impl<'a> _GENCENW<'a> {
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
pub struct _ADDRW<'a> {
    w: &'a mut W,
}
impl<'a> _ADDRW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits &= !(0x03ff << 1);
        self.w.bits |= ((value as u32) & 0x03ff) << 1;
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
pub struct _ADDRMASKW<'a> {
    w: &'a mut W,
}
impl<'a> _ADDRMASKW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits &= !(0x03ff << 17);
        self.w.bits |= ((value as u32) & 0x03ff) << 17;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - General Call Address Enable"]
    #[inline]
    pub fn gencen(&self) -> GENCENR {
        let bits = ((self.bits >> 0) & 0x01) != 0;
        GENCENR { bits }
    }
    #[doc = "Bits 1:10 - Address Value"]
    #[inline]
    pub fn addr(&self) -> ADDRR {
        let bits = ((self.bits >> 1) & 0x03ff) as u16;
        ADDRR { bits }
    }
    #[doc = "Bit 15 - Ten Bit Addressing Enable"]
    #[inline]
    pub fn tenbiten(&self) -> TENBITENR {
        let bits = ((self.bits >> 15) & 0x01) != 0;
        TENBITENR { bits }
    }
    #[doc = "Bits 17:26 - Address Mask"]
    #[inline]
    pub fn addrmask(&self) -> ADDRMASKR {
        let bits = ((self.bits >> 17) & 0x03ff) as u16;
        ADDRMASKR { bits }
    }
}
impl W {
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - General Call Address Enable"]
    #[inline]
    pub fn gencen(&mut self) -> _GENCENW {
        _GENCENW { w: self }
    }
    #[doc = "Bits 1:10 - Address Value"]
    #[inline]
    pub fn addr(&mut self) -> _ADDRW {
        _ADDRW { w: self }
    }
    #[doc = "Bit 15 - Ten Bit Addressing Enable"]
    #[inline]
    pub fn tenbiten(&mut self) -> _TENBITENW {
        _TENBITENW { w: self }
    }
    #[doc = "Bits 17:26 - Address Mask"]
    #[inline]
    pub fn addrmask(&mut self) -> _ADDRMASKW {
        _ADDRMASKW { w: self }
    }
}
