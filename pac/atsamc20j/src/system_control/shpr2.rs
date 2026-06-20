#[doc = "Register `SHPR2` reader"]
pub struct R(crate::R<SHPR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SHPR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SHPR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SHPR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SHPR2` writer"]
pub struct W(crate::W<SHPR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SHPR2_SPEC>;
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
impl From<crate::W<SHPR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SHPR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRI_11` reader - Priority of system handler 11, SVCall"]
pub struct PRI_11_R(crate::FieldReader<u8, u8>);
impl PRI_11_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PRI_11_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRI_11_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRI_11` writer - Priority of system handler 11, SVCall"]
pub struct PRI_11_W<'a> {
    w: &'a mut W,
}
impl<'a> PRI_11_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | ((value as u32 & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:31 - Priority of system handler 11, SVCall"]
    #[inline(always)]
    pub fn pri_11(&self) -> PRI_11_R {
        PRI_11_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 24:31 - Priority of system handler 11, SVCall"]
    #[inline(always)]
    pub fn pri_11(&mut self) -> PRI_11_W {
        PRI_11_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "System Handler Priority Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [shpr2](index.html) module"]
pub struct SHPR2_SPEC;
impl crate::RegisterSpec for SHPR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [shpr2::R](R) reader structure"]
impl crate::Readable for SHPR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [shpr2::W](W) writer structure"]
impl crate::Writable for SHPR2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SHPR2 to value 0"]
impl crate::Resettable for SHPR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
