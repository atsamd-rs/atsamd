#[doc = "Register `TSSN` reader"]
pub struct R(crate::R<TSSN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TSSN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TSSN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TSSN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TSSN` writer"]
pub struct W(crate::W<TSSN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TSSN_SPEC>;
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
impl From<crate::W<TSSN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TSSN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VTN` reader - Value Timer Nanoseconds Register Capture"]
pub struct VTN_R(crate::FieldReader<u32, u32>);
impl VTN_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        VTN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VTN_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VTN` writer - Value Timer Nanoseconds Register Capture"]
pub struct VTN_W<'a> {
    w: &'a mut W,
}
impl<'a> VTN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3fff_ffff) | (value as u32 & 0x3fff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:29 - Value Timer Nanoseconds Register Capture"]
    #[inline(always)]
    pub fn vtn(&self) -> VTN_R {
        VTN_R::new((self.bits & 0x3fff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:29 - Value Timer Nanoseconds Register Capture"]
    #[inline(always)]
    pub fn vtn(&mut self) -> VTN_W {
        VTN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "1588 Timer Sync Strobe Nanoseconds Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tssn](index.html) module"]
pub struct TSSN_SPEC;
impl crate::RegisterSpec for TSSN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tssn::R](R) reader structure"]
impl crate::Readable for TSSN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tssn::W](W) writer structure"]
impl crate::Writable for TSSN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TSSN to value 0"]
impl crate::Resettable for TSSN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
