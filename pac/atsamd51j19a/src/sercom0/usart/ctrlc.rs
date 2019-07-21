#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CTRLC {
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
pub struct GTIMER {
    bits: u8,
}
impl GTIMER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct BRKLENR {
    bits: u8,
}
impl BRKLENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct HDRDLYR {
    bits: u8,
}
impl HDRDLYR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct INACKR {
    bits: bool,
}
impl INACKR {
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
pub struct DSNACKR {
    bits: bool,
}
impl DSNACKR {
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
pub struct MAXITERR {
    bits: u8,
}
impl MAXITERR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DATA32BR {
    bits: u8,
}
impl DATA32BR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _GTIMEW<'a> {
    w: &'a mut W,
}
impl<'a> _GTIMEW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(0x07 << 0);
        self.w.bits |= ((value as u32) & 0x07) << 0;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _BRKLENW<'a> {
    w: &'a mut W,
}
impl<'a> _BRKLENW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(0x03 << 8);
        self.w.bits |= ((value as u32) & 0x03) << 8;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _HDRDLYW<'a> {
    w: &'a mut W,
}
impl<'a> _HDRDLYW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(0x03 << 10);
        self.w.bits |= ((value as u32) & 0x03) << 10;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _INACKW<'a> {
    w: &'a mut W,
}
impl<'a> _INACKW<'a> {
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
#[doc = r" Proxy"]
pub struct _DSNACKW<'a> {
    w: &'a mut W,
}
impl<'a> _DSNACKW<'a> {
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
        self.w.bits &= !(0x01 << 17);
        self.w.bits |= ((value as u32) & 0x01) << 17;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _MAXITERW<'a> {
    w: &'a mut W,
}
impl<'a> _MAXITERW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(0x07 << 20);
        self.w.bits |= ((value as u32) & 0x07) << 20;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DATA32BW<'a> {
    w: &'a mut W,
}
impl<'a> _DATA32BW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(0x03 << 24);
        self.w.bits |= ((value as u32) & 0x03) << 24;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:2 - Guard Time"]
    #[inline]
    pub fn gtime(&self) -> GTIMER {
        let bits = ((self.bits >> 0) & 0x07) as u8;
        GTIMER { bits }
    }
    #[doc = "Bits 8:9 - LIN Master Break Length"]
    #[inline]
    pub fn brklen(&self) -> BRKLENR {
        let bits = ((self.bits >> 8) & 0x03) as u8;
        BRKLENR { bits }
    }
    #[doc = "Bits 10:11 - LIN Master Header Delay"]
    #[inline]
    pub fn hdrdly(&self) -> HDRDLYR {
        let bits = ((self.bits >> 10) & 0x03) as u8;
        HDRDLYR { bits }
    }
    #[doc = "Bit 16 - Inhibit Not Acknowledge"]
    #[inline]
    pub fn inack(&self) -> INACKR {
        let bits = ((self.bits >> 16) & 0x01) != 0;
        INACKR { bits }
    }
    #[doc = "Bit 17 - Disable Successive NACK"]
    #[inline]
    pub fn dsnack(&self) -> DSNACKR {
        let bits = ((self.bits >> 17) & 0x01) != 0;
        DSNACKR { bits }
    }
    #[doc = "Bits 20:22 - Maximum Iterations"]
    #[inline]
    pub fn maxiter(&self) -> MAXITERR {
        let bits = ((self.bits >> 20) & 0x07) as u8;
        MAXITERR { bits }
    }
    #[doc = "Bits 24:25 - Data 32 Bit"]
    #[inline]
    pub fn data32b(&self) -> DATA32BR {
        let bits = ((self.bits >> 24) & 0x03) as u8;
        DATA32BR { bits }
    }
}
impl W {
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:2 - Guard Time"]
    #[inline]
    pub fn gtime(&mut self) -> _GTIMEW {
        _GTIMEW { w: self }
    }
    #[doc = "Bits 8:9 - LIN Master Break Length"]
    #[inline]
    pub fn brklen(&mut self) -> _BRKLENW {
        _BRKLENW { w: self }
    }
    #[doc = "Bits 10:11 - LIN Master Header Delay"]
    #[inline]
    pub fn hdrdly(&mut self) -> _HDRDLYW {
        _HDRDLYW { w: self }
    }
    #[doc = "Bit 16 - Inhibit Not Acknowledge"]
    #[inline]
    pub fn inack(&mut self) -> _INACKW {
        _INACKW { w: self }
    }
    #[doc = "Bit 17 - Disable Successive NACK"]
    #[inline]
    pub fn dsnack(&mut self) -> _DSNACKW {
        _DSNACKW { w: self }
    }
    #[doc = "Bits 20:22 - Maximum Iterations"]
    #[inline]
    pub fn maxiter(&mut self) -> _MAXITERW {
        _MAXITERW { w: self }
    }
    #[doc = "Bits 24:25 - Data 32 Bit"]
    #[inline]
    pub fn data32b(&mut self) -> _DATA32BW {
        _DATA32BW { w: self }
    }
}
