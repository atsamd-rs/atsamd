#[doc = "Register `MCAN_ENDN` reader"]
pub struct R(crate::R<MCAN_ENDN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MCAN_ENDN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MCAN_ENDN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MCAN_ENDN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ETV` reader - Endianness Test Value"]
pub struct ETV_R(crate::FieldReader<u32, u32>);
impl ETV_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        ETV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ETV_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - Endianness Test Value"]
    #[inline(always)]
    pub fn etv(&self) -> ETV_R {
        ETV_R::new(self.bits as u32)
    }
}
#[doc = "Endian Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcan_endn](index.html) module"]
pub struct MCAN_ENDN_SPEC;
impl crate::RegisterSpec for MCAN_ENDN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mcan_endn::R](R) reader structure"]
impl crate::Readable for MCAN_ENDN_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets MCAN_ENDN to value 0"]
impl crate::Resettable for MCAN_ENDN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
