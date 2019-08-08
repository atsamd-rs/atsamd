#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::IMR {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct RHCR {
    bits: u8,
}
impl RHCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct RDMR {
    bits: u8,
}
impl RDMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct RBER {
    bits: u8,
}
impl RBER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct RWCR {
    bits: u8,
}
impl RWCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct RECR {
    bits: u8,
}
impl RECR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct RSUR {
    bits: u8,
}
impl RSUR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct URADR {
    bits: bool,
}
impl URADR {
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:3 - Region Hash Completed Interrupt Mask"]
    #[inline]
    pub fn rhc(&self) -> RHCR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RHCR { bits }
    }
    #[doc = "Bits 4:7 - Region Digest Mismatch Interrupt Mask"]
    #[inline]
    pub fn rdm(&self) -> RDMR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RDMR { bits }
    }
    #[doc = "Bits 8:11 - Region Bus Error Interrupt Mask"]
    #[inline]
    pub fn rbe(&self) -> RBER {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RBER { bits }
    }
    #[doc = "Bits 12:15 - Region Wrap Condition Detected Interrupt Mask"]
    #[inline]
    pub fn rwc(&self) -> RWCR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RWCR { bits }
    }
    #[doc = "Bits 16:19 - Region End bit Condition Detected Interrupt Mask"]
    #[inline]
    pub fn rec(&self) -> RECR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RECR { bits }
    }
    #[doc = "Bits 20:23 - Region Status Updated Interrupt Mask"]
    #[inline]
    pub fn rsu(&self) -> RSUR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RSUR { bits }
    }
    #[doc = "Bit 24 - Undefined Register Access Detection Interrupt Mask"]
    #[inline]
    pub fn urad(&self) -> URADR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        URADR { bits }
    }
}
