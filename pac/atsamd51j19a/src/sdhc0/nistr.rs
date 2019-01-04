#[doc = r" Value read from the register"]
pub struct R {
    bits: u16,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u16,
}
impl super::NISTR {
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
#[doc = "Possible values of the field `CMDC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMDCR {
    #[doc = "No command complete"]
    NO,
    #[doc = "Command complete"]
    YES,
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
            CMDCR::NO => false,
            CMDCR::YES => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CMDCR {
        match value {
            false => CMDCR::NO,
            true => CMDCR::YES,
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline]
    pub fn is_no(&self) -> bool {
        *self == CMDCR::NO
    }
    #[doc = "Checks if the value of the field is `YES`"]
    #[inline]
    pub fn is_yes(&self) -> bool {
        *self == CMDCR::YES
    }
}
#[doc = "Possible values of the field `TRFC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRFCR {
    #[doc = "Not complete"]
    NO,
    #[doc = "Command execution is completed"]
    YES,
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
            TRFCR::NO => false,
            TRFCR::YES => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TRFCR {
        match value {
            false => TRFCR::NO,
            true => TRFCR::YES,
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline]
    pub fn is_no(&self) -> bool {
        *self == TRFCR::NO
    }
    #[doc = "Checks if the value of the field is `YES`"]
    #[inline]
    pub fn is_yes(&self) -> bool {
        *self == TRFCR::YES
    }
}
#[doc = "Possible values of the field `BLKGE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BLKGER {
    #[doc = "No Block Gap Event"]
    NO,
    #[doc = "Transaction stopped at block gap"]
    STOP,
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
            BLKGER::NO => false,
            BLKGER::STOP => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BLKGER {
        match value {
            false => BLKGER::NO,
            true => BLKGER::STOP,
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline]
    pub fn is_no(&self) -> bool {
        *self == BLKGER::NO
    }
    #[doc = "Checks if the value of the field is `STOP`"]
    #[inline]
    pub fn is_stop(&self) -> bool {
        *self == BLKGER::STOP
    }
}
#[doc = "Possible values of the field `DMAINT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMAINTR {
    #[doc = "No DMA Interrupt"]
    NO,
    #[doc = "DMA Interrupt is generated"]
    YES,
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
            DMAINTR::NO => false,
            DMAINTR::YES => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DMAINTR {
        match value {
            false => DMAINTR::NO,
            true => DMAINTR::YES,
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline]
    pub fn is_no(&self) -> bool {
        *self == DMAINTR::NO
    }
    #[doc = "Checks if the value of the field is `YES`"]
    #[inline]
    pub fn is_yes(&self) -> bool {
        *self == DMAINTR::YES
    }
}
#[doc = "Possible values of the field `BWRRDY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BWRRDYR {
    #[doc = "Not ready to write buffer"]
    NO,
    #[doc = "Ready to write buffer"]
    YES,
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
            BWRRDYR::NO => false,
            BWRRDYR::YES => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BWRRDYR {
        match value {
            false => BWRRDYR::NO,
            true => BWRRDYR::YES,
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline]
    pub fn is_no(&self) -> bool {
        *self == BWRRDYR::NO
    }
    #[doc = "Checks if the value of the field is `YES`"]
    #[inline]
    pub fn is_yes(&self) -> bool {
        *self == BWRRDYR::YES
    }
}
#[doc = "Possible values of the field `BRDRDY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BRDRDYR {
    #[doc = "Not ready to read buffer"]
    NO,
    #[doc = "Ready to read buffer"]
    YES,
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
            BRDRDYR::NO => false,
            BRDRDYR::YES => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BRDRDYR {
        match value {
            false => BRDRDYR::NO,
            true => BRDRDYR::YES,
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline]
    pub fn is_no(&self) -> bool {
        *self == BRDRDYR::NO
    }
    #[doc = "Checks if the value of the field is `YES`"]
    #[inline]
    pub fn is_yes(&self) -> bool {
        *self == BRDRDYR::YES
    }
}
#[doc = "Possible values of the field `CINS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CINSR {
    #[doc = "Card state stable or Debouncing"]
    NO,
    #[doc = "Card inserted"]
    YES,
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
            CINSR::NO => false,
            CINSR::YES => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CINSR {
        match value {
            false => CINSR::NO,
            true => CINSR::YES,
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline]
    pub fn is_no(&self) -> bool {
        *self == CINSR::NO
    }
    #[doc = "Checks if the value of the field is `YES`"]
    #[inline]
    pub fn is_yes(&self) -> bool {
        *self == CINSR::YES
    }
}
#[doc = "Possible values of the field `CREM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CREMR {
    #[doc = "Card state stable or Debouncing"]
    NO,
    #[doc = "Card Removed"]
    YES,
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
            CREMR::NO => false,
            CREMR::YES => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CREMR {
        match value {
            false => CREMR::NO,
            true => CREMR::YES,
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline]
    pub fn is_no(&self) -> bool {
        *self == CREMR::NO
    }
    #[doc = "Checks if the value of the field is `YES`"]
    #[inline]
    pub fn is_yes(&self) -> bool {
        *self == CREMR::YES
    }
}
#[doc = "Possible values of the field `CINT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CINTR {
    #[doc = "No Card Interrupt"]
    NO,
    #[doc = "Generate Card Interrupt"]
    YES,
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
            CINTR::NO => false,
            CINTR::YES => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CINTR {
        match value {
            false => CINTR::NO,
            true => CINTR::YES,
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline]
    pub fn is_no(&self) -> bool {
        *self == CINTR::NO
    }
    #[doc = "Checks if the value of the field is `YES`"]
    #[inline]
    pub fn is_yes(&self) -> bool {
        *self == CINTR::YES
    }
}
#[doc = "Possible values of the field `ERRINT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERRINTR {
    #[doc = "No Error"]
    NO,
    #[doc = "Error"]
    YES,
}
impl ERRINTR {
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
            ERRINTR::NO => false,
            ERRINTR::YES => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ERRINTR {
        match value {
            false => ERRINTR::NO,
            true => ERRINTR::YES,
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline]
    pub fn is_no(&self) -> bool {
        *self == ERRINTR::NO
    }
    #[doc = "Checks if the value of the field is `YES`"]
    #[inline]
    pub fn is_yes(&self) -> bool {
        *self == ERRINTR::YES
    }
}
#[doc = "Values that can be written to the field `CMDC`"]
pub enum CMDCW {
    #[doc = "No command complete"]
    NO,
    #[doc = "Command complete"]
    YES,
}
impl CMDCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CMDCW::NO => false,
            CMDCW::YES => true,
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
    #[doc = "No command complete"]
    #[inline]
    pub fn no(self) -> &'a mut W {
        self.variant(CMDCW::NO)
    }
    #[doc = "Command complete"]
    #[inline]
    pub fn yes(self) -> &'a mut W {
        self.variant(CMDCW::YES)
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
#[doc = "Values that can be written to the field `TRFC`"]
pub enum TRFCW {
    #[doc = "Not complete"]
    NO,
    #[doc = "Command execution is completed"]
    YES,
}
impl TRFCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TRFCW::NO => false,
            TRFCW::YES => true,
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
    #[doc = "Not complete"]
    #[inline]
    pub fn no(self) -> &'a mut W {
        self.variant(TRFCW::NO)
    }
    #[doc = "Command execution is completed"]
    #[inline]
    pub fn yes(self) -> &'a mut W {
        self.variant(TRFCW::YES)
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
#[doc = "Values that can be written to the field `BLKGE`"]
pub enum BLKGEW {
    #[doc = "No Block Gap Event"]
    NO,
    #[doc = "Transaction stopped at block gap"]
    STOP,
}
impl BLKGEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BLKGEW::NO => false,
            BLKGEW::STOP => true,
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
    #[doc = "No Block Gap Event"]
    #[inline]
    pub fn no(self) -> &'a mut W {
        self.variant(BLKGEW::NO)
    }
    #[doc = "Transaction stopped at block gap"]
    #[inline]
    pub fn stop(self) -> &'a mut W {
        self.variant(BLKGEW::STOP)
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
#[doc = "Values that can be written to the field `DMAINT`"]
pub enum DMAINTW {
    #[doc = "No DMA Interrupt"]
    NO,
    #[doc = "DMA Interrupt is generated"]
    YES,
}
impl DMAINTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DMAINTW::NO => false,
            DMAINTW::YES => true,
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
    #[doc = "No DMA Interrupt"]
    #[inline]
    pub fn no(self) -> &'a mut W {
        self.variant(DMAINTW::NO)
    }
    #[doc = "DMA Interrupt is generated"]
    #[inline]
    pub fn yes(self) -> &'a mut W {
        self.variant(DMAINTW::YES)
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
#[doc = "Values that can be written to the field `BWRRDY`"]
pub enum BWRRDYW {
    #[doc = "Not ready to write buffer"]
    NO,
    #[doc = "Ready to write buffer"]
    YES,
}
impl BWRRDYW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BWRRDYW::NO => false,
            BWRRDYW::YES => true,
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
    #[doc = "Not ready to write buffer"]
    #[inline]
    pub fn no(self) -> &'a mut W {
        self.variant(BWRRDYW::NO)
    }
    #[doc = "Ready to write buffer"]
    #[inline]
    pub fn yes(self) -> &'a mut W {
        self.variant(BWRRDYW::YES)
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
#[doc = "Values that can be written to the field `BRDRDY`"]
pub enum BRDRDYW {
    #[doc = "Not ready to read buffer"]
    NO,
    #[doc = "Ready to read buffer"]
    YES,
}
impl BRDRDYW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BRDRDYW::NO => false,
            BRDRDYW::YES => true,
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
    #[doc = "Not ready to read buffer"]
    #[inline]
    pub fn no(self) -> &'a mut W {
        self.variant(BRDRDYW::NO)
    }
    #[doc = "Ready to read buffer"]
    #[inline]
    pub fn yes(self) -> &'a mut W {
        self.variant(BRDRDYW::YES)
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
#[doc = "Values that can be written to the field `CINS`"]
pub enum CINSW {
    #[doc = "Card state stable or Debouncing"]
    NO,
    #[doc = "Card inserted"]
    YES,
}
impl CINSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CINSW::NO => false,
            CINSW::YES => true,
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
    #[doc = "Card state stable or Debouncing"]
    #[inline]
    pub fn no(self) -> &'a mut W {
        self.variant(CINSW::NO)
    }
    #[doc = "Card inserted"]
    #[inline]
    pub fn yes(self) -> &'a mut W {
        self.variant(CINSW::YES)
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
#[doc = "Values that can be written to the field `CREM`"]
pub enum CREMW {
    #[doc = "Card state stable or Debouncing"]
    NO,
    #[doc = "Card Removed"]
    YES,
}
impl CREMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CREMW::NO => false,
            CREMW::YES => true,
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
    #[doc = "Card state stable or Debouncing"]
    #[inline]
    pub fn no(self) -> &'a mut W {
        self.variant(CREMW::NO)
    }
    #[doc = "Card Removed"]
    #[inline]
    pub fn yes(self) -> &'a mut W {
        self.variant(CREMW::YES)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
    #[doc = "Bit 0 - Command Complete"]
    #[inline]
    pub fn cmdc(&self) -> CMDCR {
        CMDCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 1 - Transfer Complete"]
    #[inline]
    pub fn trfc(&self) -> TRFCR {
        TRFCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 2 - Block Gap Event"]
    #[inline]
    pub fn blkge(&self) -> BLKGER {
        BLKGER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 3 - DMA Interrupt"]
    #[inline]
    pub fn dmaint(&self) -> DMAINTR {
        DMAINTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 4 - Buffer Write Ready"]
    #[inline]
    pub fn bwrrdy(&self) -> BWRRDYR {
        BWRRDYR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 5 - Buffer Read Ready"]
    #[inline]
    pub fn brdrdy(&self) -> BRDRDYR {
        BRDRDYR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 6 - Card Insertion"]
    #[inline]
    pub fn cins(&self) -> CINSR {
        CINSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 7 - Card Removal"]
    #[inline]
    pub fn crem(&self) -> CREMR {
        CREMR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 8 - Card Interrupt"]
    #[inline]
    pub fn cint(&self) -> CINTR {
        CINTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 15 - Error Interrupt"]
    #[inline]
    pub fn errint(&self) -> ERRINTR {
        ERRINTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
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
    #[doc = "Bit 0 - Command Complete"]
    #[inline]
    pub fn cmdc(&mut self) -> _CMDCW {
        _CMDCW { w: self }
    }
    #[doc = "Bit 1 - Transfer Complete"]
    #[inline]
    pub fn trfc(&mut self) -> _TRFCW {
        _TRFCW { w: self }
    }
    #[doc = "Bit 2 - Block Gap Event"]
    #[inline]
    pub fn blkge(&mut self) -> _BLKGEW {
        _BLKGEW { w: self }
    }
    #[doc = "Bit 3 - DMA Interrupt"]
    #[inline]
    pub fn dmaint(&mut self) -> _DMAINTW {
        _DMAINTW { w: self }
    }
    #[doc = "Bit 4 - Buffer Write Ready"]
    #[inline]
    pub fn bwrrdy(&mut self) -> _BWRRDYW {
        _BWRRDYW { w: self }
    }
    #[doc = "Bit 5 - Buffer Read Ready"]
    #[inline]
    pub fn brdrdy(&mut self) -> _BRDRDYW {
        _BRDRDYW { w: self }
    }
    #[doc = "Bit 6 - Card Insertion"]
    #[inline]
    pub fn cins(&mut self) -> _CINSW {
        _CINSW { w: self }
    }
    #[doc = "Bit 7 - Card Removal"]
    #[inline]
    pub fn crem(&mut self) -> _CREMW {
        _CREMW { w: self }
    }
}
