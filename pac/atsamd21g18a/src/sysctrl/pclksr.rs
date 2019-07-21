#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::PCLKSR {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct XOSCRDYR {
    bits: bool,
}
impl XOSCRDYR {
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
pub struct XOSC32KRDYR {
    bits: bool,
}
impl XOSC32KRDYR {
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
pub struct OSC32KRDYR {
    bits: bool,
}
impl OSC32KRDYR {
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
pub struct OSC8MRDYR {
    bits: bool,
}
impl OSC8MRDYR {
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
pub struct DFLLRDYR {
    bits: bool,
}
impl DFLLRDYR {
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
pub struct DFLLOOBR {
    bits: bool,
}
impl DFLLOOBR {
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
pub struct DFLLLCKFR {
    bits: bool,
}
impl DFLLLCKFR {
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
pub struct DFLLLCKCR {
    bits: bool,
}
impl DFLLLCKCR {
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
pub struct DFLLRCSR {
    bits: bool,
}
impl DFLLRCSR {
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
pub struct BOD33RDYR {
    bits: bool,
}
impl BOD33RDYR {
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
pub struct BOD33DETR {
    bits: bool,
}
impl BOD33DETR {
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
pub struct B33SRDYR {
    bits: bool,
}
impl B33SRDYR {
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
pub struct DPLLLCKRR {
    bits: bool,
}
impl DPLLLCKRR {
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
pub struct DPLLLCKFR {
    bits: bool,
}
impl DPLLLCKFR {
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
pub struct DPLLLTOR {
    bits: bool,
}
impl DPLLLTOR {
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
    #[doc = "Bit 0 - XOSC Ready"]
    #[inline]
    pub fn xoscrdy(&self) -> XOSCRDYR {
        let bits = ((self.bits >> 0) & 0x01) != 0;
        XOSCRDYR { bits }
    }
    #[doc = "Bit 1 - XOSC32K Ready"]
    #[inline]
    pub fn xosc32krdy(&self) -> XOSC32KRDYR {
        let bits = ((self.bits >> 1) & 0x01) != 0;
        XOSC32KRDYR { bits }
    }
    #[doc = "Bit 2 - OSC32K Ready"]
    #[inline]
    pub fn osc32krdy(&self) -> OSC32KRDYR {
        let bits = ((self.bits >> 2) & 0x01) != 0;
        OSC32KRDYR { bits }
    }
    #[doc = "Bit 3 - OSC8M Ready"]
    #[inline]
    pub fn osc8mrdy(&self) -> OSC8MRDYR {
        let bits = ((self.bits >> 3) & 0x01) != 0;
        OSC8MRDYR { bits }
    }
    #[doc = "Bit 4 - DFLL Ready"]
    #[inline]
    pub fn dfllrdy(&self) -> DFLLRDYR {
        let bits = ((self.bits >> 4) & 0x01) != 0;
        DFLLRDYR { bits }
    }
    #[doc = "Bit 5 - DFLL Out Of Bounds"]
    #[inline]
    pub fn dflloob(&self) -> DFLLOOBR {
        let bits = ((self.bits >> 5) & 0x01) != 0;
        DFLLOOBR { bits }
    }
    #[doc = "Bit 6 - DFLL Lock Fine"]
    #[inline]
    pub fn dflllckf(&self) -> DFLLLCKFR {
        let bits = ((self.bits >> 6) & 0x01) != 0;
        DFLLLCKFR { bits }
    }
    #[doc = "Bit 7 - DFLL Lock Coarse"]
    #[inline]
    pub fn dflllckc(&self) -> DFLLLCKCR {
        let bits = ((self.bits >> 7) & 0x01) != 0;
        DFLLLCKCR { bits }
    }
    #[doc = "Bit 8 - DFLL Reference Clock Stopped"]
    #[inline]
    pub fn dfllrcs(&self) -> DFLLRCSR {
        let bits = ((self.bits >> 8) & 0x01) != 0;
        DFLLRCSR { bits }
    }
    #[doc = "Bit 9 - BOD33 Ready"]
    #[inline]
    pub fn bod33rdy(&self) -> BOD33RDYR {
        let bits = ((self.bits >> 9) & 0x01) != 0;
        BOD33RDYR { bits }
    }
    #[doc = "Bit 10 - BOD33 Detection"]
    #[inline]
    pub fn bod33det(&self) -> BOD33DETR {
        let bits = ((self.bits >> 10) & 0x01) != 0;
        BOD33DETR { bits }
    }
    #[doc = "Bit 11 - BOD33 Synchronization Ready"]
    #[inline]
    pub fn b33srdy(&self) -> B33SRDYR {
        let bits = ((self.bits >> 11) & 0x01) != 0;
        B33SRDYR { bits }
    }
    #[doc = "Bit 15 - DPLL Lock Rise"]
    #[inline]
    pub fn dplllckr(&self) -> DPLLLCKRR {
        let bits = ((self.bits >> 15) & 0x01) != 0;
        DPLLLCKRR { bits }
    }
    #[doc = "Bit 16 - DPLL Lock Fall"]
    #[inline]
    pub fn dplllckf(&self) -> DPLLLCKFR {
        let bits = ((self.bits >> 16) & 0x01) != 0;
        DPLLLCKFR { bits }
    }
    #[doc = "Bit 17 - DPLL Lock Timeout"]
    #[inline]
    pub fn dplllto(&self) -> DPLLLTOR {
        let bits = ((self.bits >> 17) & 0x01) != 0;
        DPLLLTOR { bits }
    }
}
