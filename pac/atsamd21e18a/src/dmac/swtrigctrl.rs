#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SWTRIGCTRL {
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
pub struct SWTRIG0R {
    bits: bool,
}
impl SWTRIG0R {
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
pub struct SWTRIG1R {
    bits: bool,
}
impl SWTRIG1R {
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
pub struct SWTRIG2R {
    bits: bool,
}
impl SWTRIG2R {
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
pub struct SWTRIG3R {
    bits: bool,
}
impl SWTRIG3R {
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
pub struct SWTRIG4R {
    bits: bool,
}
impl SWTRIG4R {
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
pub struct SWTRIG5R {
    bits: bool,
}
impl SWTRIG5R {
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
pub struct SWTRIG6R {
    bits: bool,
}
impl SWTRIG6R {
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
pub struct SWTRIG7R {
    bits: bool,
}
impl SWTRIG7R {
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
pub struct SWTRIG8R {
    bits: bool,
}
impl SWTRIG8R {
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
pub struct SWTRIG9R {
    bits: bool,
}
impl SWTRIG9R {
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
pub struct SWTRIG10R {
    bits: bool,
}
impl SWTRIG10R {
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
pub struct SWTRIG11R {
    bits: bool,
}
impl SWTRIG11R {
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
pub struct _SWTRIG0W<'a> {
    w: &'a mut W,
}
impl<'a> _SWTRIG0W<'a> {
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
pub struct _SWTRIG1W<'a> {
    w: &'a mut W,
}
impl<'a> _SWTRIG1W<'a> {
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
        self.w.bits |= ((value as u32) & 0x01) << 1;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SWTRIG2W<'a> {
    w: &'a mut W,
}
impl<'a> _SWTRIG2W<'a> {
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
        self.w.bits |= ((value as u32) & 0x01) << 2;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SWTRIG3W<'a> {
    w: &'a mut W,
}
impl<'a> _SWTRIG3W<'a> {
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
pub struct _SWTRIG4W<'a> {
    w: &'a mut W,
}
impl<'a> _SWTRIG4W<'a> {
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
        self.w.bits |= ((value as u32) & 0x01) << 4;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SWTRIG5W<'a> {
    w: &'a mut W,
}
impl<'a> _SWTRIG5W<'a> {
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
        self.w.bits |= ((value as u32) & 0x01) << 5;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SWTRIG6W<'a> {
    w: &'a mut W,
}
impl<'a> _SWTRIG6W<'a> {
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
        self.w.bits |= ((value as u32) & 0x01) << 6;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SWTRIG7W<'a> {
    w: &'a mut W,
}
impl<'a> _SWTRIG7W<'a> {
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
pub struct _SWTRIG8W<'a> {
    w: &'a mut W,
}
impl<'a> _SWTRIG8W<'a> {
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
pub struct _SWTRIG9W<'a> {
    w: &'a mut W,
}
impl<'a> _SWTRIG9W<'a> {
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
pub struct _SWTRIG10W<'a> {
    w: &'a mut W,
}
impl<'a> _SWTRIG10W<'a> {
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
pub struct _SWTRIG11W<'a> {
    w: &'a mut W,
}
impl<'a> _SWTRIG11W<'a> {
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Channel 0 Software Trigger"]
    #[inline]
    pub fn swtrig0(&self) -> SWTRIG0R {
        let bits = ((self.bits >> 0) & 0x01) != 0;
        SWTRIG0R { bits }
    }
    #[doc = "Bit 1 - Channel 1 Software Trigger"]
    #[inline]
    pub fn swtrig1(&self) -> SWTRIG1R {
        let bits = ((self.bits >> 1) & 0x01) != 0;
        SWTRIG1R { bits }
    }
    #[doc = "Bit 2 - Channel 2 Software Trigger"]
    #[inline]
    pub fn swtrig2(&self) -> SWTRIG2R {
        let bits = ((self.bits >> 2) & 0x01) != 0;
        SWTRIG2R { bits }
    }
    #[doc = "Bit 3 - Channel 3 Software Trigger"]
    #[inline]
    pub fn swtrig3(&self) -> SWTRIG3R {
        let bits = ((self.bits >> 3) & 0x01) != 0;
        SWTRIG3R { bits }
    }
    #[doc = "Bit 4 - Channel 4 Software Trigger"]
    #[inline]
    pub fn swtrig4(&self) -> SWTRIG4R {
        let bits = ((self.bits >> 4) & 0x01) != 0;
        SWTRIG4R { bits }
    }
    #[doc = "Bit 5 - Channel 5 Software Trigger"]
    #[inline]
    pub fn swtrig5(&self) -> SWTRIG5R {
        let bits = ((self.bits >> 5) & 0x01) != 0;
        SWTRIG5R { bits }
    }
    #[doc = "Bit 6 - Channel 6 Software Trigger"]
    #[inline]
    pub fn swtrig6(&self) -> SWTRIG6R {
        let bits = ((self.bits >> 6) & 0x01) != 0;
        SWTRIG6R { bits }
    }
    #[doc = "Bit 7 - Channel 7 Software Trigger"]
    #[inline]
    pub fn swtrig7(&self) -> SWTRIG7R {
        let bits = ((self.bits >> 7) & 0x01) != 0;
        SWTRIG7R { bits }
    }
    #[doc = "Bit 8 - Channel 8 Software Trigger"]
    #[inline]
    pub fn swtrig8(&self) -> SWTRIG8R {
        let bits = ((self.bits >> 8) & 0x01) != 0;
        SWTRIG8R { bits }
    }
    #[doc = "Bit 9 - Channel 9 Software Trigger"]
    #[inline]
    pub fn swtrig9(&self) -> SWTRIG9R {
        let bits = ((self.bits >> 9) & 0x01) != 0;
        SWTRIG9R { bits }
    }
    #[doc = "Bit 10 - Channel 10 Software Trigger"]
    #[inline]
    pub fn swtrig10(&self) -> SWTRIG10R {
        let bits = ((self.bits >> 10) & 0x01) != 0;
        SWTRIG10R { bits }
    }
    #[doc = "Bit 11 - Channel 11 Software Trigger"]
    #[inline]
    pub fn swtrig11(&self) -> SWTRIG11R {
        let bits = ((self.bits >> 11) & 0x01) != 0;
        SWTRIG11R { bits }
    }
}
impl W {
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Channel 0 Software Trigger"]
    #[inline]
    pub fn swtrig0(&mut self) -> _SWTRIG0W {
        _SWTRIG0W { w: self }
    }
    #[doc = "Bit 1 - Channel 1 Software Trigger"]
    #[inline]
    pub fn swtrig1(&mut self) -> _SWTRIG1W {
        _SWTRIG1W { w: self }
    }
    #[doc = "Bit 2 - Channel 2 Software Trigger"]
    #[inline]
    pub fn swtrig2(&mut self) -> _SWTRIG2W {
        _SWTRIG2W { w: self }
    }
    #[doc = "Bit 3 - Channel 3 Software Trigger"]
    #[inline]
    pub fn swtrig3(&mut self) -> _SWTRIG3W {
        _SWTRIG3W { w: self }
    }
    #[doc = "Bit 4 - Channel 4 Software Trigger"]
    #[inline]
    pub fn swtrig4(&mut self) -> _SWTRIG4W {
        _SWTRIG4W { w: self }
    }
    #[doc = "Bit 5 - Channel 5 Software Trigger"]
    #[inline]
    pub fn swtrig5(&mut self) -> _SWTRIG5W {
        _SWTRIG5W { w: self }
    }
    #[doc = "Bit 6 - Channel 6 Software Trigger"]
    #[inline]
    pub fn swtrig6(&mut self) -> _SWTRIG6W {
        _SWTRIG6W { w: self }
    }
    #[doc = "Bit 7 - Channel 7 Software Trigger"]
    #[inline]
    pub fn swtrig7(&mut self) -> _SWTRIG7W {
        _SWTRIG7W { w: self }
    }
    #[doc = "Bit 8 - Channel 8 Software Trigger"]
    #[inline]
    pub fn swtrig8(&mut self) -> _SWTRIG8W {
        _SWTRIG8W { w: self }
    }
    #[doc = "Bit 9 - Channel 9 Software Trigger"]
    #[inline]
    pub fn swtrig9(&mut self) -> _SWTRIG9W {
        _SWTRIG9W { w: self }
    }
    #[doc = "Bit 10 - Channel 10 Software Trigger"]
    #[inline]
    pub fn swtrig10(&mut self) -> _SWTRIG10W {
        _SWTRIG10W { w: self }
    }
    #[doc = "Bit 11 - Channel 11 Software Trigger"]
    #[inline]
    pub fn swtrig11(&mut self) -> _SWTRIG11W {
        _SWTRIG11W { w: self }
    }
}
