#[doc = "Register `AHBMASK` reader"]
pub struct R(crate::R<AHBMASK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AHBMASK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AHBMASK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AHBMASK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AHBMASK` writer"]
pub struct W(crate::W<AHBMASK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AHBMASK_SPEC>;
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
impl From<crate::W<AHBMASK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AHBMASK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HPB0_` reader - HPB0 AHB Clock Mask"]
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
#[doc = "Field `HPB0_` writer - HPB0 AHB Clock Mask"]
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Field `HPB1_` reader - HPB1 AHB Clock Mask"]
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
#[doc = "Field `HPB1_` writer - HPB1 AHB Clock Mask"]
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `HPB2_` reader - HPB2 AHB Clock Mask"]
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
#[doc = "Field `HPB2_` writer - HPB2 AHB Clock Mask"]
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `HPB3_` reader - HPB3 AHB Clock Mask"]
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
#[doc = "Field `HPB3_` writer - HPB3 AHB Clock Mask"]
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `DSU_` reader - DSU AHB Clock Mask"]
pub struct DSU__R(crate::FieldReader<bool, bool>);
impl DSU__R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DSU__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DSU__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DSU_` writer - DSU AHB Clock Mask"]
pub struct DSU__W<'a> {
    w: &'a mut W,
}
impl<'a> DSU__W<'a> {
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
#[doc = "Field `HMATRIX_` reader - HMATRIX AHB Clock Mask"]
pub struct HMATRIX__R(crate::FieldReader<bool, bool>);
impl HMATRIX__R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        HMATRIX__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HMATRIX__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HMATRIX_` writer - HMATRIX AHB Clock Mask"]
pub struct HMATRIX__W<'a> {
    w: &'a mut W,
}
impl<'a> HMATRIX__W<'a> {
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
#[doc = "Field `NVMCTRL_` reader - NVMCTRL AHB Clock Mask"]
pub struct NVMCTRL__R(crate::FieldReader<bool, bool>);
impl NVMCTRL__R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        NVMCTRL__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NVMCTRL__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NVMCTRL_` writer - NVMCTRL AHB Clock Mask"]
pub struct NVMCTRL__W<'a> {
    w: &'a mut W,
}
impl<'a> NVMCTRL__W<'a> {
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
#[doc = "Field `HSRAM_` reader - HSRAM AHB Clock Mask"]
pub struct HSRAM__R(crate::FieldReader<bool, bool>);
impl HSRAM__R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        HSRAM__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HSRAM__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HSRAM_` writer - HSRAM AHB Clock Mask"]
pub struct HSRAM__W<'a> {
    w: &'a mut W,
}
impl<'a> HSRAM__W<'a> {
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
#[doc = "Field `CMCC_` reader - CMCC AHB Clock Mask"]
pub struct CMCC__R(crate::FieldReader<bool, bool>);
impl CMCC__R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CMCC__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMCC__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMCC_` writer - CMCC AHB Clock Mask"]
pub struct CMCC__W<'a> {
    w: &'a mut W,
}
impl<'a> CMCC__W<'a> {
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
#[doc = "Field `DMAC_` reader - DMAC AHB Clock Mask"]
pub struct DMAC__R(crate::FieldReader<bool, bool>);
impl DMAC__R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DMAC__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMAC__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMAC_` writer - DMAC AHB Clock Mask"]
pub struct DMAC__W<'a> {
    w: &'a mut W,
}
impl<'a> DMAC__W<'a> {
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
#[doc = "Field `USB_` reader - USB AHB Clock Mask"]
pub struct USB__R(crate::FieldReader<bool, bool>);
impl USB__R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        USB__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USB__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USB_` writer - USB AHB Clock Mask"]
pub struct USB__W<'a> {
    w: &'a mut W,
}
impl<'a> USB__W<'a> {
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
#[doc = "Field `BKUPRAM_` reader - BKUPRAM AHB Clock Mask"]
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
#[doc = "Field `BKUPRAM_` writer - BKUPRAM AHB Clock Mask"]
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "Field `PAC_` reader - PAC AHB Clock Mask"]
pub struct PAC__R(crate::FieldReader<bool, bool>);
impl PAC__R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PAC__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAC__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAC_` writer - PAC AHB Clock Mask"]
pub struct PAC__W<'a> {
    w: &'a mut W,
}
impl<'a> PAC__W<'a> {
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
#[doc = "Field `QSPI_` reader - QSPI AHB Clock Mask"]
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
#[doc = "Field `QSPI_` writer - QSPI AHB Clock Mask"]
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "Field `SDHC0_` reader - SDHC0 AHB Clock Mask"]
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
#[doc = "Field `SDHC0_` writer - SDHC0 AHB Clock Mask"]
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
#[doc = "Field `SDHC1_` reader - SDHC1 AHB Clock Mask"]
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
#[doc = "Field `SDHC1_` writer - SDHC1 AHB Clock Mask"]
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Field `ICM_` reader - ICM AHB Clock Mask"]
pub struct ICM__R(crate::FieldReader<bool, bool>);
impl ICM__R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ICM__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ICM__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ICM_` writer - ICM AHB Clock Mask"]
pub struct ICM__W<'a> {
    w: &'a mut W,
}
impl<'a> ICM__W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
#[doc = "Field `PUKCC_` reader - PUKCC AHB Clock Mask"]
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
#[doc = "Field `PUKCC_` writer - PUKCC AHB Clock Mask"]
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
        self.w
    }
}
#[doc = "Field `QSPI_2X_` reader - QSPI_2X AHB Clock Mask"]
pub struct QSPI_2X__R(crate::FieldReader<bool, bool>);
impl QSPI_2X__R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        QSPI_2X__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for QSPI_2X__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `QSPI_2X_` writer - QSPI_2X AHB Clock Mask"]
pub struct QSPI_2X__W<'a> {
    w: &'a mut W,
}
impl<'a> QSPI_2X__W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | ((value as u32 & 0x01) << 21);
        self.w
    }
}
#[doc = "Field `NVMCTRL_SMEEPROM_` reader - NVMCTRL_SMEEPROM AHB Clock Mask"]
pub struct NVMCTRL_SMEEPROM__R(crate::FieldReader<bool, bool>);
impl NVMCTRL_SMEEPROM__R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        NVMCTRL_SMEEPROM__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NVMCTRL_SMEEPROM__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NVMCTRL_SMEEPROM_` writer - NVMCTRL_SMEEPROM AHB Clock Mask"]
pub struct NVMCTRL_SMEEPROM__W<'a> {
    w: &'a mut W,
}
impl<'a> NVMCTRL_SMEEPROM__W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | ((value as u32 & 0x01) << 22);
        self.w
    }
}
#[doc = "Field `NVMCTRL_CACHE_` reader - NVMCTRL_CACHE AHB Clock Mask"]
pub struct NVMCTRL_CACHE__R(crate::FieldReader<bool, bool>);
impl NVMCTRL_CACHE__R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        NVMCTRL_CACHE__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NVMCTRL_CACHE__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NVMCTRL_CACHE_` writer - NVMCTRL_CACHE AHB Clock Mask"]
pub struct NVMCTRL_CACHE__W<'a> {
    w: &'a mut W,
}
impl<'a> NVMCTRL_CACHE__W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | ((value as u32 & 0x01) << 23);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - HPB0 AHB Clock Mask"]
    #[inline(always)]
    pub fn hpb0_(&self) -> HPB0__R {
        HPB0__R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - HPB1 AHB Clock Mask"]
    #[inline(always)]
    pub fn hpb1_(&self) -> HPB1__R {
        HPB1__R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - HPB2 AHB Clock Mask"]
    #[inline(always)]
    pub fn hpb2_(&self) -> HPB2__R {
        HPB2__R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - HPB3 AHB Clock Mask"]
    #[inline(always)]
    pub fn hpb3_(&self) -> HPB3__R {
        HPB3__R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - DSU AHB Clock Mask"]
    #[inline(always)]
    pub fn dsu_(&self) -> DSU__R {
        DSU__R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - HMATRIX AHB Clock Mask"]
    #[inline(always)]
    pub fn hmatrix_(&self) -> HMATRIX__R {
        HMATRIX__R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - NVMCTRL AHB Clock Mask"]
    #[inline(always)]
    pub fn nvmctrl_(&self) -> NVMCTRL__R {
        NVMCTRL__R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - HSRAM AHB Clock Mask"]
    #[inline(always)]
    pub fn hsram_(&self) -> HSRAM__R {
        HSRAM__R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - CMCC AHB Clock Mask"]
    #[inline(always)]
    pub fn cmcc_(&self) -> CMCC__R {
        CMCC__R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - DMAC AHB Clock Mask"]
    #[inline(always)]
    pub fn dmac_(&self) -> DMAC__R {
        DMAC__R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - USB AHB Clock Mask"]
    #[inline(always)]
    pub fn usb_(&self) -> USB__R {
        USB__R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - BKUPRAM AHB Clock Mask"]
    #[inline(always)]
    pub fn bkupram_(&self) -> BKUPRAM__R {
        BKUPRAM__R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - PAC AHB Clock Mask"]
    #[inline(always)]
    pub fn pac_(&self) -> PAC__R {
        PAC__R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - QSPI AHB Clock Mask"]
    #[inline(always)]
    pub fn qspi_(&self) -> QSPI__R {
        QSPI__R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 15 - SDHC0 AHB Clock Mask"]
    #[inline(always)]
    pub fn sdhc0_(&self) -> SDHC0__R {
        SDHC0__R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - SDHC1 AHB Clock Mask"]
    #[inline(always)]
    pub fn sdhc1_(&self) -> SDHC1__R {
        SDHC1__R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 19 - ICM AHB Clock Mask"]
    #[inline(always)]
    pub fn icm_(&self) -> ICM__R {
        ICM__R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - PUKCC AHB Clock Mask"]
    #[inline(always)]
    pub fn pukcc_(&self) -> PUKCC__R {
        PUKCC__R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - QSPI_2X AHB Clock Mask"]
    #[inline(always)]
    pub fn qspi_2x_(&self) -> QSPI_2X__R {
        QSPI_2X__R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - NVMCTRL_SMEEPROM AHB Clock Mask"]
    #[inline(always)]
    pub fn nvmctrl_smeeprom_(&self) -> NVMCTRL_SMEEPROM__R {
        NVMCTRL_SMEEPROM__R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - NVMCTRL_CACHE AHB Clock Mask"]
    #[inline(always)]
    pub fn nvmctrl_cache_(&self) -> NVMCTRL_CACHE__R {
        NVMCTRL_CACHE__R::new(((self.bits >> 23) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - HPB0 AHB Clock Mask"]
    #[inline(always)]
    pub fn hpb0_(&mut self) -> HPB0__W {
        HPB0__W { w: self }
    }
    #[doc = "Bit 1 - HPB1 AHB Clock Mask"]
    #[inline(always)]
    pub fn hpb1_(&mut self) -> HPB1__W {
        HPB1__W { w: self }
    }
    #[doc = "Bit 2 - HPB2 AHB Clock Mask"]
    #[inline(always)]
    pub fn hpb2_(&mut self) -> HPB2__W {
        HPB2__W { w: self }
    }
    #[doc = "Bit 3 - HPB3 AHB Clock Mask"]
    #[inline(always)]
    pub fn hpb3_(&mut self) -> HPB3__W {
        HPB3__W { w: self }
    }
    #[doc = "Bit 4 - DSU AHB Clock Mask"]
    #[inline(always)]
    pub fn dsu_(&mut self) -> DSU__W {
        DSU__W { w: self }
    }
    #[doc = "Bit 5 - HMATRIX AHB Clock Mask"]
    #[inline(always)]
    pub fn hmatrix_(&mut self) -> HMATRIX__W {
        HMATRIX__W { w: self }
    }
    #[doc = "Bit 6 - NVMCTRL AHB Clock Mask"]
    #[inline(always)]
    pub fn nvmctrl_(&mut self) -> NVMCTRL__W {
        NVMCTRL__W { w: self }
    }
    #[doc = "Bit 7 - HSRAM AHB Clock Mask"]
    #[inline(always)]
    pub fn hsram_(&mut self) -> HSRAM__W {
        HSRAM__W { w: self }
    }
    #[doc = "Bit 8 - CMCC AHB Clock Mask"]
    #[inline(always)]
    pub fn cmcc_(&mut self) -> CMCC__W {
        CMCC__W { w: self }
    }
    #[doc = "Bit 9 - DMAC AHB Clock Mask"]
    #[inline(always)]
    pub fn dmac_(&mut self) -> DMAC__W {
        DMAC__W { w: self }
    }
    #[doc = "Bit 10 - USB AHB Clock Mask"]
    #[inline(always)]
    pub fn usb_(&mut self) -> USB__W {
        USB__W { w: self }
    }
    #[doc = "Bit 11 - BKUPRAM AHB Clock Mask"]
    #[inline(always)]
    pub fn bkupram_(&mut self) -> BKUPRAM__W {
        BKUPRAM__W { w: self }
    }
    #[doc = "Bit 12 - PAC AHB Clock Mask"]
    #[inline(always)]
    pub fn pac_(&mut self) -> PAC__W {
        PAC__W { w: self }
    }
    #[doc = "Bit 13 - QSPI AHB Clock Mask"]
    #[inline(always)]
    pub fn qspi_(&mut self) -> QSPI__W {
        QSPI__W { w: self }
    }
    #[doc = "Bit 15 - SDHC0 AHB Clock Mask"]
    #[inline(always)]
    pub fn sdhc0_(&mut self) -> SDHC0__W {
        SDHC0__W { w: self }
    }
    #[doc = "Bit 16 - SDHC1 AHB Clock Mask"]
    #[inline(always)]
    pub fn sdhc1_(&mut self) -> SDHC1__W {
        SDHC1__W { w: self }
    }
    #[doc = "Bit 19 - ICM AHB Clock Mask"]
    #[inline(always)]
    pub fn icm_(&mut self) -> ICM__W {
        ICM__W { w: self }
    }
    #[doc = "Bit 20 - PUKCC AHB Clock Mask"]
    #[inline(always)]
    pub fn pukcc_(&mut self) -> PUKCC__W {
        PUKCC__W { w: self }
    }
    #[doc = "Bit 21 - QSPI_2X AHB Clock Mask"]
    #[inline(always)]
    pub fn qspi_2x_(&mut self) -> QSPI_2X__W {
        QSPI_2X__W { w: self }
    }
    #[doc = "Bit 22 - NVMCTRL_SMEEPROM AHB Clock Mask"]
    #[inline(always)]
    pub fn nvmctrl_smeeprom_(&mut self) -> NVMCTRL_SMEEPROM__W {
        NVMCTRL_SMEEPROM__W { w: self }
    }
    #[doc = "Bit 23 - NVMCTRL_CACHE AHB Clock Mask"]
    #[inline(always)]
    pub fn nvmctrl_cache_(&mut self) -> NVMCTRL_CACHE__W {
        NVMCTRL_CACHE__W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AHB Mask\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ahbmask](index.html) module"]
pub struct AHBMASK_SPEC;
impl crate::RegisterSpec for AHBMASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ahbmask::R](R) reader structure"]
impl crate::Readable for AHBMASK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ahbmask::W](W) writer structure"]
impl crate::Writable for AHBMASK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AHBMASK to value 0x00ff_ffff"]
impl crate::Resettable for AHBMASK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x00ff_ffff
    }
}
