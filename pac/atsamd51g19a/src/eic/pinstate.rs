#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::PINSTATE {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct PINSTATER {
    bits: u16,
}
impl PINSTATER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:15 - Pin State"]
    #[inline]
    pub fn pinstate(&self) -> PINSTATER {
        let bits = ((self.bits >> 0) & 0xffff) as u16;
        PINSTATER { bits }
    }
}
