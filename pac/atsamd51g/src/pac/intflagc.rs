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
    #[doc = "Bit 3 - TCC2"]
    #[inline(always)]
    pub fn tcc2_(&self) -> TCC2__R {
        TCC2__R::new(((self.bits >> 3) & 0x01) != 0)
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
    #[doc = "Bit 3 - TCC2"]
    #[inline(always)]
    pub fn tcc2_(&mut self) -> TCC2__W {
        TCC2__W { w: self }
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
