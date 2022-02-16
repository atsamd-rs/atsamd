#[doc = "Register `MCAN_HPMS` reader"]
pub struct R(crate::R<MCAN_HPMS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MCAN_HPMS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MCAN_HPMS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MCAN_HPMS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `BIDX` reader - Buffer Index"]
pub struct BIDX_R(crate::FieldReader<u8, u8>);
impl BIDX_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        BIDX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BIDX_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Message Storage Indicator\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MSI_A {
    #[doc = "0: No FIFO selected."]
    NO_FIFO_SEL = 0,
    #[doc = "1: FIFO message lost."]
    LOST = 1,
    #[doc = "2: Message stored in FIFO 0."]
    FIFO_0 = 2,
    #[doc = "3: Message stored in FIFO 1."]
    FIFO_1 = 3,
}
impl From<MSI_A> for u8 {
    #[inline(always)]
    fn from(variant: MSI_A) -> Self {
        variant as _
    }
}
#[doc = "Field `MSI` reader - Message Storage Indicator"]
pub struct MSI_R(crate::FieldReader<u8, MSI_A>);
impl MSI_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        MSI_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSI_A {
        match self.bits {
            0 => MSI_A::NO_FIFO_SEL,
            1 => MSI_A::LOST,
            2 => MSI_A::FIFO_0,
            3 => MSI_A::FIFO_1,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NO_FIFO_SEL`"]
    #[inline(always)]
    pub fn is_no_fifo_sel(&self) -> bool {
        **self == MSI_A::NO_FIFO_SEL
    }
    #[doc = "Checks if the value of the field is `LOST`"]
    #[inline(always)]
    pub fn is_lost(&self) -> bool {
        **self == MSI_A::LOST
    }
    #[doc = "Checks if the value of the field is `FIFO_0`"]
    #[inline(always)]
    pub fn is_fifo_0(&self) -> bool {
        **self == MSI_A::FIFO_0
    }
    #[doc = "Checks if the value of the field is `FIFO_1`"]
    #[inline(always)]
    pub fn is_fifo_1(&self) -> bool {
        **self == MSI_A::FIFO_1
    }
}
impl core::ops::Deref for MSI_R {
    type Target = crate::FieldReader<u8, MSI_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FIDX` reader - Filter Index"]
pub struct FIDX_R(crate::FieldReader<u8, u8>);
impl FIDX_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        FIDX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FIDX_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLST` reader - Filter List"]
pub struct FLST_R(crate::FieldReader<bool, bool>);
impl FLST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FLST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:5 - Buffer Index"]
    #[inline(always)]
    pub fn bidx(&self) -> BIDX_R {
        BIDX_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:7 - Message Storage Indicator"]
    #[inline(always)]
    pub fn msi(&self) -> MSI_R {
        MSI_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 8:14 - Filter Index"]
    #[inline(always)]
    pub fn fidx(&self) -> FIDX_R {
        FIDX_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bit 15 - Filter List"]
    #[inline(always)]
    pub fn flst(&self) -> FLST_R {
        FLST_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
#[doc = "High Priority Message Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcan_hpms](index.html) module"]
pub struct MCAN_HPMS_SPEC;
impl crate::RegisterSpec for MCAN_HPMS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mcan_hpms::R](R) reader structure"]
impl crate::Readable for MCAN_HPMS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets MCAN_HPMS to value 0"]
impl crate::Resettable for MCAN_HPMS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
