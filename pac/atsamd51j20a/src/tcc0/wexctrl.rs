#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::WEXCTRL {
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
pub struct OTMXR {
    bits: u8,
}
impl OTMXR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DTIEN0R {
    bits: bool,
}
impl DTIEN0R {
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
pub struct DTIEN1R {
    bits: bool,
}
impl DTIEN1R {
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
pub struct DTIEN2R {
    bits: bool,
}
impl DTIEN2R {
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
pub struct DTIEN3R {
    bits: bool,
}
impl DTIEN3R {
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
pub struct DTLSR {
    bits: u8,
}
impl DTLSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DTHSR {
    bits: u8,
}
impl DTHSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _OTMXW<'a> {
    w: &'a mut W,
}
impl<'a> _OTMXW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(0x03 << 0);
        self.w.bits |= ((value as u32) & 0x03) << 0;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DTIEN0W<'a> {
    w: &'a mut W,
}
impl<'a> _DTIEN0W<'a> {
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
pub struct _DTIEN1W<'a> {
    w: &'a mut W,
}
impl<'a> _DTIEN1W<'a> {
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
pub struct _DTIEN2W<'a> {
    w: &'a mut W,
}
impl<'a> _DTIEN2W<'a> {
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
pub struct _DTIEN3W<'a> {
    w: &'a mut W,
}
impl<'a> _DTIEN3W<'a> {
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
pub struct _DTLSW<'a> {
    w: &'a mut W,
}
impl<'a> _DTLSW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(0xff << 16);
        self.w.bits |= ((value as u32) & 0xff) << 16;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DTHSW<'a> {
    w: &'a mut W,
}
impl<'a> _DTHSW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(0xff << 24);
        self.w.bits |= ((value as u32) & 0xff) << 24;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:1 - Output Matrix"]
    #[inline]
    pub fn otmx(&self) -> OTMXR {
        let bits = ((self.bits >> 0) & 0x03) as u8;
        OTMXR { bits }
    }
    #[doc = "Bit 8 - Dead-time Insertion Generator 0 Enable"]
    #[inline]
    pub fn dtien0(&self) -> DTIEN0R {
        let bits = ((self.bits >> 8) & 0x01) != 0;
        DTIEN0R { bits }
    }
    #[doc = "Bit 9 - Dead-time Insertion Generator 1 Enable"]
    #[inline]
    pub fn dtien1(&self) -> DTIEN1R {
        let bits = ((self.bits >> 9) & 0x01) != 0;
        DTIEN1R { bits }
    }
    #[doc = "Bit 10 - Dead-time Insertion Generator 2 Enable"]
    #[inline]
    pub fn dtien2(&self) -> DTIEN2R {
        let bits = ((self.bits >> 10) & 0x01) != 0;
        DTIEN2R { bits }
    }
    #[doc = "Bit 11 - Dead-time Insertion Generator 3 Enable"]
    #[inline]
    pub fn dtien3(&self) -> DTIEN3R {
        let bits = ((self.bits >> 11) & 0x01) != 0;
        DTIEN3R { bits }
    }
    #[doc = "Bits 16:23 - Dead-time Low Side Outputs Value"]
    #[inline]
    pub fn dtls(&self) -> DTLSR {
        let bits = ((self.bits >> 16) & 0xff) as u8;
        DTLSR { bits }
    }
    #[doc = "Bits 24:31 - Dead-time High Side Outputs Value"]
    #[inline]
    pub fn dths(&self) -> DTHSR {
        let bits = ((self.bits >> 24) & 0xff) as u8;
        DTHSR { bits }
    }
}
impl W {
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:1 - Output Matrix"]
    #[inline]
    pub fn otmx(&mut self) -> _OTMXW {
        _OTMXW { w: self }
    }
    #[doc = "Bit 8 - Dead-time Insertion Generator 0 Enable"]
    #[inline]
    pub fn dtien0(&mut self) -> _DTIEN0W {
        _DTIEN0W { w: self }
    }
    #[doc = "Bit 9 - Dead-time Insertion Generator 1 Enable"]
    #[inline]
    pub fn dtien1(&mut self) -> _DTIEN1W {
        _DTIEN1W { w: self }
    }
    #[doc = "Bit 10 - Dead-time Insertion Generator 2 Enable"]
    #[inline]
    pub fn dtien2(&mut self) -> _DTIEN2W {
        _DTIEN2W { w: self }
    }
    #[doc = "Bit 11 - Dead-time Insertion Generator 3 Enable"]
    #[inline]
    pub fn dtien3(&mut self) -> _DTIEN3W {
        _DTIEN3W { w: self }
    }
    #[doc = "Bits 16:23 - Dead-time Low Side Outputs Value"]
    #[inline]
    pub fn dtls(&mut self) -> _DTLSW {
        _DTLSW { w: self }
    }
    #[doc = "Bits 24:31 - Dead-time High Side Outputs Value"]
    #[inline]
    pub fn dths(&mut self) -> _DTHSW {
        _DTHSW { w: self }
    }
}
