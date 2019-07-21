#[doc = r" Value read from the register"]
pub struct R {
    bits: u16,
}
impl super::RESS {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct RESSR {
    bits: u16,
}
impl RESSR {
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
    #[doc = "Bits 0:15 - Last ADC conversion result"]
    #[inline]
    pub fn ress(&self) -> RESSR {
        let bits = ((self.bits >> 0) & 0xffff) as u16;
        RESSR { bits }
    }
}
