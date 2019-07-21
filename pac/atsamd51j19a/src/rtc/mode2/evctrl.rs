#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::EVCTRL {
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
pub struct PEREO0R {
    bits: bool,
}
impl PEREO0R {
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
pub struct PEREO1R {
    bits: bool,
}
impl PEREO1R {
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
pub struct PEREO2R {
    bits: bool,
}
impl PEREO2R {
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
pub struct PEREO3R {
    bits: bool,
}
impl PEREO3R {
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
pub struct PEREO4R {
    bits: bool,
}
impl PEREO4R {
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
pub struct PEREO5R {
    bits: bool,
}
impl PEREO5R {
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
pub struct PEREO6R {
    bits: bool,
}
impl PEREO6R {
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
pub struct PEREO7R {
    bits: bool,
}
impl PEREO7R {
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
pub struct ALARMEO0R {
    bits: bool,
}
impl ALARMEO0R {
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
pub struct ALARMEO1R {
    bits: bool,
}
impl ALARMEO1R {
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
pub struct TAMPEREOR {
    bits: bool,
}
impl TAMPEREOR {
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
pub struct OVFEOR {
    bits: bool,
}
impl OVFEOR {
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
pub struct TAMPEVEIR {
    bits: bool,
}
impl TAMPEVEIR {
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
pub struct _PEREO0W<'a> {
    w: &'a mut W,
}
impl<'a> _PEREO0W<'a> {
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
pub struct _PEREO1W<'a> {
    w: &'a mut W,
}
impl<'a> _PEREO1W<'a> {
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
pub struct _PEREO2W<'a> {
    w: &'a mut W,
}
impl<'a> _PEREO2W<'a> {
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
pub struct _PEREO3W<'a> {
    w: &'a mut W,
}
impl<'a> _PEREO3W<'a> {
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
pub struct _PEREO4W<'a> {
    w: &'a mut W,
}
impl<'a> _PEREO4W<'a> {
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
pub struct _PEREO5W<'a> {
    w: &'a mut W,
}
impl<'a> _PEREO5W<'a> {
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
pub struct _PEREO6W<'a> {
    w: &'a mut W,
}
impl<'a> _PEREO6W<'a> {
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
pub struct _PEREO7W<'a> {
    w: &'a mut W,
}
impl<'a> _PEREO7W<'a> {
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
pub struct _ALARMEO0W<'a> {
    w: &'a mut W,
}
impl<'a> _ALARMEO0W<'a> {
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
pub struct _ALARMEO1W<'a> {
    w: &'a mut W,
}
impl<'a> _ALARMEO1W<'a> {
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
pub struct _TAMPEREOW<'a> {
    w: &'a mut W,
}
impl<'a> _TAMPEREOW<'a> {
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
pub struct _OVFEOW<'a> {
    w: &'a mut W,
}
impl<'a> _OVFEOW<'a> {
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
pub struct _TAMPEVEIW<'a> {
    w: &'a mut W,
}
impl<'a> _TAMPEVEIW<'a> {
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
    #[doc = "Bit 0 - Periodic Interval 0 Event Output Enable"]
    #[inline]
    pub fn pereo0(&self) -> PEREO0R {
        let bits = ((self.bits >> 0) & 0x01) != 0;
        PEREO0R { bits }
    }
    #[doc = "Bit 1 - Periodic Interval 1 Event Output Enable"]
    #[inline]
    pub fn pereo1(&self) -> PEREO1R {
        let bits = ((self.bits >> 1) & 0x01) != 0;
        PEREO1R { bits }
    }
    #[doc = "Bit 2 - Periodic Interval 2 Event Output Enable"]
    #[inline]
    pub fn pereo2(&self) -> PEREO2R {
        let bits = ((self.bits >> 2) & 0x01) != 0;
        PEREO2R { bits }
    }
    #[doc = "Bit 3 - Periodic Interval 3 Event Output Enable"]
    #[inline]
    pub fn pereo3(&self) -> PEREO3R {
        let bits = ((self.bits >> 3) & 0x01) != 0;
        PEREO3R { bits }
    }
    #[doc = "Bit 4 - Periodic Interval 4 Event Output Enable"]
    #[inline]
    pub fn pereo4(&self) -> PEREO4R {
        let bits = ((self.bits >> 4) & 0x01) != 0;
        PEREO4R { bits }
    }
    #[doc = "Bit 5 - Periodic Interval 5 Event Output Enable"]
    #[inline]
    pub fn pereo5(&self) -> PEREO5R {
        let bits = ((self.bits >> 5) & 0x01) != 0;
        PEREO5R { bits }
    }
    #[doc = "Bit 6 - Periodic Interval 6 Event Output Enable"]
    #[inline]
    pub fn pereo6(&self) -> PEREO6R {
        let bits = ((self.bits >> 6) & 0x01) != 0;
        PEREO6R { bits }
    }
    #[doc = "Bit 7 - Periodic Interval 7 Event Output Enable"]
    #[inline]
    pub fn pereo7(&self) -> PEREO7R {
        let bits = ((self.bits >> 7) & 0x01) != 0;
        PEREO7R { bits }
    }
    #[doc = "Bit 8 - Alarm 0 Event Output Enable"]
    #[inline]
    pub fn alarmeo0(&self) -> ALARMEO0R {
        let bits = ((self.bits >> 8) & 0x01) != 0;
        ALARMEO0R { bits }
    }
    #[doc = "Bit 9 - Alarm 1 Event Output Enable"]
    #[inline]
    pub fn alarmeo1(&self) -> ALARMEO1R {
        let bits = ((self.bits >> 9) & 0x01) != 0;
        ALARMEO1R { bits }
    }
    #[doc = "Bit 14 - Tamper Event Output Enable"]
    #[inline]
    pub fn tampereo(&self) -> TAMPEREOR {
        let bits = ((self.bits >> 14) & 0x01) != 0;
        TAMPEREOR { bits }
    }
    #[doc = "Bit 15 - Overflow Event Output Enable"]
    #[inline]
    pub fn ovfeo(&self) -> OVFEOR {
        let bits = ((self.bits >> 15) & 0x01) != 0;
        OVFEOR { bits }
    }
    #[doc = "Bit 16 - Tamper Event Input Enable"]
    #[inline]
    pub fn tampevei(&self) -> TAMPEVEIR {
        let bits = ((self.bits >> 16) & 0x01) != 0;
        TAMPEVEIR { bits }
    }
}
impl W {
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Periodic Interval 0 Event Output Enable"]
    #[inline]
    pub fn pereo0(&mut self) -> _PEREO0W {
        _PEREO0W { w: self }
    }
    #[doc = "Bit 1 - Periodic Interval 1 Event Output Enable"]
    #[inline]
    pub fn pereo1(&mut self) -> _PEREO1W {
        _PEREO1W { w: self }
    }
    #[doc = "Bit 2 - Periodic Interval 2 Event Output Enable"]
    #[inline]
    pub fn pereo2(&mut self) -> _PEREO2W {
        _PEREO2W { w: self }
    }
    #[doc = "Bit 3 - Periodic Interval 3 Event Output Enable"]
    #[inline]
    pub fn pereo3(&mut self) -> _PEREO3W {
        _PEREO3W { w: self }
    }
    #[doc = "Bit 4 - Periodic Interval 4 Event Output Enable"]
    #[inline]
    pub fn pereo4(&mut self) -> _PEREO4W {
        _PEREO4W { w: self }
    }
    #[doc = "Bit 5 - Periodic Interval 5 Event Output Enable"]
    #[inline]
    pub fn pereo5(&mut self) -> _PEREO5W {
        _PEREO5W { w: self }
    }
    #[doc = "Bit 6 - Periodic Interval 6 Event Output Enable"]
    #[inline]
    pub fn pereo6(&mut self) -> _PEREO6W {
        _PEREO6W { w: self }
    }
    #[doc = "Bit 7 - Periodic Interval 7 Event Output Enable"]
    #[inline]
    pub fn pereo7(&mut self) -> _PEREO7W {
        _PEREO7W { w: self }
    }
    #[doc = "Bit 8 - Alarm 0 Event Output Enable"]
    #[inline]
    pub fn alarmeo0(&mut self) -> _ALARMEO0W {
        _ALARMEO0W { w: self }
    }
    #[doc = "Bit 9 - Alarm 1 Event Output Enable"]
    #[inline]
    pub fn alarmeo1(&mut self) -> _ALARMEO1W {
        _ALARMEO1W { w: self }
    }
    #[doc = "Bit 14 - Tamper Event Output Enable"]
    #[inline]
    pub fn tampereo(&mut self) -> _TAMPEREOW {
        _TAMPEREOW { w: self }
    }
    #[doc = "Bit 15 - Overflow Event Output Enable"]
    #[inline]
    pub fn ovfeo(&mut self) -> _OVFEOW {
        _OVFEOW { w: self }
    }
    #[doc = "Bit 16 - Tamper Event Input Enable"]
    #[inline]
    pub fn tampevei(&mut self) -> _TAMPEVEIW {
        _TAMPEVEIW { w: self }
    }
}
