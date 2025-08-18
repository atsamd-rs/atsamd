#[doc = "Register `RXF0S` reader"]
pub struct R(crate::R<RXF0S_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXF0S_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXF0S_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXF0S_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `F0FL` reader - Rx FIFO 0 Fill Level"]
pub struct F0FL_R(crate::FieldReader<u8, u8>);
impl F0FL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        F0FL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for F0FL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `F0GI` reader - Rx FIFO 0 Get Index"]
pub struct F0GI_R(crate::FieldReader<u8, u8>);
impl F0GI_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        F0GI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for F0GI_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `F0PI` reader - Rx FIFO 0 Put Index"]
pub struct F0PI_R(crate::FieldReader<u8, u8>);
impl F0PI_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        F0PI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for F0PI_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `F0F` reader - Rx FIFO 0 Full"]
pub struct F0F_R(crate::FieldReader<bool, bool>);
impl F0F_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        F0F_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for F0F_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RF0L` reader - Rx FIFO 0 Message Lost"]
pub struct RF0L_R(crate::FieldReader<bool, bool>);
impl RF0L_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RF0L_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RF0L_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:6 - Rx FIFO 0 Fill Level"]
    #[inline(always)]
    pub fn f0fl(&self) -> F0FL_R {
        F0FL_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:13 - Rx FIFO 0 Get Index"]
    #[inline(always)]
    pub fn f0gi(&self) -> F0GI_R {
        F0GI_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - Rx FIFO 0 Put Index"]
    #[inline(always)]
    pub fn f0pi(&self) -> F0PI_R {
        F0PI_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bit 24 - Rx FIFO 0 Full"]
    #[inline(always)]
    pub fn f0f(&self) -> F0F_R {
        F0F_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Rx FIFO 0 Message Lost"]
    #[inline(always)]
    pub fn rf0l(&self) -> RF0L_R {
        RF0L_R::new(((self.bits >> 25) & 0x01) != 0)
    }
}
#[doc = "Rx FIFO 0 Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxf0s](index.html) module"]
pub struct RXF0S_SPEC;
impl crate::RegisterSpec for RXF0S_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rxf0s::R](R) reader structure"]
impl crate::Readable for RXF0S_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RXF0S to value 0"]
impl crate::Resettable for RXF0S_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
