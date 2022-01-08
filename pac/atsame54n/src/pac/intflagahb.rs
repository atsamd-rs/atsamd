#[doc = "Register `INTFLAGAHB` reader"]
pub struct R(crate::R<INTFLAGAHB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTFLAGAHB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTFLAGAHB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTFLAGAHB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTFLAGAHB` writer"]
pub struct W(crate::W<INTFLAGAHB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTFLAGAHB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<INTFLAGAHB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTFLAGAHB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FLASH_` reader - FLASH"]
pub struct FLASH__R(crate::FieldReader<bool, bool>);
impl FLASH__R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FLASH__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLASH__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLASH_` writer - FLASH"]
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Field `FLASH_ALT_` reader - FLASH_ALT"]
pub struct FLASH_ALT__R(crate::FieldReader<bool, bool>);
impl FLASH_ALT__R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FLASH_ALT__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLASH_ALT__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLASH_ALT_` writer - FLASH_ALT"]
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `SEEPROM_` reader - SEEPROM"]
pub struct SEEPROM__R(crate::FieldReader<bool, bool>);
impl SEEPROM__R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SEEPROM__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SEEPROM__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SEEPROM_` writer - SEEPROM"]
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `RAMCM4S_` reader - RAMCM4S"]
pub struct RAMCM4S__R(crate::FieldReader<bool, bool>);
impl RAMCM4S__R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RAMCM4S__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RAMCM4S__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RAMCM4S_` writer - RAMCM4S"]
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `RAMPPPDSU_` reader - RAMPPPDSU"]
pub struct RAMPPPDSU__R(crate::FieldReader<bool, bool>);
impl RAMPPPDSU__R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RAMPPPDSU__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RAMPPPDSU__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RAMPPPDSU_` writer - RAMPPPDSU"]
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `RAMDMAWR_` reader - RAMDMAWR"]
pub struct RAMDMAWR__R(crate::FieldReader<bool, bool>);
impl RAMDMAWR__R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RAMDMAWR__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RAMDMAWR__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RAMDMAWR_` writer - RAMDMAWR"]
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `RAMDMACICM_` reader - RAMDMACICM"]
pub struct RAMDMACICM__R(crate::FieldReader<bool, bool>);
impl RAMDMACICM__R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RAMDMACICM__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RAMDMACICM__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RAMDMACICM_` writer - RAMDMACICM"]
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `HPB0_` reader - HPB0"]
pub struct HPB0__R(crate::FieldReader<bool, bool>);
impl HPB0__R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        HPB0__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HPB0__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HPB0_` writer - HPB0"]
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Field `HPB1_` reader - HPB1"]
pub struct HPB1__R(crate::FieldReader<bool, bool>);
impl HPB1__R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        HPB1__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HPB1__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HPB1_` writer - HPB1"]
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `HPB2_` reader - HPB2"]
pub struct HPB2__R(crate::FieldReader<bool, bool>);
impl HPB2__R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        HPB2__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HPB2__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HPB2_` writer - HPB2"]
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Field `HPB3_` reader - HPB3"]
pub struct HPB3__R(crate::FieldReader<bool, bool>);
impl HPB3__R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        HPB3__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HPB3__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HPB3_` writer - HPB3"]
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "Field `PUKCC_` reader - PUKCC"]
pub struct PUKCC__R(crate::FieldReader<bool, bool>);
impl PUKCC__R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PUKCC__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PUKCC__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PUKCC_` writer - PUKCC"]
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "Field `SDHC0_` reader - SDHC0"]
pub struct SDHC0__R(crate::FieldReader<bool, bool>);
impl SDHC0__R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SDHC0__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SDHC0__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SDHC0_` writer - SDHC0"]
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "Field `SDHC1_` reader - SDHC1"]
pub struct SDHC1__R(crate::FieldReader<bool, bool>);
impl SDHC1__R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SDHC1__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SDHC1__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SDHC1_` writer - SDHC1"]
pub struct SDHC1__W<'a> {
    w: &'a mut W,
}
impl<'a> SDHC1__W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "Field `QSPI_` reader - QSPI"]
pub struct QSPI__R(crate::FieldReader<bool, bool>);
impl QSPI__R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        QSPI__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for QSPI__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `QSPI_` writer - QSPI"]
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
#[doc = "Field `BKUPRAM_` reader - BKUPRAM"]
pub struct BKUPRAM__R(crate::FieldReader<bool, bool>);
impl BKUPRAM__R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BKUPRAM__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BKUPRAM__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BKUPRAM_` writer - BKUPRAM"]
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
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
    #[doc = "Bit 13 - SDHC1"]
    #[inline(always)]
    pub fn sdhc1_(&self) -> SDHC1__R {
        SDHC1__R::new(((self.bits >> 13) & 0x01) != 0)
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
    #[doc = "Bit 13 - SDHC1"]
    #[inline(always)]
    pub fn sdhc1_(&mut self) -> SDHC1__W {
        SDHC1__W { w: self }
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
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Bridge interrupt flag status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intflagahb](index.html) module"]
pub struct INTFLAGAHB_SPEC;
impl crate::RegisterSpec for INTFLAGAHB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intflagahb::R](R) reader structure"]
impl crate::Readable for INTFLAGAHB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intflagahb::W](W) writer structure"]
impl crate::Writable for INTFLAGAHB_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTFLAGAHB to value 0"]
impl crate::Resettable for INTFLAGAHB_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
