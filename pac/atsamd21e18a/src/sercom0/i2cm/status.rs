#[doc = r" Value read from the register"]
pub struct R {
    bits: u16,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u16,
}
impl super::STATUS {
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
pub struct BUSERRR {
    bits: bool,
}
impl BUSERRR {
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
pub struct ARBLOSTR {
    bits: bool,
}
impl ARBLOSTR {
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
pub struct RXNACKR {
    bits: bool,
}
impl RXNACKR {
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
pub struct BUSSTATER {
    bits: u8,
}
impl BUSSTATER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct LOWTOUTR {
    bits: bool,
}
impl LOWTOUTR {
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
pub struct CLKHOLDR {
    bits: bool,
}
impl CLKHOLDR {
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
pub struct MEXTTOUTR {
    bits: bool,
}
impl MEXTTOUTR {
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
pub struct SEXTTOUTR {
    bits: bool,
}
impl SEXTTOUTR {
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
pub struct LENERRR {
    bits: bool,
}
impl LENERRR {
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
pub struct _BUSERRW<'a> {
    w: &'a mut W,
}
impl<'a> _BUSERRW<'a> {
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
pub struct _ARBLOSTW<'a> {
    w: &'a mut W,
}
impl<'a> _ARBLOSTW<'a> {
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
pub struct _BUSSTATEW<'a> {
    w: &'a mut W,
}
impl<'a> _BUSSTATEW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(0x03 << 4);
        self.w.bits |= ((value as u16) & 0x03) << 4;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _LOWTOUTW<'a> {
    w: &'a mut W,
}
impl<'a> _LOWTOUTW<'a> {
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
        self.w.bits |= ((value as u16) & 0x01) << 6;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _MEXTTOUTW<'a> {
    w: &'a mut W,
}
impl<'a> _MEXTTOUTW<'a> {
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
pub struct _SEXTTOUTW<'a> {
    w: &'a mut W,
}
impl<'a> _SEXTTOUTW<'a> {
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
pub struct _LENERRW<'a> {
    w: &'a mut W,
}
impl<'a> _LENERRW<'a> {
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
        self.w.bits |= ((value as u16) & 0x01) << 10;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
    #[doc = "Bit 0 - Bus Error"]
    #[inline]
    pub fn buserr(&self) -> BUSERRR {
        let bits = ((self.bits >> 0) & 0x01) != 0;
        BUSERRR { bits }
    }
    #[doc = "Bit 1 - Arbitration Lost"]
    #[inline]
    pub fn arblost(&self) -> ARBLOSTR {
        let bits = ((self.bits >> 1) & 0x01) != 0;
        ARBLOSTR { bits }
    }
    #[doc = "Bit 2 - Received Not Acknowledge"]
    #[inline]
    pub fn rxnack(&self) -> RXNACKR {
        let bits = ((self.bits >> 2) & 0x01) != 0;
        RXNACKR { bits }
    }
    #[doc = "Bits 4:5 - Bus State"]
    #[inline]
    pub fn busstate(&self) -> BUSSTATER {
        let bits = ((self.bits >> 4) & 0x03) as u8;
        BUSSTATER { bits }
    }
    #[doc = "Bit 6 - SCL Low Timeout"]
    #[inline]
    pub fn lowtout(&self) -> LOWTOUTR {
        let bits = ((self.bits >> 6) & 0x01) != 0;
        LOWTOUTR { bits }
    }
    #[doc = "Bit 7 - Clock Hold"]
    #[inline]
    pub fn clkhold(&self) -> CLKHOLDR {
        let bits = ((self.bits >> 7) & 0x01) != 0;
        CLKHOLDR { bits }
    }
    #[doc = "Bit 8 - Master SCL Low Extend Timeout"]
    #[inline]
    pub fn mexttout(&self) -> MEXTTOUTR {
        let bits = ((self.bits >> 8) & 0x01) != 0;
        MEXTTOUTR { bits }
    }
    #[doc = "Bit 9 - Slave SCL Low Extend Timeout"]
    #[inline]
    pub fn sexttout(&self) -> SEXTTOUTR {
        let bits = ((self.bits >> 9) & 0x01) != 0;
        SEXTTOUTR { bits }
    }
    #[doc = "Bit 10 - Length Error"]
    #[inline]
    pub fn lenerr(&self) -> LENERRR {
        let bits = ((self.bits >> 10) & 0x01) != 0;
        LENERRR { bits }
    }
}
impl W {
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Bus Error"]
    #[inline]
    pub fn buserr(&mut self) -> _BUSERRW {
        _BUSERRW { w: self }
    }
    #[doc = "Bit 1 - Arbitration Lost"]
    #[inline]
    pub fn arblost(&mut self) -> _ARBLOSTW {
        _ARBLOSTW { w: self }
    }
    #[doc = "Bits 4:5 - Bus State"]
    #[inline]
    pub fn busstate(&mut self) -> _BUSSTATEW {
        _BUSSTATEW { w: self }
    }
    #[doc = "Bit 6 - SCL Low Timeout"]
    #[inline]
    pub fn lowtout(&mut self) -> _LOWTOUTW {
        _LOWTOUTW { w: self }
    }
    #[doc = "Bit 8 - Master SCL Low Extend Timeout"]
    #[inline]
    pub fn mexttout(&mut self) -> _MEXTTOUTW {
        _MEXTTOUTW { w: self }
    }
    #[doc = "Bit 9 - Slave SCL Low Extend Timeout"]
    #[inline]
    pub fn sexttout(&mut self) -> _SEXTTOUTW {
        _SEXTTOUTW { w: self }
    }
    #[doc = "Bit 10 - Length Error"]
    #[inline]
    pub fn lenerr(&mut self) -> _LENERRW {
        _LENERRW { w: self }
    }
}
