#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CHCTRLB {
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
    #[doc = r" Reserved"]
    _Reserved(u8),
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
            EVACTR::_Reserved(bits) => bits,
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
            i => EVACTR::_Reserved(i),
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
#[doc = "Possible values of the field `LVL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LVLR {
    #[doc = "Channel Priority Level 0"]
    LVL0,
    #[doc = "Channel Priority Level 1"]
    LVL1,
    #[doc = "Channel Priority Level 2"]
    LVL2,
    #[doc = "Channel Priority Level 3"]
    LVL3,
}
impl LVLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            LVLR::LVL0 => 0,
            LVLR::LVL1 => 1,
            LVLR::LVL2 => 2,
            LVLR::LVL3 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> LVLR {
        match value {
            0 => LVLR::LVL0,
            1 => LVLR::LVL1,
            2 => LVLR::LVL2,
            3 => LVLR::LVL3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LVL0`"]
    #[inline]
    pub fn is_lvl0(&self) -> bool {
        *self == LVLR::LVL0
    }
    #[doc = "Checks if the value of the field is `LVL1`"]
    #[inline]
    pub fn is_lvl1(&self) -> bool {
        *self == LVLR::LVL1
    }
    #[doc = "Checks if the value of the field is `LVL2`"]
    #[inline]
    pub fn is_lvl2(&self) -> bool {
        *self == LVLR::LVL2
    }
    #[doc = "Checks if the value of the field is `LVL3`"]
    #[inline]
    pub fn is_lvl3(&self) -> bool {
        *self == LVLR::LVL3
    }
}
#[doc = "Possible values of the field `TRIGSRC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRIGSRCR {
    #[doc = "Only software/event triggers"]
    DISABLE,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl TRIGSRCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TRIGSRCR::DISABLE => 0,
            TRIGSRCR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TRIGSRCR {
        match value {
            0 => TRIGSRCR::DISABLE,
            i => TRIGSRCR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == TRIGSRCR::DISABLE
    }
}
#[doc = "Possible values of the field `TRIGACT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRIGACTR {
    #[doc = "One trigger required for each block transfer"]
    BLOCK,
    #[doc = "One trigger required for each beat transfer"]
    BEAT,
    #[doc = "One trigger required for each transaction"]
    TRANSACTION,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl TRIGACTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TRIGACTR::BLOCK => 0,
            TRIGACTR::BEAT => 2,
            TRIGACTR::TRANSACTION => 3,
            TRIGACTR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TRIGACTR {
        match value {
            0 => TRIGACTR::BLOCK,
            2 => TRIGACTR::BEAT,
            3 => TRIGACTR::TRANSACTION,
            i => TRIGACTR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `BLOCK`"]
    #[inline]
    pub fn is_block(&self) -> bool {
        *self == TRIGACTR::BLOCK
    }
    #[doc = "Checks if the value of the field is `BEAT`"]
    #[inline]
    pub fn is_beat(&self) -> bool {
        *self == TRIGACTR::BEAT
    }
    #[doc = "Checks if the value of the field is `TRANSACTION`"]
    #[inline]
    pub fn is_transaction(&self) -> bool {
        *self == TRIGACTR::TRANSACTION
    }
}
#[doc = "Possible values of the field `CMD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMDR {
    #[doc = "No action"]
    NOACT,
    #[doc = "Channel suspend operation"]
    SUSPEND,
    #[doc = "Channel resume operation"]
    RESUME,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl CMDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CMDR::NOACT => 0,
            CMDR::SUSPEND => 1,
            CMDR::RESUME => 2,
            CMDR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CMDR {
        match value {
            0 => CMDR::NOACT,
            1 => CMDR::SUSPEND,
            2 => CMDR::RESUME,
            i => CMDR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `NOACT`"]
    #[inline]
    pub fn is_noact(&self) -> bool {
        *self == CMDR::NOACT
    }
    #[doc = "Checks if the value of the field is `SUSPEND`"]
    #[inline]
    pub fn is_suspend(&self) -> bool {
        *self == CMDR::SUSPEND
    }
    #[doc = "Checks if the value of the field is `RESUME`"]
    #[inline]
    pub fn is_resume(&self) -> bool {
        *self == CMDR::RESUME
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
        unsafe { self.bits(variant._bits()) }
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
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
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
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
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
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `LVL`"]
pub enum LVLW {
    #[doc = "Channel Priority Level 0"]
    LVL0,
    #[doc = "Channel Priority Level 1"]
    LVL1,
    #[doc = "Channel Priority Level 2"]
    LVL2,
    #[doc = "Channel Priority Level 3"]
    LVL3,
}
impl LVLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            LVLW::LVL0 => 0,
            LVLW::LVL1 => 1,
            LVLW::LVL2 => 2,
            LVLW::LVL3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LVLW<'a> {
    w: &'a mut W,
}
impl<'a> _LVLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LVLW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Channel Priority Level 0"]
    #[inline]
    pub fn lvl0(self) -> &'a mut W {
        self.variant(LVLW::LVL0)
    }
    #[doc = "Channel Priority Level 1"]
    #[inline]
    pub fn lvl1(self) -> &'a mut W {
        self.variant(LVLW::LVL1)
    }
    #[doc = "Channel Priority Level 2"]
    #[inline]
    pub fn lvl2(self) -> &'a mut W {
        self.variant(LVLW::LVL2)
    }
    #[doc = "Channel Priority Level 3"]
    #[inline]
    pub fn lvl3(self) -> &'a mut W {
        self.variant(LVLW::LVL3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TRIGSRC`"]
pub enum TRIGSRCW {
    #[doc = "Only software/event triggers"]
    DISABLE,
}
impl TRIGSRCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TRIGSRCW::DISABLE => 0,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TRIGSRCW<'a> {
    w: &'a mut W,
}
impl<'a> _TRIGSRCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TRIGSRCW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Only software/event triggers"]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(TRIGSRCW::DISABLE)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 63;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TRIGACT`"]
pub enum TRIGACTW {
    #[doc = "One trigger required for each block transfer"]
    BLOCK,
    #[doc = "One trigger required for each beat transfer"]
    BEAT,
    #[doc = "One trigger required for each transaction"]
    TRANSACTION,
}
impl TRIGACTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TRIGACTW::BLOCK => 0,
            TRIGACTW::BEAT => 2,
            TRIGACTW::TRANSACTION => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TRIGACTW<'a> {
    w: &'a mut W,
}
impl<'a> _TRIGACTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TRIGACTW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "One trigger required for each block transfer"]
    #[inline]
    pub fn block(self) -> &'a mut W {
        self.variant(TRIGACTW::BLOCK)
    }
    #[doc = "One trigger required for each beat transfer"]
    #[inline]
    pub fn beat(self) -> &'a mut W {
        self.variant(TRIGACTW::BEAT)
    }
    #[doc = "One trigger required for each transaction"]
    #[inline]
    pub fn transaction(self) -> &'a mut W {
        self.variant(TRIGACTW::TRANSACTION)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 22;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CMD`"]
pub enum CMDW {
    #[doc = "No action"]
    NOACT,
    #[doc = "Channel suspend operation"]
    SUSPEND,
    #[doc = "Channel resume operation"]
    RESUME,
}
impl CMDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CMDW::NOACT => 0,
            CMDW::SUSPEND => 1,
            CMDW::RESUME => 2,
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
    pub fn noact(self) -> &'a mut W {
        self.variant(CMDW::NOACT)
    }
    #[doc = "Channel suspend operation"]
    #[inline]
    pub fn suspend(self) -> &'a mut W {
        self.variant(CMDW::SUSPEND)
    }
    #[doc = "Channel resume operation"]
    #[inline]
    pub fn resume(self) -> &'a mut W {
        self.variant(CMDW::RESUME)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
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
    #[doc = "Bits 0:2 - Event Input Action"]
    #[inline]
    pub fn evact(&self) -> EVACTR {
        EVACTR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 3 - Channel Event Input Enable"]
    #[inline]
    pub fn evie(&self) -> EVIER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        EVIER { bits }
    }
    #[doc = "Bit 4 - Channel Event Output Enable"]
    #[inline]
    pub fn evoe(&self) -> EVOER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        EVOER { bits }
    }
    #[doc = "Bits 5:6 - Channel Arbitration Level"]
    #[inline]
    pub fn lvl(&self) -> LVLR {
        LVLR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:13 - Trigger Source"]
    #[inline]
    pub fn trigsrc(&self) -> TRIGSRCR {
        TRIGSRCR::_from({
            const MASK: u8 = 63;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 22:23 - Trigger Action"]
    #[inline]
    pub fn trigact(&self) -> TRIGACTR {
        TRIGACTR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 24:25 - Software Command"]
    #[inline]
    pub fn cmd(&self) -> CMDR {
        CMDR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
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
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:2 - Event Input Action"]
    #[inline]
    pub fn evact(&mut self) -> _EVACTW {
        _EVACTW { w: self }
    }
    #[doc = "Bit 3 - Channel Event Input Enable"]
    #[inline]
    pub fn evie(&mut self) -> _EVIEW {
        _EVIEW { w: self }
    }
    #[doc = "Bit 4 - Channel Event Output Enable"]
    #[inline]
    pub fn evoe(&mut self) -> _EVOEW {
        _EVOEW { w: self }
    }
    #[doc = "Bits 5:6 - Channel Arbitration Level"]
    #[inline]
    pub fn lvl(&mut self) -> _LVLW {
        _LVLW { w: self }
    }
    #[doc = "Bits 8:13 - Trigger Source"]
    #[inline]
    pub fn trigsrc(&mut self) -> _TRIGSRCW {
        _TRIGSRCW { w: self }
    }
    #[doc = "Bits 22:23 - Trigger Action"]
    #[inline]
    pub fn trigact(&mut self) -> _TRIGACTW {
        _TRIGACTW { w: self }
    }
    #[doc = "Bits 24:25 - Software Command"]
    #[inline]
    pub fn cmd(&mut self) -> _CMDW {
        _CMDW { w: self }
    }
}
