#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::PID1 {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct PARTNBHR {
    bits: u8,
}
impl PARTNBHR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct JEPIDCLR {
    bits: u8,
}
impl JEPIDCLR {
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
    #[doc = "Bits 0:3 - Part Number High"]
    #[inline]
    pub fn partnbh(&self) -> PARTNBHR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PARTNBHR { bits }
    }
    #[doc = "Bits 4:7 - Low part of the JEP-106 Identity Code"]
    #[inline]
    pub fn jepidcl(&self) -> JEPIDCLR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        JEPIDCLR { bits }
    }
}
