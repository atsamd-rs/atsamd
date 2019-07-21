#[doc = r" Value read from the register"]
pub struct R {
    bits: u16,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u16,
}
impl super::EVCTRL {
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
pub struct COMPEO0R {
    bits: bool,
}
impl COMPEO0R {
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
pub struct COMPEO1R {
    bits: bool,
}
impl COMPEO1R {
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
pub struct WINEO0R {
    bits: bool,
}
impl WINEO0R {
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
pub struct COMPEI0R {
    bits: bool,
}
impl COMPEI0R {
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
pub struct COMPEI1R {
    bits: bool,
}
impl COMPEI1R {
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
pub struct INVEI0R {
    bits: bool,
}
impl INVEI0R {
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
pub struct INVEI1R {
    bits: bool,
}
impl INVEI1R {
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
pub struct _COMPEO0W<'a> {
    w: &'a mut W,
}
impl<'a> _COMPEO0W<'a> {
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
pub struct _COMPEO1W<'a> {
    w: &'a mut W,
}
impl<'a> _COMPEO1W<'a> {
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
pub struct _WINEO0W<'a> {
    w: &'a mut W,
}
impl<'a> _WINEO0W<'a> {
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
pub struct _COMPEI0W<'a> {
    w: &'a mut W,
}
impl<'a> _COMPEI0W<'a> {
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
pub struct _COMPEI1W<'a> {
    w: &'a mut W,
}
impl<'a> _COMPEI1W<'a> {
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
pub struct _INVEI0W<'a> {
    w: &'a mut W,
}
impl<'a> _INVEI0W<'a> {
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
pub struct _INVEI1W<'a> {
    w: &'a mut W,
}
impl<'a> _INVEI1W<'a> {
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
    #[doc = "Bit 0 - Comparator 0 Event Output Enable"]
    #[inline]
    pub fn compeo0(&self) -> COMPEO0R {
        let bits = ((self.bits >> 0) & 0x01) != 0;
        COMPEO0R { bits }
    }
    #[doc = "Bit 1 - Comparator 1 Event Output Enable"]
    #[inline]
    pub fn compeo1(&self) -> COMPEO1R {
        let bits = ((self.bits >> 1) & 0x01) != 0;
        COMPEO1R { bits }
    }
    #[doc = "Bit 4 - Window 0 Event Output Enable"]
    #[inline]
    pub fn wineo0(&self) -> WINEO0R {
        let bits = ((self.bits >> 4) & 0x01) != 0;
        WINEO0R { bits }
    }
    #[doc = "Bit 8 - Comparator 0 Event Input Enable"]
    #[inline]
    pub fn compei0(&self) -> COMPEI0R {
        let bits = ((self.bits >> 8) & 0x01) != 0;
        COMPEI0R { bits }
    }
    #[doc = "Bit 9 - Comparator 1 Event Input Enable"]
    #[inline]
    pub fn compei1(&self) -> COMPEI1R {
        let bits = ((self.bits >> 9) & 0x01) != 0;
        COMPEI1R { bits }
    }
    #[doc = "Bit 12 - Comparator 0 Input Event Invert Enable"]
    #[inline]
    pub fn invei0(&self) -> INVEI0R {
        let bits = ((self.bits >> 12) & 0x01) != 0;
        INVEI0R { bits }
    }
    #[doc = "Bit 13 - Comparator 1 Input Event Invert Enable"]
    #[inline]
    pub fn invei1(&self) -> INVEI1R {
        let bits = ((self.bits >> 13) & 0x01) != 0;
        INVEI1R { bits }
    }
}
impl W {
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Comparator 0 Event Output Enable"]
    #[inline]
    pub fn compeo0(&mut self) -> _COMPEO0W {
        _COMPEO0W { w: self }
    }
    #[doc = "Bit 1 - Comparator 1 Event Output Enable"]
    #[inline]
    pub fn compeo1(&mut self) -> _COMPEO1W {
        _COMPEO1W { w: self }
    }
    #[doc = "Bit 4 - Window 0 Event Output Enable"]
    #[inline]
    pub fn wineo0(&mut self) -> _WINEO0W {
        _WINEO0W { w: self }
    }
    #[doc = "Bit 8 - Comparator 0 Event Input Enable"]
    #[inline]
    pub fn compei0(&mut self) -> _COMPEI0W {
        _COMPEI0W { w: self }
    }
    #[doc = "Bit 9 - Comparator 1 Event Input Enable"]
    #[inline]
    pub fn compei1(&mut self) -> _COMPEI1W {
        _COMPEI1W { w: self }
    }
    #[doc = "Bit 12 - Comparator 0 Input Event Invert Enable"]
    #[inline]
    pub fn invei0(&mut self) -> _INVEI0W {
        _INVEI0W { w: self }
    }
    #[doc = "Bit 13 - Comparator 1 Input Event Invert Enable"]
    #[inline]
    pub fn invei1(&mut self) -> _INVEI1W {
        _INVEI1W { w: self }
    }
}
