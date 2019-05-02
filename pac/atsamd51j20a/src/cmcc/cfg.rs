#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CFG {
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
pub struct ICDISR {
    bits: bool,
}
impl ICDISR {
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
pub struct DCDISR {
    bits: bool,
}
impl DCDISR {
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
#[doc = "Possible values of the field `CSIZESW`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CSIZESWR {
    #[doc = "the Cache Size is configured to 1KB"]
    CONF_CSIZE_1KB,
    #[doc = "the Cache Size is configured to 2KB"]
    CONF_CSIZE_2KB,
    #[doc = "the Cache Size is configured to 4KB"]
    CONF_CSIZE_4KB,
    #[doc = "the Cache Size is configured to 8KB"]
    CONF_CSIZE_8KB,
    #[doc = "the Cache Size is configured to 16KB"]
    CONF_CSIZE_16KB,
    #[doc = "the Cache Size is configured to 32KB"]
    CONF_CSIZE_32KB,
    #[doc = "the Cache Size is configured to 64KB"]
    CONF_CSIZE_64KB,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl CSIZESWR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CSIZESWR::CONF_CSIZE_1KB => 0,
            CSIZESWR::CONF_CSIZE_2KB => 1,
            CSIZESWR::CONF_CSIZE_4KB => 2,
            CSIZESWR::CONF_CSIZE_8KB => 3,
            CSIZESWR::CONF_CSIZE_16KB => 4,
            CSIZESWR::CONF_CSIZE_32KB => 5,
            CSIZESWR::CONF_CSIZE_64KB => 6,
            CSIZESWR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CSIZESWR {
        match value {
            0 => CSIZESWR::CONF_CSIZE_1KB,
            1 => CSIZESWR::CONF_CSIZE_2KB,
            2 => CSIZESWR::CONF_CSIZE_4KB,
            3 => CSIZESWR::CONF_CSIZE_8KB,
            4 => CSIZESWR::CONF_CSIZE_16KB,
            5 => CSIZESWR::CONF_CSIZE_32KB,
            6 => CSIZESWR::CONF_CSIZE_64KB,
            i => CSIZESWR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `CONF_CSIZE_1KB`"]
    #[inline]
    pub fn is_conf_csize_1kb(&self) -> bool {
        *self == CSIZESWR::CONF_CSIZE_1KB
    }
    #[doc = "Checks if the value of the field is `CONF_CSIZE_2KB`"]
    #[inline]
    pub fn is_conf_csize_2kb(&self) -> bool {
        *self == CSIZESWR::CONF_CSIZE_2KB
    }
    #[doc = "Checks if the value of the field is `CONF_CSIZE_4KB`"]
    #[inline]
    pub fn is_conf_csize_4kb(&self) -> bool {
        *self == CSIZESWR::CONF_CSIZE_4KB
    }
    #[doc = "Checks if the value of the field is `CONF_CSIZE_8KB`"]
    #[inline]
    pub fn is_conf_csize_8kb(&self) -> bool {
        *self == CSIZESWR::CONF_CSIZE_8KB
    }
    #[doc = "Checks if the value of the field is `CONF_CSIZE_16KB`"]
    #[inline]
    pub fn is_conf_csize_16kb(&self) -> bool {
        *self == CSIZESWR::CONF_CSIZE_16KB
    }
    #[doc = "Checks if the value of the field is `CONF_CSIZE_32KB`"]
    #[inline]
    pub fn is_conf_csize_32kb(&self) -> bool {
        *self == CSIZESWR::CONF_CSIZE_32KB
    }
    #[doc = "Checks if the value of the field is `CONF_CSIZE_64KB`"]
    #[inline]
    pub fn is_conf_csize_64kb(&self) -> bool {
        *self == CSIZESWR::CONF_CSIZE_64KB
    }
}
#[doc = r" Proxy"]
pub struct _ICDISW<'a> {
    w: &'a mut W,
}
impl<'a> _ICDISW<'a> {
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
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DCDISW<'a> {
    w: &'a mut W,
}
impl<'a> _DCDISW<'a> {
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
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CSIZESW`"]
pub enum CSIZESWW {
    #[doc = "the Cache Size is configured to 1KB"]
    CONF_CSIZE_1KB,
    #[doc = "the Cache Size is configured to 2KB"]
    CONF_CSIZE_2KB,
    #[doc = "the Cache Size is configured to 4KB"]
    CONF_CSIZE_4KB,
    #[doc = "the Cache Size is configured to 8KB"]
    CONF_CSIZE_8KB,
    #[doc = "the Cache Size is configured to 16KB"]
    CONF_CSIZE_16KB,
    #[doc = "the Cache Size is configured to 32KB"]
    CONF_CSIZE_32KB,
    #[doc = "the Cache Size is configured to 64KB"]
    CONF_CSIZE_64KB,
}
impl CSIZESWW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CSIZESWW::CONF_CSIZE_1KB => 0,
            CSIZESWW::CONF_CSIZE_2KB => 1,
            CSIZESWW::CONF_CSIZE_4KB => 2,
            CSIZESWW::CONF_CSIZE_8KB => 3,
            CSIZESWW::CONF_CSIZE_16KB => 4,
            CSIZESWW::CONF_CSIZE_32KB => 5,
            CSIZESWW::CONF_CSIZE_64KB => 6,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CSIZESWW<'a> {
    w: &'a mut W,
}
impl<'a> _CSIZESWW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CSIZESWW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "the Cache Size is configured to 1KB"]
    #[inline]
    pub fn conf_csize_1kb(self) -> &'a mut W {
        self.variant(CSIZESWW::CONF_CSIZE_1KB)
    }
    #[doc = "the Cache Size is configured to 2KB"]
    #[inline]
    pub fn conf_csize_2kb(self) -> &'a mut W {
        self.variant(CSIZESWW::CONF_CSIZE_2KB)
    }
    #[doc = "the Cache Size is configured to 4KB"]
    #[inline]
    pub fn conf_csize_4kb(self) -> &'a mut W {
        self.variant(CSIZESWW::CONF_CSIZE_4KB)
    }
    #[doc = "the Cache Size is configured to 8KB"]
    #[inline]
    pub fn conf_csize_8kb(self) -> &'a mut W {
        self.variant(CSIZESWW::CONF_CSIZE_8KB)
    }
    #[doc = "the Cache Size is configured to 16KB"]
    #[inline]
    pub fn conf_csize_16kb(self) -> &'a mut W {
        self.variant(CSIZESWW::CONF_CSIZE_16KB)
    }
    #[doc = "the Cache Size is configured to 32KB"]
    #[inline]
    pub fn conf_csize_32kb(self) -> &'a mut W {
        self.variant(CSIZESWW::CONF_CSIZE_32KB)
    }
    #[doc = "the Cache Size is configured to 64KB"]
    #[inline]
    pub fn conf_csize_64kb(self) -> &'a mut W {
        self.variant(CSIZESWW::CONF_CSIZE_64KB)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 4;
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
    #[doc = "Bit 1 - Instruction Cache Disable"]
    #[inline]
    pub fn icdis(&self) -> ICDISR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ICDISR { bits }
    }
    #[doc = "Bit 2 - Data Cache Disable"]
    #[inline]
    pub fn dcdis(&self) -> DCDISR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DCDISR { bits }
    }
    #[doc = "Bits 4:6 - Cache size configured by software"]
    #[inline]
    pub fn csizesw(&self) -> CSIZESWR {
        CSIZESWR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 32 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 1 - Instruction Cache Disable"]
    #[inline]
    pub fn icdis(&mut self) -> _ICDISW {
        _ICDISW { w: self }
    }
    #[doc = "Bit 2 - Data Cache Disable"]
    #[inline]
    pub fn dcdis(&mut self) -> _DCDISW {
        _DCDISW { w: self }
    }
    #[doc = "Bits 4:6 - Cache size configured by software"]
    #[inline]
    pub fn csizesw(&mut self) -> _CSIZESWW {
        _CSIZESWW { w: self }
    }
}
