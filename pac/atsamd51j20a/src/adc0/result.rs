#[doc = r" Value read from the register"]
pub struct R {
    bits: u16,
}
impl super::RESULT {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct RESULTR {
    bits: u16,
}
impl RESULTR {
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
    #[doc = "Bits 0:15 - Result Conversion Value"]
    #[inline]
    pub fn result(&self) -> RESULTR {
        let bits = ((self.bits >> 0) & 0xffff) as u16;
        RESULTR { bits }
    }
}
