#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
impl super::BKUPEXIT {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct RTCR {
    bits: bool,
}
impl RTCR {
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
pub struct BBPSR {
    bits: bool,
}
impl BBPSR {
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
pub struct HIBR {
    bits: bool,
}
impl HIBR {
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
    #[doc = "Bit 1 - Real Timer Counter Interrupt"]
    #[inline]
    pub fn rtc(&self) -> RTCR {
        let bits = ((self.bits >> 1) & 0x01) != 0;
        RTCR { bits }
    }
    #[doc = "Bit 2 - Battery Backup Power Switch"]
    #[inline]
    pub fn bbps(&self) -> BBPSR {
        let bits = ((self.bits >> 2) & 0x01) != 0;
        BBPSR { bits }
    }
    #[doc = "Bit 7 - Hibernate"]
    #[inline]
    pub fn hib(&self) -> HIBR {
        let bits = ((self.bits >> 7) & 0x01) != 0;
        HIBR { bits }
    }
}
