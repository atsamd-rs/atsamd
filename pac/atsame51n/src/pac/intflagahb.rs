#[doc = "Reader of register INTFLAGAHB"]
pub type R = crate::R<u32, super::INTFLAGAHB>;
#[doc = "Writer for register INTFLAGAHB"]
pub type W = crate::W<u32, super::INTFLAGAHB>;
#[doc = "Register INTFLAGAHB `reset()`'s with value 0"]
impl crate::ResetValue for super::INTFLAGAHB {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FLASH_`"]
pub type FLASH__R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FLASH_`"]
pub struct FLASH__W<'a> {
    w: &'a mut W,
}
impl<'a> FLASH__W<'a> {
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
#[doc = "Reader of field `FLASH_ALT_`"]
pub type FLASH_ALT__R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FLASH_ALT_`"]
pub struct FLASH_ALT__W<'a> {
    w: &'a mut W,
}
impl<'a> FLASH_ALT__W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `SEEPROM_`"]
pub type SEEPROM__R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SEEPROM_`"]
pub struct SEEPROM__W<'a> {
    w: &'a mut W,
}
impl<'a> SEEPROM__W<'a> {
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
#[doc = "Reader of field `RAMCM4S_`"]
pub type RAMCM4S__R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RAMCM4S_`"]
pub struct RAMCM4S__W<'a> {
    w: &'a mut W,
}
impl<'a> RAMCM4S__W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `RAMPPPDSU_`"]
pub type RAMPPPDSU__R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RAMPPPDSU_`"]
pub struct RAMPPPDSU__W<'a> {
    w: &'a mut W,
}
impl<'a> RAMPPPDSU__W<'a> {
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
#[doc = "Reader of field `RAMDMAWR_`"]
pub type RAMDMAWR__R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RAMDMAWR_`"]
pub struct RAMDMAWR__W<'a> {
    w: &'a mut W,
}
impl<'a> RAMDMAWR__W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `RAMDMACICM_`"]
pub type RAMDMACICM__R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RAMDMACICM_`"]
pub struct RAMDMACICM__W<'a> {
    w: &'a mut W,
}
impl<'a> RAMDMACICM__W<'a> {
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
#[doc = "Reader of field `HPB0_`"]
pub type HPB0__R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HPB0_`"]
pub struct HPB0__W<'a> {
    w: &'a mut W,
}
impl<'a> HPB0__W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `HPB1_`"]
pub type HPB1__R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HPB1_`"]
pub struct HPB1__W<'a> {
    w: &'a mut W,
}
impl<'a> HPB1__W<'a> {
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
#[doc = "Reader of field `HPB2_`"]
pub type HPB2__R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HPB2_`"]
pub struct HPB2__W<'a> {
    w: &'a mut W,
}
impl<'a> HPB2__W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `HPB3_`"]
pub type HPB3__R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HPB3_`"]
pub struct HPB3__W<'a> {
    w: &'a mut W,
}
impl<'a> HPB3__W<'a> {
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
#[doc = "Reader of field `PUKCC_`"]
pub type PUKCC__R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PUKCC_`"]
pub struct PUKCC__W<'a> {
    w: &'a mut W,
}
impl<'a> PUKCC__W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Reader of field `SDHC0_`"]
pub type SDHC0__R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SDHC0_`"]
pub struct SDHC0__W<'a> {
    w: &'a mut W,
}
impl<'a> SDHC0__W<'a> {
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
#[doc = "Reader of field `QSPI_`"]
pub type QSPI__R = crate::R<bool, bool>;
#[doc = "Write proxy for field `QSPI_`"]
pub struct QSPI__W<'a> {
    w: &'a mut W,
}
impl<'a> QSPI__W<'a> {
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
#[doc = "Reader of field `BKUPRAM_`"]
pub type BKUPRAM__R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BKUPRAM_`"]
pub struct BKUPRAM__W<'a> {
    w: &'a mut W,
}
impl<'a> BKUPRAM__W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - FLASH"]
    #[inline(always)]
    pub fn flash_(&self) -> FLASH__R {
        FLASH__R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - FLASH_ALT"]
    #[inline(always)]
    pub fn flash_alt_(&self) -> FLASH_ALT__R {
        FLASH_ALT__R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - SEEPROM"]
    #[inline(always)]
    pub fn seeprom_(&self) -> SEEPROM__R {
        SEEPROM__R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - RAMCM4S"]
    #[inline(always)]
    pub fn ramcm4s_(&self) -> RAMCM4S__R {
        RAMCM4S__R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - RAMPPPDSU"]
    #[inline(always)]
    pub fn rampppdsu_(&self) -> RAMPPPDSU__R {
        RAMPPPDSU__R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - RAMDMAWR"]
    #[inline(always)]
    pub fn ramdmawr_(&self) -> RAMDMAWR__R {
        RAMDMAWR__R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - RAMDMACICM"]
    #[inline(always)]
    pub fn ramdmacicm_(&self) -> RAMDMACICM__R {
        RAMDMACICM__R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - HPB0"]
    #[inline(always)]
    pub fn hpb0_(&self) -> HPB0__R {
        HPB0__R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - HPB1"]
    #[inline(always)]
    pub fn hpb1_(&self) -> HPB1__R {
        HPB1__R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - HPB2"]
    #[inline(always)]
    pub fn hpb2_(&self) -> HPB2__R {
        HPB2__R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - HPB3"]
    #[inline(always)]
    pub fn hpb3_(&self) -> HPB3__R {
        HPB3__R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - PUKCC"]
    #[inline(always)]
    pub fn pukcc_(&self) -> PUKCC__R {
        PUKCC__R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - SDHC0"]
    #[inline(always)]
    pub fn sdhc0_(&self) -> SDHC0__R {
        SDHC0__R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 14 - QSPI"]
    #[inline(always)]
    pub fn qspi_(&self) -> QSPI__R {
        QSPI__R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - BKUPRAM"]
    #[inline(always)]
    pub fn bkupram_(&self) -> BKUPRAM__R {
        BKUPRAM__R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - FLASH"]
    #[inline(always)]
    pub fn flash_(&mut self) -> FLASH__W {
        FLASH__W { w: self }
    }
    #[doc = "Bit 1 - FLASH_ALT"]
    #[inline(always)]
    pub fn flash_alt_(&mut self) -> FLASH_ALT__W {
        FLASH_ALT__W { w: self }
    }
    #[doc = "Bit 2 - SEEPROM"]
    #[inline(always)]
    pub fn seeprom_(&mut self) -> SEEPROM__W {
        SEEPROM__W { w: self }
    }
    #[doc = "Bit 3 - RAMCM4S"]
    #[inline(always)]
    pub fn ramcm4s_(&mut self) -> RAMCM4S__W {
        RAMCM4S__W { w: self }
    }
    #[doc = "Bit 4 - RAMPPPDSU"]
    #[inline(always)]
    pub fn rampppdsu_(&mut self) -> RAMPPPDSU__W {
        RAMPPPDSU__W { w: self }
    }
    #[doc = "Bit 5 - RAMDMAWR"]
    #[inline(always)]
    pub fn ramdmawr_(&mut self) -> RAMDMAWR__W {
        RAMDMAWR__W { w: self }
    }
    #[doc = "Bit 6 - RAMDMACICM"]
    #[inline(always)]
    pub fn ramdmacicm_(&mut self) -> RAMDMACICM__W {
        RAMDMACICM__W { w: self }
    }
    #[doc = "Bit 7 - HPB0"]
    #[inline(always)]
    pub fn hpb0_(&mut self) -> HPB0__W {
        HPB0__W { w: self }
    }
    #[doc = "Bit 8 - HPB1"]
    #[inline(always)]
    pub fn hpb1_(&mut self) -> HPB1__W {
        HPB1__W { w: self }
    }
    #[doc = "Bit 9 - HPB2"]
    #[inline(always)]
    pub fn hpb2_(&mut self) -> HPB2__W {
        HPB2__W { w: self }
    }
    #[doc = "Bit 10 - HPB3"]
    #[inline(always)]
    pub fn hpb3_(&mut self) -> HPB3__W {
        HPB3__W { w: self }
    }
    #[doc = "Bit 11 - PUKCC"]
    #[inline(always)]
    pub fn pukcc_(&mut self) -> PUKCC__W {
        PUKCC__W { w: self }
    }
    #[doc = "Bit 12 - SDHC0"]
    #[inline(always)]
    pub fn sdhc0_(&mut self) -> SDHC0__W {
        SDHC0__W { w: self }
    }
    #[doc = "Bit 14 - QSPI"]
    #[inline(always)]
    pub fn qspi_(&mut self) -> QSPI__W {
        QSPI__W { w: self }
    }
    #[doc = "Bit 15 - BKUPRAM"]
    #[inline(always)]
    pub fn bkupram_(&mut self) -> BKUPRAM__W {
        BKUPRAM__W { w: self }
    }
}
