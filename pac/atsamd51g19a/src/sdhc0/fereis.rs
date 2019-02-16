#[doc = r" Value to write to the register"]
pub struct W {
    bits: u16,
}
impl super::FEREIS {
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
#[doc = "Values that can be written to the field `CMDTEO`"]
pub enum CMDTEOW {
    #[doc = "No Interrupt"]
    NO,
    #[doc = "Interrupt is generated"]
    YES,
}
impl CMDTEOW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CMDTEOW::NO => false,
            CMDTEOW::YES => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CMDTEOW<'a> {
    w: &'a mut W,
}
impl<'a> _CMDTEOW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CMDTEOW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No Interrupt"]
    #[inline]
    pub fn no(self) -> &'a mut W {
        self.variant(CMDTEOW::NO)
    }
    #[doc = "Interrupt is generated"]
    #[inline]
    pub fn yes(self) -> &'a mut W {
        self.variant(CMDTEOW::YES)
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
#[doc = "Values that can be written to the field `CMDCRC`"]
pub enum CMDCRCW {
    #[doc = "No Interrupt"]
    NO,
    #[doc = "Interrupt is generated"]
    YES,
}
impl CMDCRCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CMDCRCW::NO => false,
            CMDCRCW::YES => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CMDCRCW<'a> {
    w: &'a mut W,
}
impl<'a> _CMDCRCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CMDCRCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No Interrupt"]
    #[inline]
    pub fn no(self) -> &'a mut W {
        self.variant(CMDCRCW::NO)
    }
    #[doc = "Interrupt is generated"]
    #[inline]
    pub fn yes(self) -> &'a mut W {
        self.variant(CMDCRCW::YES)
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
#[doc = "Values that can be written to the field `CMDEND`"]
pub enum CMDENDW {
    #[doc = "No Interrupt"]
    NO,
    #[doc = "Interrupt is generated"]
    YES,
}
impl CMDENDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CMDENDW::NO => false,
            CMDENDW::YES => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CMDENDW<'a> {
    w: &'a mut W,
}
impl<'a> _CMDENDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CMDENDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No Interrupt"]
    #[inline]
    pub fn no(self) -> &'a mut W {
        self.variant(CMDENDW::NO)
    }
    #[doc = "Interrupt is generated"]
    #[inline]
    pub fn yes(self) -> &'a mut W {
        self.variant(CMDENDW::YES)
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
#[doc = "Values that can be written to the field `CMDIDX`"]
pub enum CMDIDXW {
    #[doc = "No Interrupt"]
    NO,
    #[doc = "Interrupt is generated"]
    YES,
}
impl CMDIDXW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CMDIDXW::NO => false,
            CMDIDXW::YES => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CMDIDXW<'a> {
    w: &'a mut W,
}
impl<'a> _CMDIDXW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CMDIDXW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No Interrupt"]
    #[inline]
    pub fn no(self) -> &'a mut W {
        self.variant(CMDIDXW::NO)
    }
    #[doc = "Interrupt is generated"]
    #[inline]
    pub fn yes(self) -> &'a mut W {
        self.variant(CMDIDXW::YES)
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
#[doc = "Values that can be written to the field `DATTEO`"]
pub enum DATTEOW {
    #[doc = "No Interrupt"]
    NO,
    #[doc = "Interrupt is generated"]
    YES,
}
impl DATTEOW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DATTEOW::NO => false,
            DATTEOW::YES => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DATTEOW<'a> {
    w: &'a mut W,
}
impl<'a> _DATTEOW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DATTEOW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No Interrupt"]
    #[inline]
    pub fn no(self) -> &'a mut W {
        self.variant(DATTEOW::NO)
    }
    #[doc = "Interrupt is generated"]
    #[inline]
    pub fn yes(self) -> &'a mut W {
        self.variant(DATTEOW::YES)
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
#[doc = "Values that can be written to the field `DATCRC`"]
pub enum DATCRCW {
    #[doc = "No Interrupt"]
    NO,
    #[doc = "Interrupt is generated"]
    YES,
}
impl DATCRCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DATCRCW::NO => false,
            DATCRCW::YES => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DATCRCW<'a> {
    w: &'a mut W,
}
impl<'a> _DATCRCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DATCRCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No Interrupt"]
    #[inline]
    pub fn no(self) -> &'a mut W {
        self.variant(DATCRCW::NO)
    }
    #[doc = "Interrupt is generated"]
    #[inline]
    pub fn yes(self) -> &'a mut W {
        self.variant(DATCRCW::YES)
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
#[doc = "Values that can be written to the field `DATEND`"]
pub enum DATENDW {
    #[doc = "No Interrupt"]
    NO,
    #[doc = "Interrupt is generated"]
    YES,
}
impl DATENDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DATENDW::NO => false,
            DATENDW::YES => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DATENDW<'a> {
    w: &'a mut W,
}
impl<'a> _DATENDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DATENDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No Interrupt"]
    #[inline]
    pub fn no(self) -> &'a mut W {
        self.variant(DATENDW::NO)
    }
    #[doc = "Interrupt is generated"]
    #[inline]
    pub fn yes(self) -> &'a mut W {
        self.variant(DATENDW::YES)
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
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CURLIM`"]
pub enum CURLIMW {
    #[doc = "No Interrupt"]
    NO,
    #[doc = "Interrupt is generated"]
    YES,
}
impl CURLIMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CURLIMW::NO => false,
            CURLIMW::YES => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CURLIMW<'a> {
    w: &'a mut W,
}
impl<'a> _CURLIMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CURLIMW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No Interrupt"]
    #[inline]
    pub fn no(self) -> &'a mut W {
        self.variant(CURLIMW::NO)
    }
    #[doc = "Interrupt is generated"]
    #[inline]
    pub fn yes(self) -> &'a mut W {
        self.variant(CURLIMW::YES)
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
#[doc = "Values that can be written to the field `ACMD`"]
pub enum ACMDW {
    #[doc = "No Interrupt"]
    NO,
    #[doc = "Interrupt is generated"]
    YES,
}
impl ACMDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ACMDW::NO => false,
            ACMDW::YES => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ACMDW<'a> {
    w: &'a mut W,
}
impl<'a> _ACMDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ACMDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No Interrupt"]
    #[inline]
    pub fn no(self) -> &'a mut W {
        self.variant(ACMDW::NO)
    }
    #[doc = "Interrupt is generated"]
    #[inline]
    pub fn yes(self) -> &'a mut W {
        self.variant(ACMDW::YES)
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
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ADMA`"]
pub enum ADMAW {
    #[doc = "No Interrupt"]
    NO,
    #[doc = "Interrupt is generated"]
    YES,
}
impl ADMAW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ADMAW::NO => false,
            ADMAW::YES => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ADMAW<'a> {
    w: &'a mut W,
}
impl<'a> _ADMAW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ADMAW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No Interrupt"]
    #[inline]
    pub fn no(self) -> &'a mut W {
        self.variant(ADMAW::NO)
    }
    #[doc = "Interrupt is generated"]
    #[inline]
    pub fn yes(self) -> &'a mut W {
        self.variant(ADMAW::YES)
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
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `BOOTAE`"]
pub enum BOOTAEW {
    #[doc = "No Interrupt"]
    NO,
    #[doc = "Interrupt is generated"]
    YES,
}
impl BOOTAEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BOOTAEW::NO => false,
            BOOTAEW::YES => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BOOTAEW<'a> {
    w: &'a mut W,
}
impl<'a> _BOOTAEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BOOTAEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No Interrupt"]
    #[inline]
    pub fn no(self) -> &'a mut W {
        self.variant(BOOTAEW::NO)
    }
    #[doc = "Interrupt is generated"]
    #[inline]
    pub fn yes(self) -> &'a mut W {
        self.variant(BOOTAEW::YES)
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
        const OFFSET: u8 = 12;
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
    #[doc = "Bit 0 - Force Event for Command Timeout Error"]
    #[inline]
    pub fn cmdteo(&mut self) -> _CMDTEOW {
        _CMDTEOW { w: self }
    }
    #[doc = "Bit 1 - Force Event for Command CRC Error"]
    #[inline]
    pub fn cmdcrc(&mut self) -> _CMDCRCW {
        _CMDCRCW { w: self }
    }
    #[doc = "Bit 2 - Force Event for Command End Bit Error"]
    #[inline]
    pub fn cmdend(&mut self) -> _CMDENDW {
        _CMDENDW { w: self }
    }
    #[doc = "Bit 3 - Force Event for Command Index Error"]
    #[inline]
    pub fn cmdidx(&mut self) -> _CMDIDXW {
        _CMDIDXW { w: self }
    }
    #[doc = "Bit 4 - Force Event for Data Timeout Error"]
    #[inline]
    pub fn datteo(&mut self) -> _DATTEOW {
        _DATTEOW { w: self }
    }
    #[doc = "Bit 5 - Force Event for Data CRC Error"]
    #[inline]
    pub fn datcrc(&mut self) -> _DATCRCW {
        _DATCRCW { w: self }
    }
    #[doc = "Bit 6 - Force Event for Data End Bit Error"]
    #[inline]
    pub fn datend(&mut self) -> _DATENDW {
        _DATENDW { w: self }
    }
    #[doc = "Bit 7 - Force Event for Current Limit Error"]
    #[inline]
    pub fn curlim(&mut self) -> _CURLIMW {
        _CURLIMW { w: self }
    }
    #[doc = "Bit 8 - Force Event for Auto CMD Error"]
    #[inline]
    pub fn acmd(&mut self) -> _ACMDW {
        _ACMDW { w: self }
    }
    #[doc = "Bit 9 - Force Event for ADMA Error"]
    #[inline]
    pub fn adma(&mut self) -> _ADMAW {
        _ADMAW { w: self }
    }
    #[doc = "Bit 12 - Force Event for Boot Acknowledge Error"]
    #[inline]
    pub fn bootae(&mut self) -> _BOOTAEW {
        _BOOTAEW { w: self }
    }
}
