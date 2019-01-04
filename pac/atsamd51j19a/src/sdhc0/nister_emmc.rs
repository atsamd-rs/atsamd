#[doc = r" Value read from the register"]
pub struct R {
    bits: u16,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u16,
}
impl super::NISTER_EMMC {
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
#[doc = r" Value of the field"]
pub struct BOOTARR {
    bits: bool,
}
impl BOOTARR {
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
#[doc = "Values that can be written to the field `CMDC`"]
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
        const MASK: bool = true;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TRFC`"]
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
        const MASK: bool = true;
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `BLKGE`"]
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
        const MASK: bool = true;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DMAINT`"]
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
        const MASK: bool = true;
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `BWRRDY`"]
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
        const MASK: bool = true;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `BRDRDY`"]
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
        const MASK: bool = true;
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _BOOTARW<'a> {
    w: &'a mut W,
}
impl<'a> _BOOTARW<'a> {
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
        const OFFSET: u8 = 14;
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
    #[doc = "Bit 0 - Command Complete Status Enable"]
    #[inline]
    pub fn cmdc(&self) -> CMDCR {
        CMDCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 1 - Transfer Complete Status Enable"]
    #[inline]
    pub fn trfc(&self) -> TRFCR {
        TRFCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 2 - Block Gap Event Status Enable"]
    #[inline]
    pub fn blkge(&self) -> BLKGER {
        BLKGER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 3 - DMA Interrupt Status Enable"]
    #[inline]
    pub fn dmaint(&self) -> DMAINTR {
        DMAINTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 4 - Buffer Write Ready Status Enable"]
    #[inline]
    pub fn bwrrdy(&self) -> BWRRDYR {
        BWRRDYR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 5 - Buffer Read Ready Status Enable"]
    #[inline]
    pub fn brdrdy(&self) -> BRDRDYR {
        BRDRDYR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 14 - Boot Acknowledge Received Status Enable"]
    #[inline]
    pub fn bootar(&self) -> BOOTARR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        };
        BOOTARR { bits }
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
    #[doc = "Bit 14 - Boot Acknowledge Received Status Enable"]
    #[inline]
    pub fn bootar(&mut self) -> _BOOTARW {
        _BOOTARW { w: self }
    }
}
