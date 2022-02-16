#[doc = "Register `ISI_PSIZE` reader"]
pub struct R(crate::R<ISI_PSIZE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ISI_PSIZE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ISI_PSIZE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ISI_PSIZE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ISI_PSIZE` writer"]
pub struct W(crate::W<ISI_PSIZE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ISI_PSIZE_SPEC>;
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
impl From<crate::W<ISI_PSIZE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ISI_PSIZE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PREV_VSIZE` reader - Vertical Size for the Preview Path"]
pub struct PREV_VSIZE_R(crate::FieldReader<u16, u16>);
impl PREV_VSIZE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        PREV_VSIZE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PREV_VSIZE_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PREV_VSIZE` writer - Vertical Size for the Preview Path"]
pub struct PREV_VSIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> PREV_VSIZE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | (value as u32 & 0x03ff);
        self.w
    }
}
#[doc = "Field `PREV_HSIZE` reader - Horizontal Size for the Preview Path"]
pub struct PREV_HSIZE_R(crate::FieldReader<u16, u16>);
impl PREV_HSIZE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        PREV_HSIZE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PREV_HSIZE_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PREV_HSIZE` writer - Horizontal Size for the Preview Path"]
pub struct PREV_HSIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> PREV_HSIZE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 16)) | ((value as u32 & 0x03ff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:9 - Vertical Size for the Preview Path"]
    #[inline(always)]
    pub fn prev_vsize(&self) -> PREV_VSIZE_R {
        PREV_VSIZE_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:25 - Horizontal Size for the Preview Path"]
    #[inline(always)]
    pub fn prev_hsize(&self) -> PREV_HSIZE_R {
        PREV_HSIZE_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Vertical Size for the Preview Path"]
    #[inline(always)]
    pub fn prev_vsize(&mut self) -> PREV_VSIZE_W {
        PREV_VSIZE_W { w: self }
    }
    #[doc = "Bits 16:25 - Horizontal Size for the Preview Path"]
    #[inline(always)]
    pub fn prev_hsize(&mut self) -> PREV_HSIZE_W {
        PREV_HSIZE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ISI Preview Size Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [isi_psize](index.html) module"]
pub struct ISI_PSIZE_SPEC;
impl crate::RegisterSpec for ISI_PSIZE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [isi_psize::R](R) reader structure"]
impl crate::Readable for ISI_PSIZE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [isi_psize::W](W) writer structure"]
impl crate::Writable for ISI_PSIZE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ISI_PSIZE to value 0"]
impl crate::Resettable for ISI_PSIZE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
