#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::TYPE {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct GCLKR {
    bits: bool,
}
impl GCLKR {
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
pub struct RRPR {
    bits: bool,
}
impl RRPR {
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
#[doc = "Possible values of the field `WAYNUM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WAYNUMR {
    #[doc = "Direct Mapped Cache"]
    DMAPPED,
    #[doc = "2-WAY set associative"]
    ARCH2WAY,
    #[doc = "4-WAY set associative"]
    ARCH4WAY,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl WAYNUMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            WAYNUMR::DMAPPED => 0,
            WAYNUMR::ARCH2WAY => 1,
            WAYNUMR::ARCH4WAY => 2,
            WAYNUMR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> WAYNUMR {
        match value {
            0 => WAYNUMR::DMAPPED,
            1 => WAYNUMR::ARCH2WAY,
            2 => WAYNUMR::ARCH4WAY,
            i => WAYNUMR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `DMAPPED`"]
    #[inline]
    pub fn is_dmapped(&self) -> bool {
        *self == WAYNUMR::DMAPPED
    }
    #[doc = "Checks if the value of the field is `ARCH2WAY`"]
    #[inline]
    pub fn is_arch2way(&self) -> bool {
        *self == WAYNUMR::ARCH2WAY
    }
    #[doc = "Checks if the value of the field is `ARCH4WAY`"]
    #[inline]
    pub fn is_arch4way(&self) -> bool {
        *self == WAYNUMR::ARCH4WAY
    }
}
#[doc = r" Value of the field"]
pub struct LCKDOWNR {
    bits: bool,
}
impl LCKDOWNR {
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
#[doc = "Possible values of the field `CSIZE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CSIZER {
    #[doc = "Cache Size is 1 KB"]
    CSIZE_1KB,
    #[doc = "Cache Size is 2 KB"]
    CSIZE_2KB,
    #[doc = "Cache Size is 4 KB"]
    CSIZE_4KB,
    #[doc = "Cache Size is 8 KB"]
    CSIZE_8KB,
    #[doc = "Cache Size is 16 KB"]
    CSIZE_16KB,
    #[doc = "Cache Size is 32 KB"]
    CSIZE_32KB,
    #[doc = "Cache Size is 64 KB"]
    CSIZE_64KB,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl CSIZER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CSIZER::CSIZE_1KB => 0,
            CSIZER::CSIZE_2KB => 1,
            CSIZER::CSIZE_4KB => 2,
            CSIZER::CSIZE_8KB => 3,
            CSIZER::CSIZE_16KB => 4,
            CSIZER::CSIZE_32KB => 5,
            CSIZER::CSIZE_64KB => 6,
            CSIZER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CSIZER {
        match value {
            0 => CSIZER::CSIZE_1KB,
            1 => CSIZER::CSIZE_2KB,
            2 => CSIZER::CSIZE_4KB,
            3 => CSIZER::CSIZE_8KB,
            4 => CSIZER::CSIZE_16KB,
            5 => CSIZER::CSIZE_32KB,
            6 => CSIZER::CSIZE_64KB,
            i => CSIZER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `CSIZE_1KB`"]
    #[inline]
    pub fn is_csize_1kb(&self) -> bool {
        *self == CSIZER::CSIZE_1KB
    }
    #[doc = "Checks if the value of the field is `CSIZE_2KB`"]
    #[inline]
    pub fn is_csize_2kb(&self) -> bool {
        *self == CSIZER::CSIZE_2KB
    }
    #[doc = "Checks if the value of the field is `CSIZE_4KB`"]
    #[inline]
    pub fn is_csize_4kb(&self) -> bool {
        *self == CSIZER::CSIZE_4KB
    }
    #[doc = "Checks if the value of the field is `CSIZE_8KB`"]
    #[inline]
    pub fn is_csize_8kb(&self) -> bool {
        *self == CSIZER::CSIZE_8KB
    }
    #[doc = "Checks if the value of the field is `CSIZE_16KB`"]
    #[inline]
    pub fn is_csize_16kb(&self) -> bool {
        *self == CSIZER::CSIZE_16KB
    }
    #[doc = "Checks if the value of the field is `CSIZE_32KB`"]
    #[inline]
    pub fn is_csize_32kb(&self) -> bool {
        *self == CSIZER::CSIZE_32KB
    }
    #[doc = "Checks if the value of the field is `CSIZE_64KB`"]
    #[inline]
    pub fn is_csize_64kb(&self) -> bool {
        *self == CSIZER::CSIZE_64KB
    }
}
#[doc = "Possible values of the field `CLSIZE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLSIZER {
    #[doc = "Cache Line Size is 4 bytes"]
    CLSIZE_4B,
    #[doc = "Cache Line Size is 8 bytes"]
    CLSIZE_8B,
    #[doc = "Cache Line Size is 16 bytes"]
    CLSIZE_16B,
    #[doc = "Cache Line Size is 32 bytes"]
    CLSIZE_32B,
    #[doc = "Cache Line Size is 64 bytes"]
    CLSIZE_64B,
    #[doc = "Cache Line Size is 128 bytes"]
    CLSIZE_128B,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl CLSIZER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CLSIZER::CLSIZE_4B => 0,
            CLSIZER::CLSIZE_8B => 1,
            CLSIZER::CLSIZE_16B => 2,
            CLSIZER::CLSIZE_32B => 3,
            CLSIZER::CLSIZE_64B => 4,
            CLSIZER::CLSIZE_128B => 5,
            CLSIZER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CLSIZER {
        match value {
            0 => CLSIZER::CLSIZE_4B,
            1 => CLSIZER::CLSIZE_8B,
            2 => CLSIZER::CLSIZE_16B,
            3 => CLSIZER::CLSIZE_32B,
            4 => CLSIZER::CLSIZE_64B,
            5 => CLSIZER::CLSIZE_128B,
            i => CLSIZER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `CLSIZE_4B`"]
    #[inline]
    pub fn is_clsize_4b(&self) -> bool {
        *self == CLSIZER::CLSIZE_4B
    }
    #[doc = "Checks if the value of the field is `CLSIZE_8B`"]
    #[inline]
    pub fn is_clsize_8b(&self) -> bool {
        *self == CLSIZER::CLSIZE_8B
    }
    #[doc = "Checks if the value of the field is `CLSIZE_16B`"]
    #[inline]
    pub fn is_clsize_16b(&self) -> bool {
        *self == CLSIZER::CLSIZE_16B
    }
    #[doc = "Checks if the value of the field is `CLSIZE_32B`"]
    #[inline]
    pub fn is_clsize_32b(&self) -> bool {
        *self == CLSIZER::CLSIZE_32B
    }
    #[doc = "Checks if the value of the field is `CLSIZE_64B`"]
    #[inline]
    pub fn is_clsize_64b(&self) -> bool {
        *self == CLSIZER::CLSIZE_64B
    }
    #[doc = "Checks if the value of the field is `CLSIZE_128B`"]
    #[inline]
    pub fn is_clsize_128b(&self) -> bool {
        *self == CLSIZER::CLSIZE_128B
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 1 - dynamic Clock Gating supported"]
    #[inline]
    pub fn gclk(&self) -> GCLKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        GCLKR { bits }
    }
    #[doc = "Bit 4 - Round Robin Policy supported"]
    #[inline]
    pub fn rrp(&self) -> RRPR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RRPR { bits }
    }
    #[doc = "Bits 5:6 - Number of Way"]
    #[inline]
    pub fn waynum(&self) -> WAYNUMR {
        WAYNUMR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 7 - Lock Down supported"]
    #[inline]
    pub fn lckdown(&self) -> LCKDOWNR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        LCKDOWNR { bits }
    }
    #[doc = "Bits 8:10 - Cache Size"]
    #[inline]
    pub fn csize(&self) -> CSIZER {
        CSIZER::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 11:13 - Cache Line Size"]
    #[inline]
    pub fn clsize(&self) -> CLSIZER {
        CLSIZER::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
