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
        0x4040_4040
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
#[doc = "Possible values of the field `QOS0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum QOS0R {
    #[doc = "Regular delivery"]
    REGULAR,
    #[doc = "Bandwidth shortage"]
    SHORTAGE,
    #[doc = "Latency sensitive"]
    SENSITIVE,
    #[doc = "Latency critical"]
    CRITICAL,
}
impl QOS0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            QOS0R::REGULAR => 0,
            QOS0R::SHORTAGE => 0x01,
            QOS0R::SENSITIVE => 0x02,
            QOS0R::CRITICAL => 0x03,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> QOS0R {
        match value {
            0 => QOS0R::REGULAR,
            1 => QOS0R::SHORTAGE,
            2 => QOS0R::SENSITIVE,
            3 => QOS0R::CRITICAL,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `REGULAR`"]
    #[inline]
    pub fn is_regular(&self) -> bool {
        *self == QOS0R::REGULAR
    }
    #[doc = "Checks if the value of the field is `SHORTAGE`"]
    #[inline]
    pub fn is_shortage(&self) -> bool {
        *self == QOS0R::SHORTAGE
    }
    #[doc = "Checks if the value of the field is `SENSITIVE`"]
    #[inline]
    pub fn is_sensitive(&self) -> bool {
        *self == QOS0R::SENSITIVE
    }
    #[doc = "Checks if the value of the field is `CRITICAL`"]
    #[inline]
    pub fn is_critical(&self) -> bool {
        *self == QOS0R::CRITICAL
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
#[doc = "Possible values of the field `QOS1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum QOS1R {
    #[doc = "Regular delivery"]
    REGULAR,
    #[doc = "Bandwidth shortage"]
    SHORTAGE,
    #[doc = "Latency sensitive"]
    SENSITIVE,
    #[doc = "Latency critical"]
    CRITICAL,
}
impl QOS1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            QOS1R::REGULAR => 0,
            QOS1R::SHORTAGE => 0x01,
            QOS1R::SENSITIVE => 0x02,
            QOS1R::CRITICAL => 0x03,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> QOS1R {
        match value {
            0 => QOS1R::REGULAR,
            1 => QOS1R::SHORTAGE,
            2 => QOS1R::SENSITIVE,
            3 => QOS1R::CRITICAL,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `REGULAR`"]
    #[inline]
    pub fn is_regular(&self) -> bool {
        *self == QOS1R::REGULAR
    }
    #[doc = "Checks if the value of the field is `SHORTAGE`"]
    #[inline]
    pub fn is_shortage(&self) -> bool {
        *self == QOS1R::SHORTAGE
    }
    #[doc = "Checks if the value of the field is `SENSITIVE`"]
    #[inline]
    pub fn is_sensitive(&self) -> bool {
        *self == QOS1R::SENSITIVE
    }
    #[doc = "Checks if the value of the field is `CRITICAL`"]
    #[inline]
    pub fn is_critical(&self) -> bool {
        *self == QOS1R::CRITICAL
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
#[doc = "Possible values of the field `QOS2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum QOS2R {
    #[doc = "Regular delivery"]
    REGULAR,
    #[doc = "Bandwidth shortage"]
    SHORTAGE,
    #[doc = "Latency sensitive"]
    SENSITIVE,
    #[doc = "Latency critical"]
    CRITICAL,
}
impl QOS2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            QOS2R::REGULAR => 0,
            QOS2R::SHORTAGE => 0x01,
            QOS2R::SENSITIVE => 0x02,
            QOS2R::CRITICAL => 0x03,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> QOS2R {
        match value {
            0 => QOS2R::REGULAR,
            1 => QOS2R::SHORTAGE,
            2 => QOS2R::SENSITIVE,
            3 => QOS2R::CRITICAL,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `REGULAR`"]
    #[inline]
    pub fn is_regular(&self) -> bool {
        *self == QOS2R::REGULAR
    }
    #[doc = "Checks if the value of the field is `SHORTAGE`"]
    #[inline]
    pub fn is_shortage(&self) -> bool {
        *self == QOS2R::SHORTAGE
    }
    #[doc = "Checks if the value of the field is `SENSITIVE`"]
    #[inline]
    pub fn is_sensitive(&self) -> bool {
        *self == QOS2R::SENSITIVE
    }
    #[doc = "Checks if the value of the field is `CRITICAL`"]
    #[inline]
    pub fn is_critical(&self) -> bool {
        *self == QOS2R::CRITICAL
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
#[doc = "Possible values of the field `QOS3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum QOS3R {
    #[doc = "Regular delivery"]
    REGULAR,
    #[doc = "Bandwidth shortage"]
    SHORTAGE,
    #[doc = "Latency sensitive"]
    SENSITIVE,
    #[doc = "Latency critical"]
    CRITICAL,
}
impl QOS3R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            QOS3R::REGULAR => 0,
            QOS3R::SHORTAGE => 0x01,
            QOS3R::SENSITIVE => 0x02,
            QOS3R::CRITICAL => 0x03,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> QOS3R {
        match value {
            0 => QOS3R::REGULAR,
            1 => QOS3R::SHORTAGE,
            2 => QOS3R::SENSITIVE,
            3 => QOS3R::CRITICAL,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `REGULAR`"]
    #[inline]
    pub fn is_regular(&self) -> bool {
        *self == QOS3R::REGULAR
    }
    #[doc = "Checks if the value of the field is `SHORTAGE`"]
    #[inline]
    pub fn is_shortage(&self) -> bool {
        *self == QOS3R::SHORTAGE
    }
    #[doc = "Checks if the value of the field is `SENSITIVE`"]
    #[inline]
    pub fn is_sensitive(&self) -> bool {
        *self == QOS3R::SENSITIVE
    }
    #[doc = "Checks if the value of the field is `CRITICAL`"]
    #[inline]
    pub fn is_critical(&self) -> bool {
        *self == QOS3R::CRITICAL
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
        self.w.bits &= !(0x1f << 0);
        self.w.bits |= ((value as u32) & 0x1f) << 0;
        self.w
    }
}
#[doc = "Values that can be written to the field `QOS0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum QOS0W {
    #[doc = "Regular delivery"]
    REGULAR,
    #[doc = "Bandwidth shortage"]
    SHORTAGE,
    #[doc = "Latency sensitive"]
    SENSITIVE,
    #[doc = "Latency critical"]
    CRITICAL,
}
impl QOS0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            QOS0W::REGULAR => 0,
            QOS0W::SHORTAGE => 1,
            QOS0W::SENSITIVE => 2,
            QOS0W::CRITICAL => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _QOS0W<'a> {
    w: &'a mut W,
}
impl<'a> _QOS0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: QOS0W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Regular delivery"]
    #[inline]
    pub fn regular(self) -> &'a mut W {
        self.variant(QOS0W::REGULAR)
    }
    #[doc = "Bandwidth shortage"]
    #[inline]
    pub fn shortage(self) -> &'a mut W {
        self.variant(QOS0W::SHORTAGE)
    }
    #[doc = "Latency sensitive"]
    #[inline]
    pub fn sensitive(self) -> &'a mut W {
        self.variant(QOS0W::SENSITIVE)
    }
    #[doc = "Latency critical"]
    #[inline]
    pub fn critical(self) -> &'a mut W {
        self.variant(QOS0W::CRITICAL)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(0x03 << 5);
        self.w.bits |= ((value as u32) & 0x03) << 5;
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
        self.w.bits &= !(0x1f << 8);
        self.w.bits |= ((value as u32) & 0x1f) << 8;
        self.w
    }
}
#[doc = "Values that can be written to the field `QOS1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum QOS1W {
    #[doc = "Regular delivery"]
    REGULAR,
    #[doc = "Bandwidth shortage"]
    SHORTAGE,
    #[doc = "Latency sensitive"]
    SENSITIVE,
    #[doc = "Latency critical"]
    CRITICAL,
}
impl QOS1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            QOS1W::REGULAR => 0,
            QOS1W::SHORTAGE => 1,
            QOS1W::SENSITIVE => 2,
            QOS1W::CRITICAL => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _QOS1W<'a> {
    w: &'a mut W,
}
impl<'a> _QOS1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: QOS1W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Regular delivery"]
    #[inline]
    pub fn regular(self) -> &'a mut W {
        self.variant(QOS1W::REGULAR)
    }
    #[doc = "Bandwidth shortage"]
    #[inline]
    pub fn shortage(self) -> &'a mut W {
        self.variant(QOS1W::SHORTAGE)
    }
    #[doc = "Latency sensitive"]
    #[inline]
    pub fn sensitive(self) -> &'a mut W {
        self.variant(QOS1W::SENSITIVE)
    }
    #[doc = "Latency critical"]
    #[inline]
    pub fn critical(self) -> &'a mut W {
        self.variant(QOS1W::CRITICAL)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(0x03 << 13);
        self.w.bits |= ((value as u32) & 0x03) << 13;
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
        self.w.bits &= !(0x1f << 16);
        self.w.bits |= ((value as u32) & 0x1f) << 16;
        self.w
    }
}
#[doc = "Values that can be written to the field `QOS2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum QOS2W {
    #[doc = "Regular delivery"]
    REGULAR,
    #[doc = "Bandwidth shortage"]
    SHORTAGE,
    #[doc = "Latency sensitive"]
    SENSITIVE,
    #[doc = "Latency critical"]
    CRITICAL,
}
impl QOS2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            QOS2W::REGULAR => 0,
            QOS2W::SHORTAGE => 1,
            QOS2W::SENSITIVE => 2,
            QOS2W::CRITICAL => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _QOS2W<'a> {
    w: &'a mut W,
}
impl<'a> _QOS2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: QOS2W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Regular delivery"]
    #[inline]
    pub fn regular(self) -> &'a mut W {
        self.variant(QOS2W::REGULAR)
    }
    #[doc = "Bandwidth shortage"]
    #[inline]
    pub fn shortage(self) -> &'a mut W {
        self.variant(QOS2W::SHORTAGE)
    }
    #[doc = "Latency sensitive"]
    #[inline]
    pub fn sensitive(self) -> &'a mut W {
        self.variant(QOS2W::SENSITIVE)
    }
    #[doc = "Latency critical"]
    #[inline]
    pub fn critical(self) -> &'a mut W {
        self.variant(QOS2W::CRITICAL)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(0x03 << 21);
        self.w.bits |= ((value as u32) & 0x03) << 21;
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
        self.w.bits &= !(0x1f << 24);
        self.w.bits |= ((value as u32) & 0x1f) << 24;
        self.w
    }
}
#[doc = "Values that can be written to the field `QOS3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum QOS3W {
    #[doc = "Regular delivery"]
    REGULAR,
    #[doc = "Bandwidth shortage"]
    SHORTAGE,
    #[doc = "Latency sensitive"]
    SENSITIVE,
    #[doc = "Latency critical"]
    CRITICAL,
}
impl QOS3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            QOS3W::REGULAR => 0,
            QOS3W::SHORTAGE => 1,
            QOS3W::SENSITIVE => 2,
            QOS3W::CRITICAL => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _QOS3W<'a> {
    w: &'a mut W,
}
impl<'a> _QOS3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: QOS3W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Regular delivery"]
    #[inline]
    pub fn regular(self) -> &'a mut W {
        self.variant(QOS3W::REGULAR)
    }
    #[doc = "Bandwidth shortage"]
    #[inline]
    pub fn shortage(self) -> &'a mut W {
        self.variant(QOS3W::SHORTAGE)
    }
    #[doc = "Latency sensitive"]
    #[inline]
    pub fn sensitive(self) -> &'a mut W {
        self.variant(QOS3W::SENSITIVE)
    }
    #[doc = "Latency critical"]
    #[inline]
    pub fn critical(self) -> &'a mut W {
        self.variant(QOS3W::CRITICAL)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(0x03 << 29);
        self.w.bits |= ((value as u32) & 0x03) << 29;
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
    #[doc = "Bits 0:4 - Level 0 Channel Priority Number"]
    #[inline]
    pub fn lvlpri0(&self) -> LVLPRI0R {
        let bits = ((self.bits >> 0) & 0x1f) as u8;
        LVLPRI0R { bits }
    }
    #[doc = "Bits 5:6 - Level 0 Quality of Service"]
    #[inline]
    pub fn qos0(&self) -> QOS0R {
        QOS0R::_from(((self.bits >> 5) & 0x03) as u8)
    }
    #[doc = "Bit 7 - Level 0 Round-Robin Scheduling Enable"]
    #[inline]
    pub fn rrlvlen0(&self) -> RRLVLEN0R {
        let bits = ((self.bits >> 7) & 0x01) != 0;
        RRLVLEN0R { bits }
    }
    #[doc = "Bits 8:12 - Level 1 Channel Priority Number"]
    #[inline]
    pub fn lvlpri1(&self) -> LVLPRI1R {
        let bits = ((self.bits >> 8) & 0x1f) as u8;
        LVLPRI1R { bits }
    }
    #[doc = "Bits 13:14 - Level 1 Quality of Service"]
    #[inline]
    pub fn qos1(&self) -> QOS1R {
        QOS1R::_from(((self.bits >> 13) & 0x03) as u8)
    }
    #[doc = "Bit 15 - Level 1 Round-Robin Scheduling Enable"]
    #[inline]
    pub fn rrlvlen1(&self) -> RRLVLEN1R {
        let bits = ((self.bits >> 15) & 0x01) != 0;
        RRLVLEN1R { bits }
    }
    #[doc = "Bits 16:20 - Level 2 Channel Priority Number"]
    #[inline]
    pub fn lvlpri2(&self) -> LVLPRI2R {
        let bits = ((self.bits >> 16) & 0x1f) as u8;
        LVLPRI2R { bits }
    }
    #[doc = "Bits 21:22 - Level 2 Quality of Service"]
    #[inline]
    pub fn qos2(&self) -> QOS2R {
        QOS2R::_from(((self.bits >> 21) & 0x03) as u8)
    }
    #[doc = "Bit 23 - Level 2 Round-Robin Scheduling Enable"]
    #[inline]
    pub fn rrlvlen2(&self) -> RRLVLEN2R {
        let bits = ((self.bits >> 23) & 0x01) != 0;
        RRLVLEN2R { bits }
    }
    #[doc = "Bits 24:28 - Level 3 Channel Priority Number"]
    #[inline]
    pub fn lvlpri3(&self) -> LVLPRI3R {
        let bits = ((self.bits >> 24) & 0x1f) as u8;
        LVLPRI3R { bits }
    }
    #[doc = "Bits 29:30 - Level 3 Quality of Service"]
    #[inline]
    pub fn qos3(&self) -> QOS3R {
        QOS3R::_from(((self.bits >> 29) & 0x03) as u8)
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
    #[doc = "Bits 0:4 - Level 0 Channel Priority Number"]
    #[inline]
    pub fn lvlpri0(&mut self) -> _LVLPRI0W {
        _LVLPRI0W { w: self }
    }
    #[doc = "Bits 5:6 - Level 0 Quality of Service"]
    #[inline]
    pub fn qos0(&mut self) -> _QOS0W {
        _QOS0W { w: self }
    }
    #[doc = "Bit 7 - Level 0 Round-Robin Scheduling Enable"]
    #[inline]
    pub fn rrlvlen0(&mut self) -> _RRLVLEN0W {
        _RRLVLEN0W { w: self }
    }
    #[doc = "Bits 8:12 - Level 1 Channel Priority Number"]
    #[inline]
    pub fn lvlpri1(&mut self) -> _LVLPRI1W {
        _LVLPRI1W { w: self }
    }
    #[doc = "Bits 13:14 - Level 1 Quality of Service"]
    #[inline]
    pub fn qos1(&mut self) -> _QOS1W {
        _QOS1W { w: self }
    }
    #[doc = "Bit 15 - Level 1 Round-Robin Scheduling Enable"]
    #[inline]
    pub fn rrlvlen1(&mut self) -> _RRLVLEN1W {
        _RRLVLEN1W { w: self }
    }
    #[doc = "Bits 16:20 - Level 2 Channel Priority Number"]
    #[inline]
    pub fn lvlpri2(&mut self) -> _LVLPRI2W {
        _LVLPRI2W { w: self }
    }
    #[doc = "Bits 21:22 - Level 2 Quality of Service"]
    #[inline]
    pub fn qos2(&mut self) -> _QOS2W {
        _QOS2W { w: self }
    }
    #[doc = "Bit 23 - Level 2 Round-Robin Scheduling Enable"]
    #[inline]
    pub fn rrlvlen2(&mut self) -> _RRLVLEN2W {
        _RRLVLEN2W { w: self }
    }
    #[doc = "Bits 24:28 - Level 3 Channel Priority Number"]
    #[inline]
    pub fn lvlpri3(&mut self) -> _LVLPRI3W {
        _LVLPRI3W { w: self }
    }
    #[doc = "Bits 29:30 - Level 3 Quality of Service"]
    #[inline]
    pub fn qos3(&mut self) -> _QOS3W {
        _QOS3W { w: self }
    }
    #[doc = "Bit 31 - Level 3 Round-Robin Scheduling Enable"]
    #[inline]
    pub fn rrlvlen3(&mut self) -> _RRLVLEN3W {
        _RRLVLEN3W { w: self }
    }
}
