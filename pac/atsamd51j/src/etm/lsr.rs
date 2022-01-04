#[doc = "Register `LSR` reader"]
pub struct R(crate::R<LSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `Present` reader - "]
pub struct PRESENT_R(crate::FieldReader<bool, bool>);
impl PRESENT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PRESENT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRESENT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `Access` reader - "]
pub struct ACCESS_R(crate::FieldReader<bool, bool>);
impl ACCESS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ACCESS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ACCESS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ByteAcc` reader - "]
pub struct BYTEACC_R(crate::FieldReader<bool, bool>);
impl BYTEACC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BYTEACC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BYTEACC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn present(&self) -> PRESENT_R {
        PRESENT_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn access(&self) -> ACCESS_R {
        ACCESS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn byte_acc(&self) -> BYTEACC_R {
        BYTEACC_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
#[doc = "ETM Lock Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lsr](index.html) module"]
pub struct LSR_SPEC;
impl crate::RegisterSpec for LSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lsr::R](R) reader structure"]
impl crate::Readable for LSR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets LSR to value 0"]
impl crate::Resettable for LSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
