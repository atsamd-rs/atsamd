#[doc = "Register `AFEC_LCDR` reader"]
pub struct R(crate::R<AFEC_LCDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AFEC_LCDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AFEC_LCDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AFEC_LCDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `LDATA` reader - Last Data Converted"]
pub struct LDATA_R(crate::FieldReader<u16, u16>);
impl LDATA_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        LDATA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LDATA_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHNB` reader - Channel Number"]
pub struct CHNB_R(crate::FieldReader<u8, u8>);
impl CHNB_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CHNB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHNB_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:15 - Last Data Converted"]
    #[inline(always)]
    pub fn ldata(&self) -> LDATA_R {
        LDATA_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 24:27 - Channel Number"]
    #[inline(always)]
    pub fn chnb(&self) -> CHNB_R {
        CHNB_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
#[doc = "AFEC Last Converted Data Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [afec_lcdr](index.html) module"]
pub struct AFEC_LCDR_SPEC;
impl crate::RegisterSpec for AFEC_LCDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [afec_lcdr::R](R) reader structure"]
impl crate::Readable for AFEC_LCDR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets AFEC_LCDR to value 0"]
impl crate::Resettable for AFEC_LCDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
