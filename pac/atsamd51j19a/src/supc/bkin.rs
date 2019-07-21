#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::BKIN {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct BKINR {
    bits: u8,
}
impl BKINR {
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
    #[doc = "Bits 0:7 - Backup Input Value"]
    #[inline]
    pub fn bkin(&self) -> BKINR {
        let bits = ((self.bits >> 0) & 0xff) as u8;
        BKINR { bits }
    }
}
