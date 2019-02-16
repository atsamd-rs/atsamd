#[doc = r" Value read from the register"]
pub struct R {
    bits: u16,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u16,
}
impl super::EISIER {
    #[doc = r" Modifies the contents of the register"]
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        let r = R { bits };
        let mut w = W { bits };
        f(&r, &mut w);
        self.register.set(w.bits);
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
        let mut w = W::reset_value();
        f(&mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.write(|w| w)
    }
}
#[doc = "Possible values of the field `CMDTEO`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMDTEOR {
    #[doc = "Masked"]
    MASKED,
    #[doc = "Enabled"]
    ENABLED,
}
impl CMDTEOR {
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
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            CMDTEOR::MASKED => false,
            CMDTEOR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CMDTEOR {
        match value {
            false => CMDTEOR::MASKED,
            true => CMDTEOR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `MASKED`"]
    #[inline]
    pub fn is_masked(&self) -> bool {
        *self == CMDTEOR::MASKED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == CMDTEOR::ENABLED
    }
}
#[doc = "Possible values of the field `CMDCRC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMDCRCR {
    #[doc = "Masked"]
    MASKED,
    #[doc = "Enabled"]
    ENABLED,
}
impl CMDCRCR {
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
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            CMDCRCR::MASKED => false,
            CMDCRCR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CMDCRCR {
        match value {
            false => CMDCRCR::MASKED,
            true => CMDCRCR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `MASKED`"]
    #[inline]
    pub fn is_masked(&self) -> bool {
        *self == CMDCRCR::MASKED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == CMDCRCR::ENABLED
    }
}
#[doc = "Possible values of the field `CMDEND`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMDENDR {
    #[doc = "Masked"]
    MASKED,
    #[doc = "Enabled"]
    ENABLED,
}
impl CMDENDR {
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
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            CMDENDR::MASKED => false,
            CMDENDR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CMDENDR {
        match value {
            false => CMDENDR::MASKED,
            true => CMDENDR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `MASKED`"]
    #[inline]
    pub fn is_masked(&self) -> bool {
        *self == CMDENDR::MASKED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == CMDENDR::ENABLED
    }
}
#[doc = "Possible values of the field `CMDIDX`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMDIDXR {
    #[doc = "Masked"]
    MASKED,
    #[doc = "Enabled"]
    ENABLED,
}
impl CMDIDXR {
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
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            CMDIDXR::MASKED => false,
            CMDIDXR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CMDIDXR {
        match value {
            false => CMDIDXR::MASKED,
            true => CMDIDXR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `MASKED`"]
    #[inline]
    pub fn is_masked(&self) -> bool {
        *self == CMDIDXR::MASKED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == CMDIDXR::ENABLED
    }
}
#[doc = "Possible values of the field `DATTEO`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DATTEOR {
    #[doc = "Masked"]
    MASKED,
    #[doc = "Enabled"]
    ENABLED,
}
impl DATTEOR {
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
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            DATTEOR::MASKED => false,
            DATTEOR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DATTEOR {
        match value {
            false => DATTEOR::MASKED,
            true => DATTEOR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `MASKED`"]
    #[inline]
    pub fn is_masked(&self) -> bool {
        *self == DATTEOR::MASKED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == DATTEOR::ENABLED
    }
}
#[doc = "Possible values of the field `DATCRC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DATCRCR {
    #[doc = "Masked"]
    MASKED,
    #[doc = "Enabled"]
    ENABLED,
}
impl DATCRCR {
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
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            DATCRCR::MASKED => false,
            DATCRCR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DATCRCR {
        match value {
            false => DATCRCR::MASKED,
            true => DATCRCR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `MASKED`"]
    #[inline]
    pub fn is_masked(&self) -> bool {
        *self == DATCRCR::MASKED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == DATCRCR::ENABLED
    }
}
#[doc = "Possible values of the field `DATEND`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DATENDR {
    #[doc = "Masked"]
    MASKED,
    #[doc = "Enabled"]
    ENABLED,
}
impl DATENDR {
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
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            DATENDR::MASKED => false,
            DATENDR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DATENDR {
        match value {
            false => DATENDR::MASKED,
            true => DATENDR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `MASKED`"]
    #[inline]
    pub fn is_masked(&self) -> bool {
        *self == DATENDR::MASKED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == DATENDR::ENABLED
    }
}
#[doc = "Possible values of the field `CURLIM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CURLIMR {
    #[doc = "Masked"]
    MASKED,
    #[doc = "Enabled"]
    ENABLED,
}
impl CURLIMR {
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
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            CURLIMR::MASKED => false,
            CURLIMR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CURLIMR {
        match value {
            false => CURLIMR::MASKED,
            true => CURLIMR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `MASKED`"]
    #[inline]
    pub fn is_masked(&self) -> bool {
        *self == CURLIMR::MASKED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == CURLIMR::ENABLED
    }
}
#[doc = "Possible values of the field `ACMD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACMDR {
    #[doc = "Masked"]
    MASKED,
    #[doc = "Enabled"]
    ENABLED,
}
impl ACMDR {
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
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            ACMDR::MASKED => false,
            ACMDR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ACMDR {
        match value {
            false => ACMDR::MASKED,
            true => ACMDR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `MASKED`"]
    #[inline]
    pub fn is_masked(&self) -> bool {
        *self == ACMDR::MASKED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == ACMDR::ENABLED
    }
}
#[doc = "Possible values of the field `ADMA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADMAR {
    #[doc = "Masked"]
    MASKED,
    #[doc = "Enabled"]
    ENABLED,
}
impl ADMAR {
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
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            ADMAR::MASKED => false,
            ADMAR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ADMAR {
        match value {
            false => ADMAR::MASKED,
            true => ADMAR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `MASKED`"]
    #[inline]
    pub fn is_masked(&self) -> bool {
        *self == ADMAR::MASKED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == ADMAR::ENABLED
    }
}
#[doc = "Values that can be written to the field `CMDTEO`"]
pub enum CMDTEOW {
    #[doc = "Masked"]
    MASKED,
    #[doc = "Enabled"]
    ENABLED,
}
impl CMDTEOW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CMDTEOW::MASKED => false,
            CMDTEOW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CMDTEOW<'a> {
    w: &'a mut W,
}
impl<'a> _CMDTEOW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CMDTEOW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Masked"]
    #[inline]
    pub fn masked(self) -> &'a mut W {
        self.variant(CMDTEOW::MASKED)
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CMDTEOW::ENABLED)
    }
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
        const MASK: bool = true;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CMDCRC`"]
pub enum CMDCRCW {
    #[doc = "Masked"]
    MASKED,
    #[doc = "Enabled"]
    ENABLED,
}
impl CMDCRCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CMDCRCW::MASKED => false,
            CMDCRCW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CMDCRCW<'a> {
    w: &'a mut W,
}
impl<'a> _CMDCRCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CMDCRCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Masked"]
    #[inline]
    pub fn masked(self) -> &'a mut W {
        self.variant(CMDCRCW::MASKED)
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CMDCRCW::ENABLED)
    }
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
        const MASK: bool = true;
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CMDEND`"]
pub enum CMDENDW {
    #[doc = "Masked"]
    MASKED,
    #[doc = "Enabled"]
    ENABLED,
}
impl CMDENDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CMDENDW::MASKED => false,
            CMDENDW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CMDENDW<'a> {
    w: &'a mut W,
}
impl<'a> _CMDENDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CMDENDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Masked"]
    #[inline]
    pub fn masked(self) -> &'a mut W {
        self.variant(CMDENDW::MASKED)
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CMDENDW::ENABLED)
    }
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
        const MASK: bool = true;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CMDIDX`"]
pub enum CMDIDXW {
    #[doc = "Masked"]
    MASKED,
    #[doc = "Enabled"]
    ENABLED,
}
impl CMDIDXW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CMDIDXW::MASKED => false,
            CMDIDXW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CMDIDXW<'a> {
    w: &'a mut W,
}
impl<'a> _CMDIDXW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CMDIDXW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Masked"]
    #[inline]
    pub fn masked(self) -> &'a mut W {
        self.variant(CMDIDXW::MASKED)
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CMDIDXW::ENABLED)
    }
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
        const MASK: bool = true;
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DATTEO`"]
pub enum DATTEOW {
    #[doc = "Masked"]
    MASKED,
    #[doc = "Enabled"]
    ENABLED,
}
impl DATTEOW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DATTEOW::MASKED => false,
            DATTEOW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DATTEOW<'a> {
    w: &'a mut W,
}
impl<'a> _DATTEOW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DATTEOW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Masked"]
    #[inline]
    pub fn masked(self) -> &'a mut W {
        self.variant(DATTEOW::MASKED)
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DATTEOW::ENABLED)
    }
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
        const MASK: bool = true;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DATCRC`"]
pub enum DATCRCW {
    #[doc = "Masked"]
    MASKED,
    #[doc = "Enabled"]
    ENABLED,
}
impl DATCRCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DATCRCW::MASKED => false,
            DATCRCW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DATCRCW<'a> {
    w: &'a mut W,
}
impl<'a> _DATCRCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DATCRCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Masked"]
    #[inline]
    pub fn masked(self) -> &'a mut W {
        self.variant(DATCRCW::MASKED)
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DATCRCW::ENABLED)
    }
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
        const MASK: bool = true;
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DATEND`"]
pub enum DATENDW {
    #[doc = "Masked"]
    MASKED,
    #[doc = "Enabled"]
    ENABLED,
}
impl DATENDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DATENDW::MASKED => false,
            DATENDW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DATENDW<'a> {
    w: &'a mut W,
}
impl<'a> _DATENDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DATENDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Masked"]
    #[inline]
    pub fn masked(self) -> &'a mut W {
        self.variant(DATENDW::MASKED)
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DATENDW::ENABLED)
    }
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
        const MASK: bool = true;
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CURLIM`"]
pub enum CURLIMW {
    #[doc = "Masked"]
    MASKED,
    #[doc = "Enabled"]
    ENABLED,
}
impl CURLIMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CURLIMW::MASKED => false,
            CURLIMW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CURLIMW<'a> {
    w: &'a mut W,
}
impl<'a> _CURLIMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CURLIMW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Masked"]
    #[inline]
    pub fn masked(self) -> &'a mut W {
        self.variant(CURLIMW::MASKED)
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CURLIMW::ENABLED)
    }
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
        const MASK: bool = true;
        const OFFSET: u8 = 7;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ACMD`"]
