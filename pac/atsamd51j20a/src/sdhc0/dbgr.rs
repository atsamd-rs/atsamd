#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::DBGR {
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
#[doc = "Possible values of the field `NIDBG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NIDBGR {
    #[doc = "Debugging is intrusive (reads of BDPR from debugger are considered and increment the internal buffer pointer)"]
    IDBG,
    #[doc = "Debugging is not intrusive (reads of BDPR from debugger are discarded and do not increment the internal buffer pointer)"]
    NIDBG,
}
impl NIDBGR {
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
            NIDBGR::IDBG => false,
            NIDBGR::NIDBG => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> NIDBGR {
        match value {
            false => NIDBGR::IDBG,
            true => NIDBGR::NIDBG,
        }
    }
    #[doc = "Checks if the value of the field is `IDBG`"]
    #[inline]
    pub fn is_idbg(&self) -> bool {
        *self == NIDBGR::IDBG
    }
    #[doc = "Checks if the value of the field is `NIDBG`"]
    #[inline]
    pub fn is_nidbg(&self) -> bool {
        *self == NIDBGR::NIDBG
    }
}
#[doc = "Values that can be written to the field `NIDBG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NIDBGW {
    #[doc = "Debugging is intrusive (reads of BDPR from debugger are considered and increment the internal buffer pointer)"]
    IDBG,
    #[doc = "Debugging is not intrusive (reads of BDPR from debugger are discarded and do not increment the internal buffer pointer)"]
    NIDBG,
}
impl NIDBGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            NIDBGW::IDBG => false,
            NIDBGW::NIDBG => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _NIDBGW<'a> {
    w: &'a mut W,
}
impl<'a> _NIDBGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: NIDBGW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Debugging is intrusive (reads of BDPR from debugger are considered and increment the internal buffer pointer)"]
    #[inline]
    pub fn idbg(self) -> &'a mut W {
        self.variant(NIDBGW::IDBG)
    }
    #[doc = "Debugging is not intrusive (reads of BDPR from debugger are discarded and do not increment the internal buffer pointer)"]
    #[inline]
    pub fn nidbg(self) -> &'a mut W {
        self.variant(NIDBGW::NIDBG)
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
        self.w.bits &= !(0x01 << 0);
        self.w.bits |= ((value as u8) & 0x01) << 0;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
    #[doc = "Bit 0 - Non-intrusive debug enable"]
    #[inline]
    pub fn nidbg(&self) -> NIDBGR {
        NIDBGR::_from(((self.bits >> 0) & 0x01) != 0)
    }
}
impl W {
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Non-intrusive debug enable"]
    #[inline]
    pub fn nidbg(&mut self) -> _NIDBGW {
        _NIDBGW { w: self }
    }
}
