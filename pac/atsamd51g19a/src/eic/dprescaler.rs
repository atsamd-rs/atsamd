#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::DPRESCALER {
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
pub struct PRESCALER0R {
    bits: u8,
}
impl PRESCALER0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PRESCALER1R {
    bits: u8,
}
impl PRESCALER1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct STATES0R {
    bits: bool,
}
impl STATES0R {
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
pub struct STATES1R {
    bits: bool,
}
impl STATES1R {
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
pub struct TICKONR {
    bits: bool,
}
impl TICKONR {
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
pub struct _PRESCALER0W<'a> {
    w: &'a mut W,
}
impl<'a> _PRESCALER0W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(0x07 << 0);
        self.w.bits |= ((value as u32) & 0x07) << 0;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PRESCALER1W<'a> {
    w: &'a mut W,
}
impl<'a> _PRESCALER1W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(0x07 << 4);
        self.w.bits |= ((value as u32) & 0x07) << 4;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _STATES0W<'a> {
    w: &'a mut W,
}
impl<'a> _STATES0W<'a> {
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
        self.w.bits |= ((value as u32) & 0x01) << 3;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _STATES1W<'a> {
    w: &'a mut W,
}
impl<'a> _STATES1W<'a> {
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
        self.w.bits |= ((value as u32) & 0x01) << 7;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TICKONW<'a> {
    w: &'a mut W,
}
impl<'a> _TICKONW<'a> {
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:2 - Debouncer Prescaler"]
    #[inline]
    pub fn prescaler0(&self) -> PRESCALER0R {
        let bits = ((self.bits >> 0) & 0x07) as u8;
        PRESCALER0R { bits }
    }
    #[doc = "Bits 4:6 - Debouncer Prescaler"]
    #[inline]
    pub fn prescaler1(&self) -> PRESCALER1R {
        let bits = ((self.bits >> 4) & 0x07) as u8;
        PRESCALER1R { bits }
    }
    #[doc = "Bit 3 - Debouncer number of states"]
    #[inline]
    pub fn states0(&self) -> STATES0R {
        let bits = ((self.bits >> 3) & 0x01) != 0;
        STATES0R { bits }
    }
    #[doc = "Bit 7 - Debouncer number of states"]
    #[inline]
    pub fn states1(&self) -> STATES1R {
        let bits = ((self.bits >> 7) & 0x01) != 0;
        STATES1R { bits }
    }
    #[doc = "Bit 16 - Pin Sampler frequency selection"]
    #[inline]
    pub fn tickon(&self) -> TICKONR {
        let bits = ((self.bits >> 16) & 0x01) != 0;
        TICKONR { bits }
    }
}
impl W {
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:2 - Debouncer Prescaler"]
    #[inline]
    pub fn prescaler0(&mut self) -> _PRESCALER0W {
        _PRESCALER0W { w: self }
    }
    #[doc = "Bits 4:6 - Debouncer Prescaler"]
    #[inline]
    pub fn prescaler1(&mut self) -> _PRESCALER1W {
        _PRESCALER1W { w: self }
    }
    #[doc = "Bit 3 - Debouncer number of states"]
    #[inline]
    pub fn states0(&mut self) -> _STATES0W {
        _STATES0W { w: self }
    }
    #[doc = "Bit 7 - Debouncer number of states"]
    #[inline]
    pub fn states1(&mut self) -> _STATES1W {
        _STATES1W { w: self }
    }
    #[doc = "Bit 16 - Pin Sampler frequency selection"]
    #[inline]
    pub fn tickon(&mut self) -> _TICKONW {
        _TICKONW { w: self }
    }
}
