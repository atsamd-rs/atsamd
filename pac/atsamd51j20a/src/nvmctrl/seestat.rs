#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::SEESTAT {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct ASEESR {
    bits: bool,
}
impl ASEESR {
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
pub struct LOADR {
    bits: bool,
}
impl LOADR {
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
pub struct BUSYR {
    bits: bool,
}
impl BUSYR {
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
pub struct LOCKR {
    bits: bool,
}
impl LOCKR {
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
pub struct RLOCKR {
    bits: bool,
}
impl RLOCKR {
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
pub struct SBLKR {
    bits: u8,
}
impl SBLKR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PSZR {
    bits: u8,
}
impl PSZR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Active SmartEEPROM Sector"]
    #[inline]
    pub fn asees(&self) -> ASEESR {
        let bits = ((self.bits >> 0) & 0x01) != 0;
        ASEESR { bits }
    }
    #[doc = "Bit 1 - Page Buffer Loaded"]
    #[inline]
    pub fn load(&self) -> LOADR {
        let bits = ((self.bits >> 1) & 0x01) != 0;
        LOADR { bits }
    }
    #[doc = "Bit 2 - Busy"]
    #[inline]
    pub fn busy(&self) -> BUSYR {
        let bits = ((self.bits >> 2) & 0x01) != 0;
        BUSYR { bits }
    }
    #[doc = "Bit 3 - SmartEEPROM Write Access Is Locked"]
    #[inline]
    pub fn lock(&self) -> LOCKR {
        let bits = ((self.bits >> 3) & 0x01) != 0;
        LOCKR { bits }
    }
    #[doc = "Bit 4 - SmartEEPROM Write Access To Register Address Space Is Locked"]
    #[inline]
    pub fn rlock(&self) -> RLOCKR {
        let bits = ((self.bits >> 4) & 0x01) != 0;
        RLOCKR { bits }
    }
    #[doc = "Bits 8:11 - Blocks Number In a Sector"]
    #[inline]
    pub fn sblk(&self) -> SBLKR {
        let bits = ((self.bits >> 8) & 0x0f) as u8;
        SBLKR { bits }
    }
    #[doc = "Bits 16:18 - SmartEEPROM Page Size"]
    #[inline]
    pub fn psz(&self) -> PSZR {
        let bits = ((self.bits >> 16) & 0x07) as u8;
        PSZR { bits }
    }
}
