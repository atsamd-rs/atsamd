#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::INTFLAGC {
    #[doc = r" Modifies the contents of the register"]
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        self.register.set(f(&R { bits }, &mut W { bits }).bits);
    }
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
    #[doc = r" Writes to the register"]
    #[inline]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        self.register.set(
            f(&mut W {
                bits: Self::reset_value(),
            })
            .bits,
        );
    }
    #[doc = r" Reset value of the register"]
    #[inline]
    pub const fn reset_value() -> u32 {
        0
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.register.set(Self::reset_value())
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
pub struct TCC3_R {
    bits: bool,
}
impl TCC3_R {
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
pub struct TC4_R {
    bits: bool,
}
impl TC4_R {
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
pub struct TC5_R {
    bits: bool,
}
impl TC5_R {
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
#[doc = r" Proxy"]
pub struct _TCC2_W<'a> {
    w: &'a mut W,
}
impl<'a> _TCC2_W<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits &= !(0x01 << 3);
        self.w.bits |= ((value as u32) & 0x01) << 3;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TCC3_W<'a> {
    w: &'a mut W,
}
impl<'a> _TCC3_W<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits &= !(0x01 << 4);
        self.w.bits |= ((value as u32) & 0x01) << 4;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TC4_W<'a> {
    w: &'a mut W,
}
impl<'a> _TC4_W<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits &= !(0x01 << 5);
        self.w.bits |= ((value as u32) & 0x01) << 5;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TC5_W<'a> {
    w: &'a mut W,
}
impl<'a> _TC5_W<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits &= !(0x01 << 6);
        self.w.bits |= ((value as u32) & 0x01) << 6;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PDEC_W<'a> {
    w: &'a mut W,
}
impl<'a> _PDEC_W<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits &= !(0x01 << 7);
        self.w.bits |= ((value as u32) & 0x01) << 7;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _AC_W<'a> {
    w: &'a mut W,
}
impl<'a> _AC_W<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits &= !(0x01 << 8);
        self.w.bits |= ((value as u32) & 0x01) << 8;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _AES_W<'a> {
    w: &'a mut W,
}
impl<'a> _AES_W<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits &= !(0x01 << 9);
        self.w.bits |= ((value as u32) & 0x01) << 9;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TRNG_W<'a> {
    w: &'a mut W,
}
impl<'a> _TRNG_W<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits &= !(0x01 << 10);
        self.w.bits |= ((value as u32) & 0x01) << 10;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ICM_W<'a> {
    w: &'a mut W,
}
impl<'a> _ICM_W<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits &= !(0x01 << 11);
        self.w.bits |= ((value as u32) & 0x01) << 11;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PUKCC_W<'a> {
    w: &'a mut W,
}
impl<'a> _PUKCC_W<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits &= !(0x01 << 12);
        self.w.bits |= ((value as u32) & 0x01) << 12;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _QSPI_W<'a> {
    w: &'a mut W,
}
impl<'a> _QSPI_W<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits &= !(0x01 << 13);
        self.w.bits |= ((value as u32) & 0x01) << 13;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CCL_W<'a> {
    w: &'a mut W,
}
impl<'a> _CCL_W<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits &= !(0x01 << 14);
        self.w.bits |= ((value as u32) & 0x01) << 14;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 3 - TCC2"]
    #[inline]
    pub fn tcc2_(&self) -> TCC2_R {
        let bits = ((self.bits >> 3) & 0x01) != 0;
        TCC2_R { bits }
    }
    #[doc = "Bit 4 - TCC3"]
    #[inline]
    pub fn tcc3_(&self) -> TCC3_R {
        let bits = ((self.bits >> 4) & 0x01) != 0;
        TCC3_R { bits }
    }
    #[doc = "Bit 5 - TC4"]
    #[inline]
    pub fn tc4_(&self) -> TC4_R {
        let bits = ((self.bits >> 5) & 0x01) != 0;
        TC4_R { bits }
    }
    #[doc = "Bit 6 - TC5"]
    #[inline]
    pub fn tc5_(&self) -> TC5_R {
        let bits = ((self.bits >> 6) & 0x01) != 0;
        TC5_R { bits }
    }
    #[doc = "Bit 7 - PDEC"]
    #[inline]
    pub fn pdec_(&self) -> PDEC_R {
        let bits = ((self.bits >> 7) & 0x01) != 0;
        PDEC_R { bits }
    }
    #[doc = "Bit 8 - AC"]
    #[inline]
    pub fn ac_(&self) -> AC_R {
        let bits = ((self.bits >> 8) & 0x01) != 0;
        AC_R { bits }
    }
    #[doc = "Bit 9 - AES"]
    #[inline]
    pub fn aes_(&self) -> AES_R {
        let bits = ((self.bits >> 9) & 0x01) != 0;
        AES_R { bits }
    }
    #[doc = "Bit 10 - TRNG"]
    #[inline]
    pub fn trng_(&self) -> TRNG_R {
        let bits = ((self.bits >> 10) & 0x01) != 0;
        TRNG_R { bits }
    }
    #[doc = "Bit 11 - ICM"]
    #[inline]
    pub fn icm_(&self) -> ICM_R {
        let bits = ((self.bits >> 11) & 0x01) != 0;
        ICM_R { bits }
    }
    #[doc = "Bit 12 - PUKCC"]
    #[inline]
    pub fn pukcc_(&self) -> PUKCC_R {
        let bits = ((self.bits >> 12) & 0x01) != 0;
        PUKCC_R { bits }
    }
    #[doc = "Bit 13 - QSPI"]
    #[inline]
    pub fn qspi_(&self) -> QSPI_R {
        let bits = ((self.bits >> 13) & 0x01) != 0;
        QSPI_R { bits }
    }
    #[doc = "Bit 14 - CCL"]
    #[inline]
    pub fn ccl_(&self) -> CCL_R {
        let bits = ((self.bits >> 14) & 0x01) != 0;
        CCL_R { bits }
    }
}
impl W {
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 3 - TCC2"]
    #[inline]
    pub fn tcc2_(&mut self) -> _TCC2_W {
        _TCC2_W { w: self }
    }
    #[doc = "Bit 4 - TCC3"]
    #[inline]
    pub fn tcc3_(&mut self) -> _TCC3_W {
        _TCC3_W { w: self }
    }
    #[doc = "Bit 5 - TC4"]
    #[inline]
    pub fn tc4_(&mut self) -> _TC4_W {
        _TC4_W { w: self }
    }
    #[doc = "Bit 6 - TC5"]
    #[inline]
    pub fn tc5_(&mut self) -> _TC5_W {
        _TC5_W { w: self }
    }
    #[doc = "Bit 7 - PDEC"]
    #[inline]
    pub fn pdec_(&mut self) -> _PDEC_W {
        _PDEC_W { w: self }
    }
    #[doc = "Bit 8 - AC"]
    #[inline]
    pub fn ac_(&mut self) -> _AC_W {
        _AC_W { w: self }
    }
    #[doc = "Bit 9 - AES"]
    #[inline]
    pub fn aes_(&mut self) -> _AES_W {
        _AES_W { w: self }
    }
    #[doc = "Bit 10 - TRNG"]
    #[inline]
    pub fn trng_(&mut self) -> _TRNG_W {
        _TRNG_W { w: self }
    }
    #[doc = "Bit 11 - ICM"]
    #[inline]
    pub fn icm_(&mut self) -> _ICM_W {
        _ICM_W { w: self }
    }
    #[doc = "Bit 12 - PUKCC"]
    #[inline]
    pub fn pukcc_(&mut self) -> _PUKCC_W {
        _PUKCC_W { w: self }
    }
    #[doc = "Bit 13 - QSPI"]
    #[inline]
    pub fn qspi_(&mut self) -> _QSPI_W {
        _QSPI_W { w: self }
    }
    #[doc = "Bit 14 - CCL"]
    #[inline]
    pub fn ccl_(&mut self) -> _CCL_W {
        _CCL_W { w: self }
    }
}
