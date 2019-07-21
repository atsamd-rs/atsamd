#[doc = r" Value read from the register"]
pub struct R {
    bits: u16,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u16,
}
impl super::PATTB {
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
pub struct PGEB0R {
    bits: bool,
}
impl PGEB0R {
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
pub struct PGEB1R {
    bits: bool,
}
impl PGEB1R {
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
pub struct PGEB2R {
    bits: bool,
}
impl PGEB2R {
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
pub struct PGEB3R {
    bits: bool,
}
impl PGEB3R {
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
pub struct PGEB4R {
    bits: bool,
}
impl PGEB4R {
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
pub struct PGEB5R {
    bits: bool,
}
impl PGEB5R {
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
pub struct PGEB6R {
    bits: bool,
}
impl PGEB6R {
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
pub struct PGEB7R {
    bits: bool,
}
impl PGEB7R {
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
pub struct PGVB0R {
    bits: bool,
}
impl PGVB0R {
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
pub struct PGVB1R {
    bits: bool,
}
impl PGVB1R {
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
pub struct PGVB2R {
    bits: bool,
}
impl PGVB2R {
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
pub struct PGVB3R {
    bits: bool,
}
impl PGVB3R {
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
pub struct PGVB4R {
    bits: bool,
}
impl PGVB4R {
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
pub struct PGVB5R {
    bits: bool,
}
impl PGVB5R {
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
pub struct PGVB6R {
    bits: bool,
}
impl PGVB6R {
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
pub struct PGVB7R {
    bits: bool,
}
impl PGVB7R {
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
pub struct _PGEB0W<'a> {
    w: &'a mut W,
}
impl<'a> _PGEB0W<'a> {
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
pub struct _PGEB1W<'a> {
    w: &'a mut W,
}
impl<'a> _PGEB1W<'a> {
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
pub struct _PGEB2W<'a> {
    w: &'a mut W,
}
impl<'a> _PGEB2W<'a> {
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
        self.w.bits |= ((value as u16) & 0x01) << 2;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PGEB3W<'a> {
    w: &'a mut W,
}
impl<'a> _PGEB3W<'a> {
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
        self.w.bits |= ((value as u16) & 0x01) << 3;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PGEB4W<'a> {
    w: &'a mut W,
}
impl<'a> _PGEB4W<'a> {
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
pub struct _PGEB5W<'a> {
    w: &'a mut W,
}
impl<'a> _PGEB5W<'a> {
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
pub struct _PGEB6W<'a> {
    w: &'a mut W,
}
impl<'a> _PGEB6W<'a> {
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
pub struct _PGEB7W<'a> {
    w: &'a mut W,
}
impl<'a> _PGEB7W<'a> {
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
        self.w.bits |= ((value as u16) & 0x01) << 7;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PGVB0W<'a> {
    w: &'a mut W,
}
impl<'a> _PGVB0W<'a> {
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
pub struct _PGVB1W<'a> {
    w: &'a mut W,
}
impl<'a> _PGVB1W<'a> {
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
pub struct _PGVB2W<'a> {
    w: &'a mut W,
}
impl<'a> _PGVB2W<'a> {
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
#[doc = r" Proxy"]
pub struct _PGVB3W<'a> {
    w: &'a mut W,
}
impl<'a> _PGVB3W<'a> {
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
        self.w.bits |= ((value as u16) & 0x01) << 11;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PGVB4W<'a> {
    w: &'a mut W,
}
impl<'a> _PGVB4W<'a> {
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
pub struct _PGVB5W<'a> {
    w: &'a mut W,
}
impl<'a> _PGVB5W<'a> {
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
#[doc = r" Proxy"]
pub struct _PGVB6W<'a> {
    w: &'a mut W,
}
impl<'a> _PGVB6W<'a> {
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
        self.w.bits &= !(0x01 << 14);
        self.w.bits |= ((value as u16) & 0x01) << 14;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PGVB7W<'a> {
    w: &'a mut W,
}
impl<'a> _PGVB7W<'a> {
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
        self.w.bits |= ((value as u16) & 0x01) << 15;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
    #[doc = "Bit 0 - Pattern Generator 0 Output Enable Buffer"]
    #[inline]
    pub fn pgeb0(&self) -> PGEB0R {
        let bits = ((self.bits >> 0) & 0x01) != 0;
        PGEB0R { bits }
    }
    #[doc = "Bit 1 - Pattern Generator 1 Output Enable Buffer"]
    #[inline]
    pub fn pgeb1(&self) -> PGEB1R {
        let bits = ((self.bits >> 1) & 0x01) != 0;
        PGEB1R { bits }
    }
    #[doc = "Bit 2 - Pattern Generator 2 Output Enable Buffer"]
    #[inline]
    pub fn pgeb2(&self) -> PGEB2R {
        let bits = ((self.bits >> 2) & 0x01) != 0;
        PGEB2R { bits }
    }
    #[doc = "Bit 3 - Pattern Generator 3 Output Enable Buffer"]
    #[inline]
    pub fn pgeb3(&self) -> PGEB3R {
        let bits = ((self.bits >> 3) & 0x01) != 0;
        PGEB3R { bits }
    }
    #[doc = "Bit 4 - Pattern Generator 4 Output Enable Buffer"]
    #[inline]
    pub fn pgeb4(&self) -> PGEB4R {
        let bits = ((self.bits >> 4) & 0x01) != 0;
        PGEB4R { bits }
    }
    #[doc = "Bit 5 - Pattern Generator 5 Output Enable Buffer"]
    #[inline]
    pub fn pgeb5(&self) -> PGEB5R {
        let bits = ((self.bits >> 5) & 0x01) != 0;
        PGEB5R { bits }
    }
    #[doc = "Bit 6 - Pattern Generator 6 Output Enable Buffer"]
    #[inline]
    pub fn pgeb6(&self) -> PGEB6R {
        let bits = ((self.bits >> 6) & 0x01) != 0;
        PGEB6R { bits }
    }
    #[doc = "Bit 7 - Pattern Generator 7 Output Enable Buffer"]
    #[inline]
    pub fn pgeb7(&self) -> PGEB7R {
        let bits = ((self.bits >> 7) & 0x01) != 0;
        PGEB7R { bits }
    }
    #[doc = "Bit 8 - Pattern Generator 0 Output Enable"]
    #[inline]
    pub fn pgvb0(&self) -> PGVB0R {
        let bits = ((self.bits >> 8) & 0x01) != 0;
        PGVB0R { bits }
    }
    #[doc = "Bit 9 - Pattern Generator 1 Output Enable"]
    #[inline]
    pub fn pgvb1(&self) -> PGVB1R {
        let bits = ((self.bits >> 9) & 0x01) != 0;
        PGVB1R { bits }
    }
    #[doc = "Bit 10 - Pattern Generator 2 Output Enable"]
    #[inline]
    pub fn pgvb2(&self) -> PGVB2R {
        let bits = ((self.bits >> 10) & 0x01) != 0;
        PGVB2R { bits }
    }
    #[doc = "Bit 11 - Pattern Generator 3 Output Enable"]
    #[inline]
    pub fn pgvb3(&self) -> PGVB3R {
        let bits = ((self.bits >> 11) & 0x01) != 0;
        PGVB3R { bits }
    }
    #[doc = "Bit 12 - Pattern Generator 4 Output Enable"]
    #[inline]
    pub fn pgvb4(&self) -> PGVB4R {
        let bits = ((self.bits >> 12) & 0x01) != 0;
        PGVB4R { bits }
    }
    #[doc = "Bit 13 - Pattern Generator 5 Output Enable"]
    #[inline]
    pub fn pgvb5(&self) -> PGVB5R {
        let bits = ((self.bits >> 13) & 0x01) != 0;
        PGVB5R { bits }
    }
    #[doc = "Bit 14 - Pattern Generator 6 Output Enable"]
    #[inline]
    pub fn pgvb6(&self) -> PGVB6R {
        let bits = ((self.bits >> 14) & 0x01) != 0;
        PGVB6R { bits }
    }
    #[doc = "Bit 15 - Pattern Generator 7 Output Enable"]
    #[inline]
    pub fn pgvb7(&self) -> PGVB7R {
        let bits = ((self.bits >> 15) & 0x01) != 0;
        PGVB7R { bits }
    }
}
impl W {
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Pattern Generator 0 Output Enable Buffer"]
    #[inline]
    pub fn pgeb0(&mut self) -> _PGEB0W {
        _PGEB0W { w: self }
    }
    #[doc = "Bit 1 - Pattern Generator 1 Output Enable Buffer"]
    #[inline]
    pub fn pgeb1(&mut self) -> _PGEB1W {
        _PGEB1W { w: self }
    }
    #[doc = "Bit 2 - Pattern Generator 2 Output Enable Buffer"]
    #[inline]
    pub fn pgeb2(&mut self) -> _PGEB2W {
        _PGEB2W { w: self }
    }
    #[doc = "Bit 3 - Pattern Generator 3 Output Enable Buffer"]
    #[inline]
    pub fn pgeb3(&mut self) -> _PGEB3W {
        _PGEB3W { w: self }
    }
    #[doc = "Bit 4 - Pattern Generator 4 Output Enable Buffer"]
    #[inline]
    pub fn pgeb4(&mut self) -> _PGEB4W {
        _PGEB4W { w: self }
    }
    #[doc = "Bit 5 - Pattern Generator 5 Output Enable Buffer"]
    #[inline]
    pub fn pgeb5(&mut self) -> _PGEB5W {
        _PGEB5W { w: self }
    }
    #[doc = "Bit 6 - Pattern Generator 6 Output Enable Buffer"]
    #[inline]
    pub fn pgeb6(&mut self) -> _PGEB6W {
        _PGEB6W { w: self }
    }
    #[doc = "Bit 7 - Pattern Generator 7 Output Enable Buffer"]
    #[inline]
    pub fn pgeb7(&mut self) -> _PGEB7W {
        _PGEB7W { w: self }
    }
    #[doc = "Bit 8 - Pattern Generator 0 Output Enable"]
    #[inline]
    pub fn pgvb0(&mut self) -> _PGVB0W {
        _PGVB0W { w: self }
    }
    #[doc = "Bit 9 - Pattern Generator 1 Output Enable"]
    #[inline]
    pub fn pgvb1(&mut self) -> _PGVB1W {
        _PGVB1W { w: self }
    }
    #[doc = "Bit 10 - Pattern Generator 2 Output Enable"]
    #[inline]
    pub fn pgvb2(&mut self) -> _PGVB2W {
        _PGVB2W { w: self }
    }
    #[doc = "Bit 11 - Pattern Generator 3 Output Enable"]
    #[inline]
    pub fn pgvb3(&mut self) -> _PGVB3W {
        _PGVB3W { w: self }
    }
    #[doc = "Bit 12 - Pattern Generator 4 Output Enable"]
    #[inline]
    pub fn pgvb4(&mut self) -> _PGVB4W {
        _PGVB4W { w: self }
    }
    #[doc = "Bit 13 - Pattern Generator 5 Output Enable"]
    #[inline]
    pub fn pgvb5(&mut self) -> _PGVB5W {
        _PGVB5W { w: self }
    }
    #[doc = "Bit 14 - Pattern Generator 6 Output Enable"]
    #[inline]
    pub fn pgvb6(&mut self) -> _PGVB6W {
        _PGVB6W { w: self }
    }
    #[doc = "Bit 15 - Pattern Generator 7 Output Enable"]
    #[inline]
    pub fn pgvb7(&mut self) -> _PGVB7W {
        _PGVB7W { w: self }
    }
}
