#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::PID3 {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct CUSMODR {
    bits: u8,
}
impl CUSMODR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct REVANDR {
    bits: u8,
}
impl REVANDR {
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
    #[doc = "Bits 0:3 - ARM CUSMOD"]
    #[inline]
    pub fn cusmod(&self) -> CUSMODR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CUSMODR { bits }
    }
    #[doc = "Bits 4:7 - Revision Number"]
    #[inline]
    pub fn revand(&self) -> REVANDR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        REVANDR { bits }
    }
}
