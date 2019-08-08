#[doc = r" Value read from the register"]
pub struct R {
    bits: u16,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u16,
}
impl super::EISTR_EMMC {
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
    #[doc = "No Error"]
    NO,
    #[doc = "Timeout"]
    YES,
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
            CMDTEOR::NO => false,
            CMDTEOR::YES => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CMDTEOR {
        match value {
            false => CMDTEOR::NO,
            true => CMDTEOR::YES,
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline]
    pub fn is_no(&self) -> bool {
        *self == CMDTEOR::NO
    }
    #[doc = "Checks if the value of the field is `YES`"]
    #[inline]
    pub fn is_yes(&self) -> bool {
        *self == CMDTEOR::YES
    }
}
#[doc = "Possible values of the field `CMDCRC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMDCRCR {
    #[doc = "No Error"]
    NO,
    #[doc = "CRC Error Generated"]
    YES,
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
            CMDCRCR::NO => false,
            CMDCRCR::YES => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CMDCRCR {
        match value {
            false => CMDCRCR::NO,
            true => CMDCRCR::YES,
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline]
    pub fn is_no(&self) -> bool {
        *self == CMDCRCR::NO
    }
    #[doc = "Checks if the value of the field is `YES`"]
    #[inline]
    pub fn is_yes(&self) -> bool {
        *self == CMDCRCR::YES
    }
}
#[doc = "Possible values of the field `CMDEND`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMDENDR {
    #[doc = "No error"]
    NO,
    #[doc = "End Bit Error Generated"]
    YES,
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
            CMDENDR::NO => false,
            CMDENDR::YES => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CMDENDR {
        match value {
            false => CMDENDR::NO,
            true => CMDENDR::YES,
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline]
    pub fn is_no(&self) -> bool {
        *self == CMDENDR::NO
    }
    #[doc = "Checks if the value of the field is `YES`"]
    #[inline]
    pub fn is_yes(&self) -> bool {
        *self == CMDENDR::YES
    }
}
#[doc = "Possible values of the field `CMDIDX`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMDIDXR {
    #[doc = "No Error"]
    NO,
    #[doc = "Error"]
    YES,
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
            CMDIDXR::NO => false,
            CMDIDXR::YES => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CMDIDXR {
        match value {
            false => CMDIDXR::NO,
            true => CMDIDXR::YES,
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline]
    pub fn is_no(&self) -> bool {
        *self == CMDIDXR::NO
    }
    #[doc = "Checks if the value of the field is `YES`"]
    #[inline]
    pub fn is_yes(&self) -> bool {
        *self == CMDIDXR::YES
    }
}
#[doc = "Possible values of the field `DATTEO`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DATTEOR {
    #[doc = "No Error"]
    NO,
    #[doc = "Timeout"]
    YES,
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
            DATTEOR::NO => false,
            DATTEOR::YES => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DATTEOR {
        match value {
            false => DATTEOR::NO,
            true => DATTEOR::YES,
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline]
    pub fn is_no(&self) -> bool {
        *self == DATTEOR::NO
    }
    #[doc = "Checks if the value of the field is `YES`"]
    #[inline]
    pub fn is_yes(&self) -> bool {
        *self == DATTEOR::YES
    }
}
#[doc = "Possible values of the field `DATCRC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DATCRCR {
    #[doc = "No Error"]
    NO,
    #[doc = "Error"]
    YES,
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
            DATCRCR::NO => false,
            DATCRCR::YES => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DATCRCR {
        match value {
            false => DATCRCR::NO,
            true => DATCRCR::YES,
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline]
    pub fn is_no(&self) -> bool {
        *self == DATCRCR::NO
    }
    #[doc = "Checks if the value of the field is `YES`"]
    #[inline]
    pub fn is_yes(&self) -> bool {
        *self == DATCRCR::YES
    }
}
#[doc = "Possible values of the field `DATEND`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DATENDR {
    #[doc = "No Error"]
    NO,
    #[doc = "Error"]
    YES,
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
            DATENDR::NO => false,
            DATENDR::YES => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DATENDR {
        match value {
            false => DATENDR::NO,
            true => DATENDR::YES,
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline]
    pub fn is_no(&self) -> bool {
        *self == DATENDR::NO
    }
    #[doc = "Checks if the value of the field is `YES`"]
    #[inline]
    pub fn is_yes(&self) -> bool {
        *self == DATENDR::YES
    }
}
#[doc = "Possible values of the field `CURLIM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CURLIMR {
    #[doc = "No Error"]
    NO,
    #[doc = "Power Fail"]
    YES,
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
            CURLIMR::NO => false,
            CURLIMR::YES => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CURLIMR {
        match value {
            false => CURLIMR::NO,
            true => CURLIMR::YES,
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline]
    pub fn is_no(&self) -> bool {
        *self == CURLIMR::NO
    }
    #[doc = "Checks if the value of the field is `YES`"]
    #[inline]
    pub fn is_yes(&self) -> bool {
        *self == CURLIMR::YES
    }
}
#[doc = "Possible values of the field `ACMD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACMDR {
    #[doc = "No Error"]
    NO,
    #[doc = "Error"]
    YES,
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
            ACMDR::NO => false,
            ACMDR::YES => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ACMDR {
        match value {
            false => ACMDR::NO,
            true => ACMDR::YES,
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline]
    pub fn is_no(&self) -> bool {
        *self == ACMDR::NO
    }
    #[doc = "Checks if the value of the field is `YES`"]
    #[inline]
    pub fn is_yes(&self) -> bool {
        *self == ACMDR::YES
    }
}
#[doc = "Possible values of the field `ADMA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADMAR {
    #[doc = "No Error"]
    NO,
    #[doc = "Error"]
    YES,
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
            ADMAR::NO => false,
            ADMAR::YES => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ADMAR {
        match value {
            false => ADMAR::NO,
            true => ADMAR::YES,
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline]
    pub fn is_no(&self) -> bool {
        *self == ADMAR::NO
    }
    #[doc = "Checks if the value of the field is `YES`"]
    #[inline]
    pub fn is_yes(&self) -> bool {
        *self == ADMAR::YES
    }
}
#[doc = "Possible values of the field `BOOTAE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BOOTAER {
    #[doc = "FIFO contains at least one byte"]
    _0,
    #[doc = "FIFO is empty"]
    _1,
}
impl BOOTAER {
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
            BOOTAER::_0 => false,
            BOOTAER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BOOTAER {
        match value {
            false => BOOTAER::_0,
            true => BOOTAER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == BOOTAER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == BOOTAER::_1
    }
}
#[doc = "Values that can be written to the field `CMDTEO`"]
pub enum CMDTEOW {
    #[doc = "No Error"]
    NO,
    #[doc = "Timeout"]
    YES,
}
impl CMDTEOW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CMDTEOW::NO => false,
            CMDTEOW::YES => true,
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
    #[doc = "No Error"]
    #[inline]
    pub fn no(self) -> &'a mut W {
        self.variant(CMDTEOW::NO)
    }
    #[doc = "Timeout"]
    #[inline]
    pub fn yes(self) -> &'a mut W {
        self.variant(CMDTEOW::YES)
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
    #[doc = "No Error"]
    NO,
    #[doc = "CRC Error Generated"]
    YES,
}
impl CMDCRCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CMDCRCW::NO => false,
            CMDCRCW::YES => true,
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
    #[doc = "No Error"]
    #[inline]
    pub fn no(self) -> &'a mut W {
        self.variant(CMDCRCW::NO)
    }
    #[doc = "CRC Error Generated"]
    #[inline]
    pub fn yes(self) -> &'a mut W {
        self.variant(CMDCRCW::YES)
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
    #[doc = "No error"]
    NO,
    #[doc = "End Bit Error Generated"]
    YES,
}
impl CMDENDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CMDENDW::NO => false,
            CMDENDW::YES => true,
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
    #[doc = "No error"]
    #[inline]
    pub fn no(self) -> &'a mut W {
        self.variant(CMDENDW::NO)
    }
    #[doc = "End Bit Error Generated"]
    #[inline]
    pub fn yes(self) -> &'a mut W {
        self.variant(CMDENDW::YES)
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
    #[doc = "No Error"]
    NO,
    #[doc = "Error"]
    YES,
}
impl CMDIDXW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CMDIDXW::NO => false,
            CMDIDXW::YES => true,
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
    #[doc = "No Error"]
    #[inline]
    pub fn no(self) -> &'a mut W {
        self.variant(CMDIDXW::NO)
    }
    #[doc = "Error"]
    #[inline]
    pub fn yes(self) -> &'a mut W {
        self.variant(CMDIDXW::YES)
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
    #[doc = "No Error"]
    NO,
    #[doc = "Timeout"]
    YES,
}
impl DATTEOW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DATTEOW::NO => false,
            DATTEOW::YES => true,
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
    #[doc = "No Error"]
    #[inline]
    pub fn no(self) -> &'a mut W {
        self.variant(DATTEOW::NO)
    }
    #[doc = "Timeout"]
    #[inline]
    pub fn yes(self) -> &'a mut W {
        self.variant(DATTEOW::YES)
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
    #[doc = "No Error"]
    NO,
    #[doc = "Error"]
    YES,
}
impl DATCRCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DATCRCW::NO => false,
            DATCRCW::YES => true,
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
    #[doc = "No Error"]
    #[inline]
    pub fn no(self) -> &'a mut W {
        self.variant(DATCRCW::NO)
    }
    #[doc = "Error"]
    #[inline]
    pub fn yes(self) -> &'a mut W {
        self.variant(DATCRCW::YES)
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
    #[doc = "No Error"]
    NO,
    #[doc = "Error"]
    YES,
}
impl DATENDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DATENDW::NO => false,
            DATENDW::YES => true,
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
    #[doc = "No Error"]
    #[inline]
    pub fn no(self) -> &'a mut W {
        self.variant(DATENDW::NO)
    }
    #[doc = "Error"]
    #[inline]
    pub fn yes(self) -> &'a mut W {
        self.variant(DATENDW::YES)
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
    #[doc = "No Error"]
    NO,
    #[doc = "Power Fail"]
    YES,
}
impl CURLIMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CURLIMW::NO => false,
            CURLIMW::YES => true,
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
    #[doc = "No Error"]
    #[inline]
    pub fn no(self) -> &'a mut W {
        self.variant(CURLIMW::NO)
    }
    #[doc = "Power Fail"]
    #[inline]
    pub fn yes(self) -> &'a mut W {
        self.variant(CURLIMW::YES)
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
    #[doc = "No Error"]
    NO,
    #[doc = "Error"]
    YES,
}
impl ACMDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ACMDW::NO => false,
            ACMDW::YES => true,
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
    #[doc = "No Error"]
    #[inline]
    pub fn no(self) -> &'a mut W {
        self.variant(ACMDW::NO)
    }
    #[doc = "Error"]
    #[inline]
    pub fn yes(self) -> &'a mut W {
        self.variant(ACMDW::YES)
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
    #[doc = "No Error"]
    NO,
    #[doc = "Error"]
    YES,
}
impl ADMAW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ADMAW::NO => false,
            ADMAW::YES => true,
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
    #[doc = "No Error"]
    #[inline]
    pub fn no(self) -> &'a mut W {
        self.variant(ADMAW::NO)
    }
    #[doc = "Error"]
    #[inline]
    pub fn yes(self) -> &'a mut W {
        self.variant(ADMAW::YES)
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
#[doc = "Values that can be written to the field `BOOTAE`"]
pub enum BOOTAEW {
    #[doc = "FIFO contains at least one byte"]
    _0,
    #[doc = "FIFO is empty"]
    _1,
}
impl BOOTAEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BOOTAEW::_0 => false,
            BOOTAEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BOOTAEW<'a> {
    w: &'a mut W,
}
impl<'a> _BOOTAEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BOOTAEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "FIFO contains at least one byte"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(BOOTAEW::_0)
    }
    #[doc = "FIFO is empty"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(BOOTAEW::_1)
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
        const OFFSET: u8 = 12;
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
    #[doc = "Bit 0 - Command Timeout Error"]
    #[inline]
    pub fn cmdteo(&self) -> CMDTEOR {
        CMDTEOR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 1 - Command CRC Error"]
    #[inline]
    pub fn cmdcrc(&self) -> CMDCRCR {
        CMDCRCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 2 - Command End Bit Error"]
    #[inline]
    pub fn cmdend(&self) -> CMDENDR {
        CMDENDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 3 - Command Index Error"]
    #[inline]
    pub fn cmdidx(&self) -> CMDIDXR {
        CMDIDXR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 4 - Data Timeout Error"]
    #[inline]
    pub fn datteo(&self) -> DATTEOR {
        DATTEOR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 5 - Data CRC Error"]
    #[inline]
    pub fn datcrc(&self) -> DATCRCR {
        DATCRCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 6 - Data End Bit Error"]
    #[inline]
    pub fn datend(&self) -> DATENDR {
        DATENDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 7 - Current Limit Error"]
    #[inline]
    pub fn curlim(&self) -> CURLIMR {
        CURLIMR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 8 - Auto CMD Error"]
    #[inline]
    pub fn acmd(&self) -> ACMDR {
        ACMDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 9 - ADMA Error"]
    #[inline]
    pub fn adma(&self) -> ADMAR {
        ADMAR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 12 - Boot Acknowledge Error"]
    #[inline]
    pub fn bootae(&self) -> BOOTAER {
        BOOTAER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
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
    #[doc = "Bit 0 - Command Timeout Error"]
    #[inline]
    pub fn cmdteo(&mut self) -> _CMDTEOW {
        _CMDTEOW { w: self }
    }
    #[doc = "Bit 1 - Command CRC Error"]
    #[inline]
    pub fn cmdcrc(&mut self) -> _CMDCRCW {
        _CMDCRCW { w: self }
    }
    #[doc = "Bit 2 - Command End Bit Error"]
    #[inline]
    pub fn cmdend(&mut self) -> _CMDENDW {
        _CMDENDW { w: self }
    }
    #[doc = "Bit 3 - Command Index Error"]
    #[inline]
    pub fn cmdidx(&mut self) -> _CMDIDXW {
        _CMDIDXW { w: self }
    }
    #[doc = "Bit 4 - Data Timeout Error"]
    #[inline]
    pub fn datteo(&mut self) -> _DATTEOW {
        _DATTEOW { w: self }
    }
    #[doc = "Bit 5 - Data CRC Error"]
    #[inline]
    pub fn datcrc(&mut self) -> _DATCRCW {
        _DATCRCW { w: self }
    }
    #[doc = "Bit 6 - Data End Bit Error"]
    #[inline]
    pub fn datend(&mut self) -> _DATENDW {
        _DATENDW { w: self }
    }
    #[doc = "Bit 7 - Current Limit Error"]
    #[inline]
    pub fn curlim(&mut self) -> _CURLIMW {
        _CURLIMW { w: self }
    }
    #[doc = "Bit 8 - Auto CMD Error"]
    #[inline]
    pub fn acmd(&mut self) -> _ACMDW {
        _ACMDW { w: self }
    }
    #[doc = "Bit 9 - ADMA Error"]
    #[inline]
    pub fn adma(&mut self) -> _ADMAW {
        _ADMAW { w: self }
    }
    #[doc = "Bit 12 - Boot Acknowledge Error"]
    #[inline]
    pub fn bootae(&mut self) -> _BOOTAEW {
        _BOOTAEW { w: self }
    }
}
