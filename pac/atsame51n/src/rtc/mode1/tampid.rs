#[doc = "Register `TAMPID` reader"]
pub struct R(crate::R<TAMPID_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TAMPID_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TAMPID_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TAMPID_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TAMPID` writer"]
pub struct W(crate::W<TAMPID_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TAMPID_SPEC>;
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
impl From<crate::W<TAMPID_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TAMPID_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TAMPID0` reader - Tamper Input 0 Detected"]
pub struct TAMPID0_R(crate::FieldReader<bool, bool>);
impl TAMPID0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TAMPID0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TAMPID0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TAMPID0` writer - Tamper Input 0 Detected"]
pub struct TAMPID0_W<'a> {
    w: &'a mut W,
}
impl<'a> TAMPID0_W<'a> {
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
#[doc = "Field `TAMPID1` reader - Tamper Input 1 Detected"]
pub struct TAMPID1_R(crate::FieldReader<bool, bool>);
impl TAMPID1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TAMPID1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TAMPID1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TAMPID1` writer - Tamper Input 1 Detected"]
pub struct TAMPID1_W<'a> {
    w: &'a mut W,
}
impl<'a> TAMPID1_W<'a> {
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
#[doc = "Field `TAMPID2` reader - Tamper Input 2 Detected"]
pub struct TAMPID2_R(crate::FieldReader<bool, bool>);
impl TAMPID2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TAMPID2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TAMPID2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TAMPID2` writer - Tamper Input 2 Detected"]
pub struct TAMPID2_W<'a> {
    w: &'a mut W,
}
impl<'a> TAMPID2_W<'a> {
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
#[doc = "Field `TAMPID3` reader - Tamper Input 3 Detected"]
pub struct TAMPID3_R(crate::FieldReader<bool, bool>);
impl TAMPID3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TAMPID3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TAMPID3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TAMPID3` writer - Tamper Input 3 Detected"]
pub struct TAMPID3_W<'a> {
    w: &'a mut W,
}
impl<'a> TAMPID3_W<'a> {
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
#[doc = "Field `TAMPID4` reader - Tamper Input 4 Detected"]
pub struct TAMPID4_R(crate::FieldReader<bool, bool>);
impl TAMPID4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TAMPID4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TAMPID4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TAMPID4` writer - Tamper Input 4 Detected"]
pub struct TAMPID4_W<'a> {
    w: &'a mut W,
}
impl<'a> TAMPID4_W<'a> {
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
#[doc = "Field `TAMPEVT` reader - Tamper Event Detected"]
pub struct TAMPEVT_R(crate::FieldReader<bool, bool>);
impl TAMPEVT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TAMPEVT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TAMPEVT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TAMPEVT` writer - Tamper Event Detected"]
pub struct TAMPEVT_W<'a> {
    w: &'a mut W,
}
impl<'a> TAMPEVT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Tamper Input 0 Detected"]
    #[inline(always)]
    pub fn tampid0(&self) -> TAMPID0_R {
        TAMPID0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Tamper Input 1 Detected"]
    #[inline(always)]
    pub fn tampid1(&self) -> TAMPID1_R {
        TAMPID1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Tamper Input 2 Detected"]
    #[inline(always)]
    pub fn tampid2(&self) -> TAMPID2_R {
        TAMPID2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Tamper Input 3 Detected"]
    #[inline(always)]
    pub fn tampid3(&self) -> TAMPID3_R {
        TAMPID3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Tamper Input 4 Detected"]
    #[inline(always)]
    pub fn tampid4(&self) -> TAMPID4_R {
        TAMPID4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Tamper Event Detected"]
    #[inline(always)]
    pub fn tampevt(&self) -> TAMPEVT_R {
        TAMPEVT_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Tamper Input 0 Detected"]
    #[inline(always)]
    pub fn tampid0(&mut self) -> TAMPID0_W {
        TAMPID0_W { w: self }
    }
    #[doc = "Bit 1 - Tamper Input 1 Detected"]
    #[inline(always)]
    pub fn tampid1(&mut self) -> TAMPID1_W {
        TAMPID1_W { w: self }
    }
    #[doc = "Bit 2 - Tamper Input 2 Detected"]
    #[inline(always)]
    pub fn tampid2(&mut self) -> TAMPID2_W {
        TAMPID2_W { w: self }
    }
    #[doc = "Bit 3 - Tamper Input 3 Detected"]
    #[inline(always)]
    pub fn tampid3(&mut self) -> TAMPID3_W {
        TAMPID3_W { w: self }
    }
    #[doc = "Bit 4 - Tamper Input 4 Detected"]
    #[inline(always)]
    pub fn tampid4(&mut self) -> TAMPID4_W {
        TAMPID4_W { w: self }
    }
    #[doc = "Bit 31 - Tamper Event Detected"]
    #[inline(always)]
    pub fn tampevt(&mut self) -> TAMPEVT_W {
        TAMPEVT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Tamper ID\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tampid](index.html) module"]
pub struct TAMPID_SPEC;
impl crate::RegisterSpec for TAMPID_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tampid::R](R) reader structure"]
impl crate::Readable for TAMPID_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tampid::W](W) writer structure"]
impl crate::Writable for TAMPID_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TAMPID to value 0"]
impl crate::Resettable for TAMPID_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
