#[doc = "Register `INTFLAGC` reader"]
pub struct R(crate::R<INTFLAGC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTFLAGC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTFLAGC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTFLAGC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTFLAGC` writer"]
pub struct W(crate::W<INTFLAGC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTFLAGC_SPEC>;
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
impl From<crate::W<INTFLAGC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTFLAGC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CAN0_` reader - CAN0"]
pub struct CAN0__R(crate::FieldReader<bool, bool>);
impl CAN0__R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CAN0__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CAN0__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAN0_` writer - CAN0"]
pub struct CAN0__W<'a> {
    w: &'a mut W,
}
impl<'a> CAN0__W<'a> {
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
#[doc = "Field `CAN1_` reader - CAN1"]
pub struct CAN1__R(crate::FieldReader<bool, bool>);
impl CAN1__R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CAN1__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CAN1__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAN1_` writer - CAN1"]
pub struct CAN1__W<'a> {
    w: &'a mut W,
}
impl<'a> CAN1__W<'a> {
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
#[doc = "Field `GMAC_` reader - GMAC"]
pub struct GMAC__R(crate::FieldReader<bool, bool>);
impl GMAC__R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GMAC__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GMAC__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GMAC_` writer - GMAC"]
pub struct GMAC__W<'a> {
    w: &'a mut W,
}
impl<'a> GMAC__W<'a> {
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
#[doc = "Field `TCC2_` reader - TCC2"]
pub struct TCC2__R(crate::FieldReader<bool, bool>);
impl TCC2__R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TCC2__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TCC2__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TCC2_` writer - TCC2"]
pub struct TCC2__W<'a> {
    w: &'a mut W,
}
impl<'a> TCC2__W<'a> {
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
#[doc = "Field `TCC3_` reader - TCC3"]
pub struct TCC3__R(crate::FieldReader<bool, bool>);
impl TCC3__R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TCC3__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TCC3__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TCC3_` writer - TCC3"]
pub struct TCC3__W<'a> {
    w: &'a mut W,
}
impl<'a> TCC3__W<'a> {
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
#[doc = "Field `TC4_` reader - TC4"]
pub struct TC4__R(crate::FieldReader<bool, bool>);
impl TC4__R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TC4__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TC4__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TC4_` writer - TC4"]
pub struct TC4__W<'a> {
    w: &'a mut W,
}
impl<'a> TC4__W<'a> {
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
#[doc = "Field `TC5_` reader - TC5"]
pub struct TC5__R(crate::FieldReader<bool, bool>);
impl TC5__R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TC5__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TC5__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TC5_` writer - TC5"]
pub struct TC5__W<'a> {
    w: &'a mut W,
}
impl<'a> TC5__W<'a> {
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
#[doc = "Field `PDEC_` reader - PDEC"]
pub struct PDEC__R(crate::FieldReader<bool, bool>);
impl PDEC__R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PDEC__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PDEC__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PDEC_` writer - PDEC"]
pub struct PDEC__W<'a> {
    w: &'a mut W,
}
impl<'a> PDEC__W<'a> {
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
#[doc = "Field `AC_` reader - AC"]
pub struct AC__R(crate::FieldReader<bool, bool>);
impl AC__R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        AC__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AC__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AC_` writer - AC"]
pub struct AC__W<'a> {
    w: &'a mut W,
}
impl<'a> AC__W<'a> {
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
#[doc = "Field `AES_` reader - AES"]
pub struct AES__R(crate::FieldReader<bool, bool>);
impl AES__R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        AES__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AES__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AES_` writer - AES"]
pub struct AES__W<'a> {
    w: &'a mut W,
}
impl<'a> AES__W<'a> {
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
#[doc = "Field `TRNG_` reader - TRNG"]
pub struct TRNG__R(crate::FieldReader<bool, bool>);
impl TRNG__R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TRNG__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRNG__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRNG_` writer - TRNG"]
pub struct TRNG__W<'a> {
    w: &'a mut W,
}
impl<'a> TRNG__W<'a> {
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
#[doc = "Field `ICM_` reader - ICM"]
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
#[doc = "Field `ICM_` writer - ICM"]
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "Field `CCL_` reader - CCL"]
pub struct CCL__R(crate::FieldReader<bool, bool>);
impl CCL__R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CCL__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CCL__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CCL_` writer - CCL"]
pub struct CCL__W<'a> {
    w: &'a mut W,
}
impl<'a> CCL__W<'a> {
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
impl R {
    #[doc = "Bit 0 - CAN0"]
    #[inline(always)]
    pub fn can0_(&self) -> CAN0__R {
        CAN0__R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - CAN1"]
    #[inline(always)]
    pub fn can1_(&self) -> CAN1__R {
        CAN1__R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - GMAC"]
    #[inline(always)]
    pub fn gmac_(&self) -> GMAC__R {
        GMAC__R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - TCC2"]
    #[inline(always)]
    pub fn tcc2_(&self) -> TCC2__R {
        TCC2__R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - TCC3"]
    #[inline(always)]
    pub fn tcc3_(&self) -> TCC3__R {
        TCC3__R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - TC4"]
    #[inline(always)]
    pub fn tc4_(&self) -> TC4__R {
        TC4__R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - TC5"]
    #[inline(always)]
    pub fn tc5_(&self) -> TC5__R {
        TC5__R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - PDEC"]
    #[inline(always)]
    pub fn pdec_(&self) -> PDEC__R {
        PDEC__R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - AC"]
    #[inline(always)]
    pub fn ac_(&self) -> AC__R {
        AC__R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - AES"]
    #[inline(always)]
    pub fn aes_(&self) -> AES__R {
        AES__R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - TRNG"]
    #[inline(always)]
    pub fn trng_(&self) -> TRNG__R {
        TRNG__R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - ICM"]
    #[inline(always)]
    pub fn icm_(&self) -> ICM__R {
        ICM__R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - PUKCC"]
    #[inline(always)]
    pub fn pukcc_(&self) -> PUKCC__R {
        PUKCC__R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - QSPI"]
    #[inline(always)]
    pub fn qspi_(&self) -> QSPI__R {
        QSPI__R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - CCL"]
    #[inline(always)]
    pub fn ccl_(&self) -> CCL__R {
        CCL__R::new(((self.bits >> 14) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CAN0"]
    #[inline(always)]
    pub fn can0_(&mut self) -> CAN0__W {
        CAN0__W { w: self }
    }
    #[doc = "Bit 1 - CAN1"]
    #[inline(always)]
    pub fn can1_(&mut self) -> CAN1__W {
        CAN1__W { w: self }
    }
    #[doc = "Bit 2 - GMAC"]
    #[inline(always)]
    pub fn gmac_(&mut self) -> GMAC__W {
        GMAC__W { w: self }
    }
    #[doc = "Bit 3 - TCC2"]
    #[inline(always)]
    pub fn tcc2_(&mut self) -> TCC2__W {
        TCC2__W { w: self }
    }
    #[doc = "Bit 4 - TCC3"]
    #[inline(always)]
    pub fn tcc3_(&mut self) -> TCC3__W {
        TCC3__W { w: self }
    }
    #[doc = "Bit 5 - TC4"]
    #[inline(always)]
    pub fn tc4_(&mut self) -> TC4__W {
        TC4__W { w: self }
    }
    #[doc = "Bit 6 - TC5"]
    #[inline(always)]
    pub fn tc5_(&mut self) -> TC5__W {
        TC5__W { w: self }
    }
    #[doc = "Bit 7 - PDEC"]
    #[inline(always)]
    pub fn pdec_(&mut self) -> PDEC__W {
        PDEC__W { w: self }
    }
    #[doc = "Bit 8 - AC"]
    #[inline(always)]
    pub fn ac_(&mut self) -> AC__W {
        AC__W { w: self }
    }
    #[doc = "Bit 9 - AES"]
    #[inline(always)]
    pub fn aes_(&mut self) -> AES__W {
        AES__W { w: self }
    }
    #[doc = "Bit 10 - TRNG"]
    #[inline(always)]
    pub fn trng_(&mut self) -> TRNG__W {
        TRNG__W { w: self }
    }
    #[doc = "Bit 11 - ICM"]
    #[inline(always)]
    pub fn icm_(&mut self) -> ICM__W {
        ICM__W { w: self }
    }
    #[doc = "Bit 12 - PUKCC"]
    #[inline(always)]
    pub fn pukcc_(&mut self) -> PUKCC__W {
        PUKCC__W { w: self }
    }
    #[doc = "Bit 13 - QSPI"]
    #[inline(always)]
    pub fn qspi_(&mut self) -> QSPI__W {
        QSPI__W { w: self }
    }
    #[doc = "Bit 14 - CCL"]
    #[inline(always)]
    pub fn ccl_(&mut self) -> CCL__W {
        CCL__W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Peripheral interrupt flag status - Bridge C\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intflagc](index.html) module"]
pub struct INTFLAGC_SPEC;
impl crate::RegisterSpec for INTFLAGC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intflagc::R](R) reader structure"]
impl crate::Readable for INTFLAGC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intflagc::W](W) writer structure"]
impl crate::Writable for INTFLAGC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTFLAGC to value 0"]
impl crate::Resettable for INTFLAGC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
