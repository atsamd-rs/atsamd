#[doc = "Register `DATABUF` reader"]
pub struct R(crate::R<DATABUF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DATABUF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DATABUF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DATABUF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DATABUF` writer"]
pub struct W(crate::W<DATABUF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DATABUF_SPEC>;
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
impl From<crate::W<DATABUF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DATABUF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATABUF` reader - Data Buffer"]
pub type DATABUF_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DATABUF` writer - Data Buffer"]
pub type DATABUF_W<'a, const O: u8> = crate::FieldWriter<'a, u16, DATABUF_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Data Buffer"]
    #[inline(always)]
    pub fn databuf(&self) -> DATABUF_R {
        DATABUF_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - Data Buffer"]
    #[inline(always)]
    #[must_use]
    pub fn databuf(&mut self) -> DATABUF_W<0> {
        DATABUF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Data Buffer\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [databuf](index.html) module"]
pub struct DATABUF_SPEC;
impl crate::RegisterSpec for DATABUF_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [databuf::R](R) reader structure"]
impl crate::Readable for DATABUF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [databuf::W](W) writer structure"]
impl crate::Writable for DATABUF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DATABUF to value 0"]
impl crate::Resettable for DATABUF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
