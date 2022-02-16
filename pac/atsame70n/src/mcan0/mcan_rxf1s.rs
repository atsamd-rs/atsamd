#[doc = "Register `MCAN_RXF1S` reader"]
pub struct R(crate::R<MCAN_RXF1S_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MCAN_RXF1S_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MCAN_RXF1S_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MCAN_RXF1S_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `F1FL` reader - Receive FIFO 1 Fill Level"]
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
#[doc = "Field `F1GI` reader - Receive FIFO 1 Get Index"]
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
#[doc = "Field `F1PI` reader - Receive FIFO 1 Put Index"]
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
#[doc = "Field `F1F` reader - Receive FIFO 1 Fill Level"]
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
#[doc = "Field `RF1L` reader - Receive FIFO 1 Message Lost"]
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
    #[doc = "0: Idle state, wait for reception of debug messages, DMA request is cleared."]
    IDLE = 0,
    #[doc = "1: Debug message A received."]
    MSG_A = 1,
    #[doc = "2: Debug messages A, B received."]
    MSG_AB = 2,
    #[doc = "3: Debug messages A, B, C received, DMA request is set."]
    MSG_ABC = 3,
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
            1 => DMS_A::MSG_A,
            2 => DMS_A::MSG_AB,
            3 => DMS_A::MSG_ABC,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `IDLE`"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        **self == DMS_A::IDLE
    }
    #[doc = "Checks if the value of the field is `MSG_A`"]
    #[inline(always)]
    pub fn is_msg_a(&self) -> bool {
        **self == DMS_A::MSG_A
    }
    #[doc = "Checks if the value of the field is `MSG_AB`"]
    #[inline(always)]
    pub fn is_msg_ab(&self) -> bool {
        **self == DMS_A::MSG_AB
    }
    #[doc = "Checks if the value of the field is `MSG_ABC`"]
    #[inline(always)]
    pub fn is_msg_abc(&self) -> bool {
        **self == DMS_A::MSG_ABC
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
    #[doc = "Bits 0:6 - Receive FIFO 1 Fill Level"]
    #[inline(always)]
    pub fn f1fl(&self) -> F1FL_R {
        F1FL_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:13 - Receive FIFO 1 Get Index"]
    #[inline(always)]
    pub fn f1gi(&self) -> F1GI_R {
        F1GI_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - Receive FIFO 1 Put Index"]
    #[inline(always)]
    pub fn f1pi(&self) -> F1PI_R {
        F1PI_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bit 24 - Receive FIFO 1 Fill Level"]
    #[inline(always)]
    pub fn f1f(&self) -> F1F_R {
        F1F_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Receive FIFO 1 Message Lost"]
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
#[doc = "Receive FIFO 1 Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcan_rxf1s](index.html) module"]
pub struct MCAN_RXF1S_SPEC;
impl crate::RegisterSpec for MCAN_RXF1S_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mcan_rxf1s::R](R) reader structure"]
impl crate::Readable for MCAN_RXF1S_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets MCAN_RXF1S to value 0"]
impl crate::Resettable for MCAN_RXF1S_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
