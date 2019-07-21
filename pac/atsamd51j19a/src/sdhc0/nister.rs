#[doc = r" Value read from the register"]
pub struct R {
    bits: u16,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u16,
}
impl super::NISTER {
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
#[doc = "Possible values of the field `CMDC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMDCR {
    #[doc = "Masked"]
    MASKED,
    #[doc = "Enabled"]
    ENABLED,
}
impl CMDCR {
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
            CMDCR::MASKED => false,
            CMDCR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CMDCR {
        match value {
            false => CMDCR::MASKED,
            true => CMDCR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `MASKED`"]
    #[inline]
    pub fn is_masked(&self) -> bool {
        *self == CMDCR::MASKED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == CMDCR::ENABLED
    }
}
#[doc = "Possible values of the field `TRFC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRFCR {
    #[doc = "Masked"]
    MASKED,
    #[doc = "Enabled"]
    ENABLED,
}
impl TRFCR {
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
            TRFCR::MASKED => false,
            TRFCR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TRFCR {
        match value {
            false => TRFCR::MASKED,
            true => TRFCR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `MASKED`"]
    #[inline]
    pub fn is_masked(&self) -> bool {
        *self == TRFCR::MASKED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == TRFCR::ENABLED
    }
}
#[doc = "Possible values of the field `BLKGE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BLKGER {
    #[doc = "Masked"]
    MASKED,
    #[doc = "Enabled"]
    ENABLED,
}
impl BLKGER {
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
            BLKGER::MASKED => false,
            BLKGER::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BLKGER {
        match value {
            false => BLKGER::MASKED,
            true => BLKGER::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `MASKED`"]
    #[inline]
    pub fn is_masked(&self) -> bool {
        *self == BLKGER::MASKED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == BLKGER::ENABLED
    }
}
#[doc = "Possible values of the field `DMAINT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMAINTR {
    #[doc = "Masked"]
    MASKED,
    #[doc = "Enabled"]
    ENABLED,
}
impl DMAINTR {
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
            DMAINTR::MASKED => false,
            DMAINTR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DMAINTR {
        match value {
            false => DMAINTR::MASKED,
            true => DMAINTR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `MASKED`"]
    #[inline]
    pub fn is_masked(&self) -> bool {
        *self == DMAINTR::MASKED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == DMAINTR::ENABLED
    }
}
#[doc = "Possible values of the field `BWRRDY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BWRRDYR {
    #[doc = "Masked"]
    MASKED,
    #[doc = "Enabled"]
    ENABLED,
}
impl BWRRDYR {
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
            BWRRDYR::MASKED => false,
            BWRRDYR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BWRRDYR {
        match value {
            false => BWRRDYR::MASKED,
            true => BWRRDYR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `MASKED`"]
    #[inline]
    pub fn is_masked(&self) -> bool {
        *self == BWRRDYR::MASKED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == BWRRDYR::ENABLED
    }
}
#[doc = "Possible values of the field `BRDRDY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BRDRDYR {
    #[doc = "Masked"]
    MASKED,
    #[doc = "Enabled"]
    ENABLED,
}
impl BRDRDYR {
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
            BRDRDYR::MASKED => false,
            BRDRDYR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BRDRDYR {
        match value {
            false => BRDRDYR::MASKED,
            true => BRDRDYR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `MASKED`"]
    #[inline]
    pub fn is_masked(&self) -> bool {
        *self == BRDRDYR::MASKED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == BRDRDYR::ENABLED
    }
}
#[doc = "Possible values of the field `CINS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CINSR {
    #[doc = "Masked"]
    MASKED,
    #[doc = "Enabled"]
    ENABLED,
}
impl CINSR {
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
            CINSR::MASKED => false,
            CINSR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CINSR {
        match value {
            false => CINSR::MASKED,
            true => CINSR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `MASKED`"]
    #[inline]
    pub fn is_masked(&self) -> bool {
        *self == CINSR::MASKED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == CINSR::ENABLED
    }
}
#[doc = "Possible values of the field `CREM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CREMR {
    #[doc = "Masked"]
    MASKED,
    #[doc = "Enabled"]
    ENABLED,
}
impl CREMR {
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
            CREMR::MASKED => false,
            CREMR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CREMR {
        match value {
            false => CREMR::MASKED,
            true => CREMR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `MASKED`"]
    #[inline]
    pub fn is_masked(&self) -> bool {
        *self == CREMR::MASKED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == CREMR::ENABLED
    }
}
#[doc = "Possible values of the field `CINT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CINTR {
    #[doc = "Masked"]
    MASKED,
    #[doc = "Enabled"]
    ENABLED,
}
impl CINTR {
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
            CINTR::MASKED => false,
            CINTR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CINTR {
        match value {
            false => CINTR::MASKED,
            true => CINTR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `MASKED`"]
    #[inline]
    pub fn is_masked(&self) -> bool {
        *self == CINTR::MASKED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == CINTR::ENABLED
    }
}
#[doc = "Values that can be written to the field `CMDC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMDCW {
    #[doc = "Masked"]
    MASKED,
    #[doc = "Enabled"]
    ENABLED,
}
impl CMDCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CMDCW::MASKED => false,
            CMDCW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CMDCW<'a> {
    w: &'a mut W,
}
impl<'a> _CMDCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CMDCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Masked"]
    #[inline]
    pub fn masked(self) -> &'a mut W {
        self.variant(CMDCW::MASKED)
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CMDCW::ENABLED)
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
        self.w.bits &= !(0x01 << 0);
        self.w.bits |= ((value as u16) & 0x01) << 0;
        self.w
    }
}
#[doc = "Values that can be written to the field `TRFC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRFCW {
    #[doc = "Masked"]
    MASKED,
    #[doc = "Enabled"]
    ENABLED,
}
impl TRFCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TRFCW::MASKED => false,
            TRFCW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TRFCW<'a> {
    w: &'a mut W,
}
impl<'a> _TRFCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TRFCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Masked"]
    #[inline]
    pub fn masked(self) -> &'a mut W {
        self.variant(TRFCW::MASKED)
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TRFCW::ENABLED)
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
        self.w.bits &= !(0x01 << 1);
        self.w.bits |= ((value as u16) & 0x01) << 1;
        self.w
    }
}
#[doc = "Values that can be written to the field `BLKGE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BLKGEW {
    #[doc = "Masked"]
    MASKED,
    #[doc = "Enabled"]
    ENABLED,
}
impl BLKGEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BLKGEW::MASKED => false,
            BLKGEW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BLKGEW<'a> {
    w: &'a mut W,
}
impl<'a> _BLKGEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BLKGEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Masked"]
    #[inline]
    pub fn masked(self) -> &'a mut W {
        self.variant(BLKGEW::MASKED)
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(BLKGEW::ENABLED)
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
        self.w.bits &= !(0x01 << 2);
        self.w.bits |= ((value as u16) & 0x01) << 2;
        self.w
    }
}
#[doc = "Values that can be written to the field `DMAINT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMAINTW {
    #[doc = "Masked"]
    MASKED,
    #[doc = "Enabled"]
    ENABLED,
}
impl DMAINTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DMAINTW::MASKED => false,
            DMAINTW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DMAINTW<'a> {
    w: &'a mut W,
}
impl<'a> _DMAINTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DMAINTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Masked"]
    #[inline]
    pub fn masked(self) -> &'a mut W {
        self.variant(DMAINTW::MASKED)
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DMAINTW::ENABLED)
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
        self.w.bits &= !(0x01 << 3);
        self.w.bits |= ((value as u16) & 0x01) << 3;
        self.w
    }
}
#[doc = "Values that can be written to the field `BWRRDY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BWRRDYW {
    #[doc = "Masked"]
    MASKED,
    #[doc = "Enabled"]
    ENABLED,
}
impl BWRRDYW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BWRRDYW::MASKED => false,
            BWRRDYW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BWRRDYW<'a> {
    w: &'a mut W,
}
impl<'a> _BWRRDYW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BWRRDYW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Masked"]
    #[inline]
    pub fn masked(self) -> &'a mut W {
        self.variant(BWRRDYW::MASKED)
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(BWRRDYW::ENABLED)
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
        self.w.bits &= !(0x01 << 4);
        self.w.bits |= ((value as u16) & 0x01) << 4;
        self.w
    }
}
#[doc = "Values that can be written to the field `BRDRDY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BRDRDYW {
    #[doc = "Masked"]
    MASKED,
    #[doc = "Enabled"]
    ENABLED,
}
impl BRDRDYW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BRDRDYW::MASKED => false,
            BRDRDYW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BRDRDYW<'a> {
    w: &'a mut W,
}
impl<'a> _BRDRDYW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BRDRDYW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Masked"]
    #[inline]
    pub fn masked(self) -> &'a mut W {
        self.variant(BRDRDYW::MASKED)
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(BRDRDYW::ENABLED)
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
        self.w.bits &= !(0x01 << 5);
        self.w.bits |= ((value as u16) & 0x01) << 5;
        self.w
    }
}
#[doc = "Values that can be written to the field `CINS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CINSW {
    #[doc = "Masked"]
    MASKED,
    #[doc = "Enabled"]
    ENABLED,
}
impl CINSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CINSW::MASKED => false,
            CINSW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CINSW<'a> {
    w: &'a mut W,
}
impl<'a> _CINSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CINSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Masked"]
    #[inline]
    pub fn masked(self) -> &'a mut W {
        self.variant(CINSW::MASKED)
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CINSW::ENABLED)
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
        self.w.bits &= !(0x01 << 6);
        self.w.bits |= ((value as u16) & 0x01) << 6;
        self.w
    }
}
#[doc = "Values that can be written to the field `CREM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CREMW {
    #[doc = "Masked"]
    MASKED,
    #[doc = "Enabled"]
    ENABLED,
}
impl CREMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CREMW::MASKED => false,
            CREMW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CREMW<'a> {
    w: &'a mut W,
}
impl<'a> _CREMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CREMW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Masked"]
    #[inline]
    pub fn masked(self) -> &'a mut W {
        self.variant(CREMW::MASKED)
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CREMW::ENABLED)
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
        self.w.bits &= !(0x01 << 7);
        self.w.bits |= ((value as u16) & 0x01) << 7;
        self.w
    }
}
#[doc = "Values that can be written to the field `CINT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CINTW {
    #[doc = "Masked"]
    MASKED,
    #[doc = "Enabled"]
    ENABLED,
}
impl CINTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CINTW::MASKED => false,
            CINTW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CINTW<'a> {
    w: &'a mut W,
}
impl<'a> _CINTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CINTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Masked"]
    #[inline]
    pub fn masked(self) -> &'a mut W {
        self.variant(CINTW::MASKED)
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CINTW::ENABLED)
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
        self.w.bits &= !(0x01 << 8);
        self.w.bits |= ((value as u16) & 0x01) << 8;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
    #[doc = "Bit 0 - Command Complete Status Enable"]
    #[inline]
    pub fn cmdc(&self) -> CMDCR {
        CMDCR::_from(((self.bits >> 0) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Transfer Complete Status Enable"]
    #[inline]
    pub fn trfc(&self) -> TRFCR {
        TRFCR::_from(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Block Gap Event Status Enable"]
    #[inline]
    pub fn blkge(&self) -> BLKGER {
        BLKGER::_from(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - DMA Interrupt Status Enable"]
    #[inline]
    pub fn dmaint(&self) -> DMAINTR {
        DMAINTR::_from(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Buffer Write Ready Status Enable"]
    #[inline]
    pub fn bwrrdy(&self) -> BWRRDYR {
        BWRRDYR::_from(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Buffer Read Ready Status Enable"]
    #[inline]
    pub fn brdrdy(&self) -> BRDRDYR {
        BRDRDYR::_from(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Card Insertion Status Enable"]
    #[inline]
    pub fn cins(&self) -> CINSR {
        CINSR::_from(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Card Removal Status Enable"]
    #[inline]
    pub fn crem(&self) -> CREMR {
        CREMR::_from(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Card Interrupt Status Enable"]
    #[inline]
    pub fn cint(&self) -> CINTR {
        CINTR::_from(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Command Complete Status Enable"]
    #[inline]
    pub fn cmdc(&mut self) -> _CMDCW {
        _CMDCW { w: self }
    }
    #[doc = "Bit 1 - Transfer Complete Status Enable"]
    #[inline]
    pub fn trfc(&mut self) -> _TRFCW {
        _TRFCW { w: self }
    }
    #[doc = "Bit 2 - Block Gap Event Status Enable"]
    #[inline]
    pub fn blkge(&mut self) -> _BLKGEW {
        _BLKGEW { w: self }
    }
    #[doc = "Bit 3 - DMA Interrupt Status Enable"]
    #[inline]
    pub fn dmaint(&mut self) -> _DMAINTW {
        _DMAINTW { w: self }
    }
    #[doc = "Bit 4 - Buffer Write Ready Status Enable"]
    #[inline]
    pub fn bwrrdy(&mut self) -> _BWRRDYW {
        _BWRRDYW { w: self }
    }
    #[doc = "Bit 5 - Buffer Read Ready Status Enable"]
    #[inline]
    pub fn brdrdy(&mut self) -> _BRDRDYW {
        _BRDRDYW { w: self }
    }
    #[doc = "Bit 6 - Card Insertion Status Enable"]
    #[inline]
    pub fn cins(&mut self) -> _CINSW {
        _CINSW { w: self }
    }
    #[doc = "Bit 7 - Card Removal Status Enable"]
    #[inline]
    pub fn crem(&mut self) -> _CREMW {
        _CREMW { w: self }
    }
    #[doc = "Bit 8 - Card Interrupt Status Enable"]
    #[inline]
    pub fn cint(&mut self) -> _CINTW {
        _CINTW { w: self }
    }
}
