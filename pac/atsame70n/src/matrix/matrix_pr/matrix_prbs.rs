#[doc = "Register `MATRIX_PRBS` reader"]
pub struct R(crate::R<MATRIX_PRBS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MATRIX_PRBS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MATRIX_PRBS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MATRIX_PRBS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MATRIX_PRBS` writer"]
pub struct W(crate::W<MATRIX_PRBS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MATRIX_PRBS_SPEC>;
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
impl From<crate::W<MATRIX_PRBS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MATRIX_PRBS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `M8PR` reader - Master 8 Priority"]
pub struct M8PR_R(crate::FieldReader<u8, u8>);
impl M8PR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        M8PR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for M8PR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `M8PR` writer - Master 8 Priority"]
pub struct M8PR_W<'a> {
    w: &'a mut W,
}
impl<'a> M8PR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "Field `M9PR` reader - Master 9 Priority"]
pub struct M9PR_R(crate::FieldReader<u8, u8>);
impl M9PR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        M9PR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for M9PR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `M9PR` writer - Master 9 Priority"]
pub struct M9PR_W<'a> {
    w: &'a mut W,
}
impl<'a> M9PR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
#[doc = "Field `M10PR` reader - Master 10 Priority"]
pub struct M10PR_R(crate::FieldReader<u8, u8>);
impl M10PR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        M10PR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for M10PR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `M10PR` writer - Master 10 Priority"]
pub struct M10PR_W<'a> {
    w: &'a mut W,
}
impl<'a> M10PR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | ((value as u32 & 0x03) << 8);
        self.w
    }
}
#[doc = "Field `M11PR` reader - Master 11 Priority"]
pub struct M11PR_R(crate::FieldReader<u8, u8>);
impl M11PR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        M11PR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for M11PR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `M11PR` writer - Master 11 Priority"]
pub struct M11PR_W<'a> {
    w: &'a mut W,
}
impl<'a> M11PR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | ((value as u32 & 0x03) << 12);
        self.w
    }
}
#[doc = "Field `M12PR` reader - Master 12 Priority"]
pub struct M12PR_R(crate::FieldReader<u8, u8>);
impl M12PR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        M12PR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for M12PR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `M12PR` writer - Master 12 Priority"]
pub struct M12PR_W<'a> {
    w: &'a mut W,
}
impl<'a> M12PR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | ((value as u32 & 0x03) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Master 8 Priority"]
    #[inline(always)]
    pub fn m8pr(&self) -> M8PR_R {
        M8PR_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - Master 9 Priority"]
    #[inline(always)]
    pub fn m9pr(&self) -> M9PR_R {
        M9PR_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - Master 10 Priority"]
    #[inline(always)]
    pub fn m10pr(&self) -> M10PR_R {
        M10PR_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 12:13 - Master 11 Priority"]
    #[inline(always)]
    pub fn m11pr(&self) -> M11PR_R {
        M11PR_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 16:17 - Master 12 Priority"]
    #[inline(always)]
    pub fn m12pr(&self) -> M12PR_R {
        M12PR_R::new(((self.bits >> 16) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Master 8 Priority"]
    #[inline(always)]
    pub fn m8pr(&mut self) -> M8PR_W {
        M8PR_W { w: self }
    }
    #[doc = "Bits 4:5 - Master 9 Priority"]
    #[inline(always)]
    pub fn m9pr(&mut self) -> M9PR_W {
        M9PR_W { w: self }
    }
    #[doc = "Bits 8:9 - Master 10 Priority"]
    #[inline(always)]
    pub fn m10pr(&mut self) -> M10PR_W {
        M10PR_W { w: self }
    }
    #[doc = "Bits 12:13 - Master 11 Priority"]
    #[inline(always)]
    pub fn m11pr(&mut self) -> M11PR_W {
        M11PR_W { w: self }
    }
    #[doc = "Bits 16:17 - Master 12 Priority"]
    #[inline(always)]
    pub fn m12pr(&mut self) -> M12PR_W {
        M12PR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Priority Register B for Slave 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [matrix_prbs](index.html) module"]
pub struct MATRIX_PRBS_SPEC;
impl crate::RegisterSpec for MATRIX_PRBS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [matrix_prbs::R](R) reader structure"]
impl crate::Readable for MATRIX_PRBS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [matrix_prbs::W](W) writer structure"]
impl crate::Writable for MATRIX_PRBS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MATRIX_PRBS to value 0"]
impl crate::Resettable for MATRIX_PRBS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
