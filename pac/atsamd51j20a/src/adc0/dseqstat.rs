#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::DSEQSTAT {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct INPUTCTRLR {
    bits: bool,
}
impl INPUTCTRLR {
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
pub struct CTRLBR {
    bits: bool,
}
impl CTRLBR {
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
pub struct REFCTRLR {
    bits: bool,
}
impl REFCTRLR {
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
pub struct AVGCTRLR {
    bits: bool,
}
impl AVGCTRLR {
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
pub struct SAMPCTRLR {
    bits: bool,
}
impl SAMPCTRLR {
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
pub struct WINLTR {
    bits: bool,
}
impl WINLTR {
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
pub struct WINUTR {
    bits: bool,
}
impl WINUTR {
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
pub struct GAINCORRR {
    bits: bool,
}
impl GAINCORRR {
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
pub struct OFFSETCORRR {
    bits: bool,
}
impl OFFSETCORRR {
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
pub struct BUSYR {
    bits: bool,
}
impl BUSYR {
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
    #[doc = "Bit 0 - Input Control"]
    #[inline]
    pub fn inputctrl(&self) -> INPUTCTRLR {
        let bits = ((self.bits >> 0) & 0x01) != 0;
        INPUTCTRLR { bits }
    }
    #[doc = "Bit 1 - Control B"]
    #[inline]
    pub fn ctrlb(&self) -> CTRLBR {
        let bits = ((self.bits >> 1) & 0x01) != 0;
        CTRLBR { bits }
    }
    #[doc = "Bit 2 - Reference Control"]
    #[inline]
    pub fn refctrl(&self) -> REFCTRLR {
        let bits = ((self.bits >> 2) & 0x01) != 0;
        REFCTRLR { bits }
    }
    #[doc = "Bit 3 - Average Control"]
    #[inline]
    pub fn avgctrl(&self) -> AVGCTRLR {
        let bits = ((self.bits >> 3) & 0x01) != 0;
        AVGCTRLR { bits }
    }
    #[doc = "Bit 4 - Sampling Time Control"]
    #[inline]
    pub fn sampctrl(&self) -> SAMPCTRLR {
        let bits = ((self.bits >> 4) & 0x01) != 0;
        SAMPCTRLR { bits }
    }
    #[doc = "Bit 5 - Window Monitor Lower Threshold"]
    #[inline]
    pub fn winlt(&self) -> WINLTR {
        let bits = ((self.bits >> 5) & 0x01) != 0;
        WINLTR { bits }
    }
    #[doc = "Bit 6 - Window Monitor Upper Threshold"]
    #[inline]
    pub fn winut(&self) -> WINUTR {
        let bits = ((self.bits >> 6) & 0x01) != 0;
        WINUTR { bits }
    }
    #[doc = "Bit 7 - Gain Correction"]
    #[inline]
    pub fn gaincorr(&self) -> GAINCORRR {
        let bits = ((self.bits >> 7) & 0x01) != 0;
        GAINCORRR { bits }
    }
    #[doc = "Bit 8 - Offset Correction"]
    #[inline]
    pub fn offsetcorr(&self) -> OFFSETCORRR {
        let bits = ((self.bits >> 8) & 0x01) != 0;
        OFFSETCORRR { bits }
    }
    #[doc = "Bit 31 - DMA Sequencing Busy"]
    #[inline]
    pub fn busy(&self) -> BUSYR {
        let bits = ((self.bits >> 31) & 0x01) != 0;
        BUSYR { bits }
    }
}
