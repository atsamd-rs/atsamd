#[doc = r" Value read from the register"]
pub struct R {
    bits: u16,
}
impl super::STATUS {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct READYR {
    bits: bool,
}
impl READYR {
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
pub struct PRMR {
    bits: bool,
}
impl PRMR {
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
pub struct SUSPR {
    bits: bool,
}
impl SUSPR {
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
pub struct AFIRSTR {
    bits: bool,
}
impl AFIRSTR {
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
pub struct BPDISR {
    bits: bool,
}
impl BPDISR {
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
pub struct BOOTPROTR {
    bits: u8,
}
impl BOOTPROTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
    #[doc = "Bit 0 - Ready to accept a command"]
    #[inline]
    pub fn ready(&self) -> READYR {
        let bits = ((self.bits >> 0) & 0x01) != 0;
        READYR { bits }
    }
    #[doc = "Bit 1 - Power Reduction Mode"]
    #[inline]
    pub fn prm(&self) -> PRMR {
        let bits = ((self.bits >> 1) & 0x01) != 0;
        PRMR { bits }
    }
    #[doc = "Bit 2 - NVM Page Buffer Active Loading"]
    #[inline]
    pub fn load(&self) -> LOADR {
        let bits = ((self.bits >> 2) & 0x01) != 0;
        LOADR { bits }
    }
    #[doc = "Bit 3 - NVM Write Or Erase Operation Is Suspended"]
    #[inline]
    pub fn susp(&self) -> SUSPR {
        let bits = ((self.bits >> 3) & 0x01) != 0;
        SUSPR { bits }
    }
    #[doc = "Bit 4 - BANKA First"]
    #[inline]
    pub fn afirst(&self) -> AFIRSTR {
        let bits = ((self.bits >> 4) & 0x01) != 0;
        AFIRSTR { bits }
    }
    #[doc = "Bit 5 - Boot Loader Protection Disable"]
    #[inline]
    pub fn bpdis(&self) -> BPDISR {
        let bits = ((self.bits >> 5) & 0x01) != 0;
        BPDISR { bits }
    }
    #[doc = "Bits 8:11 - Boot Loader Protection Size"]
    #[inline]
    pub fn bootprot(&self) -> BOOTPROTR {
        let bits = ((self.bits >> 8) & 0x0f) as u8;
        BOOTPROTR { bits }
    }
}
