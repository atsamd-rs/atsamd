#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::WCR {
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
#[doc = "Possible values of the field `WKENCINT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKENCINTR {
    #[doc = "Disable"]
    DISABLE,
    #[doc = "Enable"]
    ENABLE,
}
impl WKENCINTR {
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
            WKENCINTR::DISABLE => false,
            WKENCINTR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WKENCINTR {
        match value {
            false => WKENCINTR::DISABLE,
            true => WKENCINTR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == WKENCINTR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == WKENCINTR::ENABLE
    }
}
#[doc = "Possible values of the field `WKENCINS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKENCINSR {
    #[doc = "Disable"]
    DISABLE,
    #[doc = "Enable"]
    ENABLE,
}
impl WKENCINSR {
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
            WKENCINSR::DISABLE => false,
            WKENCINSR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WKENCINSR {
        match value {
            false => WKENCINSR::DISABLE,
            true => WKENCINSR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == WKENCINSR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == WKENCINSR::ENABLE
    }
}
#[doc = "Possible values of the field `WKENCREM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKENCREMR {
    #[doc = "Disable"]
    DISABLE,
    #[doc = "Enable"]
    ENABLE,
}
impl WKENCREMR {
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
            WKENCREMR::DISABLE => false,
            WKENCREMR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WKENCREMR {
        match value {
            false => WKENCREMR::DISABLE,
            true => WKENCREMR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == WKENCREMR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == WKENCREMR::ENABLE
    }
}
#[doc = "Values that can be written to the field `WKENCINT`"]
pub enum WKENCINTW {
    #[doc = "Disable"]
    DISABLE,
    #[doc = "Enable"]
    ENABLE,
}
impl WKENCINTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WKENCINTW::DISABLE => false,
            WKENCINTW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WKENCINTW<'a> {
    w: &'a mut W,
}
impl<'a> _WKENCINTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WKENCINTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable"]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(WKENCINTW::DISABLE)
    }
    #[doc = "Enable"]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(WKENCINTW::ENABLE)
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
#[doc = "Values that can be written to the field `WKENCINS`"]
pub enum WKENCINSW {
    #[doc = "Disable"]
    DISABLE,
    #[doc = "Enable"]
    ENABLE,
}
impl WKENCINSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WKENCINSW::DISABLE => false,
            WKENCINSW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WKENCINSW<'a> {
    w: &'a mut W,
}
impl<'a> _WKENCINSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WKENCINSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable"]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(WKENCINSW::DISABLE)
    }
    #[doc = "Enable"]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(WKENCINSW::ENABLE)
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
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `WKENCREM`"]
pub enum WKENCREMW {
    #[doc = "Disable"]
    DISABLE,
    #[doc = "Enable"]
    ENABLE,
}
impl WKENCREMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WKENCREMW::DISABLE => false,
            WKENCREMW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WKENCREMW<'a> {
    w: &'a mut W,
}
impl<'a> _WKENCREMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WKENCREMW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable"]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(WKENCREMW::DISABLE)
    }
    #[doc = "Enable"]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(WKENCREMW::ENABLE)
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
    #[doc = "Bit 0 - Wakeup Event Enable on Card Interrupt"]
    #[inline]
    pub fn wkencint(&self) -> WKENCINTR {
        WKENCINTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 1 - Wakeup Event Enable on Card Insertion"]
    #[inline]
    pub fn wkencins(&self) -> WKENCINSR {
        WKENCINSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 2 - Wakeup Event Enable on Card Removal"]
    #[inline]
    pub fn wkencrem(&self) -> WKENCREMR {
        WKENCREMR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u8) != 0
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
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Wakeup Event Enable on Card Interrupt"]
    #[inline]
    pub fn wkencint(&mut self) -> _WKENCINTW {
        _WKENCINTW { w: self }
    }
    #[doc = "Bit 1 - Wakeup Event Enable on Card Insertion"]
    #[inline]
    pub fn wkencins(&mut self) -> _WKENCINSW {
        _WKENCINSW { w: self }
    }
    #[doc = "Bit 2 - Wakeup Event Enable on Card Removal"]
    #[inline]
    pub fn wkencrem(&mut self) -> _WKENCREMW {
        _WKENCREMW { w: self }
    }
}
