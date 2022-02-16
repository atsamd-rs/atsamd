#[doc = "Register `ISI_PDECF` reader"]
pub struct R(crate::R<ISI_PDECF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ISI_PDECF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ISI_PDECF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ISI_PDECF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ISI_PDECF` writer"]
pub struct W(crate::W<ISI_PDECF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ISI_PDECF_SPEC>;
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
impl From<crate::W<ISI_PDECF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ISI_PDECF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DEC_FACTOR` reader - Decimation Factor"]
pub struct DEC_FACTOR_R(crate::FieldReader<u8, u8>);
impl DEC_FACTOR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DEC_FACTOR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DEC_FACTOR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DEC_FACTOR` writer - Decimation Factor"]
pub struct DEC_FACTOR_W<'a> {
    w: &'a mut W,
}
impl<'a> DEC_FACTOR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Decimation Factor"]
    #[inline(always)]
    pub fn dec_factor(&self) -> DEC_FACTOR_R {
        DEC_FACTOR_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Decimation Factor"]
    #[inline(always)]
    pub fn dec_factor(&mut self) -> DEC_FACTOR_W {
        DEC_FACTOR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ISI Preview Decimation Factor Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [isi_pdecf](index.html) module"]
pub struct ISI_PDECF_SPEC;
impl crate::RegisterSpec for ISI_PDECF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [isi_pdecf::R](R) reader structure"]
impl crate::Readable for ISI_PDECF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [isi_pdecf::W](W) writer structure"]
impl crate::Writable for ISI_PDECF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ISI_PDECF to value 0"]
impl crate::Resettable for ISI_PDECF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
