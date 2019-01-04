#[doc = r" Value to write to the register"]
pub struct W {
    bits: u16,
}
impl super::FERACES {
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
}
#[doc = "Values that can be written to the field `ACMD12NE`"]
pub enum ACMD12NEW {
    #[doc = "No Interrupt"]
    NO,
    #[doc = "Interrupt is generated"]
    YES,
}
impl ACMD12NEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ACMD12NEW::NO => false,
            ACMD12NEW::YES => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ACMD12NEW<'a> {
    w: &'a mut W,
}
impl<'a> _ACMD12NEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ACMD12NEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No Interrupt"]
    #[inline]
    pub fn no(self) -> &'a mut W {
        self.variant(ACMD12NEW::NO)
    }
    #[doc = "Interrupt is generated"]
    #[inline]
    pub fn yes(self) -> &'a mut W {
        self.variant(ACMD12NEW::YES)
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
#[doc = "Values that can be written to the field `ACMDTEO`"]
pub enum ACMDTEOW {
    #[doc = "No Interrupt"]
    NO,
    #[doc = "Interrupt is generated"]
    YES,
}
impl ACMDTEOW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ACMDTEOW::NO => false,
            ACMDTEOW::YES => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ACMDTEOW<'a> {
    w: &'a mut W,
}
impl<'a> _ACMDTEOW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ACMDTEOW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No Interrupt"]
    #[inline]
    pub fn no(self) -> &'a mut W {
        self.variant(ACMDTEOW::NO)
    }
    #[doc = "Interrupt is generated"]
    #[inline]
    pub fn yes(self) -> &'a mut W {
        self.variant(ACMDTEOW::YES)
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
#[doc = "Values that can be written to the field `ACMDCRC`"]
pub enum ACMDCRCW {
    #[doc = "No Interrupt"]
    NO,
    #[doc = "Interrupt is generated"]
    YES,
}
impl ACMDCRCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ACMDCRCW::NO => false,
            ACMDCRCW::YES => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ACMDCRCW<'a> {
    w: &'a mut W,
}
impl<'a> _ACMDCRCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ACMDCRCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No Interrupt"]
    #[inline]
    pub fn no(self) -> &'a mut W {
        self.variant(ACMDCRCW::NO)
    }
    #[doc = "Interrupt is generated"]
    #[inline]
    pub fn yes(self) -> &'a mut W {
        self.variant(ACMDCRCW::YES)
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
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ACMDEND`"]
pub enum ACMDENDW {
    #[doc = "No Interrupt"]
    NO,
    #[doc = "Interrupt is generated"]
    YES,
}
impl ACMDENDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ACMDENDW::NO => false,
            ACMDENDW::YES => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ACMDENDW<'a> {
    w: &'a mut W,
}
impl<'a> _ACMDENDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ACMDENDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No Interrupt"]
    #[inline]
    pub fn no(self) -> &'a mut W {
        self.variant(ACMDENDW::NO)
    }
    #[doc = "Interrupt is generated"]
    #[inline]
    pub fn yes(self) -> &'a mut W {
        self.variant(ACMDENDW::YES)
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
#[doc = "Values that can be written to the field `ACMDIDX`"]
pub enum ACMDIDXW {
    #[doc = "No Interrupt"]
    NO,
    #[doc = "Interrupt is generated"]
    YES,
}
impl ACMDIDXW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ACMDIDXW::NO => false,
            ACMDIDXW::YES => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ACMDIDXW<'a> {
    w: &'a mut W,
}
impl<'a> _ACMDIDXW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ACMDIDXW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No Interrupt"]
    #[inline]
    pub fn no(self) -> &'a mut W {
        self.variant(ACMDIDXW::NO)
    }
    #[doc = "Interrupt is generated"]
    #[inline]
    pub fn yes(self) -> &'a mut W {
        self.variant(ACMDIDXW::YES)
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
#[doc = "Values that can be written to the field `CMDNI`"]
pub enum CMDNIW {
    #[doc = "No Interrupt"]
    NO,
    #[doc = "Interrupt is generated"]
    YES,
}
impl CMDNIW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CMDNIW::NO => false,
            CMDNIW::YES => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CMDNIW<'a> {
    w: &'a mut W,
}
impl<'a> _CMDNIW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CMDNIW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No Interrupt"]
    #[inline]
    pub fn no(self) -> &'a mut W {
        self.variant(CMDNIW::NO)
    }
    #[doc = "Interrupt is generated"]
    #[inline]
    pub fn yes(self) -> &'a mut W {
        self.variant(CMDNIW::YES)
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
        const OFFSET: u8 = 7;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
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
    #[doc = "Bit 0 - Force Event for Auto CMD12 Not Executed"]
    #[inline]
    pub fn acmd12ne(&mut self) -> _ACMD12NEW {
        _ACMD12NEW { w: self }
    }
    #[doc = "Bit 1 - Force Event for Auto CMD Timeout Error"]
    #[inline]
    pub fn acmdteo(&mut self) -> _ACMDTEOW {
        _ACMDTEOW { w: self }
    }
    #[doc = "Bit 2 - Force Event for Auto CMD CRC Error"]
    #[inline]
    pub fn acmdcrc(&mut self) -> _ACMDCRCW {
        _ACMDCRCW { w: self }
    }
    #[doc = "Bit 3 - Force Event for Auto CMD End Bit Error"]
    #[inline]
    pub fn acmdend(&mut self) -> _ACMDENDW {
        _ACMDENDW { w: self }
    }
    #[doc = "Bit 4 - Force Event for Auto CMD Index Error"]
    #[inline]
    pub fn acmdidx(&mut self) -> _ACMDIDXW {
        _ACMDIDXW { w: self }
    }
    #[doc = "Bit 7 - Force Event for Command Not Issued By Auto CMD12 Error"]
    #[inline]
    pub fn cmdni(&mut self) -> _CMDNIW {
        _CMDNIW { w: self }
    }
}
