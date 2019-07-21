#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::CID1 {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct PREAMBLER {
    bits: u8,
}
impl PREAMBLER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CCLASSR {
    bits: u8,
}
impl CCLASSR {
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
    #[doc = "Bits 0:3 - Preamble"]
    #[inline]
    pub fn preamble(&self) -> PREAMBLER {
        let bits = ((self.bits >> 0) & 0x0f) as u8;
        PREAMBLER { bits }
    }
    #[doc = "Bits 4:7 - Component Class"]
    #[inline]
    pub fn cclass(&self) -> CCLASSR {
        let bits = ((self.bits >> 4) & 0x0f) as u8;
        CCLASSR { bits }
    }
}
