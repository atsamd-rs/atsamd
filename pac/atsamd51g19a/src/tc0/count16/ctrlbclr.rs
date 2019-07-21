#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::CTRLBCLR {
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
    pub const fn reset_value() -> u8 {
        0
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.register.set(Self::reset_value())
    }
}
#[doc = r" Value of the field"]
pub struct DIRR {
    bits: bool,
}
impl DIRR {
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
pub struct LUPDR {
    bits: bool,
}
impl LUPDR {
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
pub struct ONESHOTR {
    bits: bool,
}
impl ONESHOTR {
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
#[doc = "Possible values of the field `CMD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMDR {
    #[doc = "No action"]
    NONE,
    #[doc = "Force a start, restart or retrigger"]
    RETRIGGER,
    #[doc = "Force a stop"]
    STOP,
    #[doc = "Force update of double-buffered register"]
    UPDATE,
    #[doc = "Force a read synchronization of COUNT"]
    READSYNC,
    #[doc = "One-shot DMA trigger"]
    DMAOS,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl CMDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CMDR::NONE => 0,
            CMDR::RETRIGGER => 0x01,
            CMDR::STOP => 0x02,
            CMDR::UPDATE => 0x03,
            CMDR::READSYNC => 0x04,
            CMDR::DMAOS => 0x05,
            CMDR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CMDR {
        match value {
            0 => CMDR::NONE,
            1 => CMDR::RETRIGGER,
            2 => CMDR::STOP,
            3 => CMDR::UPDATE,
            4 => CMDR::READSYNC,
            5 => CMDR::DMAOS,
            i => CMDR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline]
    pub fn is_none(&self) -> bool {
        *self == CMDR::NONE
    }
    #[doc = "Checks if the value of the field is `RETRIGGER`"]
    #[inline]
    pub fn is_retrigger(&self) -> bool {
        *self == CMDR::RETRIGGER
    }
    #[doc = "Checks if the value of the field is `STOP`"]
    #[inline]
    pub fn is_stop(&self) -> bool {
        *self == CMDR::STOP
    }
    #[doc = "Checks if the value of the field is `UPDATE`"]
    #[inline]
    pub fn is_update(&self) -> bool {
        *self == CMDR::UPDATE
    }
    #[doc = "Checks if the value of the field is `READSYNC`"]
    #[inline]
    pub fn is_readsync(&self) -> bool {
        *self == CMDR::READSYNC
    }
    #[doc = "Checks if the value of the field is `DMAOS`"]
    #[inline]
    pub fn is_dmaos(&self) -> bool {
        *self == CMDR::DMAOS
    }
}
#[doc = r" Proxy"]
pub struct _DIRW<'a> {
    w: &'a mut W,
}
impl<'a> _DIRW<'a> {
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
        self.w.bits |= ((value as u8) & 0x01) << 0;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _LUPDW<'a> {
    w: &'a mut W,
}
impl<'a> _LUPDW<'a> {
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
        self.w.bits |= ((value as u8) & 0x01) << 1;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ONESHOTW<'a> {
    w: &'a mut W,
}
impl<'a> _ONESHOTW<'a> {
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
        self.w.bits |= ((value as u8) & 0x01) << 2;
        self.w
    }
}
#[doc = "Values that can be written to the field `CMD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMDW {
    #[doc = "No action"]
    NONE,
    #[doc = "Force a start, restart or retrigger"]
    RETRIGGER,
    #[doc = "Force a stop"]
    STOP,
    #[doc = "Force update of double-buffered register"]
    UPDATE,
    #[doc = "Force a read synchronization of COUNT"]
    READSYNC,
    #[doc = "One-shot DMA trigger"]
    DMAOS,
}
impl CMDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CMDW::NONE => 0,
            CMDW::RETRIGGER => 1,
            CMDW::STOP => 2,
            CMDW::UPDATE => 3,
            CMDW::READSYNC => 4,
            CMDW::DMAOS => 5,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CMDW<'a> {
    w: &'a mut W,
}
impl<'a> _CMDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CMDW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "No action"]
    #[inline]
    pub fn none(self) -> &'a mut W {
        self.variant(CMDW::NONE)
    }
    #[doc = "Force a start, restart or retrigger"]
    #[inline]
    pub fn retrigger(self) -> &'a mut W {
        self.variant(CMDW::RETRIGGER)
    }
    #[doc = "Force a stop"]
    #[inline]
    pub fn stop(self) -> &'a mut W {
        self.variant(CMDW::STOP)
    }
    #[doc = "Force update of double-buffered register"]
    #[inline]
    pub fn update(self) -> &'a mut W {
        self.variant(CMDW::UPDATE)
    }
    #[doc = "Force a read synchronization of COUNT"]
    #[inline]
    pub fn readsync(self) -> &'a mut W {
        self.variant(CMDW::READSYNC)
    }
    #[doc = "One-shot DMA trigger"]
    #[inline]
    pub fn dmaos(self) -> &'a mut W {
        self.variant(CMDW::DMAOS)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(0x07 << 5);
        self.w.bits |= ((value as u8) & 0x07) << 5;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
    #[doc = "Bit 0 - Counter Direction"]
    #[inline]
    pub fn dir(&self) -> DIRR {
        let bits = ((self.bits >> 0) & 0x01) != 0;
        DIRR { bits }
    }
    #[doc = "Bit 1 - Lock Update"]
    #[inline]
    pub fn lupd(&self) -> LUPDR {
        let bits = ((self.bits >> 1) & 0x01) != 0;
        LUPDR { bits }
    }
    #[doc = "Bit 2 - One-Shot on Counter"]
    #[inline]
    pub fn oneshot(&self) -> ONESHOTR {
        let bits = ((self.bits >> 2) & 0x01) != 0;
        ONESHOTR { bits }
    }
    #[doc = "Bits 5:7 - Command"]
    #[inline]
    pub fn cmd(&self) -> CMDR {
        CMDR::_from(((self.bits >> 5) & 0x07) as u8)
    }
}
impl W {
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Counter Direction"]
    #[inline]
    pub fn dir(&mut self) -> _DIRW {
        _DIRW { w: self }
    }
    #[doc = "Bit 1 - Lock Update"]
    #[inline]
    pub fn lupd(&mut self) -> _LUPDW {
        _LUPDW { w: self }
    }
    #[doc = "Bit 2 - One-Shot on Counter"]
    #[inline]
    pub fn oneshot(&mut self) -> _ONESHOTW {
        _ONESHOTW { w: self }
    }
    #[doc = "Bits 5:7 - Command"]
    #[inline]
    pub fn cmd(&mut self) -> _CMDW {
        _CMDW { w: self }
    }
}
