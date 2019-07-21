#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::DRVCTRL {
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
pub struct NRE0R {
    bits: bool,
}
impl NRE0R {
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
pub struct NRE1R {
    bits: bool,
}
impl NRE1R {
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
pub struct NRE2R {
    bits: bool,
}
impl NRE2R {
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
pub struct NRE3R {
    bits: bool,
}
impl NRE3R {
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
pub struct NRE4R {
    bits: bool,
}
impl NRE4R {
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
pub struct NRE5R {
    bits: bool,
}
impl NRE5R {
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
pub struct NRE6R {
    bits: bool,
}
impl NRE6R {
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
pub struct NRE7R {
    bits: bool,
}
impl NRE7R {
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
pub struct NRV0R {
    bits: bool,
}
impl NRV0R {
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
pub struct NRV1R {
    bits: bool,
}
impl NRV1R {
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
pub struct NRV2R {
    bits: bool,
}
impl NRV2R {
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
pub struct NRV3R {
    bits: bool,
}
impl NRV3R {
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
pub struct NRV4R {
    bits: bool,
}
impl NRV4R {
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
pub struct NRV5R {
    bits: bool,
}
impl NRV5R {
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
pub struct NRV6R {
    bits: bool,
}
impl NRV6R {
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
pub struct NRV7R {
    bits: bool,
}
impl NRV7R {
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
pub struct INVEN0R {
    bits: bool,
}
impl INVEN0R {
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
pub struct INVEN1R {
    bits: bool,
}
impl INVEN1R {
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
pub struct INVEN2R {
    bits: bool,
}
impl INVEN2R {
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
pub struct INVEN3R {
    bits: bool,
}
impl INVEN3R {
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
pub struct INVEN4R {
    bits: bool,
}
impl INVEN4R {
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
pub struct INVEN5R {
    bits: bool,
}
impl INVEN5R {
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
pub struct INVEN6R {
    bits: bool,
}
impl INVEN6R {
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
pub struct INVEN7R {
    bits: bool,
}
impl INVEN7R {
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
pub struct FILTERVAL0R {
    bits: u8,
}
impl FILTERVAL0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct FILTERVAL1R {
    bits: u8,
}
impl FILTERVAL1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _NRE0W<'a> {
    w: &'a mut W,
}
impl<'a> _NRE0W<'a> {
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
pub struct _NRE1W<'a> {
    w: &'a mut W,
}
impl<'a> _NRE1W<'a> {
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
pub struct _NRE2W<'a> {
    w: &'a mut W,
}
impl<'a> _NRE2W<'a> {
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
pub struct _NRE3W<'a> {
    w: &'a mut W,
}
impl<'a> _NRE3W<'a> {
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
pub struct _NRE4W<'a> {
    w: &'a mut W,
}
impl<'a> _NRE4W<'a> {
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
pub struct _NRE5W<'a> {
    w: &'a mut W,
}
impl<'a> _NRE5W<'a> {
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
pub struct _NRE6W<'a> {
    w: &'a mut W,
}
impl<'a> _NRE6W<'a> {
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
pub struct _NRE7W<'a> {
    w: &'a mut W,
}
impl<'a> _NRE7W<'a> {
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
pub struct _NRV0W<'a> {
    w: &'a mut W,
}
impl<'a> _NRV0W<'a> {
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
pub struct _NRV1W<'a> {
    w: &'a mut W,
}
impl<'a> _NRV1W<'a> {
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
pub struct _NRV2W<'a> {
    w: &'a mut W,
}
impl<'a> _NRV2W<'a> {
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
pub struct _NRV3W<'a> {
    w: &'a mut W,
}
impl<'a> _NRV3W<'a> {
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
pub struct _NRV4W<'a> {
    w: &'a mut W,
}
impl<'a> _NRV4W<'a> {
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
        self.w.bits |= ((value as u32) & 0x01) << 12;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _NRV5W<'a> {
    w: &'a mut W,
}
impl<'a> _NRV5W<'a> {
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
        self.w.bits |= ((value as u32) & 0x01) << 13;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _NRV6W<'a> {
    w: &'a mut W,
}
impl<'a> _NRV6W<'a> {
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
        self.w.bits |= ((value as u32) & 0x01) << 14;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _NRV7W<'a> {
    w: &'a mut W,
}
impl<'a> _NRV7W<'a> {
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
pub struct _INVEN0W<'a> {
    w: &'a mut W,
}
impl<'a> _INVEN0W<'a> {
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
pub struct _INVEN1W<'a> {
    w: &'a mut W,
}
impl<'a> _INVEN1W<'a> {
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
pub struct _INVEN2W<'a> {
    w: &'a mut W,
}
impl<'a> _INVEN2W<'a> {
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
        self.w.bits &= !(0x01 << 18);
        self.w.bits |= ((value as u32) & 0x01) << 18;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _INVEN3W<'a> {
    w: &'a mut W,
}
impl<'a> _INVEN3W<'a> {
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
        self.w.bits &= !(0x01 << 19);
        self.w.bits |= ((value as u32) & 0x01) << 19;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _INVEN4W<'a> {
    w: &'a mut W,
}
impl<'a> _INVEN4W<'a> {
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
        self.w.bits &= !(0x01 << 20);
        self.w.bits |= ((value as u32) & 0x01) << 20;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _INVEN5W<'a> {
    w: &'a mut W,
}
impl<'a> _INVEN5W<'a> {
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
        self.w.bits &= !(0x01 << 21);
        self.w.bits |= ((value as u32) & 0x01) << 21;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _INVEN6W<'a> {
    w: &'a mut W,
}
impl<'a> _INVEN6W<'a> {
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
        self.w.bits &= !(0x01 << 22);
        self.w.bits |= ((value as u32) & 0x01) << 22;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _INVEN7W<'a> {
    w: &'a mut W,
}
impl<'a> _INVEN7W<'a> {
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
pub struct _FILTERVAL0W<'a> {
    w: &'a mut W,
}
impl<'a> _FILTERVAL0W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(0x0f << 24);
        self.w.bits |= ((value as u32) & 0x0f) << 24;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _FILTERVAL1W<'a> {
    w: &'a mut W,
}
impl<'a> _FILTERVAL1W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(0x0f << 28);
        self.w.bits |= ((value as u32) & 0x0f) << 28;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Non-Recoverable State 0 Output Enable"]
    #[inline]
    pub fn nre0(&self) -> NRE0R {
        let bits = ((self.bits >> 0) & 0x01) != 0;
        NRE0R { bits }
    }
    #[doc = "Bit 1 - Non-Recoverable State 1 Output Enable"]
    #[inline]
    pub fn nre1(&self) -> NRE1R {
        let bits = ((self.bits >> 1) & 0x01) != 0;
        NRE1R { bits }
    }
    #[doc = "Bit 2 - Non-Recoverable State 2 Output Enable"]
    #[inline]
    pub fn nre2(&self) -> NRE2R {
        let bits = ((self.bits >> 2) & 0x01) != 0;
        NRE2R { bits }
    }
    #[doc = "Bit 3 - Non-Recoverable State 3 Output Enable"]
    #[inline]
    pub fn nre3(&self) -> NRE3R {
        let bits = ((self.bits >> 3) & 0x01) != 0;
        NRE3R { bits }
    }
    #[doc = "Bit 4 - Non-Recoverable State 4 Output Enable"]
    #[inline]
    pub fn nre4(&self) -> NRE4R {
        let bits = ((self.bits >> 4) & 0x01) != 0;
        NRE4R { bits }
    }
    #[doc = "Bit 5 - Non-Recoverable State 5 Output Enable"]
    #[inline]
    pub fn nre5(&self) -> NRE5R {
        let bits = ((self.bits >> 5) & 0x01) != 0;
        NRE5R { bits }
    }
    #[doc = "Bit 6 - Non-Recoverable State 6 Output Enable"]
    #[inline]
    pub fn nre6(&self) -> NRE6R {
        let bits = ((self.bits >> 6) & 0x01) != 0;
        NRE6R { bits }
    }
    #[doc = "Bit 7 - Non-Recoverable State 7 Output Enable"]
    #[inline]
    pub fn nre7(&self) -> NRE7R {
        let bits = ((self.bits >> 7) & 0x01) != 0;
        NRE7R { bits }
    }
    #[doc = "Bit 8 - Non-Recoverable State 0 Output Value"]
    #[inline]
    pub fn nrv0(&self) -> NRV0R {
        let bits = ((self.bits >> 8) & 0x01) != 0;
        NRV0R { bits }
    }
    #[doc = "Bit 9 - Non-Recoverable State 1 Output Value"]
    #[inline]
    pub fn nrv1(&self) -> NRV1R {
        let bits = ((self.bits >> 9) & 0x01) != 0;
        NRV1R { bits }
    }
    #[doc = "Bit 10 - Non-Recoverable State 2 Output Value"]
    #[inline]
    pub fn nrv2(&self) -> NRV2R {
        let bits = ((self.bits >> 10) & 0x01) != 0;
        NRV2R { bits }
    }
    #[doc = "Bit 11 - Non-Recoverable State 3 Output Value"]
    #[inline]
    pub fn nrv3(&self) -> NRV3R {
        let bits = ((self.bits >> 11) & 0x01) != 0;
        NRV3R { bits }
    }
    #[doc = "Bit 12 - Non-Recoverable State 4 Output Value"]
    #[inline]
    pub fn nrv4(&self) -> NRV4R {
        let bits = ((self.bits >> 12) & 0x01) != 0;
        NRV4R { bits }
    }
    #[doc = "Bit 13 - Non-Recoverable State 5 Output Value"]
    #[inline]
    pub fn nrv5(&self) -> NRV5R {
        let bits = ((self.bits >> 13) & 0x01) != 0;
        NRV5R { bits }
    }
    #[doc = "Bit 14 - Non-Recoverable State 6 Output Value"]
    #[inline]
    pub fn nrv6(&self) -> NRV6R {
        let bits = ((self.bits >> 14) & 0x01) != 0;
        NRV6R { bits }
    }
    #[doc = "Bit 15 - Non-Recoverable State 7 Output Value"]
    #[inline]
    pub fn nrv7(&self) -> NRV7R {
        let bits = ((self.bits >> 15) & 0x01) != 0;
        NRV7R { bits }
    }
    #[doc = "Bit 16 - Output Waveform 0 Inversion"]
    #[inline]
    pub fn inven0(&self) -> INVEN0R {
        let bits = ((self.bits >> 16) & 0x01) != 0;
        INVEN0R { bits }
    }
    #[doc = "Bit 17 - Output Waveform 1 Inversion"]
    #[inline]
    pub fn inven1(&self) -> INVEN1R {
        let bits = ((self.bits >> 17) & 0x01) != 0;
        INVEN1R { bits }
    }
    #[doc = "Bit 18 - Output Waveform 2 Inversion"]
    #[inline]
    pub fn inven2(&self) -> INVEN2R {
        let bits = ((self.bits >> 18) & 0x01) != 0;
        INVEN2R { bits }
    }
    #[doc = "Bit 19 - Output Waveform 3 Inversion"]
    #[inline]
    pub fn inven3(&self) -> INVEN3R {
        let bits = ((self.bits >> 19) & 0x01) != 0;
        INVEN3R { bits }
    }
    #[doc = "Bit 20 - Output Waveform 4 Inversion"]
    #[inline]
    pub fn inven4(&self) -> INVEN4R {
        let bits = ((self.bits >> 20) & 0x01) != 0;
        INVEN4R { bits }
    }
    #[doc = "Bit 21 - Output Waveform 5 Inversion"]
    #[inline]
    pub fn inven5(&self) -> INVEN5R {
        let bits = ((self.bits >> 21) & 0x01) != 0;
        INVEN5R { bits }
    }
    #[doc = "Bit 22 - Output Waveform 6 Inversion"]
    #[inline]
    pub fn inven6(&self) -> INVEN6R {
        let bits = ((self.bits >> 22) & 0x01) != 0;
        INVEN6R { bits }
    }
    #[doc = "Bit 23 - Output Waveform 7 Inversion"]
    #[inline]
    pub fn inven7(&self) -> INVEN7R {
        let bits = ((self.bits >> 23) & 0x01) != 0;
        INVEN7R { bits }
    }
    #[doc = "Bits 24:27 - Non-Recoverable Fault Input 0 Filter Value"]
    #[inline]
    pub fn filterval0(&self) -> FILTERVAL0R {
        let bits = ((self.bits >> 24) & 0x0f) as u8;
        FILTERVAL0R { bits }
    }
    #[doc = "Bits 28:31 - Non-Recoverable Fault Input 1 Filter Value"]
    #[inline]
    pub fn filterval1(&self) -> FILTERVAL1R {
        let bits = ((self.bits >> 28) & 0x0f) as u8;
        FILTERVAL1R { bits }
    }
}
impl W {
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Non-Recoverable State 0 Output Enable"]
    #[inline]
    pub fn nre0(&mut self) -> _NRE0W {
        _NRE0W { w: self }
    }
    #[doc = "Bit 1 - Non-Recoverable State 1 Output Enable"]
    #[inline]
    pub fn nre1(&mut self) -> _NRE1W {
        _NRE1W { w: self }
    }
    #[doc = "Bit 2 - Non-Recoverable State 2 Output Enable"]
    #[inline]
    pub fn nre2(&mut self) -> _NRE2W {
        _NRE2W { w: self }
    }
    #[doc = "Bit 3 - Non-Recoverable State 3 Output Enable"]
    #[inline]
    pub fn nre3(&mut self) -> _NRE3W {
        _NRE3W { w: self }
    }
    #[doc = "Bit 4 - Non-Recoverable State 4 Output Enable"]
    #[inline]
    pub fn nre4(&mut self) -> _NRE4W {
        _NRE4W { w: self }
    }
    #[doc = "Bit 5 - Non-Recoverable State 5 Output Enable"]
    #[inline]
    pub fn nre5(&mut self) -> _NRE5W {
        _NRE5W { w: self }
    }
    #[doc = "Bit 6 - Non-Recoverable State 6 Output Enable"]
    #[inline]
    pub fn nre6(&mut self) -> _NRE6W {
        _NRE6W { w: self }
    }
    #[doc = "Bit 7 - Non-Recoverable State 7 Output Enable"]
    #[inline]
    pub fn nre7(&mut self) -> _NRE7W {
        _NRE7W { w: self }
    }
    #[doc = "Bit 8 - Non-Recoverable State 0 Output Value"]
    #[inline]
    pub fn nrv0(&mut self) -> _NRV0W {
        _NRV0W { w: self }
    }
    #[doc = "Bit 9 - Non-Recoverable State 1 Output Value"]
    #[inline]
    pub fn nrv1(&mut self) -> _NRV1W {
        _NRV1W { w: self }
    }
    #[doc = "Bit 10 - Non-Recoverable State 2 Output Value"]
    #[inline]
    pub fn nrv2(&mut self) -> _NRV2W {
        _NRV2W { w: self }
    }
    #[doc = "Bit 11 - Non-Recoverable State 3 Output Value"]
    #[inline]
    pub fn nrv3(&mut self) -> _NRV3W {
        _NRV3W { w: self }
    }
    #[doc = "Bit 12 - Non-Recoverable State 4 Output Value"]
    #[inline]
    pub fn nrv4(&mut self) -> _NRV4W {
        _NRV4W { w: self }
    }
    #[doc = "Bit 13 - Non-Recoverable State 5 Output Value"]
    #[inline]
    pub fn nrv5(&mut self) -> _NRV5W {
        _NRV5W { w: self }
    }
    #[doc = "Bit 14 - Non-Recoverable State 6 Output Value"]
    #[inline]
    pub fn nrv6(&mut self) -> _NRV6W {
        _NRV6W { w: self }
    }
    #[doc = "Bit 15 - Non-Recoverable State 7 Output Value"]
    #[inline]
    pub fn nrv7(&mut self) -> _NRV7W {
        _NRV7W { w: self }
    }
    #[doc = "Bit 16 - Output Waveform 0 Inversion"]
    #[inline]
    pub fn inven0(&mut self) -> _INVEN0W {
        _INVEN0W { w: self }
    }
    #[doc = "Bit 17 - Output Waveform 1 Inversion"]
    #[inline]
    pub fn inven1(&mut self) -> _INVEN1W {
        _INVEN1W { w: self }
    }
    #[doc = "Bit 18 - Output Waveform 2 Inversion"]
    #[inline]
    pub fn inven2(&mut self) -> _INVEN2W {
        _INVEN2W { w: self }
    }
    #[doc = "Bit 19 - Output Waveform 3 Inversion"]
    #[inline]
    pub fn inven3(&mut self) -> _INVEN3W {
        _INVEN3W { w: self }
    }
    #[doc = "Bit 20 - Output Waveform 4 Inversion"]
    #[inline]
    pub fn inven4(&mut self) -> _INVEN4W {
        _INVEN4W { w: self }
    }
    #[doc = "Bit 21 - Output Waveform 5 Inversion"]
    #[inline]
    pub fn inven5(&mut self) -> _INVEN5W {
        _INVEN5W { w: self }
    }
    #[doc = "Bit 22 - Output Waveform 6 Inversion"]
    #[inline]
    pub fn inven6(&mut self) -> _INVEN6W {
        _INVEN6W { w: self }
    }
    #[doc = "Bit 23 - Output Waveform 7 Inversion"]
    #[inline]
    pub fn inven7(&mut self) -> _INVEN7W {
        _INVEN7W { w: self }
    }
    #[doc = "Bits 24:27 - Non-Recoverable Fault Input 0 Filter Value"]
    #[inline]
    pub fn filterval0(&mut self) -> _FILTERVAL0W {
        _FILTERVAL0W { w: self }
    }
    #[doc = "Bits 28:31 - Non-Recoverable Fault Input 1 Filter Value"]
    #[inline]
    pub fn filterval1(&mut self) -> _FILTERVAL1W {
        _FILTERVAL1W { w: self }
    }
}
