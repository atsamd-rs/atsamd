#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::MC1R {
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
#[doc = "Possible values of the field `CMDTYP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMDTYPR {
    #[doc = "Not a MMC specific command"]
    NORMAL,
    #[doc = "Wait IRQ Command"]
    WAITIRQ,
    #[doc = "Stream Command"]
    STREAM,
    #[doc = "Boot Command"]
    BOOT,
}
impl CMDTYPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CMDTYPR::NORMAL => 0,
            CMDTYPR::WAITIRQ => 1,
            CMDTYPR::STREAM => 2,
            CMDTYPR::BOOT => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CMDTYPR {
        match value {
            0 => CMDTYPR::NORMAL,
            1 => CMDTYPR::WAITIRQ,
            2 => CMDTYPR::STREAM,
            3 => CMDTYPR::BOOT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline]
    pub fn is_normal(&self) -> bool {
        *self == CMDTYPR::NORMAL
    }
    #[doc = "Checks if the value of the field is `WAITIRQ`"]
    #[inline]
    pub fn is_waitirq(&self) -> bool {
        *self == CMDTYPR::WAITIRQ
    }
    #[doc = "Checks if the value of the field is `STREAM`"]
    #[inline]
    pub fn is_stream(&self) -> bool {
        *self == CMDTYPR::STREAM
    }
    #[doc = "Checks if the value of the field is `BOOT`"]
    #[inline]
    pub fn is_boot(&self) -> bool {
        *self == CMDTYPR::BOOT
    }
}
#[doc = r" Value of the field"]
pub struct DDRR {
    bits: bool,
}
impl DDRR {
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
pub struct OPDR {
    bits: bool,
}
impl OPDR {
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
pub struct BOOTAR {
    bits: bool,
}
impl BOOTAR {
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
pub struct RSTNR {
    bits: bool,
}
impl RSTNR {
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
pub struct FCDR {
    bits: bool,
}
impl FCDR {
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
#[doc = "Values that can be written to the field `CMDTYP`"]
pub enum CMDTYPW {
    #[doc = "Not a MMC specific command"]
    NORMAL,
    #[doc = "Wait IRQ Command"]
    WAITIRQ,
    #[doc = "Stream Command"]
    STREAM,
    #[doc = "Boot Command"]
    BOOT,
}
impl CMDTYPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CMDTYPW::NORMAL => 0,
            CMDTYPW::WAITIRQ => 1,
            CMDTYPW::STREAM => 2,
            CMDTYPW::BOOT => 3,
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
    #[doc = "Not a MMC specific command"]
    #[inline]
    pub fn normal(self) -> &'a mut W {
        self.variant(CMDTYPW::NORMAL)
    }
    #[doc = "Wait IRQ Command"]
    #[inline]
    pub fn waitirq(self) -> &'a mut W {
        self.variant(CMDTYPW::WAITIRQ)
    }
    #[doc = "Stream Command"]
    #[inline]
    pub fn stream(self) -> &'a mut W {
        self.variant(CMDTYPW::STREAM)
    }
    #[doc = "Boot Command"]
    #[inline]
    pub fn boot(self) -> &'a mut W {
        self.variant(CMDTYPW::BOOT)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DDRW<'a> {
    w: &'a mut W,
}
impl<'a> _DDRW<'a> {
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
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _OPDW<'a> {
    w: &'a mut W,
}
impl<'a> _OPDW<'a> {
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
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _BOOTAW<'a> {
    w: &'a mut W,
}
impl<'a> _BOOTAW<'a> {
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
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RSTNW<'a> {
    w: &'a mut W,
}
impl<'a> _RSTNW<'a> {
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
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _FCDW<'a> {
    w: &'a mut W,
}
impl<'a> _FCDW<'a> {
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
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
    #[doc = "Bits 0:1 - e.MMC Command Type"]
    #[inline]
    pub fn cmdtyp(&self) -> CMDTYPR {
        CMDTYPR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u8) as u8
        })
    }
    #[doc = "Bit 3 - e.MMC HSDDR Mode"]
    #[inline]
    pub fn ddr(&self) -> DDRR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        };
        DDRR { bits }
    }
    #[doc = "Bit 4 - e.MMC Open Drain Mode"]
    #[inline]
    pub fn opd(&self) -> OPDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        };
        OPDR { bits }
    }
    #[doc = "Bit 5 - e.MMC Boot Acknowledge Enable"]
    #[inline]
    pub fn boota(&self) -> BOOTAR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        };
        BOOTAR { bits }
    }
    #[doc = "Bit 6 - e.MMC Reset Signal"]
    #[inline]
    pub fn rstn(&self) -> RSTNR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        };
        RSTNR { bits }
    }
    #[doc = "Bit 7 - e.MMC Force Card Detect"]
    #[inline]
    pub fn fcd(&self) -> FCDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        };
        FCDR { bits }
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
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:1 - e.MMC Command Type"]
    #[inline]
    pub fn cmdtyp(&mut self) -> _CMDTYPW {
        _CMDTYPW { w: self }
    }
    #[doc = "Bit 3 - e.MMC HSDDR Mode"]
    #[inline]
    pub fn ddr(&mut self) -> _DDRW {
        _DDRW { w: self }
    }
    #[doc = "Bit 4 - e.MMC Open Drain Mode"]
    #[inline]
    pub fn opd(&mut self) -> _OPDW {
        _OPDW { w: self }
    }
    #[doc = "Bit 5 - e.MMC Boot Acknowledge Enable"]
    #[inline]
    pub fn boota(&mut self) -> _BOOTAW {
        _BOOTAW { w: self }
    }
    #[doc = "Bit 6 - e.MMC Reset Signal"]
    #[inline]
    pub fn rstn(&mut self) -> _RSTNW {
        _RSTNW { w: self }
    }
    #[doc = "Bit 7 - e.MMC Force Card Detect"]
    #[inline]
    pub fn fcd(&mut self) -> _FCDW {
        _FCDW { w: self }
    }
}
