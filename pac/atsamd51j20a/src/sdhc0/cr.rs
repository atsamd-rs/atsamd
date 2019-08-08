#[doc = r" Value read from the register"]
pub struct R {
    bits: u16,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u16,
}
impl super::CR {
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
#[doc = "Possible values of the field `RESPTYP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RESPTYPR {
    #[doc = "No response"]
    NONE,
    #[doc = "136-bit response"]
    _136_BIT,
    #[doc = "48-bit response"]
    _48_BIT,
    #[doc = "48-bit response check busy after response"]
    _48_BIT_BUSY,
}
impl RESPTYPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            RESPTYPR::NONE => 0,
            RESPTYPR::_136_BIT => 1,
            RESPTYPR::_48_BIT => 2,
            RESPTYPR::_48_BIT_BUSY => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> RESPTYPR {
        match value {
            0 => RESPTYPR::NONE,
            1 => RESPTYPR::_136_BIT,
            2 => RESPTYPR::_48_BIT,
            3 => RESPTYPR::_48_BIT_BUSY,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline]
    pub fn is_none(&self) -> bool {
        *self == RESPTYPR::NONE
    }
    #[doc = "Checks if the value of the field is `_136_BIT`"]
    #[inline]
    pub fn is_136_bit(&self) -> bool {
        *self == RESPTYPR::_136_BIT
    }
    #[doc = "Checks if the value of the field is `_48_BIT`"]
    #[inline]
    pub fn is_48_bit(&self) -> bool {
        *self == RESPTYPR::_48_BIT
    }
    #[doc = "Checks if the value of the field is `_48_BIT_BUSY`"]
    #[inline]
    pub fn is_48_bit_busy(&self) -> bool {
        *self == RESPTYPR::_48_BIT_BUSY
    }
}
#[doc = "Possible values of the field `CMDCCEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMDCCENR {
    #[doc = "Disable"]
    DISABLE,
    #[doc = "Enable"]
    ENABLE,
}
impl CMDCCENR {
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
            CMDCCENR::DISABLE => false,
            CMDCCENR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CMDCCENR {
        match value {
            false => CMDCCENR::DISABLE,
            true => CMDCCENR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == CMDCCENR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == CMDCCENR::ENABLE
    }
}
#[doc = "Possible values of the field `CMDICEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMDICENR {
    #[doc = "Disable"]
    DISABLE,
    #[doc = "Enable"]
    ENABLE,
}
impl CMDICENR {
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
            CMDICENR::DISABLE => false,
            CMDICENR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CMDICENR {
        match value {
            false => CMDICENR::DISABLE,
            true => CMDICENR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == CMDICENR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == CMDICENR::ENABLE
    }
}
#[doc = "Possible values of the field `DPSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DPSELR {
    #[doc = "No Data Present"]
    NO_DATA,
    #[doc = "Data Present"]
    DATA,
}
impl DPSELR {
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
            DPSELR::NO_DATA => false,
            DPSELR::DATA => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DPSELR {
        match value {
            false => DPSELR::NO_DATA,
            true => DPSELR::DATA,
        }
    }
    #[doc = "Checks if the value of the field is `NO_DATA`"]
    #[inline]
    pub fn is_no_data(&self) -> bool {
        *self == DPSELR::NO_DATA
    }
    #[doc = "Checks if the value of the field is `DATA`"]
    #[inline]
    pub fn is_data(&self) -> bool {
        *self == DPSELR::DATA
    }
}
#[doc = "Possible values of the field `CMDTYP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMDTYPR {
    #[doc = "Other commands"]
    NORMAL,
    #[doc = "CMD52 for writing Bus Suspend in CCCR"]
    SUSPEND,
    #[doc = "CMD52 for writing Function Select in CCCR"]
    RESUME,
    #[doc = "CMD12, CMD52 for writing I/O Abort in CCCR"]
    ABORT,
}
impl CMDTYPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CMDTYPR::NORMAL => 0,
            CMDTYPR::SUSPEND => 1,
            CMDTYPR::RESUME => 2,
            CMDTYPR::ABORT => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CMDTYPR {
        match value {
            0 => CMDTYPR::NORMAL,
            1 => CMDTYPR::SUSPEND,
            2 => CMDTYPR::RESUME,
            3 => CMDTYPR::ABORT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline]
    pub fn is_normal(&self) -> bool {
        *self == CMDTYPR::NORMAL
    }
    #[doc = "Checks if the value of the field is `SUSPEND`"]
    #[inline]
    pub fn is_suspend(&self) -> bool {
        *self == CMDTYPR::SUSPEND
    }
    #[doc = "Checks if the value of the field is `RESUME`"]
    #[inline]
    pub fn is_resume(&self) -> bool {
        *self == CMDTYPR::RESUME
    }
    #[doc = "Checks if the value of the field is `ABORT`"]
    #[inline]
    pub fn is_abort(&self) -> bool {
        *self == CMDTYPR::ABORT
    }
}
#[doc = r" Value of the field"]
pub struct CMDIDXR {
    bits: u8,
}
impl CMDIDXR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Values that can be written to the field `RESPTYP`"]
pub enum RESPTYPW {
    #[doc = "No response"]
    NONE,
    #[doc = "136-bit response"]
    _136_BIT,
    #[doc = "48-bit response"]
    _48_BIT,
    #[doc = "48-bit response check busy after response"]
    _48_BIT_BUSY,
}
impl RESPTYPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            RESPTYPW::NONE => 0,
            RESPTYPW::_136_BIT => 1,
            RESPTYPW::_48_BIT => 2,
            RESPTYPW::_48_BIT_BUSY => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RESPTYPW<'a> {
    w: &'a mut W,
}
impl<'a> _RESPTYPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RESPTYPW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "No response"]
    #[inline]
    pub fn none(self) -> &'a mut W {
        self.variant(RESPTYPW::NONE)
    }
    #[doc = "136-bit response"]
    #[inline]
    pub fn _136_bit(self) -> &'a mut W {
        self.variant(RESPTYPW::_136_BIT)
    }
    #[doc = "48-bit response"]
    #[inline]
    pub fn _48_bit(self) -> &'a mut W {
        self.variant(RESPTYPW::_48_BIT)
    }
    #[doc = "48-bit response check busy after response"]
    #[inline]
    pub fn _48_bit_busy(self) -> &'a mut W {
        self.variant(RESPTYPW::_48_BIT_BUSY)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CMDCCEN`"]
pub enum CMDCCENW {
    #[doc = "Disable"]
    DISABLE,
    #[doc = "Enable"]
    ENABLE,
}
impl CMDCCENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CMDCCENW::DISABLE => false,
            CMDCCENW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CMDCCENW<'a> {
    w: &'a mut W,
}
impl<'a> _CMDCCENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CMDCCENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable"]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(CMDCCENW::DISABLE)
    }
    #[doc = "Enable"]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(CMDCCENW::ENABLE)
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
#[doc = "Values that can be written to the field `CMDICEN`"]
pub enum CMDICENW {
    #[doc = "Disable"]
    DISABLE,
    #[doc = "Enable"]
    ENABLE,
}
impl CMDICENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CMDICENW::DISABLE => false,
            CMDICENW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CMDICENW<'a> {
    w: &'a mut W,
}
impl<'a> _CMDICENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CMDICENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable"]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(CMDICENW::DISABLE)
    }
    #[doc = "Enable"]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(CMDICENW::ENABLE)
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
#[doc = "Values that can be written to the field `DPSEL`"]
pub enum DPSELW {
    #[doc = "No Data Present"]
    NO_DATA,
    #[doc = "Data Present"]
    DATA,
}
impl DPSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DPSELW::NO_DATA => false,
            DPSELW::DATA => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DPSELW<'a> {
    w: &'a mut W,
}
impl<'a> _DPSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DPSELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No Data Present"]
    #[inline]
    pub fn no_data(self) -> &'a mut W {
        self.variant(DPSELW::NO_DATA)
    }
    #[doc = "Data Present"]
    #[inline]
    pub fn data(self) -> &'a mut W {
        self.variant(DPSELW::DATA)
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
#[doc = "Values that can be written to the field `CMDTYP`"]
pub enum CMDTYPW {
    #[doc = "Other commands"]
    NORMAL,
    #[doc = "CMD52 for writing Bus Suspend in CCCR"]
    SUSPEND,
    #[doc = "CMD52 for writing Function Select in CCCR"]
    RESUME,
    #[doc = "CMD12, CMD52 for writing I/O Abort in CCCR"]
    ABORT,
}
impl CMDTYPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CMDTYPW::NORMAL => 0,
            CMDTYPW::SUSPEND => 1,
            CMDTYPW::RESUME => 2,
            CMDTYPW::ABORT => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CMDTYPW<'a> {
    w: &'a mut W,
}
impl<'a> _CMDTYPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CMDTYPW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Other commands"]
    #[inline]
    pub fn normal(self) -> &'a mut W {
        self.variant(CMDTYPW::NORMAL)
    }
    #[doc = "CMD52 for writing Bus Suspend in CCCR"]
    #[inline]
    pub fn suspend(self) -> &'a mut W {
        self.variant(CMDTYPW::SUSPEND)
    }
    #[doc = "CMD52 for writing Function Select in CCCR"]
    #[inline]
    pub fn resume(self) -> &'a mut W {
        self.variant(CMDTYPW::RESUME)
    }
    #[doc = "CMD12, CMD52 for writing I/O Abort in CCCR"]
    #[inline]
    pub fn abort(self) -> &'a mut W {
        self.variant(CMDTYPW::ABORT)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CMDIDXW<'a> {
    w: &'a mut W,
}
impl<'a> _CMDIDXW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 63;
        const OFFSET: u8 = 8;
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
    #[doc = "Bits 0:1 - Response Type"]
    #[inline]
    pub fn resptyp(&self) -> RESPTYPR {
        RESPTYPR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u16) as u8
        })
    }
    #[doc = "Bit 3 - Command CRC Check Enable"]
    #[inline]
    pub fn cmdccen(&self) -> CMDCCENR {
        CMDCCENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 4 - Command Index Check Enable"]
    #[inline]
    pub fn cmdicen(&self) -> CMDICENR {
        CMDICENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 5 - Data Present Select"]
    #[inline]
    pub fn dpsel(&self) -> DPSELR {
        DPSELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bits 6:7 - Command Type"]
    #[inline]
    pub fn cmdtyp(&self) -> CMDTYPR {
        CMDTYPR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u16) as u8
        })
    }
    #[doc = "Bits 8:13 - Command Index"]
    #[inline]
    pub fn cmdidx(&self) -> CMDIDXR {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u16) as u8
        };
        CMDIDXR { bits }
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
    #[doc = "Bits 0:1 - Response Type"]
    #[inline]
    pub fn resptyp(&mut self) -> _RESPTYPW {
        _RESPTYPW { w: self }
    }
    #[doc = "Bit 3 - Command CRC Check Enable"]
    #[inline]
    pub fn cmdccen(&mut self) -> _CMDCCENW {
        _CMDCCENW { w: self }
    }
    #[doc = "Bit 4 - Command Index Check Enable"]
    #[inline]
    pub fn cmdicen(&mut self) -> _CMDICENW {
        _CMDICENW { w: self }
    }
    #[doc = "Bit 5 - Data Present Select"]
    #[inline]
    pub fn dpsel(&mut self) -> _DPSELW {
        _DPSELW { w: self }
    }
    #[doc = "Bits 6:7 - Command Type"]
    #[inline]
    pub fn cmdtyp(&mut self) -> _CMDTYPW {
        _CMDTYPW { w: self }
    }
    #[doc = "Bits 8:13 - Command Index"]
    #[inline]
    pub fn cmdidx(&mut self) -> _CMDIDXW {
        _CMDIDXW { w: self }
    }
}
