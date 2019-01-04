#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::CHEVCTRL {
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
#[doc = "Possible values of the field `EVACT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EVACTR {
    #[doc = "No action"]
    NOACT,
    #[doc = "Transfer and periodic transfer trigger"]
    TRIG,
    #[doc = "Conditional transfer trigger"]
    CTRIG,
    #[doc = "Conditional block transfer"]
    CBLOCK,
    #[doc = "Channel suspend operation"]
    SUSPEND,
    #[doc = "Channel resume operation"]
    RESUME,
    #[doc = "Skip next block suspend action"]
    SSKIP,
    #[doc = "Increase priority"]
    INCPRI,
}
impl EVACTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            EVACTR::NOACT => 0,
            EVACTR::TRIG => 1,
            EVACTR::CTRIG => 2,
            EVACTR::CBLOCK => 3,
            EVACTR::SUSPEND => 4,
            EVACTR::RESUME => 5,
            EVACTR::SSKIP => 6,
            EVACTR::INCPRI => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> EVACTR {
        match value {
            0 => EVACTR::NOACT,
            1 => EVACTR::TRIG,
            2 => EVACTR::CTRIG,
            3 => EVACTR::CBLOCK,
            4 => EVACTR::SUSPEND,
            5 => EVACTR::RESUME,
            6 => EVACTR::SSKIP,
            7 => EVACTR::INCPRI,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NOACT`"]
    #[inline]
    pub fn is_noact(&self) -> bool {
        *self == EVACTR::NOACT
    }
    #[doc = "Checks if the value of the field is `TRIG`"]
    #[inline]
    pub fn is_trig(&self) -> bool {
        *self == EVACTR::TRIG
    }
    #[doc = "Checks if the value of the field is `CTRIG`"]
    #[inline]
    pub fn is_ctrig(&self) -> bool {
        *self == EVACTR::CTRIG
    }
    #[doc = "Checks if the value of the field is `CBLOCK`"]
    #[inline]
    pub fn is_cblock(&self) -> bool {
        *self == EVACTR::CBLOCK
    }
    #[doc = "Checks if the value of the field is `SUSPEND`"]
    #[inline]
    pub fn is_suspend(&self) -> bool {
        *self == EVACTR::SUSPEND
    }
    #[doc = "Checks if the value of the field is `RESUME`"]
    #[inline]
    pub fn is_resume(&self) -> bool {
        *self == EVACTR::RESUME
    }
    #[doc = "Checks if the value of the field is `SSKIP`"]
    #[inline]
    pub fn is_sskip(&self) -> bool {
        *self == EVACTR::SSKIP
    }
    #[doc = "Checks if the value of the field is `INCPRI`"]
    #[inline]
    pub fn is_incpri(&self) -> bool {
        *self == EVACTR::INCPRI
    }
}
#[doc = "Possible values of the field `EVOMODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EVOMODER {
    #[doc = "Block event output selection. Refer to BTCTRL.EVOSEL for available selections."]
    DEFAULT,
    #[doc = "Ongoing trigger action"]
    TRIGACT,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl EVOMODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            EVOMODER::DEFAULT => 0,
            EVOMODER::TRIGACT => 1,
            EVOMODER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> EVOMODER {
        match value {
            0 => EVOMODER::DEFAULT,
            1 => EVOMODER::TRIGACT,
            i => EVOMODER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `DEFAULT`"]
    #[inline]
    pub fn is_default(&self) -> bool {
        *self == EVOMODER::DEFAULT
    }
    #[doc = "Checks if the value of the field is `TRIGACT`"]
    #[inline]
    pub fn is_trigact(&self) -> bool {
        *self == EVOMODER::TRIGACT
    }
}
#[doc = r" Value of the field"]
pub struct EVIER {
    bits: bool,
}
impl EVIER {
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
pub struct EVOER {
    bits: bool,
}
impl EVOER {
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
#[doc = "Values that can be written to the field `EVACT`"]
pub enum EVACTW {
    #[doc = "No action"]
    NOACT,
    #[doc = "Transfer and periodic transfer trigger"]
    TRIG,
    #[doc = "Conditional transfer trigger"]
    CTRIG,
    #[doc = "Conditional block transfer"]
    CBLOCK,
    #[doc = "Channel suspend operation"]
    SUSPEND,
    #[doc = "Channel resume operation"]
    RESUME,
    #[doc = "Skip next block suspend action"]
    SSKIP,
    #[doc = "Increase priority"]
    INCPRI,
}
impl EVACTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            EVACTW::NOACT => 0,
            EVACTW::TRIG => 1,
            EVACTW::CTRIG => 2,
            EVACTW::CBLOCK => 3,
            EVACTW::SUSPEND => 4,
            EVACTW::RESUME => 5,
            EVACTW::SSKIP => 6,
            EVACTW::INCPRI => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EVACTW<'a> {
    w: &'a mut W,
}
impl<'a> _EVACTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EVACTW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "No action"]
    #[inline]
    pub fn noact(self) -> &'a mut W {
        self.variant(EVACTW::NOACT)
    }
    #[doc = "Transfer and periodic transfer trigger"]
    #[inline]
    pub fn trig(self) -> &'a mut W {
        self.variant(EVACTW::TRIG)
    }
    #[doc = "Conditional transfer trigger"]
    #[inline]
    pub fn ctrig(self) -> &'a mut W {
        self.variant(EVACTW::CTRIG)
    }
    #[doc = "Conditional block transfer"]
    #[inline]
    pub fn cblock(self) -> &'a mut W {
        self.variant(EVACTW::CBLOCK)
    }
    #[doc = "Channel suspend operation"]
    #[inline]
    pub fn suspend(self) -> &'a mut W {
        self.variant(EVACTW::SUSPEND)
    }
    #[doc = "Channel resume operation"]
    #[inline]
    pub fn resume(self) -> &'a mut W {
        self.variant(EVACTW::RESUME)
    }
    #[doc = "Skip next block suspend action"]
    #[inline]
    pub fn sskip(self) -> &'a mut W {
        self.variant(EVACTW::SSKIP)
    }
    #[doc = "Increase priority"]
    #[inline]
    pub fn incpri(self) -> &'a mut W {
        self.variant(EVACTW::INCPRI)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `EVOMODE`"]
