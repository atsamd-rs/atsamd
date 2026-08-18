#[doc = "Register `RXF1S` reader"]
pub struct R(crate::R<RXF1S_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXF1S_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXF1S_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXF1S_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `F1FL` reader - Rx FIFO 1 Fill Level"]
pub struct F1FL_R(crate::FieldReader<u8, u8>);
impl F1FL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        F1FL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for F1FL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `F1GI` reader - Rx FIFO 1 Get Index"]
pub struct F1GI_R(crate::FieldReader<u8, u8>);
impl F1GI_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        F1GI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for F1GI_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `F1PI` reader - Rx FIFO 1 Put Index"]
pub struct F1PI_R(crate::FieldReader<u8, u8>);
impl F1PI_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        F1PI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for F1PI_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `F1F` reader - Rx FIFO 1 Full"]
pub struct F1F_R(crate::FieldReader<bool, bool>);
impl F1F_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        F1F_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for F1F_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RF1L` reader - Rx FIFO 1 Message Lost"]
pub struct RF1L_R(crate::FieldReader<bool, bool>);
impl RF1L_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RF1L_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RF1L_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Debug Message Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DMS_A {
    #[doc = "0: Idle state"]
    IDLE = 0,
    #[doc = "1: Debug message A received"]
    DBGA = 1,
    #[doc = "2: Debug message A/B received"]
    DBGB = 2,
    #[doc = "3: Debug message A/B/C received, DMA request set"]
    DBGC = 3,
}
impl From<DMS_A> for u8 {
    #[inline(always)]
    fn from(variant: DMS_A) -> Self {
        variant as _
    }
}
#[doc = "Field `DMS` reader - Debug Message Status"]
pub struct DMS_R(crate::FieldReader<u8, DMS_A>);
impl DMS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DMS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMS_A {
        match self.bits {
            0 => DMS_A::IDLE,
            1 => DMS_A::DBGA,
            2 => DMS_A::DBGB,
            3 => DMS_A::DBGC,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `IDLE`"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        **self == DMS_A::IDLE
    }
    #[doc = "Checks if the value of the field is `DBGA`"]
    #[inline(always)]
    pub fn is_dbga(&self) -> bool {
        **self == DMS_A::DBGA
    }
    #[doc = "Checks if the value of the field is `DBGB`"]
    #[inline(always)]
    pub fn is_dbgb(&self) -> bool {
        **self == DMS_A::DBGB
    }
    #[doc = "Checks if the value of the field is `DBGC`"]
    #[inline(always)]
    pub fn is_dbgc(&self) -> bool {
        **self == DMS_A::DBGC
    }
}
impl core::ops::Deref for DMS_R {
    type Target = crate::FieldReader<u8, DMS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:6 - Rx FIFO 1 Fill Level"]
    #[inline(always)]
    pub fn f1fl(&self) -> F1FL_R {
        F1FL_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:13 - Rx FIFO 1 Get Index"]
    #[inline(always)]
    pub fn f1gi(&self) -> F1GI_R {
        F1GI_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - Rx FIFO 1 Put Index"]
    #[inline(always)]
    pub fn f1pi(&self) -> F1PI_R {
        F1PI_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bit 24 - Rx FIFO 1 Full"]
    #[inline(always)]
    pub fn f1f(&self) -> F1F_R {
        F1F_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Rx FIFO 1 Message Lost"]
    #[inline(always)]
    pub fn rf1l(&self) -> RF1L_R {
        RF1L_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bits 30:31 - Debug Message Status"]
    #[inline(always)]
    pub fn dms(&self) -> DMS_R {
        DMS_R::new(((self.bits >> 30) & 0x03) as u8)
    }
}
#[doc = "Rx FIFO 1 Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxf1s](index.html) module"]
pub struct RXF1S_SPEC;
impl crate::RegisterSpec for RXF1S_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rxf1s::R](R) reader structure"]
impl crate::Readable for RXF1S_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RXF1S to value 0"]
impl crate::Resettable for RXF1S_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
