#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::EVCTRL {
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
pub struct PID0R {
    bits: u8,
}
impl PID0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `EVACT0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EVACT0R {
    #[doc = "Event output to pin"]
    OUT,
    #[doc = "Set output register of pin on event"]
    SET,
    #[doc = "Clear output register of pin on event"]
    CLR,
    #[doc = "Toggle output register of pin on event"]
    TGL,
}
impl EVACT0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            EVACT0R::OUT => 0,
            EVACT0R::SET => 1,
            EVACT0R::CLR => 2,
            EVACT0R::TGL => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> EVACT0R {
        match value {
            0 => EVACT0R::OUT,
            1 => EVACT0R::SET,
            2 => EVACT0R::CLR,
            3 => EVACT0R::TGL,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `OUT`"]
    #[inline]
    pub fn is_out(&self) -> bool {
        *self == EVACT0R::OUT
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline]
    pub fn is_set(&self) -> bool {
        *self == EVACT0R::SET
    }
    #[doc = "Checks if the value of the field is `CLR`"]
    #[inline]
    pub fn is_clr(&self) -> bool {
        *self == EVACT0R::CLR
    }
    #[doc = "Checks if the value of the field is `TGL`"]
    #[inline]
    pub fn is_tgl(&self) -> bool {
        *self == EVACT0R::TGL
    }
}
#[doc = r" Value of the field"]
pub struct PORTEI0R {
    bits: bool,
}
impl PORTEI0R {
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
pub struct PID1R {
    bits: u8,
}
impl PID1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct EVACT1R {
    bits: u8,
}
impl EVACT1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PORTEI1R {
    bits: bool,
}
impl PORTEI1R {
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
pub struct PID2R {
    bits: u8,
}
impl PID2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct EVACT2R {
    bits: u8,
}
impl EVACT2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PORTEI2R {
    bits: bool,
}
impl PORTEI2R {
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
pub struct PID3R {
    bits: u8,
}
impl PID3R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct EVACT3R {
    bits: u8,
}
impl EVACT3R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PORTEI3R {
    bits: bool,
}
impl PORTEI3R {
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
#[doc = r" Proxy"]
pub struct _PID0W<'a> {
    w: &'a mut W,
}
impl<'a> _PID0W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `EVACT0`"]
pub enum EVACT0W {
    #[doc = "Event output to pin"]
    OUT,
    #[doc = "Set output register of pin on event"]
    SET,
    #[doc = "Clear output register of pin on event"]
    CLR,
    #[doc = "Toggle output register of pin on event"]
    TGL,
}
impl EVACT0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            EVACT0W::OUT => 0,
            EVACT0W::SET => 1,
            EVACT0W::CLR => 2,
            EVACT0W::TGL => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EVACT0W<'a> {
    w: &'a mut W,
}
impl<'a> _EVACT0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EVACT0W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Event output to pin"]
    #[inline]
    pub fn out(self) -> &'a mut W {
        self.variant(EVACT0W::OUT)
    }
    #[doc = "Set output register of pin on event"]
    #[inline]
    pub fn set(self) -> &'a mut W {
        self.variant(EVACT0W::SET)
    }
    #[doc = "Clear output register of pin on event"]
    #[inline]
    pub fn clr(self) -> &'a mut W {
        self.variant(EVACT0W::CLR)
    }
    #[doc = "Toggle output register of pin on event"]
    #[inline]
    pub fn tgl(self) -> &'a mut W {
        self.variant(EVACT0W::TGL)
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
#[doc = r" Proxy"]
pub struct _PORTEI0W<'a> {
    w: &'a mut W,
}
impl<'a> _PORTEI0W<'a> {
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
pub struct _PID1W<'a> {
    w: &'a mut W,
}
impl<'a> _PID1W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _EVACT1W<'a> {
    w: &'a mut W,
}
impl<'a> _EVACT1W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 13;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PORTEI1W<'a> {
    w: &'a mut W,
}
impl<'a> _PORTEI1W<'a> {
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
        const OFFSET: u8 = 15;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PID2W<'a> {
    w: &'a mut W,
}
impl<'a> _PID2W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _EVACT2W<'a> {
    w: &'a mut W,
}
impl<'a> _EVACT2W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 21;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PORTEI2W<'a> {
    w: &'a mut W,
}
impl<'a> _PORTEI2W<'a> {
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
        const OFFSET: u8 = 23;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PID3W<'a> {
    w: &'a mut W,
}
impl<'a> _PID3W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _EVACT3W<'a> {
    w: &'a mut W,
}
impl<'a> _EVACT3W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 29;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PORTEI3W<'a> {
    w: &'a mut W,
}
impl<'a> _PORTEI3W<'a> {
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
        const OFFSET: u8 = 31;
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
    #[doc = "Bits 0:4 - PORT Event Pin Identifier 0"]
    #[inline]
    pub fn pid0(&self) -> PID0R {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PID0R { bits }
    }
    #[doc = "Bits 5:6 - PORT Event Action 0"]
    #[inline]
    pub fn evact0(&self) -> EVACT0R {
        EVACT0R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 7 - PORT Event Input Enable 0"]
    #[inline]
    pub fn portei0(&self) -> PORTEI0R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PORTEI0R { bits }
    }
    #[doc = "Bits 8:12 - PORT Event Pin Identifier 1"]
    #[inline]
    pub fn pid1(&self) -> PID1R {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PID1R { bits }
    }
    #[doc = "Bits 13:14 - PORT Event Action 1"]
    #[inline]
    pub fn evact1(&self) -> EVACT1R {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        EVACT1R { bits }
    }
    #[doc = "Bit 15 - PORT Event Input Enable 1"]
    #[inline]
    pub fn portei1(&self) -> PORTEI1R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PORTEI1R { bits }
    }
    #[doc = "Bits 16:20 - PORT Event Pin Identifier 2"]
    #[inline]
    pub fn pid2(&self) -> PID2R {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PID2R { bits }
    }
    #[doc = "Bits 21:22 - PORT Event Action 2"]
    #[inline]
    pub fn evact2(&self) -> EVACT2R {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        EVACT2R { bits }
    }
    #[doc = "Bit 23 - PORT Event Input Enable 2"]
    #[inline]
    pub fn portei2(&self) -> PORTEI2R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PORTEI2R { bits }
    }
    #[doc = "Bits 24:28 - PORT Event Pin Identifier 3"]
    #[inline]
    pub fn pid3(&self) -> PID3R {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PID3R { bits }
    }
    #[doc = "Bits 29:30 - PORT Event Action 3"]
    #[inline]
    pub fn evact3(&self) -> EVACT3R {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        EVACT3R { bits }
    }
    #[doc = "Bit 31 - PORT Event Input Enable 3"]
    #[inline]
    pub fn portei3(&self) -> PORTEI3R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PORTEI3R { bits }
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
    #[doc = "Bits 0:4 - PORT Event Pin Identifier 0"]
    #[inline]
    pub fn pid0(&mut self) -> _PID0W {
        _PID0W { w: self }
    }
    #[doc = "Bits 5:6 - PORT Event Action 0"]
    #[inline]
    pub fn evact0(&mut self) -> _EVACT0W {
        _EVACT0W { w: self }
    }
    #[doc = "Bit 7 - PORT Event Input Enable 0"]
    #[inline]
    pub fn portei0(&mut self) -> _PORTEI0W {
        _PORTEI0W { w: self }
    }
    #[doc = "Bits 8:12 - PORT Event Pin Identifier 1"]
    #[inline]
    pub fn pid1(&mut self) -> _PID1W {
        _PID1W { w: self }
    }
    #[doc = "Bits 13:14 - PORT Event Action 1"]
    #[inline]
    pub fn evact1(&mut self) -> _EVACT1W {
        _EVACT1W { w: self }
    }
    #[doc = "Bit 15 - PORT Event Input Enable 1"]
    #[inline]
    pub fn portei1(&mut self) -> _PORTEI1W {
        _PORTEI1W { w: self }
    }
    #[doc = "Bits 16:20 - PORT Event Pin Identifier 2"]
    #[inline]
    pub fn pid2(&mut self) -> _PID2W {
        _PID2W { w: self }
    }
    #[doc = "Bits 21:22 - PORT Event Action 2"]
    #[inline]
    pub fn evact2(&mut self) -> _EVACT2W {
        _EVACT2W { w: self }
    }
    #[doc = "Bit 23 - PORT Event Input Enable 2"]
    #[inline]
    pub fn portei2(&mut self) -> _PORTEI2W {
        _PORTEI2W { w: self }
    }
    #[doc = "Bits 24:28 - PORT Event Pin Identifier 3"]
    #[inline]
    pub fn pid3(&mut self) -> _PID3W {
        _PID3W { w: self }
    }
    #[doc = "Bits 29:30 - PORT Event Action 3"]
    #[inline]
    pub fn evact3(&mut self) -> _EVACT3W {
        _EVACT3W { w: self }
    }
    #[doc = "Bit 31 - PORT Event Input Enable 3"]
    #[inline]
    pub fn portei3(&mut self) -> _PORTEI3W {
        _PORTEI3W { w: self }
    }
}
