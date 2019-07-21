#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::STATUSC {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct TCC2_R {
    bits: bool,
}
impl TCC2_R {
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
pub struct PDEC_R {
    bits: bool,
}
impl PDEC_R {
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
pub struct AC_R {
    bits: bool,
}
impl AC_R {
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
pub struct AES_R {
    bits: bool,
}
impl AES_R {
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
pub struct TRNG_R {
    bits: bool,
}
impl TRNG_R {
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
pub struct ICM_R {
    bits: bool,
}
impl ICM_R {
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
pub struct PUKCC_R {
    bits: bool,
}
impl PUKCC_R {
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
pub struct QSPI_R {
    bits: bool,
}
impl QSPI_R {
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
pub struct CCL_R {
    bits: bool,
}
impl CCL_R {
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
    #[doc = "Bit 3 - TCC2 APB Protect Enable"]
    #[inline]
    pub fn tcc2_(&self) -> TCC2_R {
        let bits = ((self.bits >> 3) & 0x01) != 0;
        TCC2_R { bits }
    }
    #[doc = "Bit 7 - PDEC APB Protect Enable"]
    #[inline]
    pub fn pdec_(&self) -> PDEC_R {
        let bits = ((self.bits >> 7) & 0x01) != 0;
        PDEC_R { bits }
    }
    #[doc = "Bit 8 - AC APB Protect Enable"]
    #[inline]
    pub fn ac_(&self) -> AC_R {
        let bits = ((self.bits >> 8) & 0x01) != 0;
        AC_R { bits }
    }
    #[doc = "Bit 9 - AES APB Protect Enable"]
    #[inline]
    pub fn aes_(&self) -> AES_R {
        let bits = ((self.bits >> 9) & 0x01) != 0;
        AES_R { bits }
    }
    #[doc = "Bit 10 - TRNG APB Protect Enable"]
    #[inline]
    pub fn trng_(&self) -> TRNG_R {
        let bits = ((self.bits >> 10) & 0x01) != 0;
        TRNG_R { bits }
    }
    #[doc = "Bit 11 - ICM APB Protect Enable"]
    #[inline]
    pub fn icm_(&self) -> ICM_R {
        let bits = ((self.bits >> 11) & 0x01) != 0;
        ICM_R { bits }
    }
    #[doc = "Bit 12 - PUKCC APB Protect Enable"]
    #[inline]
    pub fn pukcc_(&self) -> PUKCC_R {
        let bits = ((self.bits >> 12) & 0x01) != 0;
        PUKCC_R { bits }
    }
    #[doc = "Bit 13 - QSPI APB Protect Enable"]
    #[inline]
    pub fn qspi_(&self) -> QSPI_R {
        let bits = ((self.bits >> 13) & 0x01) != 0;
        QSPI_R { bits }
    }
    #[doc = "Bit 14 - CCL APB Protect Enable"]
    #[inline]
    pub fn ccl_(&self) -> CCL_R {
        let bits = ((self.bits >> 14) & 0x01) != 0;
        CCL_R { bits }
    }
}
