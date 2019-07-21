#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PRICTRL0 {
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
pub struct LVLPRI0R {
    bits: u8,
}
impl LVLPRI0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct RRLVLEN0R {
    bits: bool,
}
impl RRLVLEN0R {
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
pub struct LVLPRI1R {
    bits: u8,
}
impl LVLPRI1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct RRLVLEN1R {
    bits: bool,
}
impl RRLVLEN1R {
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
pub struct LVLPRI2R {
    bits: u8,
}
impl LVLPRI2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct RRLVLEN2R {
    bits: bool,
}
impl RRLVLEN2R {
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
pub struct LVLPRI3R {
    bits: u8,
}
impl LVLPRI3R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct RRLVLEN3R {
    bits: bool,
}
impl RRLVLEN3R {
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
pub struct _LVLPRI0W<'a> {
    w: &'a mut W,
}
impl<'a> _LVLPRI0W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(0x0f << 0);
        self.w.bits |= ((value as u32) & 0x0f) << 0;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RRLVLEN0W<'a> {
    w: &'a mut W,
}
impl<'a> _RRLVLEN0W<'a> {
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
pub struct _LVLPRI1W<'a> {
    w: &'a mut W,
}
impl<'a> _LVLPRI1W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(0x0f << 8);
        self.w.bits |= ((value as u32) & 0x0f) << 8;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RRLVLEN1W<'a> {
    w: &'a mut W,
}
impl<'a> _RRLVLEN1W<'a> {
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
pub struct _LVLPRI2W<'a> {
    w: &'a mut W,
}
impl<'a> _LVLPRI2W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(0x0f << 16);
        self.w.bits |= ((value as u32) & 0x0f) << 16;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RRLVLEN2W<'a> {
    w: &'a mut W,
}
impl<'a> _RRLVLEN2W<'a> {
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
        self.w.bits &= !(0x01 << 23);
        self.w.bits |= ((value as u32) & 0x01) << 23;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _LVLPRI3W<'a> {
    w: &'a mut W,
}
impl<'a> _LVLPRI3W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(0x0f << 24);
        self.w.bits |= ((value as u32) & 0x0f) << 24;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RRLVLEN3W<'a> {
    w: &'a mut W,
}
impl<'a> _RRLVLEN3W<'a> {
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
        self.w.bits &= !(0x01 << 31);
        self.w.bits |= ((value as u32) & 0x01) << 31;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:3 - Level 0 Channel Priority Number"]
    #[inline]
    pub fn lvlpri0(&self) -> LVLPRI0R {
        let bits = ((self.bits >> 0) & 0x0f) as u8;
        LVLPRI0R { bits }
    }
    #[doc = "Bit 7 - Level 0 Round-Robin Scheduling Enable"]
    #[inline]
    pub fn rrlvlen0(&self) -> RRLVLEN0R {
        let bits = ((self.bits >> 7) & 0x01) != 0;
        RRLVLEN0R { bits }
    }
    #[doc = "Bits 8:11 - Level 1 Channel Priority Number"]
    #[inline]
    pub fn lvlpri1(&self) -> LVLPRI1R {
        let bits = ((self.bits >> 8) & 0x0f) as u8;
        LVLPRI1R { bits }
    }
    #[doc = "Bit 15 - Level 1 Round-Robin Scheduling Enable"]
    #[inline]
    pub fn rrlvlen1(&self) -> RRLVLEN1R {
        let bits = ((self.bits >> 15) & 0x01) != 0;
        RRLVLEN1R { bits }
    }
    #[doc = "Bits 16:19 - Level 2 Channel Priority Number"]
    #[inline]
    pub fn lvlpri2(&self) -> LVLPRI2R {
        let bits = ((self.bits >> 16) & 0x0f) as u8;
        LVLPRI2R { bits }
    }
    #[doc = "Bit 23 - Level 2 Round-Robin Scheduling Enable"]
    #[inline]
    pub fn rrlvlen2(&self) -> RRLVLEN2R {
        let bits = ((self.bits >> 23) & 0x01) != 0;
        RRLVLEN2R { bits }
    }
    #[doc = "Bits 24:27 - Level 3 Channel Priority Number"]
    #[inline]
    pub fn lvlpri3(&self) -> LVLPRI3R {
        let bits = ((self.bits >> 24) & 0x0f) as u8;
        LVLPRI3R { bits }
    }
    #[doc = "Bit 31 - Level 3 Round-Robin Scheduling Enable"]
    #[inline]
    pub fn rrlvlen3(&self) -> RRLVLEN3R {
        let bits = ((self.bits >> 31) & 0x01) != 0;
        RRLVLEN3R { bits }
    }
}
impl W {
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:3 - Level 0 Channel Priority Number"]
    #[inline]
    pub fn lvlpri0(&mut self) -> _LVLPRI0W {
        _LVLPRI0W { w: self }
    }
    #[doc = "Bit 7 - Level 0 Round-Robin Scheduling Enable"]
    #[inline]
    pub fn rrlvlen0(&mut self) -> _RRLVLEN0W {
        _RRLVLEN0W { w: self }
    }
    #[doc = "Bits 8:11 - Level 1 Channel Priority Number"]
    #[inline]
    pub fn lvlpri1(&mut self) -> _LVLPRI1W {
        _LVLPRI1W { w: self }
    }
    #[doc = "Bit 15 - Level 1 Round-Robin Scheduling Enable"]
    #[inline]
    pub fn rrlvlen1(&mut self) -> _RRLVLEN1W {
        _RRLVLEN1W { w: self }
    }
    #[doc = "Bits 16:19 - Level 2 Channel Priority Number"]
    #[inline]
    pub fn lvlpri2(&mut self) -> _LVLPRI2W {
        _LVLPRI2W { w: self }
    }
    #[doc = "Bit 23 - Level 2 Round-Robin Scheduling Enable"]
    #[inline]
    pub fn rrlvlen2(&mut self) -> _RRLVLEN2W {
        _RRLVLEN2W { w: self }
    }
    #[doc = "Bits 24:27 - Level 3 Channel Priority Number"]
    #[inline]
    pub fn lvlpri3(&mut self) -> _LVLPRI3W {
        _LVLPRI3W { w: self }
    }
    #[doc = "Bit 31 - Level 3 Round-Robin Scheduling Enable"]
    #[inline]
    pub fn rrlvlen3(&mut self) -> _RRLVLEN3W {
        _RRLVLEN3W { w: self }
    }
}
