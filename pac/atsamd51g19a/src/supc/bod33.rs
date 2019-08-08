#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::BOD33 {
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
#[doc = r" Value of the field"]
pub struct ENABLER {
    bits: bool,
}
impl ENABLER {
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
#[doc = "Possible values of the field `ACTION`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACTIONR {
    #[doc = "No action"]
    NONE,
    #[doc = "The BOD33 generates a reset"]
    RESET,
    #[doc = "The BOD33 generates an interrupt"]
    INT,
    #[doc = "The BOD33 puts the device in backup sleep mode"]
    BKUP,
}
impl ACTIONR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ACTIONR::NONE => 0,
            ACTIONR::RESET => 1,
            ACTIONR::INT => 2,
            ACTIONR::BKUP => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ACTIONR {
        match value {
            0 => ACTIONR::NONE,
            1 => ACTIONR::RESET,
            2 => ACTIONR::INT,
            3 => ACTIONR::BKUP,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline]
    pub fn is_none(&self) -> bool {
        *self == ACTIONR::NONE
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline]
    pub fn is_reset(&self) -> bool {
        *self == ACTIONR::RESET
    }
    #[doc = "Checks if the value of the field is `INT`"]
    #[inline]
    pub fn is_int(&self) -> bool {
        *self == ACTIONR::INT
    }
    #[doc = "Checks if the value of the field is `BKUP`"]
    #[inline]
    pub fn is_bkup(&self) -> bool {
        *self == ACTIONR::BKUP
    }
}
#[doc = r" Value of the field"]
pub struct STDBYCFGR {
    bits: bool,
}
impl STDBYCFGR {
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
pub struct RUNSTDBYR {
    bits: bool,
}
impl RUNSTDBYR {
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
pub struct RUNHIBR {
    bits: bool,
}
impl RUNHIBR {
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
pub struct RUNBKUPR {
    bits: bool,
}
impl RUNBKUPR {
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
pub struct HYSTR {
    bits: u8,
}
impl HYSTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `PSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PSELR {
    #[doc = "Not divided"]
    NODIV,
    #[doc = "Divide clock by 4"]
    DIV4,
    #[doc = "Divide clock by 8"]
    DIV8,
    #[doc = "Divide clock by 16"]
    DIV16,
    #[doc = "Divide clock by 32"]
    DIV32,
    #[doc = "Divide clock by 64"]
    DIV64,
    #[doc = "Divide clock by 128"]
    DIV128,
    #[doc = "Divide clock by 256"]
    DIV256,
}
impl PSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PSELR::NODIV => 0,
            PSELR::DIV4 => 1,
            PSELR::DIV8 => 2,
            PSELR::DIV16 => 3,
            PSELR::DIV32 => 4,
            PSELR::DIV64 => 5,
            PSELR::DIV128 => 6,
            PSELR::DIV256 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PSELR {
        match value {
            0 => PSELR::NODIV,
            1 => PSELR::DIV4,
            2 => PSELR::DIV8,
            3 => PSELR::DIV16,
            4 => PSELR::DIV32,
            5 => PSELR::DIV64,
            6 => PSELR::DIV128,
            7 => PSELR::DIV256,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NODIV`"]
    #[inline]
    pub fn is_nodiv(&self) -> bool {
        *self == PSELR::NODIV
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline]
    pub fn is_div4(&self) -> bool {
        *self == PSELR::DIV4
    }
    #[doc = "Checks if the value of the field is `DIV8`"]
    #[inline]
    pub fn is_div8(&self) -> bool {
        *self == PSELR::DIV8
    }
    #[doc = "Checks if the value of the field is `DIV16`"]
    #[inline]
    pub fn is_div16(&self) -> bool {
        *self == PSELR::DIV16
    }
    #[doc = "Checks if the value of the field is `DIV32`"]
    #[inline]
    pub fn is_div32(&self) -> bool {
        *self == PSELR::DIV32
    }
    #[doc = "Checks if the value of the field is `DIV64`"]
    #[inline]
    pub fn is_div64(&self) -> bool {
        *self == PSELR::DIV64
    }
    #[doc = "Checks if the value of the field is `DIV128`"]
    #[inline]
    pub fn is_div128(&self) -> bool {
        *self == PSELR::DIV128
    }
    #[doc = "Checks if the value of the field is `DIV256`"]
    #[inline]
    pub fn is_div256(&self) -> bool {
        *self == PSELR::DIV256
    }
}
#[doc = r" Value of the field"]
pub struct LEVELR {
    bits: u8,
}
impl LEVELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct VBATLEVELR {
    bits: u8,
}
impl VBATLEVELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _ENABLEW<'a> {
    w: &'a mut W,
}
impl<'a> _ENABLEW<'a> {
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
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ACTION`"]
pub enum ACTIONW {
    #[doc = "No action"]
    NONE,
    #[doc = "The BOD33 generates a reset"]
    RESET,
    #[doc = "The BOD33 generates an interrupt"]
    INT,
    #[doc = "The BOD33 puts the device in backup sleep mode"]
    BKUP,
}
impl ACTIONW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            ACTIONW::NONE => 0,
            ACTIONW::RESET => 1,
            ACTIONW::INT => 2,
            ACTIONW::BKUP => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ACTIONW<'a> {
    w: &'a mut W,
}
impl<'a> _ACTIONW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ACTIONW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "No action"]
    #[inline]
    pub fn none(self) -> &'a mut W {
        self.variant(ACTIONW::NONE)
    }
    #[doc = "The BOD33 generates a reset"]
    #[inline]
    pub fn reset(self) -> &'a mut W {
        self.variant(ACTIONW::RESET)
    }
    #[doc = "The BOD33 generates an interrupt"]
    #[inline]
    pub fn int(self) -> &'a mut W {
        self.variant(ACTIONW::INT)
    }
    #[doc = "The BOD33 puts the device in backup sleep mode"]
    #[inline]
    pub fn bkup(self) -> &'a mut W {
        self.variant(ACTIONW::BKUP)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _STDBYCFGW<'a> {
    w: &'a mut W,
}
impl<'a> _STDBYCFGW<'a> {
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
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RUNSTDBYW<'a> {
    w: &'a mut W,
}
impl<'a> _RUNSTDBYW<'a> {
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
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RUNHIBW<'a> {
    w: &'a mut W,
}
impl<'a> _RUNHIBW<'a> {
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
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RUNBKUPW<'a> {
    w: &'a mut W,
}
impl<'a> _RUNBKUPW<'a> {
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
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _HYSTW<'a> {
    w: &'a mut W,
}
impl<'a> _HYSTW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PSEL`"]
pub enum PSELW {
    #[doc = "Not divided"]
    NODIV,
    #[doc = "Divide clock by 4"]
    DIV4,
    #[doc = "Divide clock by 8"]
    DIV8,
    #[doc = "Divide clock by 16"]
    DIV16,
    #[doc = "Divide clock by 32"]
    DIV32,
    #[doc = "Divide clock by 64"]
    DIV64,
    #[doc = "Divide clock by 128"]
    DIV128,
    #[doc = "Divide clock by 256"]
    DIV256,
}
impl PSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PSELW::NODIV => 0,
            PSELW::DIV4 => 1,
            PSELW::DIV8 => 2,
            PSELW::DIV16 => 3,
            PSELW::DIV32 => 4,
            PSELW::DIV64 => 5,
            PSELW::DIV128 => 6,
            PSELW::DIV256 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PSELW<'a> {
    w: &'a mut W,
}
impl<'a> _PSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PSELW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Not divided"]
    #[inline]
    pub fn nodiv(self) -> &'a mut W {
        self.variant(PSELW::NODIV)
    }
    #[doc = "Divide clock by 4"]
    #[inline]
    pub fn div4(self) -> &'a mut W {
        self.variant(PSELW::DIV4)
    }
    #[doc = "Divide clock by 8"]
    #[inline]
    pub fn div8(self) -> &'a mut W {
        self.variant(PSELW::DIV8)
    }
    #[doc = "Divide clock by 16"]
    #[inline]
    pub fn div16(self) -> &'a mut W {
        self.variant(PSELW::DIV16)
    }
    #[doc = "Divide clock by 32"]
    #[inline]
    pub fn div32(self) -> &'a mut W {
        self.variant(PSELW::DIV32)
    }
    #[doc = "Divide clock by 64"]
    #[inline]
    pub fn div64(self) -> &'a mut W {
        self.variant(PSELW::DIV64)
    }
    #[doc = "Divide clock by 128"]
    #[inline]
    pub fn div128(self) -> &'a mut W {
        self.variant(PSELW::DIV128)
    }
    #[doc = "Divide clock by 256"]
    #[inline]
    pub fn div256(self) -> &'a mut W {
        self.variant(PSELW::DIV256)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _LEVELW<'a> {
    w: &'a mut W,
}
impl<'a> _LEVELW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _VBATLEVELW<'a> {
    w: &'a mut W,
}
impl<'a> _VBATLEVELW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 1 - Enable"]
    #[inline]
    pub fn enable(&self) -> ENABLER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ENABLER { bits }
    }
    #[doc = "Bits 2:3 - Action when Threshold Crossed"]
    #[inline]
    pub fn action(&self) -> ACTIONR {
        ACTIONR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 4 - Configuration in Standby mode"]
    #[inline]
    pub fn stdbycfg(&self) -> STDBYCFGR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        STDBYCFGR { bits }
    }
    #[doc = "Bit 5 - Run in Standby mode"]
    #[inline]
    pub fn runstdby(&self) -> RUNSTDBYR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RUNSTDBYR { bits }
    }
    #[doc = "Bit 6 - Run in Hibernate mode"]
    #[inline]
    pub fn runhib(&self) -> RUNHIBR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RUNHIBR { bits }
    }
    #[doc = "Bit 7 - Run in Backup mode"]
    #[inline]
    pub fn runbkup(&self) -> RUNBKUPR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RUNBKUPR { bits }
    }
    #[doc = "Bits 8:11 - Hysteresis value"]
    #[inline]
    pub fn hyst(&self) -> HYSTR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        HYSTR { bits }
    }
    #[doc = "Bits 12:14 - Prescaler Select"]
    #[inline]
    pub fn psel(&self) -> PSELR {
        PSELR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:23 - Threshold Level for VDD"]
    #[inline]
    pub fn level(&self) -> LEVELR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        LEVELR { bits }
    }
    #[doc = "Bits 24:31 - Threshold Level in battery backup sleep mode for VBAT"]
    #[inline]
    pub fn vbatlevel(&self) -> VBATLEVELR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        VBATLEVELR { bits }
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
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 1 - Enable"]
    #[inline]
    pub fn enable(&mut self) -> _ENABLEW {
        _ENABLEW { w: self }
    }
    #[doc = "Bits 2:3 - Action when Threshold Crossed"]
    #[inline]
    pub fn action(&mut self) -> _ACTIONW {
        _ACTIONW { w: self }
    }
    #[doc = "Bit 4 - Configuration in Standby mode"]
    #[inline]
    pub fn stdbycfg(&mut self) -> _STDBYCFGW {
        _STDBYCFGW { w: self }
    }
    #[doc = "Bit 5 - Run in Standby mode"]
    #[inline]
    pub fn runstdby(&mut self) -> _RUNSTDBYW {
        _RUNSTDBYW { w: self }
    }
    #[doc = "Bit 6 - Run in Hibernate mode"]
    #[inline]
    pub fn runhib(&mut self) -> _RUNHIBW {
        _RUNHIBW { w: self }
    }
    #[doc = "Bit 7 - Run in Backup mode"]
    #[inline]
    pub fn runbkup(&mut self) -> _RUNBKUPW {
        _RUNBKUPW { w: self }
    }
    #[doc = "Bits 8:11 - Hysteresis value"]
    #[inline]
    pub fn hyst(&mut self) -> _HYSTW {
        _HYSTW { w: self }
    }
    #[doc = "Bits 12:14 - Prescaler Select"]
    #[inline]
    pub fn psel(&mut self) -> _PSELW {
        _PSELW { w: self }
    }
    #[doc = "Bits 16:23 - Threshold Level for VDD"]
    #[inline]
    pub fn level(&mut self) -> _LEVELW {
        _LEVELW { w: self }
    }
    #[doc = "Bits 24:31 - Threshold Level in battery backup sleep mode for VBAT"]
    #[inline]
    pub fn vbatlevel(&mut self) -> _VBATLEVELW {
        _VBATLEVELW { w: self }
    }
}
