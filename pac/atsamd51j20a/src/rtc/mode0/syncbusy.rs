#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::SYNCBUSY {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct SWRSTR {
    bits: bool,
}
impl SWRSTR {
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
pub struct ENABLER {
    bits: bool,
}
impl ENABLER {
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
pub struct FREQCORRR {
    bits: bool,
}
impl FREQCORRR {
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
pub struct COUNTR {
    bits: bool,
}
impl COUNTR {
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
pub struct COMP0R {
    bits: bool,
}
impl COMP0R {
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
pub struct COMP1R {
    bits: bool,
}
impl COMP1R {
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
pub struct COUNTSYNCR {
    bits: bool,
}
impl COUNTSYNCR {
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
pub struct GP0R {
    bits: bool,
}
impl GP0R {
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
pub struct GP1R {
    bits: bool,
}
impl GP1R {
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
pub struct GP2R {
    bits: bool,
}
impl GP2R {
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
pub struct GP3R {
    bits: bool,
}
impl GP3R {
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
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Software Reset Busy"]
    #[inline]
    pub fn swrst(&self) -> SWRSTR {
        let bits = ((self.bits >> 0) & 0x01) != 0;
        SWRSTR { bits }
    }
    #[doc = "Bit 1 - Enable Bit Busy"]
    #[inline]
    pub fn enable(&self) -> ENABLER {
        let bits = ((self.bits >> 1) & 0x01) != 0;
        ENABLER { bits }
    }
    #[doc = "Bit 2 - FREQCORR Register Busy"]
    #[inline]
    pub fn freqcorr(&self) -> FREQCORRR {
        let bits = ((self.bits >> 2) & 0x01) != 0;
        FREQCORRR { bits }
    }
    #[doc = "Bit 3 - COUNT Register Busy"]
    #[inline]
    pub fn count(&self) -> COUNTR {
        let bits = ((self.bits >> 3) & 0x01) != 0;
        COUNTR { bits }
    }
    #[doc = "Bit 5 - COMP 0 Register Busy"]
    #[inline]
    pub fn comp0(&self) -> COMP0R {
        let bits = ((self.bits >> 5) & 0x01) != 0;
        COMP0R { bits }
    }
    #[doc = "Bit 6 - COMP 1 Register Busy"]
    #[inline]
    pub fn comp1(&self) -> COMP1R {
        let bits = ((self.bits >> 6) & 0x01) != 0;
        COMP1R { bits }
    }
    #[doc = "Bit 15 - Count Synchronization Enable Bit Busy"]
    #[inline]
    pub fn countsync(&self) -> COUNTSYNCR {
        let bits = ((self.bits >> 15) & 0x01) != 0;
        COUNTSYNCR { bits }
    }
    #[doc = "Bit 16 - General Purpose 0 Register Busy"]
    #[inline]
    pub fn gp0(&self) -> GP0R {
        let bits = ((self.bits >> 16) & 0x01) != 0;
        GP0R { bits }
    }
    #[doc = "Bit 17 - General Purpose 1 Register Busy"]
    #[inline]
    pub fn gp1(&self) -> GP1R {
        let bits = ((self.bits >> 17) & 0x01) != 0;
        GP1R { bits }
    }
    #[doc = "Bit 18 - General Purpose 2 Register Busy"]
    #[inline]
    pub fn gp2(&self) -> GP2R {
        let bits = ((self.bits >> 18) & 0x01) != 0;
        GP2R { bits }
    }
    #[doc = "Bit 19 - General Purpose 3 Register Busy"]
    #[inline]
    pub fn gp3(&self) -> GP3R {
        let bits = ((self.bits >> 19) & 0x01) != 0;
        GP3R { bits }
    }
}
