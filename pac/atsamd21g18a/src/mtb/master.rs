#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::MASTER {
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
pub struct MASKR {
    bits: u8,
}
impl MASKR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct TSTARTENR {
    bits: bool,
}
impl TSTARTENR {
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
pub struct TSTOPENR {
    bits: bool,
}
impl TSTOPENR {
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
pub struct SFRWPRIVR {
    bits: bool,
}
impl SFRWPRIVR {
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
pub struct RAMPRIVR {
    bits: bool,
}
impl RAMPRIVR {
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
pub struct HALTREQR {
    bits: bool,
}
impl HALTREQR {
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
pub struct ENR {
    bits: bool,
}
impl ENR {
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
pub struct _MASKW<'a> {
    w: &'a mut W,
}
impl<'a> _MASKW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(0x1f << 0);
        self.w.bits |= ((value as u32) & 0x1f) << 0;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TSTARTENW<'a> {
    w: &'a mut W,
}
impl<'a> _TSTARTENW<'a> {
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
pub struct _TSTOPENW<'a> {
    w: &'a mut W,
}
impl<'a> _TSTOPENW<'a> {
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
pub struct _SFRWPRIVW<'a> {
    w: &'a mut W,
}
impl<'a> _SFRWPRIVW<'a> {
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
pub struct _RAMPRIVW<'a> {
    w: &'a mut W,
}
impl<'a> _RAMPRIVW<'a> {
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
pub struct _HALTREQW<'a> {
    w: &'a mut W,
}
impl<'a> _HALTREQW<'a> {
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
pub struct _ENW<'a> {
    w: &'a mut W,
}
impl<'a> _ENW<'a> {
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
    #[doc = "Bits 0:4 - Maximum Value of the Trace Buffer in SRAM"]
    #[inline]
    pub fn mask(&self) -> MASKR {
        let bits = ((self.bits >> 0) & 0x1f) as u8;
        MASKR { bits }
    }
    #[doc = "Bit 5 - Trace Start Input Enable"]
    #[inline]
    pub fn tstarten(&self) -> TSTARTENR {
        let bits = ((self.bits >> 5) & 0x01) != 0;
        TSTARTENR { bits }
    }
    #[doc = "Bit 6 - Trace Stop Input Enable"]
    #[inline]
    pub fn tstopen(&self) -> TSTOPENR {
        let bits = ((self.bits >> 6) & 0x01) != 0;
        TSTOPENR { bits }
    }
    #[doc = "Bit 7 - Special Function Register Write Privilege"]
    #[inline]
    pub fn sfrwpriv(&self) -> SFRWPRIVR {
        let bits = ((self.bits >> 7) & 0x01) != 0;
        SFRWPRIVR { bits }
    }
    #[doc = "Bit 8 - SRAM Privilege"]
    #[inline]
    pub fn rampriv(&self) -> RAMPRIVR {
        let bits = ((self.bits >> 8) & 0x01) != 0;
        RAMPRIVR { bits }
    }
    #[doc = "Bit 9 - Halt Request"]
    #[inline]
    pub fn haltreq(&self) -> HALTREQR {
        let bits = ((self.bits >> 9) & 0x01) != 0;
        HALTREQR { bits }
    }
    #[doc = "Bit 31 - Main Trace Enable"]
    #[inline]
    pub fn en(&self) -> ENR {
        let bits = ((self.bits >> 31) & 0x01) != 0;
        ENR { bits }
    }
}
impl W {
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:4 - Maximum Value of the Trace Buffer in SRAM"]
    #[inline]
    pub fn mask(&mut self) -> _MASKW {
        _MASKW { w: self }
    }
    #[doc = "Bit 5 - Trace Start Input Enable"]
    #[inline]
    pub fn tstarten(&mut self) -> _TSTARTENW {
        _TSTARTENW { w: self }
    }
    #[doc = "Bit 6 - Trace Stop Input Enable"]
    #[inline]
    pub fn tstopen(&mut self) -> _TSTOPENW {
        _TSTOPENW { w: self }
    }
    #[doc = "Bit 7 - Special Function Register Write Privilege"]
    #[inline]
    pub fn sfrwpriv(&mut self) -> _SFRWPRIVW {
        _SFRWPRIVW { w: self }
    }
    #[doc = "Bit 8 - SRAM Privilege"]
    #[inline]
    pub fn rampriv(&mut self) -> _RAMPRIVW {
        _RAMPRIVW { w: self }
    }
    #[doc = "Bit 9 - Halt Request"]
    #[inline]
    pub fn haltreq(&mut self) -> _HALTREQW {
        _HALTREQW { w: self }
    }
    #[doc = "Bit 31 - Main Trace Enable"]
    #[inline]
    pub fn en(&mut self) -> _ENW {
        _ENW { w: self }
    }
}
