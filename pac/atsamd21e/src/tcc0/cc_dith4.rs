#[doc = "Register `CC%s_DITH4` reader"]
pub struct R(crate::R<CC_DITH4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CC_DITH4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CC_DITH4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CC_DITH4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CC%s_DITH4` writer"]
pub struct W(crate::W<CC_DITH4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CC_DITH4_SPEC>;
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
impl From<crate::W<CC_DITH4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CC_DITH4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DITHERCY` reader - Dithering Cycle Number"]
pub struct DITHERCY_R(crate::FieldReader<u8, u8>);
impl DITHERCY_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DITHERCY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DITHERCY_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DITHERCY` writer - Dithering Cycle Number"]
pub struct DITHERCY_W<'a> {
    w: &'a mut W,
}
impl<'a> DITHERCY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
#[doc = "Field `CC` reader - Channel Compare/Capture Value"]
pub struct CC_R(crate::FieldReader<u32, u32>);
impl CC_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        CC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CC_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CC` writer - Channel Compare/Capture Value"]
pub struct CC_W<'a> {
    w: &'a mut W,
}
impl<'a> CC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x000f_ffff << 4)) | ((value as u32 & 0x000f_ffff) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Dithering Cycle Number"]
    #[inline(always)]
    pub fn dithercy(&self) -> DITHERCY_R {
        DITHERCY_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:23 - Channel Compare/Capture Value"]
    #[inline(always)]
    pub fn cc(&self) -> CC_R {
        CC_R::new(((self.bits >> 4) & 0x000f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:3 - Dithering Cycle Number"]
    #[inline(always)]
    pub fn dithercy(&mut self) -> DITHERCY_W {
        DITHERCY_W { w: self }
    }
    #[doc = "Bits 4:23 - Channel Compare/Capture Value"]
    #[inline(always)]
    pub fn cc(&mut self) -> CC_W {
        CC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Compare and Capture\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cc_dith4](index.html) module"]
pub struct CC_DITH4_SPEC;
impl crate::RegisterSpec for CC_DITH4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cc_dith4::R](R) reader structure"]
impl crate::Readable for CC_DITH4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cc_dith4::W](W) writer structure"]
impl crate::Writable for CC_DITH4_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CC%s_DITH4 to value 0"]
impl crate::Resettable for CC_DITH4_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
