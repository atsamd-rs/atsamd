#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::INTFLAG {
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
pub struct BOD33RDYR {
    bits: bool,
}
impl BOD33RDYR {
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
pub struct BOD33DETR {
    bits: bool,
}
impl BOD33DETR {
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
pub struct B33SRDYR {
    bits: bool,
}
impl B33SRDYR {
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
pub struct BOD12RDYR {
    bits: bool,
}
impl BOD12RDYR {
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
pub struct BOD12DETR {
    bits: bool,
}
impl BOD12DETR {
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
pub struct B12SRDYR {
    bits: bool,
}
impl B12SRDYR {
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
pub struct VREGRDYR {
    bits: bool,
}
impl VREGRDYR {
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
pub struct VCORERDYR {
    bits: bool,
}
impl VCORERDYR {
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
pub struct _BOD33RDYW<'a> {
    w: &'a mut W,
}
impl<'a> _BOD33RDYW<'a> {
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
pub struct _BOD33DETW<'a> {
    w: &'a mut W,
}
impl<'a> _BOD33DETW<'a> {
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
pub struct _B33SRDYW<'a> {
    w: &'a mut W,
}
impl<'a> _B33SRDYW<'a> {
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
pub struct _BOD12RDYW<'a> {
    w: &'a mut W,
}
impl<'a> _BOD12RDYW<'a> {
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
pub struct _BOD12DETW<'a> {
    w: &'a mut W,
}
impl<'a> _BOD12DETW<'a> {
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
pub struct _B12SRDYW<'a> {
    w: &'a mut W,
}
impl<'a> _B12SRDYW<'a> {
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
pub struct _VREGRDYW<'a> {
    w: &'a mut W,
}
impl<'a> _VREGRDYW<'a> {
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
pub struct _VCORERDYW<'a> {
    w: &'a mut W,
}
impl<'a> _VCORERDYW<'a> {
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - BOD33 Ready"]
    #[inline]
    pub fn bod33rdy(&self) -> BOD33RDYR {
        let bits = ((self.bits >> 0) & 0x01) != 0;
        BOD33RDYR { bits }
    }
    #[doc = "Bit 1 - BOD33 Detection"]
    #[inline]
    pub fn bod33det(&self) -> BOD33DETR {
        let bits = ((self.bits >> 1) & 0x01) != 0;
        BOD33DETR { bits }
    }
    #[doc = "Bit 2 - BOD33 Synchronization Ready"]
    #[inline]
    pub fn b33srdy(&self) -> B33SRDYR {
        let bits = ((self.bits >> 2) & 0x01) != 0;
        B33SRDYR { bits }
    }
    #[doc = "Bit 3 - BOD12 Ready"]
    #[inline]
    pub fn bod12rdy(&self) -> BOD12RDYR {
        let bits = ((self.bits >> 3) & 0x01) != 0;
        BOD12RDYR { bits }
    }
    #[doc = "Bit 4 - BOD12 Detection"]
    #[inline]
    pub fn bod12det(&self) -> BOD12DETR {
        let bits = ((self.bits >> 4) & 0x01) != 0;
        BOD12DETR { bits }
    }
    #[doc = "Bit 5 - BOD12 Synchronization Ready"]
    #[inline]
    pub fn b12srdy(&self) -> B12SRDYR {
        let bits = ((self.bits >> 5) & 0x01) != 0;
        B12SRDYR { bits }
    }
    #[doc = "Bit 8 - Voltage Regulator Ready"]
    #[inline]
    pub fn vregrdy(&self) -> VREGRDYR {
        let bits = ((self.bits >> 8) & 0x01) != 0;
        VREGRDYR { bits }
    }
    #[doc = "Bit 10 - VDDCORE Ready"]
    #[inline]
    pub fn vcorerdy(&self) -> VCORERDYR {
        let bits = ((self.bits >> 10) & 0x01) != 0;
        VCORERDYR { bits }
    }
}
impl W {
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - BOD33 Ready"]
    #[inline]
    pub fn bod33rdy(&mut self) -> _BOD33RDYW {
        _BOD33RDYW { w: self }
    }
    #[doc = "Bit 1 - BOD33 Detection"]
    #[inline]
    pub fn bod33det(&mut self) -> _BOD33DETW {
        _BOD33DETW { w: self }
    }
    #[doc = "Bit 2 - BOD33 Synchronization Ready"]
    #[inline]
    pub fn b33srdy(&mut self) -> _B33SRDYW {
        _B33SRDYW { w: self }
    }
    #[doc = "Bit 3 - BOD12 Ready"]
    #[inline]
    pub fn bod12rdy(&mut self) -> _BOD12RDYW {
        _BOD12RDYW { w: self }
    }
    #[doc = "Bit 4 - BOD12 Detection"]
    #[inline]
    pub fn bod12det(&mut self) -> _BOD12DETW {
        _BOD12DETW { w: self }
    }
    #[doc = "Bit 5 - BOD12 Synchronization Ready"]
    #[inline]
    pub fn b12srdy(&mut self) -> _B12SRDYW {
        _B12SRDYW { w: self }
    }
    #[doc = "Bit 8 - Voltage Regulator Ready"]
    #[inline]
    pub fn vregrdy(&mut self) -> _VREGRDYW {
        _VREGRDYW { w: self }
    }
    #[doc = "Bit 10 - VDDCORE Ready"]
    #[inline]
    pub fn vcorerdy(&mut self) -> _VCORERDYW {
        _VCORERDYW { w: self }
    }
}
