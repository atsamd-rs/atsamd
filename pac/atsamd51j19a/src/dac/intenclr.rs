#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::INTENCLR {
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
        0
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.register.set(Self::reset_value())
    }
}
#[doc = r" Value of the field"]
pub struct UNDERRUN0R {
    bits: bool,
}
impl UNDERRUN0R {
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
pub struct UNDERRUN1R {
    bits: bool,
}
impl UNDERRUN1R {
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
pub struct EMPTY0R {
    bits: bool,
}
impl EMPTY0R {
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
pub struct EMPTY1R {
    bits: bool,
}
impl EMPTY1R {
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
pub struct RESRDY0R {
    bits: bool,
}
impl RESRDY0R {
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
pub struct RESRDY1R {
    bits: bool,
}
impl RESRDY1R {
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
pub struct OVERRUN0R {
    bits: bool,
}
impl OVERRUN0R {
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
pub struct OVERRUN1R {
    bits: bool,
}
impl OVERRUN1R {
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
pub struct _UNDERRUN0W<'a> {
    w: &'a mut W,
}
impl<'a> _UNDERRUN0W<'a> {
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
        self.w.bits |= ((value as u8) & 0x01) << 0;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _UNDERRUN1W<'a> {
    w: &'a mut W,
}
impl<'a> _UNDERRUN1W<'a> {
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
        self.w.bits |= ((value as u8) & 0x01) << 1;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _EMPTY0W<'a> {
    w: &'a mut W,
}
impl<'a> _EMPTY0W<'a> {
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
        self.w.bits &= !(0x01 << 2);
        self.w.bits |= ((value as u8) & 0x01) << 2;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _EMPTY1W<'a> {
    w: &'a mut W,
}
impl<'a> _EMPTY1W<'a> {
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
pub struct _RESRDY0W<'a> {
    w: &'a mut W,
}
impl<'a> _RESRDY0W<'a> {
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
pub struct _RESRDY1W<'a> {
    w: &'a mut W,
}
impl<'a> _RESRDY1W<'a> {
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
#[doc = r" Proxy"]
pub struct _OVERRUN0W<'a> {
    w: &'a mut W,
}
impl<'a> _OVERRUN0W<'a> {
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
        self.w.bits |= ((value as u8) & 0x01) << 6;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _OVERRUN1W<'a> {
    w: &'a mut W,
}
impl<'a> _OVERRUN1W<'a> {
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
        self.w.bits &= !(0x01 << 7);
        self.w.bits |= ((value as u8) & 0x01) << 7;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
    #[doc = "Bit 0 - Underrun 0 Interrupt Enable"]
    #[inline]
    pub fn underrun0(&self) -> UNDERRUN0R {
        let bits = ((self.bits >> 0) & 0x01) != 0;
        UNDERRUN0R { bits }
    }
    #[doc = "Bit 1 - Underrun 1 Interrupt Enable"]
    #[inline]
    pub fn underrun1(&self) -> UNDERRUN1R {
        let bits = ((self.bits >> 1) & 0x01) != 0;
        UNDERRUN1R { bits }
    }
    #[doc = "Bit 2 - Data Buffer 0 Empty Interrupt Enable"]
    #[inline]
    pub fn empty0(&self) -> EMPTY0R {
        let bits = ((self.bits >> 2) & 0x01) != 0;
        EMPTY0R { bits }
    }
    #[doc = "Bit 3 - Data Buffer 1 Empty Interrupt Enable"]
    #[inline]
    pub fn empty1(&self) -> EMPTY1R {
        let bits = ((self.bits >> 3) & 0x01) != 0;
        EMPTY1R { bits }
    }
    #[doc = "Bit 4 - Result 0 Ready Interrupt Enable"]
    #[inline]
    pub fn resrdy0(&self) -> RESRDY0R {
        let bits = ((self.bits >> 4) & 0x01) != 0;
        RESRDY0R { bits }
    }
    #[doc = "Bit 5 - Result 1 Ready Interrupt Enable"]
    #[inline]
    pub fn resrdy1(&self) -> RESRDY1R {
        let bits = ((self.bits >> 5) & 0x01) != 0;
        RESRDY1R { bits }
    }
    #[doc = "Bit 6 - Overrun 0 Interrupt Enable"]
    #[inline]
    pub fn overrun0(&self) -> OVERRUN0R {
        let bits = ((self.bits >> 6) & 0x01) != 0;
        OVERRUN0R { bits }
    }
    #[doc = "Bit 7 - Overrun 1 Interrupt Enable"]
    #[inline]
    pub fn overrun1(&self) -> OVERRUN1R {
        let bits = ((self.bits >> 7) & 0x01) != 0;
        OVERRUN1R { bits }
    }
}
impl W {
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Underrun 0 Interrupt Enable"]
    #[inline]
    pub fn underrun0(&mut self) -> _UNDERRUN0W {
        _UNDERRUN0W { w: self }
    }
    #[doc = "Bit 1 - Underrun 1 Interrupt Enable"]
    #[inline]
    pub fn underrun1(&mut self) -> _UNDERRUN1W {
        _UNDERRUN1W { w: self }
    }
    #[doc = "Bit 2 - Data Buffer 0 Empty Interrupt Enable"]
    #[inline]
    pub fn empty0(&mut self) -> _EMPTY0W {
        _EMPTY0W { w: self }
    }
    #[doc = "Bit 3 - Data Buffer 1 Empty Interrupt Enable"]
    #[inline]
    pub fn empty1(&mut self) -> _EMPTY1W {
        _EMPTY1W { w: self }
    }
    #[doc = "Bit 4 - Result 0 Ready Interrupt Enable"]
    #[inline]
    pub fn resrdy0(&mut self) -> _RESRDY0W {
        _RESRDY0W { w: self }
    }
    #[doc = "Bit 5 - Result 1 Ready Interrupt Enable"]
    #[inline]
    pub fn resrdy1(&mut self) -> _RESRDY1W {
        _RESRDY1W { w: self }
    }
    #[doc = "Bit 6 - Overrun 0 Interrupt Enable"]
    #[inline]
    pub fn overrun0(&mut self) -> _OVERRUN0W {
        _OVERRUN0W { w: self }
    }
    #[doc = "Bit 7 - Overrun 1 Interrupt Enable"]
    #[inline]
    pub fn overrun1(&mut self) -> _OVERRUN1W {
        _OVERRUN1W { w: self }
    }
}
