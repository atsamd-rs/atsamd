#[doc = "Register `DFLLMUL` reader"]
pub struct R(crate::R<DFLLMUL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DFLLMUL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DFLLMUL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DFLLMUL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DFLLMUL` writer"]
pub struct W(crate::W<DFLLMUL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DFLLMUL_SPEC>;
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
impl From<crate::W<DFLLMUL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DFLLMUL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MUL` reader - DFLL Multiply Factor"]
pub struct MUL_R(crate::FieldReader<u16, u16>);
impl MUL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        MUL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MUL_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MUL` writer - DFLL Multiply Factor"]
pub struct MUL_W<'a> {
    w: &'a mut W,
}
impl<'a> MUL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
#[doc = "Field `FSTEP` reader - Fine Maximum Step"]
pub struct FSTEP_R(crate::FieldReader<u8, u8>);
impl FSTEP_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        FSTEP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FSTEP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FSTEP` writer - Fine Maximum Step"]
pub struct FSTEP_W<'a> {
    w: &'a mut W,
}
impl<'a> FSTEP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
#[doc = "Field `CSTEP` reader - Coarse Maximum Step"]
pub struct CSTEP_R(crate::FieldReader<u8, u8>);
impl CSTEP_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CSTEP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CSTEP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CSTEP` writer - Coarse Maximum Step"]
pub struct CSTEP_W<'a> {
    w: &'a mut W,
}
impl<'a> CSTEP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 26)) | ((value as u32 & 0x3f) << 26);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - DFLL Multiply Factor"]
    #[inline(always)]
    pub fn mul(&self) -> MUL_R {
        MUL_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:23 - Fine Maximum Step"]
    #[inline(always)]
    pub fn fstep(&self) -> FSTEP_R {
        FSTEP_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 26:31 - Coarse Maximum Step"]
    #[inline(always)]
    pub fn cstep(&self) -> CSTEP_R {
        CSTEP_R::new(((self.bits >> 26) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - DFLL Multiply Factor"]
    #[inline(always)]
    pub fn mul(&mut self) -> MUL_W {
        MUL_W { w: self }
    }
    #[doc = "Bits 16:23 - Fine Maximum Step"]
    #[inline(always)]
    pub fn fstep(&mut self) -> FSTEP_W {
        FSTEP_W { w: self }
    }
    #[doc = "Bits 26:31 - Coarse Maximum Step"]
    #[inline(always)]
    pub fn cstep(&mut self) -> CSTEP_W {
        CSTEP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DFLL48M Multiplier\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfllmul](index.html) module"]
pub struct DFLLMUL_SPEC;
impl crate::RegisterSpec for DFLLMUL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dfllmul::R](R) reader structure"]
impl crate::Readable for DFLLMUL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dfllmul::W](W) writer structure"]
impl crate::Writable for DFLLMUL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DFLLMUL to value 0"]
impl crate::Resettable for DFLLMUL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
