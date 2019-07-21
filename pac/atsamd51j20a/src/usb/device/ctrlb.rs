#[doc = r" Value read from the register"]
pub struct R {
    bits: u16,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u16,
}
impl super::CTRLB {
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
        0x01
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.register.set(Self::reset_value())
    }
}
#[doc = r" Value of the field"]
pub struct DETACHR {
    bits: bool,
}
impl DETACHR {
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
pub struct UPRSMR {
    bits: bool,
}
impl UPRSMR {
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
#[doc = "Possible values of the field `SPDCONF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPDCONFR {
    #[doc = "FS : Full Speed"]
    FS,
    #[doc = "LS : Low Speed"]
    LS,
    #[doc = "HS : High Speed capable"]
    HS,
    #[doc = "HSTM: High Speed Test Mode (force high-speed mode for test mode)"]
    HSTM,
}
impl SPDCONFR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SPDCONFR::FS => 0,
            SPDCONFR::LS => 0x01,
            SPDCONFR::HS => 0x02,
            SPDCONFR::HSTM => 0x03,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SPDCONFR {
        match value {
            0 => SPDCONFR::FS,
            1 => SPDCONFR::LS,
            2 => SPDCONFR::HS,
            3 => SPDCONFR::HSTM,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `FS`"]
    #[inline]
    pub fn is_fs(&self) -> bool {
        *self == SPDCONFR::FS
    }
    #[doc = "Checks if the value of the field is `LS`"]
    #[inline]
    pub fn is_ls(&self) -> bool {
        *self == SPDCONFR::LS
    }
    #[doc = "Checks if the value of the field is `HS`"]
    #[inline]
    pub fn is_hs(&self) -> bool {
        *self == SPDCONFR::HS
    }
    #[doc = "Checks if the value of the field is `HSTM`"]
    #[inline]
    pub fn is_hstm(&self) -> bool {
        *self == SPDCONFR::HSTM
    }
}
#[doc = r" Value of the field"]
pub struct NREPLYR {
    bits: bool,
}
impl NREPLYR {
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
pub struct TSTJR {
    bits: bool,
}
impl TSTJR {
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
pub struct TSTKR {
    bits: bool,
}
impl TSTKR {
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
pub struct TSTPCKTR {
    bits: bool,
}
impl TSTPCKTR {
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
pub struct OPMODE2R {
    bits: bool,
}
impl OPMODE2R {
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
pub struct GNAKR {
    bits: bool,
}
impl GNAKR {
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
#[doc = "Possible values of the field `LPMHDSK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPMHDSKR {
    #[doc = "No handshake. LPM is not supported"]
    NO,
    #[doc = "ACK"]
    ACK,
    #[doc = "NYET"]
    NYET,
    #[doc = "STALL"]
    STALL,
}
impl LPMHDSKR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            LPMHDSKR::NO => 0,
            LPMHDSKR::ACK => 0x01,
            LPMHDSKR::NYET => 0x02,
            LPMHDSKR::STALL => 0x03,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> LPMHDSKR {
        match value {
            0 => LPMHDSKR::NO,
            1 => LPMHDSKR::ACK,
            2 => LPMHDSKR::NYET,
            3 => LPMHDSKR::STALL,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline]
    pub fn is_no(&self) -> bool {
        *self == LPMHDSKR::NO
    }
    #[doc = "Checks if the value of the field is `ACK`"]
    #[inline]
    pub fn is_ack(&self) -> bool {
        *self == LPMHDSKR::ACK
    }
    #[doc = "Checks if the value of the field is `NYET`"]
    #[inline]
    pub fn is_nyet(&self) -> bool {
        *self == LPMHDSKR::NYET
    }
    #[doc = "Checks if the value of the field is `STALL`"]
    #[inline]
    pub fn is_stall(&self) -> bool {
        *self == LPMHDSKR::STALL
    }
}
#[doc = r" Proxy"]
pub struct _DETACHW<'a> {
    w: &'a mut W,
}
impl<'a> _DETACHW<'a> {
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
#[doc = r" Proxy"]
pub struct _UPRSMW<'a> {
    w: &'a mut W,
}
impl<'a> _UPRSMW<'a> {
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
#[doc = "Values that can be written to the field `SPDCONF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPDCONFW {
    #[doc = "FS : Full Speed"]
    FS,
    #[doc = "LS : Low Speed"]
    LS,
    #[doc = "HS : High Speed capable"]
    HS,
    #[doc = "HSTM: High Speed Test Mode (force high-speed mode for test mode)"]
    HSTM,
}
impl SPDCONFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SPDCONFW::FS => 0,
            SPDCONFW::LS => 1,
            SPDCONFW::HS => 2,
            SPDCONFW::HSTM => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SPDCONFW<'a> {
    w: &'a mut W,
}
impl<'a> _SPDCONFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SPDCONFW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "FS : Full Speed"]
    #[inline]
    pub fn fs(self) -> &'a mut W {
        self.variant(SPDCONFW::FS)
    }
    #[doc = "LS : Low Speed"]
    #[inline]
    pub fn ls(self) -> &'a mut W {
        self.variant(SPDCONFW::LS)
    }
    #[doc = "HS : High Speed capable"]
    #[inline]
    pub fn hs(self) -> &'a mut W {
        self.variant(SPDCONFW::HS)
    }
    #[doc = "HSTM: High Speed Test Mode (force high-speed mode for test mode)"]
    #[inline]
    pub fn hstm(self) -> &'a mut W {
        self.variant(SPDCONFW::HSTM)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(0x03 << 2);
        self.w.bits |= ((value as u16) & 0x03) << 2;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _NREPLYW<'a> {
    w: &'a mut W,
}
impl<'a> _NREPLYW<'a> {
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
#[doc = r" Proxy"]
pub struct _TSTJW<'a> {
    w: &'a mut W,
}
impl<'a> _TSTJW<'a> {
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
#[doc = r" Proxy"]
pub struct _TSTKW<'a> {
    w: &'a mut W,
}
impl<'a> _TSTKW<'a> {
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
#[doc = r" Proxy"]
pub struct _TSTPCKTW<'a> {
    w: &'a mut W,
}
impl<'a> _TSTPCKTW<'a> {
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
#[doc = r" Proxy"]
pub struct _OPMODE2W<'a> {
    w: &'a mut W,
}
impl<'a> _OPMODE2W<'a> {
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
#[doc = r" Proxy"]
pub struct _GNAKW<'a> {
    w: &'a mut W,
}
impl<'a> _GNAKW<'a> {
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
        self.w.bits |= ((value as u16) & 0x01) << 9;
        self.w
    }
}
#[doc = "Values that can be written to the field `LPMHDSK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPMHDSKW {
    #[doc = "No handshake. LPM is not supported"]
    NO,
    #[doc = "ACK"]
    ACK,
    #[doc = "NYET"]
    NYET,
    #[doc = "STALL"]
    STALL,
}
impl LPMHDSKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            LPMHDSKW::NO => 0,
            LPMHDSKW::ACK => 1,
            LPMHDSKW::NYET => 2,
            LPMHDSKW::STALL => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LPMHDSKW<'a> {
    w: &'a mut W,
}
impl<'a> _LPMHDSKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LPMHDSKW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "No handshake. LPM is not supported"]
    #[inline]
    pub fn no(self) -> &'a mut W {
        self.variant(LPMHDSKW::NO)
    }
    #[doc = "ACK"]
    #[inline]
    pub fn ack(self) -> &'a mut W {
        self.variant(LPMHDSKW::ACK)
    }
    #[doc = "NYET"]
    #[inline]
    pub fn nyet(self) -> &'a mut W {
        self.variant(LPMHDSKW::NYET)
    }
    #[doc = "STALL"]
    #[inline]
    pub fn stall(self) -> &'a mut W {
        self.variant(LPMHDSKW::STALL)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(0x03 << 10);
        self.w.bits |= ((value as u16) & 0x03) << 10;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
    #[doc = "Bit 0 - Detach"]
    #[inline]
    pub fn detach(&self) -> DETACHR {
        let bits = ((self.bits >> 0) & 0x01) != 0;
        DETACHR { bits }
    }
    #[doc = "Bit 1 - Upstream Resume"]
    #[inline]
    pub fn uprsm(&self) -> UPRSMR {
        let bits = ((self.bits >> 1) & 0x01) != 0;
        UPRSMR { bits }
    }
    #[doc = "Bits 2:3 - Speed Configuration"]
    #[inline]
    pub fn spdconf(&self) -> SPDCONFR {
        SPDCONFR::_from(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bit 4 - No Reply"]
    #[inline]
    pub fn nreply(&self) -> NREPLYR {
        let bits = ((self.bits >> 4) & 0x01) != 0;
        NREPLYR { bits }
    }
    #[doc = "Bit 5 - Test mode J"]
    #[inline]
    pub fn tstj(&self) -> TSTJR {
        let bits = ((self.bits >> 5) & 0x01) != 0;
        TSTJR { bits }
    }
    #[doc = "Bit 6 - Test mode K"]
    #[inline]
    pub fn tstk(&self) -> TSTKR {
        let bits = ((self.bits >> 6) & 0x01) != 0;
        TSTKR { bits }
    }
    #[doc = "Bit 7 - Test packet mode"]
    #[inline]
    pub fn tstpckt(&self) -> TSTPCKTR {
        let bits = ((self.bits >> 7) & 0x01) != 0;
        TSTPCKTR { bits }
    }
    #[doc = "Bit 8 - Specific Operational Mode"]
    #[inline]
    pub fn opmode2(&self) -> OPMODE2R {
        let bits = ((self.bits >> 8) & 0x01) != 0;
        OPMODE2R { bits }
    }
    #[doc = "Bit 9 - Global NAK"]
    #[inline]
    pub fn gnak(&self) -> GNAKR {
        let bits = ((self.bits >> 9) & 0x01) != 0;
        GNAKR { bits }
    }
    #[doc = "Bits 10:11 - Link Power Management Handshake"]
    #[inline]
    pub fn lpmhdsk(&self) -> LPMHDSKR {
        LPMHDSKR::_from(((self.bits >> 10) & 0x03) as u8)
    }
}
impl W {
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Detach"]
    #[inline]
    pub fn detach(&mut self) -> _DETACHW {
        _DETACHW { w: self }
    }
    #[doc = "Bit 1 - Upstream Resume"]
    #[inline]
    pub fn uprsm(&mut self) -> _UPRSMW {
        _UPRSMW { w: self }
    }
    #[doc = "Bits 2:3 - Speed Configuration"]
    #[inline]
    pub fn spdconf(&mut self) -> _SPDCONFW {
        _SPDCONFW { w: self }
    }
    #[doc = "Bit 4 - No Reply"]
    #[inline]
    pub fn nreply(&mut self) -> _NREPLYW {
        _NREPLYW { w: self }
    }
    #[doc = "Bit 5 - Test mode J"]
    #[inline]
    pub fn tstj(&mut self) -> _TSTJW {
        _TSTJW { w: self }
    }
    #[doc = "Bit 6 - Test mode K"]
    #[inline]
    pub fn tstk(&mut self) -> _TSTKW {
        _TSTKW { w: self }
    }
    #[doc = "Bit 7 - Test packet mode"]
    #[inline]
    pub fn tstpckt(&mut self) -> _TSTPCKTW {
        _TSTPCKTW { w: self }
    }
    #[doc = "Bit 8 - Specific Operational Mode"]
    #[inline]
    pub fn opmode2(&mut self) -> _OPMODE2W {
        _OPMODE2W { w: self }
    }
    #[doc = "Bit 9 - Global NAK"]
    #[inline]
    pub fn gnak(&mut self) -> _GNAKW {
        _GNAKW { w: self }
    }
    #[doc = "Bits 10:11 - Link Power Management Handshake"]
    #[inline]
    pub fn lpmhdsk(&mut self) -> _LPMHDSKW {
        _LPMHDSKW { w: self }
    }
}
