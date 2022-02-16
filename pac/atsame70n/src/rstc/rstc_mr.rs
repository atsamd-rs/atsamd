#[doc = "Register `RSTC_MR` reader"]
pub struct R(crate::R<RSTC_MR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RSTC_MR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RSTC_MR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RSTC_MR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RSTC_MR` writer"]
pub struct W(crate::W<RSTC_MR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RSTC_MR_SPEC>;
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
impl From<crate::W<RSTC_MR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RSTC_MR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `URSTEN` reader - User Reset Enable"]
pub struct URSTEN_R(crate::FieldReader<bool, bool>);
impl URSTEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        URSTEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for URSTEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `URSTEN` writer - User Reset Enable"]
pub struct URSTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> URSTEN_W<'a> {
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
#[doc = "Field `URSTIEN` reader - User Reset Interrupt Enable"]
pub struct URSTIEN_R(crate::FieldReader<bool, bool>);
impl URSTIEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        URSTIEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for URSTIEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `URSTIEN` writer - User Reset Interrupt Enable"]
pub struct URSTIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> URSTIEN_W<'a> {
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
#[doc = "Field `ERSTL` reader - External Reset Length"]
pub struct ERSTL_R(crate::FieldReader<u8, u8>);
impl ERSTL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        ERSTL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ERSTL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ERSTL` writer - External Reset Length"]
pub struct ERSTL_W<'a> {
    w: &'a mut W,
}
impl<'a> ERSTL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u32 & 0x0f) << 8);
        self.w
    }
}
#[doc = "Write Access Password\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum KEY_A {
    #[doc = "165: Writing any other value in this field aborts the write operation.Always reads as 0."]
    PASSWD = 165,
}
impl From<KEY_A> for u8 {
    #[inline(always)]
    fn from(variant: KEY_A) -> Self {
        variant as _
    }
}
#[doc = "Field `KEY` reader - Write Access Password"]
pub struct KEY_R(crate::FieldReader<u8, KEY_A>);
impl KEY_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        KEY_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<KEY_A> {
        match self.bits {
            165 => Some(KEY_A::PASSWD),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `PASSWD`"]
    #[inline(always)]
    pub fn is_passwd(&self) -> bool {
        **self == KEY_A::PASSWD
    }
}
impl core::ops::Deref for KEY_R {
    type Target = crate::FieldReader<u8, KEY_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `KEY` writer - Write Access Password"]
pub struct KEY_W<'a> {
    w: &'a mut W,
}
impl<'a> KEY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: KEY_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Writing any other value in this field aborts the write operation.Always reads as 0."]
    #[inline(always)]
    pub fn passwd(self) -> &'a mut W {
        self.variant(KEY_A::PASSWD)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | ((value as u32 & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - User Reset Enable"]
    #[inline(always)]
    pub fn ursten(&self) -> URSTEN_R {
        URSTEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 4 - User Reset Interrupt Enable"]
    #[inline(always)]
    pub fn urstien(&self) -> URSTIEN_R {
        URSTIEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 8:11 - External Reset Length"]
    #[inline(always)]
    pub fn erstl(&self) -> ERSTL_R {
        ERSTL_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 24:31 - Write Access Password"]
    #[inline(always)]
    pub fn key(&self) -> KEY_R {
        KEY_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - User Reset Enable"]
    #[inline(always)]
    pub fn ursten(&mut self) -> URSTEN_W {
        URSTEN_W { w: self }
    }
    #[doc = "Bit 4 - User Reset Interrupt Enable"]
    #[inline(always)]
    pub fn urstien(&mut self) -> URSTIEN_W {
        URSTIEN_W { w: self }
    }
    #[doc = "Bits 8:11 - External Reset Length"]
    #[inline(always)]
    pub fn erstl(&mut self) -> ERSTL_W {
        ERSTL_W { w: self }
    }
    #[doc = "Bits 24:31 - Write Access Password"]
    #[inline(always)]
    pub fn key(&mut self) -> KEY_W {
        KEY_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rstc_mr](index.html) module"]
pub struct RSTC_MR_SPEC;
impl crate::RegisterSpec for RSTC_MR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rstc_mr::R](R) reader structure"]
impl crate::Readable for RSTC_MR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rstc_mr::W](W) writer structure"]
impl crate::Writable for RSTC_MR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RSTC_MR to value 0"]
impl crate::Resettable for RSTC_MR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
