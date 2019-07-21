#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::RR {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct CMDRESPR {
    bits: u32,
}
impl CMDRESPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:31 - Command Response"]
    #[inline]
    pub fn cmdresp(&self) -> CMDRESPR {
        let bits = ((self.bits >> 0) & 0xffff_ffff) as u32;
        CMDRESPR { bits }
    }
}
