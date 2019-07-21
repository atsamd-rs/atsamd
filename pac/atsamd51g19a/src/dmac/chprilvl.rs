#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::CHPRILVL {
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
#[doc = "Possible values of the field `PRILVL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRILVLR {
    #[doc = "Channel Priority Level 0 (Lowest Level)"]
    LVL0,
    #[doc = "Channel Priority Level 1"]
    LVL1,
    #[doc = "Channel Priority Level 2"]
    LVL2,
    #[doc = "Channel Priority Level 3"]
    LVL3,
    #[doc = "Channel Priority Level 4"]
    LVL4,
    #[doc = "Channel Priority Level 5"]
    LVL5,
    #[doc = "Channel Priority Level 6"]
    LVL6,
    #[doc = "Channel Priority Level 7 (Highest Level)"]
    LVL7,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PRILVLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PRILVLR::LVL0 => 0,
            PRILVLR::LVL1 => 0x01,
            PRILVLR::LVL2 => 0x02,
            PRILVLR::LVL3 => 0x03,
            PRILVLR::LVL4 => 0x04,
            PRILVLR::LVL5 => 0x05,
            PRILVLR::LVL6 => 0x06,
            PRILVLR::LVL7 => 0x07,
            PRILVLR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PRILVLR {
        match value {
            0 => PRILVLR::LVL0,
            1 => PRILVLR::LVL1,
            2 => PRILVLR::LVL2,
            3 => PRILVLR::LVL3,
            4 => PRILVLR::LVL4,
            5 => PRILVLR::LVL5,
            6 => PRILVLR::LVL6,
            7 => PRILVLR::LVL7,
            i => PRILVLR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `LVL0`"]
    #[inline]
    pub fn is_lvl0(&self) -> bool {
        *self == PRILVLR::LVL0
    }
    #[doc = "Checks if the value of the field is `LVL1`"]
    #[inline]
    pub fn is_lvl1(&self) -> bool {
        *self == PRILVLR::LVL1
    }
    #[doc = "Checks if the value of the field is `LVL2`"]
    #[inline]
    pub fn is_lvl2(&self) -> bool {
        *self == PRILVLR::LVL2
    }
    #[doc = "Checks if the value of the field is `LVL3`"]
    #[inline]
    pub fn is_lvl3(&self) -> bool {
        *self == PRILVLR::LVL3
    }
    #[doc = "Checks if the value of the field is `LVL4`"]
    #[inline]
    pub fn is_lvl4(&self) -> bool {
        *self == PRILVLR::LVL4
    }
    #[doc = "Checks if the value of the field is `LVL5`"]
    #[inline]
    pub fn is_lvl5(&self) -> bool {
        *self == PRILVLR::LVL5
    }
    #[doc = "Checks if the value of the field is `LVL6`"]
    #[inline]
    pub fn is_lvl6(&self) -> bool {
        *self == PRILVLR::LVL6
    }
    #[doc = "Checks if the value of the field is `LVL7`"]
    #[inline]
    pub fn is_lvl7(&self) -> bool {
        *self == PRILVLR::LVL7
    }
}
#[doc = "Values that can be written to the field `PRILVL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRILVLW {
    #[doc = "Channel Priority Level 0 (Lowest Level)"]
    LVL0,
    #[doc = "Channel Priority Level 1"]
    LVL1,
    #[doc = "Channel Priority Level 2"]
    LVL2,
    #[doc = "Channel Priority Level 3"]
    LVL3,
    #[doc = "Channel Priority Level 4"]
    LVL4,
    #[doc = "Channel Priority Level 5"]
    LVL5,
    #[doc = "Channel Priority Level 6"]
    LVL6,
    #[doc = "Channel Priority Level 7 (Highest Level)"]
    LVL7,
}
impl PRILVLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PRILVLW::LVL0 => 0,
            PRILVLW::LVL1 => 1,
            PRILVLW::LVL2 => 2,
            PRILVLW::LVL3 => 3,
            PRILVLW::LVL4 => 4,
            PRILVLW::LVL5 => 5,
            PRILVLW::LVL6 => 6,
            PRILVLW::LVL7 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PRILVLW<'a> {
    w: &'a mut W,
}
impl<'a> _PRILVLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PRILVLW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Channel Priority Level 0 (Lowest Level)"]
    #[inline]
    pub fn lvl0(self) -> &'a mut W {
        self.variant(PRILVLW::LVL0)
    }
    #[doc = "Channel Priority Level 1"]
    #[inline]
    pub fn lvl1(self) -> &'a mut W {
        self.variant(PRILVLW::LVL1)
    }
    #[doc = "Channel Priority Level 2"]
    #[inline]
    pub fn lvl2(self) -> &'a mut W {
        self.variant(PRILVLW::LVL2)
    }
    #[doc = "Channel Priority Level 3"]
    #[inline]
    pub fn lvl3(self) -> &'a mut W {
        self.variant(PRILVLW::LVL3)
    }
    #[doc = "Channel Priority Level 4"]
    #[inline]
    pub fn lvl4(self) -> &'a mut W {
        self.variant(PRILVLW::LVL4)
    }
    #[doc = "Channel Priority Level 5"]
    #[inline]
    pub fn lvl5(self) -> &'a mut W {
        self.variant(PRILVLW::LVL5)
    }
    #[doc = "Channel Priority Level 6"]
    #[inline]
    pub fn lvl6(self) -> &'a mut W {
        self.variant(PRILVLW::LVL6)
    }
    #[doc = "Channel Priority Level 7 (Highest Level)"]
    #[inline]
    pub fn lvl7(self) -> &'a mut W {
        self.variant(PRILVLW::LVL7)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(0x03 << 0);
        self.w.bits |= ((value as u8) & 0x03) << 0;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
    #[doc = "Bits 0:1 - Channel Priority Level"]
    #[inline]
    pub fn prilvl(&self) -> PRILVLR {
        PRILVLR::_from(((self.bits >> 0) & 0x03) as u8)
    }
}
impl W {
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:1 - Channel Priority Level"]
    #[inline]
    pub fn prilvl(&mut self) -> _PRILVLW {
        _PRILVLW { w: self }
    }
}
