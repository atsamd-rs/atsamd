#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::STATUS {
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
    pub const fn reset_value() -> u8 {
        0x01
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.register.set(Self::reset_value())
    }
}
#[doc = r" Value of the field"]
pub struct STOPR {
    bits: bool,
}
impl STOPR {
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
pub struct SLAVER {
    bits: bool,
}
impl SLAVER {
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
pub struct PERBUFVR {
    bits: bool,
}
impl PERBUFVR {
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
pub struct CCBUFV0R {
    bits: bool,
}
impl CCBUFV0R {
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
pub struct CCBUFV1R {
    bits: bool,
}
impl CCBUFV1R {
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
pub struct _PERBUFVW<'a> {
    w: &'a mut W,
}
impl<'a> _PERBUFVW<'a> {
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
        self.w.bits &= !(0x01 << 3);
        self.w.bits |= ((value as u8) & 0x01) << 3;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CCBUFV0W<'a> {
    w: &'a mut W,
}
impl<'a> _CCBUFV0W<'a> {
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
        self.w.bits |= ((value as u8) & 0x01) << 4;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CCBUFV1W<'a> {
    w: &'a mut W,
}
impl<'a> _CCBUFV1W<'a> {
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
        self.w.bits |= ((value as u8) & 0x01) << 5;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
    #[doc = "Bit 0 - Stop Status Flag"]
    #[inline]
    pub fn stop(&self) -> STOPR {
        let bits = ((self.bits >> 0) & 0x01) != 0;
        STOPR { bits }
    }
    #[doc = "Bit 1 - Slave Status Flag"]
    #[inline]
    pub fn slave(&self) -> SLAVER {
        let bits = ((self.bits >> 1) & 0x01) != 0;
        SLAVER { bits }
    }
    #[doc = "Bit 3 - Synchronization Busy Status"]
    #[inline]
    pub fn perbufv(&self) -> PERBUFVR {
        let bits = ((self.bits >> 3) & 0x01) != 0;
        PERBUFVR { bits }
    }
    #[doc = "Bit 4 - Compare channel buffer 0 valid"]
    #[inline]
    pub fn ccbufv0(&self) -> CCBUFV0R {
        let bits = ((self.bits >> 4) & 0x01) != 0;
        CCBUFV0R { bits }
    }
    #[doc = "Bit 5 - Compare channel buffer 1 valid"]
    #[inline]
    pub fn ccbufv1(&self) -> CCBUFV1R {
        let bits = ((self.bits >> 5) & 0x01) != 0;
        CCBUFV1R { bits }
    }
}
impl W {
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 3 - Synchronization Busy Status"]
    #[inline]
    pub fn perbufv(&mut self) -> _PERBUFVW {
        _PERBUFVW { w: self }
    }
    #[doc = "Bit 4 - Compare channel buffer 0 valid"]
    #[inline]
    pub fn ccbufv0(&mut self) -> _CCBUFV0W {
        _CCBUFV0W { w: self }
    }
    #[doc = "Bit 5 - Compare channel buffer 1 valid"]
    #[inline]
    pub fn ccbufv1(&mut self) -> _CCBUFV1W {
        _CCBUFV1W { w: self }
    }
}
