#[doc = r" Value read from the register"]
pub struct R {
    bits: u16,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u16,
}
impl super::TMR {
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
#[doc = "Possible values of the field `DMAEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMAENR {
    #[doc = "No data transfer or Non DMA data transfer"]
    DISABLE,
    #[doc = "DMA data transfer"]
    ENABLE,
}
impl DMAENR {
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
            DMAENR::DISABLE => false,
            DMAENR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DMAENR {
        match value {
            false => DMAENR::DISABLE,
            true => DMAENR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == DMAENR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == DMAENR::ENABLE
    }
}
#[doc = "Possible values of the field `BCEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BCENR {
    #[doc = "Disable"]
    DISABLE,
    #[doc = "Enable"]
    ENABLE,
}
impl BCENR {
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
            BCENR::DISABLE => false,
            BCENR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BCENR {
        match value {
            false => BCENR::DISABLE,
            true => BCENR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == BCENR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == BCENR::ENABLE
    }
}
#[doc = "Possible values of the field `ACMDEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACMDENR {
    #[doc = "Auto Command Disabled"]
    DISABLED,
    #[doc = "Auto CMD12 Enable"]
    CMD12,
    #[doc = "Auto CMD23 Enable"]
    CMD23,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl ACMDENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ACMDENR::DISABLED => 0,
            ACMDENR::CMD12 => 1,
            ACMDENR::CMD23 => 2,
            ACMDENR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ACMDENR {
        match value {
            0 => ACMDENR::DISABLED,
            1 => ACMDENR::CMD12,
            2 => ACMDENR::CMD23,
            i => ACMDENR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == ACMDENR::DISABLED
    }
    #[doc = "Checks if the value of the field is `CMD12`"]
    #[inline]
    pub fn is_cmd12(&self) -> bool {
        *self == ACMDENR::CMD12
    }
    #[doc = "Checks if the value of the field is `CMD23`"]
    #[inline]
    pub fn is_cmd23(&self) -> bool {
        *self == ACMDENR::CMD23
    }
}
#[doc = "Possible values of the field `DTDSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DTDSELR {
    #[doc = "Write (Host to Card)"]
    WRITE,
    #[doc = "Read (Card to Host)"]
    READ,
}
impl DTDSELR {
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
            DTDSELR::WRITE => false,
            DTDSELR::READ => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DTDSELR {
        match value {
            false => DTDSELR::WRITE,
            true => DTDSELR::READ,
        }
    }
    #[doc = "Checks if the value of the field is `WRITE`"]
    #[inline]
    pub fn is_write(&self) -> bool {
        *self == DTDSELR::WRITE
    }
    #[doc = "Checks if the value of the field is `READ`"]
    #[inline]
    pub fn is_read(&self) -> bool {
        *self == DTDSELR::READ
    }
}
#[doc = "Possible values of the field `MSBSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSBSELR {
    #[doc = "Single Block"]
    SINGLE,
    #[doc = "Multiple Block"]
    MULTIPLE,
}
impl MSBSELR {
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
            MSBSELR::SINGLE => false,
            MSBSELR::MULTIPLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MSBSELR {
        match value {
            false => MSBSELR::SINGLE,
            true => MSBSELR::MULTIPLE,
        }
    }
    #[doc = "Checks if the value of the field is `SINGLE`"]
    #[inline]
    pub fn is_single(&self) -> bool {
        *self == MSBSELR::SINGLE
    }
    #[doc = "Checks if the value of the field is `MULTIPLE`"]
    #[inline]
    pub fn is_multiple(&self) -> bool {
        *self == MSBSELR::MULTIPLE
    }
}
#[doc = "Values that can be written to the field `DMAEN`"]
pub enum DMAENW {
    #[doc = "No data transfer or Non DMA data transfer"]
    DISABLE,
    #[doc = "DMA data transfer"]
    ENABLE,
}
impl DMAENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DMAENW::DISABLE => false,
            DMAENW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DMAENW<'a> {
    w: &'a mut W,
}
impl<'a> _DMAENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DMAENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No data transfer or Non DMA data transfer"]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(DMAENW::DISABLE)
    }
    #[doc = "DMA data transfer"]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(DMAENW::ENABLE)
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
#[doc = "Values that can be written to the field `BCEN`"]
pub enum BCENW {
    #[doc = "Disable"]
    DISABLE,
    #[doc = "Enable"]
    ENABLE,
}
impl BCENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BCENW::DISABLE => false,
            BCENW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BCENW<'a> {
    w: &'a mut W,
}
impl<'a> _BCENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BCENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable"]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(BCENW::DISABLE)
    }
    #[doc = "Enable"]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(BCENW::ENABLE)
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
#[doc = "Values that can be written to the field `ACMDEN`"]
pub enum ACMDENW {
    #[doc = "Auto Command Disabled"]
    DISABLED,
    #[doc = "Auto CMD12 Enable"]
    CMD12,
    #[doc = "Auto CMD23 Enable"]
    CMD23,
}
impl ACMDENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            ACMDENW::DISABLED => 0,
            ACMDENW::CMD12 => 1,
            ACMDENW::CMD23 => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ACMDENW<'a> {
    w: &'a mut W,
}
impl<'a> _ACMDENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ACMDENW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Auto Command Disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ACMDENW::DISABLED)
    }
    #[doc = "Auto CMD12 Enable"]
    #[inline]
    pub fn cmd12(self) -> &'a mut W {
        self.variant(ACMDENW::CMD12)
    }
    #[doc = "Auto CMD23 Enable"]
    #[inline]
    pub fn cmd23(self) -> &'a mut W {
        self.variant(ACMDENW::CMD23)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DTDSEL`"]
pub enum DTDSELW {
    #[doc = "Write (Host to Card)"]
    WRITE,
    #[doc = "Read (Card to Host)"]
    READ,
}
impl DTDSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DTDSELW::WRITE => false,
            DTDSELW::READ => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DTDSELW<'a> {
    w: &'a mut W,
}
impl<'a> _DTDSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DTDSELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Write (Host to Card)"]
    #[inline]
    pub fn write(self) -> &'a mut W {
        self.variant(DTDSELW::WRITE)
    }
    #[doc = "Read (Card to Host)"]
    #[inline]
    pub fn read(self) -> &'a mut W {
        self.variant(DTDSELW::READ)
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
#[doc = "Values that can be written to the field `MSBSEL`"]
pub enum MSBSELW {
    #[doc = "Single Block"]
    SINGLE,
    #[doc = "Multiple Block"]
    MULTIPLE,
}
impl MSBSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MSBSELW::SINGLE => false,
            MSBSELW::MULTIPLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MSBSELW<'a> {
    w: &'a mut W,
}
impl<'a> _MSBSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MSBSELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Single Block"]
    #[inline]
    pub fn single(self) -> &'a mut W {
        self.variant(MSBSELW::SINGLE)
    }
    #[doc = "Multiple Block"]
    #[inline]
    pub fn multiple(self) -> &'a mut W {
        self.variant(MSBSELW::MULTIPLE)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
    #[doc = "Bit 0 - DMA Enable"]
    #[inline]
    pub fn dmaen(&self) -> DMAENR {
        DMAENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 1 - Block Count Enable"]
    #[inline]
    pub fn bcen(&self) -> BCENR {
        BCENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bits 2:3 - Auto Command Enable"]
    #[inline]
    pub fn acmden(&self) -> ACMDENR {
        ACMDENR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u16) as u8
        })
    }
    #[doc = "Bit 4 - Data Transfer Direction Selection"]
    #[inline]
    pub fn dtdsel(&self) -> DTDSELR {
        DTDSELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 5 - Multi/Single Block Selection"]
    #[inline]
    pub fn msbsel(&self) -> MSBSELR {
        MSBSELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
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
    #[doc = "Bit 0 - DMA Enable"]
    #[inline]
    pub fn dmaen(&mut self) -> _DMAENW {
        _DMAENW { w: self }
    }
    #[doc = "Bit 1 - Block Count Enable"]
    #[inline]
    pub fn bcen(&mut self) -> _BCENW {
        _BCENW { w: self }
    }
    #[doc = "Bits 2:3 - Auto Command Enable"]
    #[inline]
    pub fn acmden(&mut self) -> _ACMDENW {
        _ACMDENW { w: self }
    }
    #[doc = "Bit 4 - Data Transfer Direction Selection"]
    #[inline]
    pub fn dtdsel(&mut self) -> _DTDSELW {
        _DTDSELW { w: self }
    }
    #[doc = "Bit 5 - Multi/Single Block Selection"]
    #[inline]
    pub fn msbsel(&mut self) -> _MSBSELW {
        _MSBSELW { w: self }
    }
}
