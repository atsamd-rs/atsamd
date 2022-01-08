#[doc = "Register `WPCLR` reader"]
pub struct R(crate::R<WPCLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WPCLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WPCLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WPCLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WPCLR` writer"]
pub struct W(crate::W<WPCLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WPCLR_SPEC>;
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
impl From<crate::W<WPCLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WPCLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WP` reader - Write Protection Clear"]
pub struct WP_R(crate::FieldReader<u32, u32>);
impl WP_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        WP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WP_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WP` writer - Write Protection Clear"]
pub struct WP_W<'a> {
    w: &'a mut W,
}
impl<'a> WP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7fff_ffff << 1)) | ((value as u32 & 0x7fff_ffff) << 1);
        self.w
    }
}
impl R {
    #[doc = "Bits 1:31 - Write Protection Clear"]
    #[inline(always)]
    pub fn wp(&self) -> WP_R {
        WP_R::new(((self.bits >> 1) & 0x7fff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 1:31 - Write Protection Clear"]
    #[inline(always)]
    pub fn wp(&mut self) -> WP_W {
        WP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Write Protection Clear\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wpclr](index.html) module"]
pub struct WPCLR_SPEC;
impl crate::RegisterSpec for WPCLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wpclr::R](R) reader structure"]
impl crate::Readable for WPCLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wpclr::W](W) writer structure"]
impl crate::Writable for WPCLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WPCLR to value 0"]
impl crate::Resettable for WPCLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
