#[doc = r" Value read from the register"]
pub struct R {
    bits: u16,
}
impl super::LOCK {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct LOCKR {
    bits: u16,
}
impl LOCKR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
    #[doc = "Bits 0:15 - Region Lock Bits"]
    #[inline]
    pub fn lock(&self) -> LOCKR {
        let bits = ((self.bits >> 0) & 0xffff) as u16;
        LOCKR { bits }
    }
}
