#[doc = "Register `DEMCR` reader"]
pub struct R(crate::R<DEMCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DEMCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DEMCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DEMCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DEMCR` writer"]
pub struct W(crate::W<DEMCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DEMCR_SPEC>;
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
impl From<crate::W<DEMCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DEMCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VC_CORERESET` reader - "]
pub struct VC_CORERESET_R(crate::FieldReader<bool, bool>);
impl VC_CORERESET_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        VC_CORERESET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VC_CORERESET_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VC_CORERESET` writer - "]
pub struct VC_CORERESET_W<'a> {
    w: &'a mut W,
}
impl<'a> VC_CORERESET_W<'a> {
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
#[doc = "Field `VC_MMERR` reader - "]
pub struct VC_MMERR_R(crate::FieldReader<bool, bool>);
impl VC_MMERR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        VC_MMERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VC_MMERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VC_MMERR` writer - "]
pub struct VC_MMERR_W<'a> {
    w: &'a mut W,
}
impl<'a> VC_MMERR_W<'a> {
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
#[doc = "Field `VC_NOCPERR` reader - "]
pub struct VC_NOCPERR_R(crate::FieldReader<bool, bool>);
impl VC_NOCPERR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        VC_NOCPERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VC_NOCPERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VC_NOCPERR` writer - "]
pub struct VC_NOCPERR_W<'a> {
    w: &'a mut W,
}
impl<'a> VC_NOCPERR_W<'a> {
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
#[doc = "Field `VC_CHKERR` reader - "]
pub struct VC_CHKERR_R(crate::FieldReader<bool, bool>);
impl VC_CHKERR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        VC_CHKERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VC_CHKERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VC_CHKERR` writer - "]
pub struct VC_CHKERR_W<'a> {
    w: &'a mut W,
}
impl<'a> VC_CHKERR_W<'a> {
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
#[doc = "Field `VC_STATERR` reader - "]
pub struct VC_STATERR_R(crate::FieldReader<bool, bool>);
impl VC_STATERR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        VC_STATERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VC_STATERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VC_STATERR` writer - "]
pub struct VC_STATERR_W<'a> {
    w: &'a mut W,
}
impl<'a> VC_STATERR_W<'a> {
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
#[doc = "Field `VC_BUSERR` reader - "]
pub struct VC_BUSERR_R(crate::FieldReader<bool, bool>);
impl VC_BUSERR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        VC_BUSERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VC_BUSERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VC_BUSERR` writer - "]
pub struct VC_BUSERR_W<'a> {
    w: &'a mut W,
}
impl<'a> VC_BUSERR_W<'a> {
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
#[doc = "Field `VC_INTERR` reader - "]
pub struct VC_INTERR_R(crate::FieldReader<bool, bool>);
impl VC_INTERR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        VC_INTERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VC_INTERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VC_INTERR` writer - "]
pub struct VC_INTERR_W<'a> {
    w: &'a mut W,
}
impl<'a> VC_INTERR_W<'a> {
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
#[doc = "Field `VC_HARDERR` reader - "]
pub struct VC_HARDERR_R(crate::FieldReader<bool, bool>);
impl VC_HARDERR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        VC_HARDERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VC_HARDERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VC_HARDERR` writer - "]
pub struct VC_HARDERR_W<'a> {
    w: &'a mut W,
}
impl<'a> VC_HARDERR_W<'a> {
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
#[doc = "Field `MON_EN` reader - "]
pub struct MON_EN_R(crate::FieldReader<bool, bool>);
impl MON_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MON_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MON_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MON_EN` writer - "]
pub struct MON_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> MON_EN_W<'a> {
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
#[doc = "Field `MON_PEND` reader - "]
pub struct MON_PEND_R(crate::FieldReader<bool, bool>);
impl MON_PEND_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MON_PEND_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MON_PEND_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MON_PEND` writer - "]
pub struct MON_PEND_W<'a> {
    w: &'a mut W,
}
impl<'a> MON_PEND_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "Field `MON_STEP` reader - "]
pub struct MON_STEP_R(crate::FieldReader<bool, bool>);
impl MON_STEP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MON_STEP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MON_STEP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MON_STEP` writer - "]
pub struct MON_STEP_W<'a> {
    w: &'a mut W,
}
impl<'a> MON_STEP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
#[doc = "Field `MON_REQ` reader - "]
pub struct MON_REQ_R(crate::FieldReader<bool, bool>);
impl MON_REQ_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MON_REQ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MON_REQ_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MON_REQ` writer - "]
pub struct MON_REQ_W<'a> {
    w: &'a mut W,
}
impl<'a> MON_REQ_W<'a> {
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
#[doc = "Field `TRCENA` reader - "]
pub struct TRCENA_R(crate::FieldReader<bool, bool>);
impl TRCENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TRCENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRCENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRCENA` writer - "]
pub struct TRCENA_W<'a> {
    w: &'a mut W,
}
impl<'a> TRCENA_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn vc_corereset(&self) -> VC_CORERESET_R {
        VC_CORERESET_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn vc_mmerr(&self) -> VC_MMERR_R {
        VC_MMERR_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn vc_nocperr(&self) -> VC_NOCPERR_R {
        VC_NOCPERR_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn vc_chkerr(&self) -> VC_CHKERR_R {
        VC_CHKERR_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn vc_staterr(&self) -> VC_STATERR_R {
        VC_STATERR_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn vc_buserr(&self) -> VC_BUSERR_R {
        VC_BUSERR_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn vc_interr(&self) -> VC_INTERR_R {
        VC_INTERR_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn vc_harderr(&self) -> VC_HARDERR_R {
        VC_HARDERR_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn mon_en(&self) -> MON_EN_R {
        MON_EN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn mon_pend(&self) -> MON_PEND_R {
        MON_PEND_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn mon_step(&self) -> MON_STEP_R {
        MON_STEP_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn mon_req(&self) -> MON_REQ_R {
        MON_REQ_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn trcena(&self) -> TRCENA_R {
        TRCENA_R::new(((self.bits >> 24) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn vc_corereset(&mut self) -> VC_CORERESET_W {
        VC_CORERESET_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn vc_mmerr(&mut self) -> VC_MMERR_W {
        VC_MMERR_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn vc_nocperr(&mut self) -> VC_NOCPERR_W {
        VC_NOCPERR_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn vc_chkerr(&mut self) -> VC_CHKERR_W {
        VC_CHKERR_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn vc_staterr(&mut self) -> VC_STATERR_W {
        VC_STATERR_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn vc_buserr(&mut self) -> VC_BUSERR_W {
        VC_BUSERR_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn vc_interr(&mut self) -> VC_INTERR_W {
        VC_INTERR_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn vc_harderr(&mut self) -> VC_HARDERR_W {
        VC_HARDERR_W { w: self }
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn mon_en(&mut self) -> MON_EN_W {
        MON_EN_W { w: self }
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn mon_pend(&mut self) -> MON_PEND_W {
        MON_PEND_W { w: self }
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn mon_step(&mut self) -> MON_STEP_W {
        MON_STEP_W { w: self }
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn mon_req(&mut self) -> MON_REQ_W {
        MON_REQ_W { w: self }
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn trcena(&mut self) -> TRCENA_W {
        TRCENA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Debug Exception and Monitor Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [demcr](index.html) module"]
pub struct DEMCR_SPEC;
impl crate::RegisterSpec for DEMCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [demcr::R](R) reader structure"]
impl crate::Readable for DEMCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [demcr::W](W) writer structure"]
impl crate::Writable for DEMCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DEMCR to value 0"]
impl crate::Resettable for DEMCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
