#[doc = "Register `STATUS` reader"]
pub struct R(crate::R<STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STATUS` writer"]
pub struct W(crate::W<STATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STATUS_SPEC>;
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
impl From<crate::W<STATUS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `QERR` reader - Quadrature Error Flag"]
pub struct QERR_R(crate::FieldReader<bool, bool>);
impl QERR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        QERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for QERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `QERR` writer - Quadrature Error Flag"]
pub struct QERR_W<'a> {
    w: &'a mut W,
}
impl<'a> QERR_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | (value as u16 & 0x01);
        self.w
    }
}
#[doc = "Field `IDXERR` reader - Index Error Flag"]
pub struct IDXERR_R(crate::FieldReader<bool, bool>);
impl IDXERR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IDXERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IDXERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IDXERR` writer - Index Error Flag"]
pub struct IDXERR_W<'a> {
    w: &'a mut W,
}
impl<'a> IDXERR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u16 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `MPERR` reader - Missing Pulse Error flag"]
pub struct MPERR_R(crate::FieldReader<bool, bool>);
impl MPERR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MPERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MPERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MPERR` writer - Missing Pulse Error flag"]
pub struct MPERR_W<'a> {
    w: &'a mut W,
}
impl<'a> MPERR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u16 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `WINERR` reader - Window Error Flag"]
pub struct WINERR_R(crate::FieldReader<bool, bool>);
impl WINERR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WINERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WINERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WINERR` writer - Window Error Flag"]
pub struct WINERR_W<'a> {
    w: &'a mut W,
}
impl<'a> WINERR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u16 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `HERR` reader - Hall Error Flag"]
pub struct HERR_R(crate::FieldReader<bool, bool>);
impl HERR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        HERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HERR` writer - Hall Error Flag"]
pub struct HERR_W<'a> {
    w: &'a mut W,
}
impl<'a> HERR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u16 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `STOP` reader - Stop"]
pub struct STOP_R(crate::FieldReader<bool, bool>);
impl STOP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        STOP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STOP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STOP` writer - Stop"]
pub struct STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> STOP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u16 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `DIR` reader - Direction Status Flag"]
pub struct DIR_R(crate::FieldReader<bool, bool>);
impl DIR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DIR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIR` writer - Direction Status Flag"]
pub struct DIR_W<'a> {
    w: &'a mut W,
}
impl<'a> DIR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u16 & 0x01) << 7);
        self.w
    }
}
#[doc = "Field `PRESCBUFV` reader - Prescaler Buffer Valid"]
pub struct PRESCBUFV_R(crate::FieldReader<bool, bool>);
impl PRESCBUFV_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PRESCBUFV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRESCBUFV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRESCBUFV` writer - Prescaler Buffer Valid"]
pub struct PRESCBUFV_W<'a> {
    w: &'a mut W,
}
impl<'a> PRESCBUFV_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u16 & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `FILTERBUFV` reader - Filter Buffer Valid"]
pub struct FILTERBUFV_R(crate::FieldReader<bool, bool>);
impl FILTERBUFV_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FILTERBUFV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FILTERBUFV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FILTERBUFV` writer - Filter Buffer Valid"]
pub struct FILTERBUFV_W<'a> {
    w: &'a mut W,
}
impl<'a> FILTERBUFV_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u16 & 0x01) << 9);
        self.w
    }
}
#[doc = "Field `CCBUFV0` reader - Compare Channel 0 Buffer Valid"]
pub struct CCBUFV0_R(crate::FieldReader<bool, bool>);
impl CCBUFV0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CCBUFV0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CCBUFV0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CCBUFV0` writer - Compare Channel 0 Buffer Valid"]
pub struct CCBUFV0_W<'a> {
    w: &'a mut W,
}
impl<'a> CCBUFV0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u16 & 0x01) << 12);
        self.w
    }
}
#[doc = "Field `CCBUFV1` reader - Compare Channel 1 Buffer Valid"]
pub struct CCBUFV1_R(crate::FieldReader<bool, bool>);
impl CCBUFV1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CCBUFV1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CCBUFV1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CCBUFV1` writer - Compare Channel 1 Buffer Valid"]
pub struct CCBUFV1_W<'a> {
    w: &'a mut W,
}
impl<'a> CCBUFV1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u16 & 0x01) << 13);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Quadrature Error Flag"]
    #[inline(always)]
    pub fn qerr(&self) -> QERR_R {
        QERR_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Index Error Flag"]
    #[inline(always)]
    pub fn idxerr(&self) -> IDXERR_R {
        IDXERR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Missing Pulse Error flag"]
    #[inline(always)]
    pub fn mperr(&self) -> MPERR_R {
        MPERR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Window Error Flag"]
    #[inline(always)]
    pub fn winerr(&self) -> WINERR_R {
        WINERR_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Hall Error Flag"]
    #[inline(always)]
    pub fn herr(&self) -> HERR_R {
        HERR_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Stop"]
    #[inline(always)]
    pub fn stop(&self) -> STOP_R {
        STOP_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Direction Status Flag"]
    #[inline(always)]
    pub fn dir(&self) -> DIR_R {
        DIR_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Prescaler Buffer Valid"]
    #[inline(always)]
    pub fn prescbufv(&self) -> PRESCBUFV_R {
        PRESCBUFV_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Filter Buffer Valid"]
    #[inline(always)]
    pub fn filterbufv(&self) -> FILTERBUFV_R {
        FILTERBUFV_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Compare Channel 0 Buffer Valid"]
    #[inline(always)]
    pub fn ccbufv0(&self) -> CCBUFV0_R {
        CCBUFV0_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Compare Channel 1 Buffer Valid"]
    #[inline(always)]
    pub fn ccbufv1(&self) -> CCBUFV1_R {
        CCBUFV1_R::new(((self.bits >> 13) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Quadrature Error Flag"]
    #[inline(always)]
    pub fn qerr(&mut self) -> QERR_W {
        QERR_W { w: self }
    }
    #[doc = "Bit 1 - Index Error Flag"]
    #[inline(always)]
    pub fn idxerr(&mut self) -> IDXERR_W {
        IDXERR_W { w: self }
    }
    #[doc = "Bit 2 - Missing Pulse Error flag"]
    #[inline(always)]
    pub fn mperr(&mut self) -> MPERR_W {
        MPERR_W { w: self }
    }
    #[doc = "Bit 4 - Window Error Flag"]
    #[inline(always)]
    pub fn winerr(&mut self) -> WINERR_W {
        WINERR_W { w: self }
    }
    #[doc = "Bit 5 - Hall Error Flag"]
    #[inline(always)]
    pub fn herr(&mut self) -> HERR_W {
        HERR_W { w: self }
    }
    #[doc = "Bit 6 - Stop"]
    #[inline(always)]
    pub fn stop(&mut self) -> STOP_W {
        STOP_W { w: self }
    }
    #[doc = "Bit 7 - Direction Status Flag"]
    #[inline(always)]
    pub fn dir(&mut self) -> DIR_W {
        DIR_W { w: self }
    }
    #[doc = "Bit 8 - Prescaler Buffer Valid"]
    #[inline(always)]
    pub fn prescbufv(&mut self) -> PRESCBUFV_W {
        PRESCBUFV_W { w: self }
    }
    #[doc = "Bit 9 - Filter Buffer Valid"]
    #[inline(always)]
    pub fn filterbufv(&mut self) -> FILTERBUFV_W {
        FILTERBUFV_W { w: self }
    }
    #[doc = "Bit 12 - Compare Channel 0 Buffer Valid"]
    #[inline(always)]
    pub fn ccbufv0(&mut self) -> CCBUFV0_W {
        CCBUFV0_W { w: self }
    }
    #[doc = "Bit 13 - Compare Channel 1 Buffer Valid"]
    #[inline(always)]
    pub fn ccbufv1(&mut self) -> CCBUFV1_W {
        CCBUFV1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status](index.html) module"]
pub struct STATUS_SPEC;
impl crate::RegisterSpec for STATUS_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [status::R](R) reader structure"]
impl crate::Readable for STATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [status::W](W) writer structure"]
impl crate::Writable for STATUS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets STATUS to value 0x40"]
impl crate::Resettable for STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x40
    }
}