pub enum ACMDW {
    #[doc = "Masked"]
    MASKED,
    #[doc = "Enabled"]
    ENABLED,
}
impl ACMDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ACMDW::MASKED => false,
            ACMDW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ACMDW<'a> {
    w: &'a mut W,
}
impl<'a> _ACMDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ACMDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Masked"]
    #[inline]
    pub fn masked(self) -> &'a mut W {
        self.variant(ACMDW::MASKED)
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ACMDW::ENABLED)
    }
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
        const MASK: bool = true;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ADMA`"]
pub enum ADMAW {
    #[doc = "Masked"]
    MASKED,
    #[doc = "Enabled"]
    ENABLED,
}
impl ADMAW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ADMAW::MASKED => false,
            ADMAW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ADMAW<'a> {
    w: &'a mut W,
}
impl<'a> _ADMAW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ADMAW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Masked"]
    #[inline]
    pub fn masked(self) -> &'a mut W {
        self.variant(ADMAW::MASKED)
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ADMAW::ENABLED)
    }
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
        const MASK: bool = true;
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
    #[doc = "Bit 0 - Command Timeout Error Signal Enable"]
    #[inline]
    pub fn cmdteo(&self) -> CMDTEOR {
        CMDTEOR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 1 - Command CRC Error Signal Enable"]
    #[inline]
    pub fn cmdcrc(&self) -> CMDCRCR {
        CMDCRCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 2 - Command End Bit Error Signal Enable"]
    #[inline]
    pub fn cmdend(&self) -> CMDENDR {
        CMDENDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 3 - Command Index Error Signal Enable"]
    #[inline]
    pub fn cmdidx(&self) -> CMDIDXR {
        CMDIDXR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 4 - Data Timeout Error Signal Enable"]
    #[inline]
    pub fn datteo(&self) -> DATTEOR {
        DATTEOR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 5 - Data CRC Error Signal Enable"]
    #[inline]
    pub fn datcrc(&self) -> DATCRCR {
        DATCRCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 6 - Data End Bit Error Signal Enable"]
    #[inline]
    pub fn datend(&self) -> DATENDR {
        DATENDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 7 - Current Limit Error Signal Enable"]
    #[inline]
    pub fn curlim(&self) -> CURLIMR {
        CURLIMR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 8 - Auto CMD Error Signal Enable"]
    #[inline]
    pub fn acmd(&self) -> ACMDR {
        ACMDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 9 - ADMA Error Signal Enable"]
    #[inline]
    pub fn adma(&self) -> ADMAR {
        ADMAR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Command Timeout Error Signal Enable"]
    #[inline]
    pub fn cmdteo(&mut self) -> _CMDTEOW {
        _CMDTEOW { w: self }
    }
    #[doc = "Bit 1 - Command CRC Error Signal Enable"]
    #[inline]
    pub fn cmdcrc(&mut self) -> _CMDCRCW {
        _CMDCRCW { w: self }
    }
    #[doc = "Bit 2 - Command End Bit Error Signal Enable"]
    #[inline]
    pub fn cmdend(&mut self) -> _CMDENDW {
        _CMDENDW { w: self }
    }
    #[doc = "Bit 3 - Command Index Error Signal Enable"]
    #[inline]
    pub fn cmdidx(&mut self) -> _CMDIDXW {
        _CMDIDXW { w: self }
    }
    #[doc = "Bit 4 - Data Timeout Error Signal Enable"]
    #[inline]
    pub fn datteo(&mut self) -> _DATTEOW {
        _DATTEOW { w: self }
    }
    #[doc = "Bit 5 - Data CRC Error Signal Enable"]
    #[inline]
    pub fn datcrc(&mut self) -> _DATCRCW {
        _DATCRCW { w: self }
    }
    #[doc = "Bit 6 - Data End Bit Error Signal Enable"]
    #[inline]
    pub fn datend(&mut self) -> _DATENDW {
        _DATENDW { w: self }
    }
    #[doc = "Bit 7 - Current Limit Error Signal Enable"]
    #[inline]
    pub fn curlim(&mut self) -> _CURLIMW {
        _CURLIMW { w: self }
    }
    #[doc = "Bit 8 - Auto CMD Error Signal Enable"]
    #[inline]
    pub fn acmd(&mut self) -> _ACMDW {
        _ACMDW { w: self }
    }
    #[doc = "Bit 9 - ADMA Error Signal Enable"]
    #[inline]
    pub fn adma(&mut self) -> _ADMAW {
        _ADMAW { w: self }
    }
}
