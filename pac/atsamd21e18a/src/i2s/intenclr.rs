#[doc = r" Value read from the register"]
pub struct R {
    bits: u16,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u16,
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
pub struct RXRDY0R {
    bits: bool,
}
impl RXRDY0R {
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
pub struct RXRDY1R {
    bits: bool,
}
impl RXRDY1R {
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
pub struct RXOR0R {
    bits: bool,
}
impl RXOR0R {
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
pub struct RXOR1R {
    bits: bool,
}
impl RXOR1R {
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
pub struct TXRDY0R {
    bits: bool,
}
impl TXRDY0R {
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
pub struct TXRDY1R {
    bits: bool,
}
impl TXRDY1R {
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
pub struct TXUR0R {
    bits: bool,
}
impl TXUR0R {
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
pub struct TXUR1R {
    bits: bool,
}
impl TXUR1R {
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
pub struct _RXRDY0W<'a> {
    w: &'a mut W,
}
impl<'a> _RXRDY0W<'a> {
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
        self.w.bits |= ((value as u16) & 0x01) << 0;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RXRDY1W<'a> {
    w: &'a mut W,
}
impl<'a> _RXRDY1W<'a> {
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
#[doc = r" Proxy"]
pub struct _RXOR0W<'a> {
    w: &'a mut W,
}
impl<'a> _RXOR0W<'a> {
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
pub struct _RXOR1W<'a> {
    w: &'a mut W,
}
impl<'a> _RXOR1W<'a> {
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
pub struct _TXRDY0W<'a> {
    w: &'a mut W,
}
impl<'a> _TXRDY0W<'a> {
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
pub struct _TXRDY1W<'a> {
    w: &'a mut W,
}
impl<'a> _TXRDY1W<'a> {
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
pub struct _TXUR0W<'a> {
    w: &'a mut W,
}
impl<'a> _TXUR0W<'a> {
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
        self.w.bits &= !(0x01 << 12);
        self.w.bits |= ((value as u16) & 0x01) << 12;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TXUR1W<'a> {
    w: &'a mut W,
}
impl<'a> _TXUR1W<'a> {
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
        self.w.bits |= ((value as u16) & 0x01) << 13;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
    #[doc = "Bit 0 - Receive Ready 0 Interrupt Enable"]
    #[inline]
    pub fn rxrdy0(&self) -> RXRDY0R {
        let bits = ((self.bits >> 0) & 0x01) != 0;
        RXRDY0R { bits }
    }
    #[doc = "Bit 1 - Receive Ready 1 Interrupt Enable"]
    #[inline]
    pub fn rxrdy1(&self) -> RXRDY1R {
        let bits = ((self.bits >> 1) & 0x01) != 0;
        RXRDY1R { bits }
    }
    #[doc = "Bit 4 - Receive Overrun 0 Interrupt Enable"]
    #[inline]
    pub fn rxor0(&self) -> RXOR0R {
        let bits = ((self.bits >> 4) & 0x01) != 0;
        RXOR0R { bits }
    }
    #[doc = "Bit 5 - Receive Overrun 1 Interrupt Enable"]
    #[inline]
    pub fn rxor1(&self) -> RXOR1R {
        let bits = ((self.bits >> 5) & 0x01) != 0;
        RXOR1R { bits }
    }
    #[doc = "Bit 8 - Transmit Ready 0 Interrupt Enable"]
    #[inline]
    pub fn txrdy0(&self) -> TXRDY0R {
        let bits = ((self.bits >> 8) & 0x01) != 0;
        TXRDY0R { bits }
    }
    #[doc = "Bit 9 - Transmit Ready 1 Interrupt Enable"]
    #[inline]
    pub fn txrdy1(&self) -> TXRDY1R {
        let bits = ((self.bits >> 9) & 0x01) != 0;
        TXRDY1R { bits }
    }
    #[doc = "Bit 12 - Transmit Underrun 0 Interrupt Enable"]
    #[inline]
    pub fn txur0(&self) -> TXUR0R {
        let bits = ((self.bits >> 12) & 0x01) != 0;
        TXUR0R { bits }
    }
    #[doc = "Bit 13 - Transmit Underrun 1 Interrupt Enable"]
    #[inline]
    pub fn txur1(&self) -> TXUR1R {
        let bits = ((self.bits >> 13) & 0x01) != 0;
        TXUR1R { bits }
    }
}
impl W {
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Receive Ready 0 Interrupt Enable"]
    #[inline]
    pub fn rxrdy0(&mut self) -> _RXRDY0W {
        _RXRDY0W { w: self }
    }
    #[doc = "Bit 1 - Receive Ready 1 Interrupt Enable"]
    #[inline]
    pub fn rxrdy1(&mut self) -> _RXRDY1W {
        _RXRDY1W { w: self }
    }
    #[doc = "Bit 4 - Receive Overrun 0 Interrupt Enable"]
    #[inline]
    pub fn rxor0(&mut self) -> _RXOR0W {
        _RXOR0W { w: self }
    }
    #[doc = "Bit 5 - Receive Overrun 1 Interrupt Enable"]
    #[inline]
    pub fn rxor1(&mut self) -> _RXOR1W {
        _RXOR1W { w: self }
    }
    #[doc = "Bit 8 - Transmit Ready 0 Interrupt Enable"]
    #[inline]
    pub fn txrdy0(&mut self) -> _TXRDY0W {
        _TXRDY0W { w: self }
    }
    #[doc = "Bit 9 - Transmit Ready 1 Interrupt Enable"]
    #[inline]
    pub fn txrdy1(&mut self) -> _TXRDY1W {
        _TXRDY1W { w: self }
    }
    #[doc = "Bit 12 - Transmit Underrun 0 Interrupt Enable"]
    #[inline]
    pub fn txur0(&mut self) -> _TXUR0W {
        _TXUR0W { w: self }
    }
    #[doc = "Bit 13 - Transmit Underrun 1 Interrupt Enable"]
    #[inline]
    pub fn txur1(&mut self) -> _TXUR1W {
        _TXUR1W { w: self }
    }
}
