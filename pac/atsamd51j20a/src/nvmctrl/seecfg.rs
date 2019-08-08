#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::SEECFG {
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
#[doc = "Possible values of the field `WMODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WMODER {
    #[doc = "A NVM write command is issued after each write in the pagebuffer"]
    UNBUFFERED,
    #[doc = "A NVM write command is issued when a write to a new page is requested"]
    BUFFERED,
}
impl WMODER {
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
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            WMODER::UNBUFFERED => false,
            WMODER::BUFFERED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WMODER {
        match value {
            false => WMODER::UNBUFFERED,
            true => WMODER::BUFFERED,
        }
    }
    #[doc = "Checks if the value of the field is `UNBUFFERED`"]
    #[inline]
    pub fn is_unbuffered(&self) -> bool {
        *self == WMODER::UNBUFFERED
    }
    #[doc = "Checks if the value of the field is `BUFFERED`"]
    #[inline]
    pub fn is_buffered(&self) -> bool {
        *self == WMODER::BUFFERED
    }
}
#[doc = r" Value of the field"]
pub struct APRDISR {
    bits: bool,
}
impl APRDISR {
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
#[doc = "Values that can be written to the field `WMODE`"]
pub enum WMODEW {
    #[doc = "A NVM write command is issued after each write in the pagebuffer"]
    UNBUFFERED,
    #[doc = "A NVM write command is issued when a write to a new page is requested"]
    BUFFERED,
}
impl WMODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WMODEW::UNBUFFERED => false,
            WMODEW::BUFFERED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WMODEW<'a> {
    w: &'a mut W,
}
impl<'a> _WMODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WMODEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "A NVM write command is issued after each write in the pagebuffer"]
    #[inline]
    pub fn unbuffered(self) -> &'a mut W {
        self.variant(WMODEW::UNBUFFERED)
    }
    #[doc = "A NVM write command is issued when a write to a new page is requested"]
    #[inline]
    pub fn buffered(self) -> &'a mut W {
        self.variant(WMODEW::BUFFERED)
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
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _APRDISW<'a> {
    w: &'a mut W,
}
impl<'a> _APRDISW<'a> {
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
    #[doc = "Bit 0 - Write Mode"]
    #[inline]
    pub fn wmode(&self) -> WMODER {
        WMODER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 1 - Automatic Page Reallocation Disable"]
    #[inline]
    pub fn aprdis(&self) -> APRDISR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        };
        APRDISR { bits }
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
    #[doc = "Bit 0 - Write Mode"]
    #[inline]
    pub fn wmode(&mut self) -> _WMODEW {
        _WMODEW { w: self }
    }
    #[doc = "Bit 1 - Automatic Page Reallocation Disable"]
    #[inline]
    pub fn aprdis(&mut self) -> _APRDISW {
        _APRDISW { w: self }
    }
}
