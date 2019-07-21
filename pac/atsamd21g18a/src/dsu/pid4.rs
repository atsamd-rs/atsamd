#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::PID4 {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct JEPCCR {
    bits: u8,
}
impl JEPCCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct FKBCR {
    bits: u8,
}
impl FKBCR {
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
    #[doc = "Bits 0:3 - JEP-106 Continuation Code"]
    #[inline]
    pub fn jepcc(&self) -> JEPCCR {
        let bits = ((self.bits >> 0) & 0x0f) as u8;
        JEPCCR { bits }
    }
    #[doc = "Bits 4:7 - 4KB Count"]
    #[inline]
    pub fn fkbc(&self) -> FKBCR {
        let bits = ((self.bits >> 4) & 0x0f) as u8;
        FKBCR { bits }
    }
}
