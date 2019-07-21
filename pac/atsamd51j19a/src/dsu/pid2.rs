#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::PID2 {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct JEPIDCHR {
    bits: u8,
}
impl JEPIDCHR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct JEPUR {
    bits: bool,
}
impl JEPUR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct REVISIONR {
    bits: u8,
}
impl REVISIONR {
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
    #[doc = "Bits 0:2 - JEP-106 Identity Code High"]
    #[inline]
    pub fn jepidch(&self) -> JEPIDCHR {
        let bits = ((self.bits >> 0) & 0x07) as u8;
        JEPIDCHR { bits }
    }
    #[doc = "Bit 3 - JEP-106 Identity Code is used"]
    #[inline]
    pub fn jepu(&self) -> JEPUR {
        let bits = ((self.bits >> 3) & 0x01) != 0;
        JEPUR { bits }
    }
    #[doc = "Bits 4:7 - Revision Number"]
    #[inline]
    pub fn revision(&self) -> REVISIONR {
        let bits = ((self.bits >> 4) & 0x0f) as u8;
        REVISIONR { bits }
    }
}
