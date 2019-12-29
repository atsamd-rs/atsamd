#[doc = "Reader of register INTCPUSEL4"]
pub type R = crate::R<u32, super::INTCPUSEL4>;
#[doc = "Writer for register INTCPUSEL4"]
pub type W = crate::W<u32, super::INTCPUSEL4>;
#[doc = "Register INTCPUSEL4 `reset()`'s with value 0"]
impl crate::ResetValue for super::INTCPUSEL4 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CAN0`"]
pub type CAN0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CAN0`"]
pub struct CAN0_W<'a> {
    w: &'a mut W,
}
impl<'a> CAN0_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `CAN1`"]
pub type CAN1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CAN1`"]
pub struct CAN1_W<'a> {
    w: &'a mut W,
}
impl<'a> CAN1_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `GMAC`"]
pub type GMAC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GMAC`"]
pub struct GMAC_W<'a> {
    w: &'a mut W,
}
impl<'a> GMAC_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `TCC2`"]
pub type TCC2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TCC2`"]
pub struct TCC2_W<'a> {
    w: &'a mut W,
}
impl<'a> TCC2_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `TCC3`"]
pub type TCC3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TCC3`"]
pub struct TCC3_W<'a> {
    w: &'a mut W,
}
impl<'a> TCC3_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `TC4`"]
pub type TC4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TC4`"]
pub struct TC4_W<'a> {
    w: &'a mut W,
}
impl<'a> TC4_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `TC5`"]
pub type TC5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TC5`"]
pub struct TC5_W<'a> {
    w: &'a mut W,
}
impl<'a> TC5_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `PDEC`"]
pub type PDEC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PDEC`"]
pub struct PDEC_W<'a> {
    w: &'a mut W,
}
impl<'a> PDEC_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Reader of field `AC`"]
pub type AC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AC`"]
pub struct AC_W<'a> {
    w: &'a mut W,
}
impl<'a> AC_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `AES`"]
pub type AES_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AES`"]
pub struct AES_W<'a> {
    w: &'a mut W,
}
impl<'a> AES_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Reader of field `TRNG`"]
pub type TRNG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TRNG`"]
pub struct TRNG_W<'a> {
    w: &'a mut W,
}
impl<'a> TRNG_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Reader of field `ICM`"]
pub type ICM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ICM`"]
pub struct ICM_W<'a> {
    w: &'a mut W,
}
impl<'a> ICM_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "Reader of field `PUKCC`"]
pub type PUKCC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PUKCC`"]
pub struct PUKCC_W<'a> {
    w: &'a mut W,
}
impl<'a> PUKCC_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Reader of field `QSPI`"]
pub type QSPI_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `QSPI`"]
pub struct QSPI_W<'a> {
    w: &'a mut W,
}
impl<'a> QSPI_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - CAN0 Interrupt CPU Select"]
    #[inline(always)]
    pub fn can0(&self) -> CAN0_R {
        CAN0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 2 - CAN1 Interrupt CPU Select"]
    #[inline(always)]
    pub fn can1(&self) -> CAN1_R {
        CAN1_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 4 - GMAC Interrupt CPU Select"]
    #[inline(always)]
    pub fn gmac(&self) -> GMAC_R {
        GMAC_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 6 - TCC2 Interrupt CPU Select"]
    #[inline(always)]
    pub fn tcc2(&self) -> TCC2_R {
        TCC2_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 8 - TCC3 Interrupt CPU Select"]
    #[inline(always)]
    pub fn tcc3(&self) -> TCC3_R {
        TCC3_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 10 - TC4 Interrupt CPU Select"]
    #[inline(always)]
    pub fn tc4(&self) -> TC4_R {
        TC4_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 12 - TC5 Interrupt CPU Select"]
    #[inline(always)]
    pub fn tc5(&self) -> TC5_R {
        TC5_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 14 - PDEC Interrupt CPU Select"]
    #[inline(always)]
    pub fn pdec(&self) -> PDEC_R {
        PDEC_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 16 - AC Interrupt CPU Select"]
    #[inline(always)]
    pub fn ac(&self) -> AC_R {
        AC_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 18 - AES Interrupt CPU Select"]
    #[inline(always)]
    pub fn aes(&self) -> AES_R {
        AES_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 20 - TRNG Interrupt CPU Select"]
    #[inline(always)]
    pub fn trng(&self) -> TRNG_R {
        TRNG_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 22 - ICM Interrupt CPU Select"]
    #[inline(always)]
    pub fn icm(&self) -> ICM_R {
        ICM_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 24 - PUKCC Interrupt CPU Select"]
    #[inline(always)]
    pub fn pukcc(&self) -> PUKCC_R {
        PUKCC_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 26 - QSPI Interrupt CPU Select"]
    #[inline(always)]
    pub fn qspi(&self) -> QSPI_R {
        QSPI_R::new(((self.bits >> 26) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CAN0 Interrupt CPU Select"]
    #[inline(always)]
    pub fn can0(&mut self) -> CAN0_W {
        CAN0_W { w: self }
    }
    #[doc = "Bit 2 - CAN1 Interrupt CPU Select"]
    #[inline(always)]
    pub fn can1(&mut self) -> CAN1_W {
        CAN1_W { w: self }
    }
    #[doc = "Bit 4 - GMAC Interrupt CPU Select"]
    #[inline(always)]
    pub fn gmac(&mut self) -> GMAC_W {
        GMAC_W { w: self }
    }
    #[doc = "Bit 6 - TCC2 Interrupt CPU Select"]
    #[inline(always)]
    pub fn tcc2(&mut self) -> TCC2_W {
        TCC2_W { w: self }
    }
    #[doc = "Bit 8 - TCC3 Interrupt CPU Select"]
    #[inline(always)]
    pub fn tcc3(&mut self) -> TCC3_W {
        TCC3_W { w: self }
    }
    #[doc = "Bit 10 - TC4 Interrupt CPU Select"]
    #[inline(always)]
    pub fn tc4(&mut self) -> TC4_W {
        TC4_W { w: self }
    }
    #[doc = "Bit 12 - TC5 Interrupt CPU Select"]
    #[inline(always)]
    pub fn tc5(&mut self) -> TC5_W {
        TC5_W { w: self }
    }
    #[doc = "Bit 14 - PDEC Interrupt CPU Select"]
    #[inline(always)]
    pub fn pdec(&mut self) -> PDEC_W {
        PDEC_W { w: self }
    }
    #[doc = "Bit 16 - AC Interrupt CPU Select"]
    #[inline(always)]
    pub fn ac(&mut self) -> AC_W {
        AC_W { w: self }
    }
    #[doc = "Bit 18 - AES Interrupt CPU Select"]
    #[inline(always)]
    pub fn aes(&mut self) -> AES_W {
        AES_W { w: self }
    }
    #[doc = "Bit 20 - TRNG Interrupt CPU Select"]
    #[inline(always)]
    pub fn trng(&mut self) -> TRNG_W {
        TRNG_W { w: self }
    }
    #[doc = "Bit 22 - ICM Interrupt CPU Select"]
    #[inline(always)]
    pub fn icm(&mut self) -> ICM_W {
        ICM_W { w: self }
    }
    #[doc = "Bit 24 - PUKCC Interrupt CPU Select"]
    #[inline(always)]
    pub fn pukcc(&mut self) -> PUKCC_W {
        PUKCC_W { w: self }
    }
    #[doc = "Bit 26 - QSPI Interrupt CPU Select"]
    #[inline(always)]
    pub fn qspi(&mut self) -> QSPI_W {
        QSPI_W { w: self }
    }
}
