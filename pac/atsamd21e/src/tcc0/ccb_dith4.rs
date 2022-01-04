#[doc = "Register `CCB%s_DITH4` reader"]
pub struct R(crate::R<CCB_DITH4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCB_DITH4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CCB_DITH4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CCB_DITH4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CCB%s_DITH4` writer"]
pub struct W(crate::W<CCB_DITH4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CCB_DITH4_SPEC>;
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
impl From<crate::W<CCB_DITH4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CCB_DITH4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DITHERCYB` reader - Dithering Buffer Cycle Number"]
pub struct DITHERCYB_R(crate::FieldReader<u8, u8>);
impl DITHERCYB_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DITHERCYB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DITHERCYB_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DITHERCYB` writer - Dithering Buffer Cycle Number"]
pub struct DITHERCYB_W<'a> {
    w: &'a mut W,
}
impl<'a> DITHERCYB_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
#[doc = "Field `CCB` reader - Channel Compare/Capture Buffer Value"]
pub struct CCB_R(crate::FieldReader<u32, u32>);
impl CCB_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        CCB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CCB_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CCB` writer - Channel Compare/Capture Buffer Value"]
pub struct CCB_W<'a> {
    w: &'a mut W,
}
impl<'a> CCB_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x000f_ffff << 4)) | ((value as u32 & 0x000f_ffff) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Dithering Buffer Cycle Number"]
    #[inline(always)]
    pub fn dithercyb(&self) -> DITHERCYB_R {
        DITHERCYB_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:23 - Channel Compare/Capture Buffer Value"]
    #[inline(always)]
    pub fn ccb(&self) -> CCB_R {
        CCB_R::new(((self.bits >> 4) & 0x000f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:3 - Dithering Buffer Cycle Number"]
    #[inline(always)]
    pub fn dithercyb(&mut self) -> DITHERCYB_W {
        DITHERCYB_W { w: self }
    }
    #[doc = "Bits 4:23 - Channel Compare/Capture Buffer Value"]
    #[inline(always)]
    pub fn ccb(&mut self) -> CCB_W {
        CCB_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Compare and Capture Buffer\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccb_dith4](index.html) module"]
pub struct CCB_DITH4_SPEC;
impl crate::RegisterSpec for CCB_DITH4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ccb_dith4::R](R) reader structure"]
impl crate::Readable for CCB_DITH4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ccb_dith4::W](W) writer structure"]
impl crate::Writable for CCB_DITH4_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CCB%s_DITH4 to value 0"]
impl crate::Resettable for CCB_DITH4_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
