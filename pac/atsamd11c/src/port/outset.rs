#[doc = "Register `OUTSET%s` reader"]
pub struct R(crate::R<OUTSET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OUTSET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OUTSET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OUTSET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OUTSET%s` writer"]
pub struct W(crate::W<OUTSET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OUTSET_SPEC>;
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
impl From<crate::W<OUTSET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OUTSET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OUTSET` reader - Port Data Output Value Set"]
pub struct OUTSET_R(crate::FieldReader<u32, u32>);
impl OUTSET_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        OUTSET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OUTSET_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OUTSET` writer - Port Data Output Value Set"]
pub struct OUTSET_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTSET_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value as u32;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Port Data Output Value Set"]
    #[inline(always)]
    pub fn outset(&self) -> OUTSET_R {
        OUTSET_R::new(self.bits as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Port Data Output Value Set"]
    #[inline(always)]
    pub fn outset(&mut self) -> OUTSET_W {
        OUTSET_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Data Output Value Set\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [outset](index.html) module"]
pub struct OUTSET_SPEC;
impl crate::RegisterSpec for OUTSET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [outset::R](R) reader structure"]
impl crate::Readable for OUTSET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [outset::W](W) writer structure"]
impl crate::Writable for OUTSET_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OUTSET%s to value 0"]
impl crate::Resettable for OUTSET_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
