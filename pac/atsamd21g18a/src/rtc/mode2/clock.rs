#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CLOCK {
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
pub struct SECONDR {
    bits: u8,
}
impl SECONDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct MINUTER {
    bits: u8,
}
impl MINUTER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `HOUR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HOURR {
    #[doc = "AM when CLKREP in 12-hour"]
    AM,
    #[doc = "PM when CLKREP in 12-hour"]
    PM,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl HOURR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            HOURR::AM => 0,
            HOURR::PM => 16,
            HOURR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> HOURR {
        match value {
            0 => HOURR::AM,
            16 => HOURR::PM,
            i => HOURR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `AM`"]
    #[inline]
    pub fn is_am(&self) -> bool {
        *self == HOURR::AM
    }
    #[doc = "Checks if the value of the field is `PM`"]
    #[inline]
    pub fn is_pm(&self) -> bool {
        *self == HOURR::PM
    }
}
#[doc = r" Value of the field"]
pub struct DAYR {
    bits: u8,
}
impl DAYR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct MONTHR {
    bits: u8,
}
impl MONTHR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct YEARR {
    bits: u8,
}
impl YEARR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _SECONDW<'a> {
    w: &'a mut W,
}
impl<'a> _SECONDW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 63;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _MINUTEW<'a> {
    w: &'a mut W,
}
impl<'a> _MINUTEW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 63;
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `HOUR`"]
pub enum HOURW {
    #[doc = "AM when CLKREP in 12-hour"]
    AM,
    #[doc = "PM when CLKREP in 12-hour"]
    PM,
}
impl HOURW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            HOURW::AM => 0,
            HOURW::PM => 16,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HOURW<'a> {
    w: &'a mut W,
}
impl<'a> _HOURW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HOURW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "AM when CLKREP in 12-hour"]
    #[inline]
    pub fn am(self) -> &'a mut W {
        self.variant(HOURW::AM)
    }
    #[doc = "PM when CLKREP in 12-hour"]
    #[inline]
    pub fn pm(self) -> &'a mut W {
        self.variant(HOURW::PM)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DAYW<'a> {
    w: &'a mut W,
}
impl<'a> _DAYW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 17;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _MONTHW<'a> {
    w: &'a mut W,
}
impl<'a> _MONTHW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 22;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _YEARW<'a> {
    w: &'a mut W,
}
impl<'a> _YEARW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 63;
        const OFFSET: u8 = 26;
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
    #[doc = "Bits 0:5 - Second"]
    #[inline]
    pub fn second(&self) -> SECONDR {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SECONDR { bits }
    }
    #[doc = "Bits 6:11 - Minute"]
    #[inline]
    pub fn minute(&self) -> MINUTER {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        MINUTER { bits }
    }
    #[doc = "Bits 12:16 - Hour"]
    #[inline]
    pub fn hour(&self) -> HOURR {
        HOURR::_from({
            const MASK: u8 = 31;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 17:21 - Day"]
    #[inline]
    pub fn day(&self) -> DAYR {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DAYR { bits }
    }
    #[doc = "Bits 22:25 - Month"]
    #[inline]
    pub fn month(&self) -> MONTHR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        MONTHR { bits }
    }
    #[doc = "Bits 26:31 - Year"]
    #[inline]
    pub fn year(&self) -> YEARR {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        YEARR { bits }
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
    #[doc = "Bits 0:5 - Second"]
    #[inline]
    pub fn second(&mut self) -> _SECONDW {
        _SECONDW { w: self }
    }
    #[doc = "Bits 6:11 - Minute"]
    #[inline]
    pub fn minute(&mut self) -> _MINUTEW {
        _MINUTEW { w: self }
    }
    #[doc = "Bits 12:16 - Hour"]
    #[inline]
    pub fn hour(&mut self) -> _HOURW {
        _HOURW { w: self }
    }
    #[doc = "Bits 17:21 - Day"]
    #[inline]
    pub fn day(&mut self) -> _DAYW {
        _DAYW { w: self }
    }
    #[doc = "Bits 22:25 - Month"]
    #[inline]
    pub fn month(&mut self) -> _MONTHW {
        _MONTHW { w: self }
    }
    #[doc = "Bits 26:31 - Year"]
    #[inline]
    pub fn year(&mut self) -> _YEARW {
        _YEARW { w: self }
    }
}
