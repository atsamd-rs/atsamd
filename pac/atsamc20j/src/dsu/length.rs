#[doc = "Register `LENGTH` reader"]
pub struct R(crate::R<LENGTH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LENGTH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LENGTH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LENGTH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LENGTH` writer"]
pub struct W(crate::W<LENGTH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LENGTH_SPEC>;
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
impl From<crate::W<LENGTH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LENGTH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LENGTH` reader - Length"]
pub struct LENGTH_R(crate::FieldReader<u32, u32>);
impl LENGTH_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        LENGTH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LENGTH_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LENGTH` writer - Length"]
pub struct LENGTH_W<'a> {
    w: &'a mut W,
}
impl<'a> LENGTH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3fff_ffff << 2)) | ((value as u32 & 0x3fff_ffff) << 2);
        self.w
    }
}
impl R {
    #[doc = "Bits 2:31 - Length"]
    #[inline(always)]
    pub fn length(&self) -> LENGTH_R {
        LENGTH_R::new(((self.bits >> 2) & 0x3fff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 2:31 - Length"]
    #[inline(always)]
    pub fn length(&mut self) -> LENGTH_W {
        LENGTH_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Length\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [length](index.html) module"]
pub struct LENGTH_SPEC;
impl crate::RegisterSpec for LENGTH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [length::R](R) reader structure"]
impl crate::Readable for LENGTH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [length::W](W) writer structure"]
impl crate::Writable for LENGTH_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LENGTH to value 0"]
impl crate::Resettable for LENGTH_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