pub enum EVOMODEW {
    #[doc = "Block event output selection. Refer to BTCTRL.EVOSEL for available selections."]
    DEFAULT,
    #[doc = "Ongoing trigger action"]
    TRIGACT,
}
impl EVOMODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            EVOMODEW::DEFAULT => 0,
            EVOMODEW::TRIGACT => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EVOMODEW<'a> {
    w: &'a mut W,
}
impl<'a> _EVOMODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EVOMODEW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Block event output selection. Refer to BTCTRL.EVOSEL for available selections."]
    #[inline]
    pub fn default(self) -> &'a mut W {
        self.variant(EVOMODEW::DEFAULT)
    }
    #[doc = "Ongoing trigger action"]
    #[inline]
    pub fn trigact(self) -> &'a mut W {
        self.variant(EVOMODEW::TRIGACT)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _EVIEW<'a> {
    w: &'a mut W,
}
impl<'a> _EVIEW<'a> {
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
pub struct _EVOEW<'a> {
    w: &'a mut W,
}
impl<'a> _EVOEW<'a> {
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
    #[doc = "Bits 0:2 - Channel Event Input Action"]
    #[inline]
    pub fn evact(&self) -> EVACTR {
        EVACTR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u8) as u8
        })
    }
    #[doc = "Bits 4:5 - Channel Event Output Mode"]
    #[inline]
    pub fn evomode(&self) -> EVOMODER {
        EVOMODER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u8) as u8
        })
    }
    #[doc = "Bit 6 - Channel Event Input Enable"]
    #[inline]
    pub fn evie(&self) -> EVIER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        };
        EVIER { bits }
    }
    #[doc = "Bit 7 - Channel Event Output Enable"]
    #[inline]
    pub fn evoe(&self) -> EVOER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        };
        EVOER { bits }
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
    #[doc = "Bits 0:2 - Channel Event Input Action"]
    #[inline]
    pub fn evact(&mut self) -> _EVACTW {
        _EVACTW { w: self }
    }
    #[doc = "Bits 4:5 - Channel Event Output Mode"]
    #[inline]
    pub fn evomode(&mut self) -> _EVOMODEW {
        _EVOMODEW { w: self }
    }
    #[doc = "Bit 6 - Channel Event Input Enable"]
    #[inline]
    pub fn evie(&mut self) -> _EVIEW {
        _EVIEW { w: self }
    }
    #[doc = "Bit 7 - Channel Event Output Enable"]
    #[inline]
    pub fn evoe(&mut self) -> _EVOEW {
        _EVOEW { w: self }
    }
}
