#[doc = "Register `CKGR_UCKR` reader"]
pub struct R(crate::R<CKGR_UCKR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CKGR_UCKR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CKGR_UCKR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CKGR_UCKR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CKGR_UCKR` writer"]
pub struct W(crate::W<CKGR_UCKR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CKGR_UCKR_SPEC>;
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
impl From<crate::W<CKGR_UCKR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CKGR_UCKR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UPLLEN` reader - UTMI PLL Enable"]
pub struct UPLLEN_R(crate::FieldReader<bool, bool>);
impl UPLLEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        UPLLEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UPLLEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UPLLEN` writer - UTMI PLL Enable"]
pub struct UPLLEN_W<'a> {
    w: &'a mut W,
}
impl<'a> UPLLEN_W<'a> {
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
#[doc = "Field `UPLLCOUNT` reader - UTMI PLL Start-up Time"]
pub struct UPLLCOUNT_R(crate::FieldReader<u8, u8>);
impl UPLLCOUNT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        UPLLCOUNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UPLLCOUNT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UPLLCOUNT` writer - UTMI PLL Start-up Time"]
pub struct UPLLCOUNT_W<'a> {
    w: &'a mut W,
}
impl<'a> UPLLCOUNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | ((value as u32 & 0x0f) << 20);
        self.w
    }
}
impl R {
    #[doc = "Bit 16 - UTMI PLL Enable"]
    #[inline(always)]
    pub fn upllen(&self) -> UPLLEN_R {
        UPLLEN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 20:23 - UTMI PLL Start-up Time"]
    #[inline(always)]
    pub fn upllcount(&self) -> UPLLCOUNT_R {
        UPLLCOUNT_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 16 - UTMI PLL Enable"]
    #[inline(always)]
    pub fn upllen(&mut self) -> UPLLEN_W {
        UPLLEN_W { w: self }
    }
    #[doc = "Bits 20:23 - UTMI PLL Start-up Time"]
    #[inline(always)]
    pub fn upllcount(&mut self) -> UPLLCOUNT_W {
        UPLLCOUNT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UTMI Clock Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ckgr_uckr](index.html) module"]
pub struct CKGR_UCKR_SPEC;
impl crate::RegisterSpec for CKGR_UCKR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ckgr_uckr::R](R) reader structure"]
impl crate::Readable for CKGR_UCKR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ckgr_uckr::W](W) writer structure"]
impl crate::Writable for CKGR_UCKR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CKGR_UCKR to value 0"]
impl crate::Resettable for CKGR_UCKR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
