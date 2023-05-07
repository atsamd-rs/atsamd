#[doc = "Register `HPMS` reader"]
pub struct R(crate::R<HPMS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HPMS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HPMS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HPMS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `BIDX` reader - Buffer Index"]
pub type BIDX_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MSI` reader - Message Storage Indicator"]
pub type MSI_R = crate::FieldReader<u8, MSISELECT_A>;
#[doc = "Message Storage Indicator\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MSISELECT_A {
    #[doc = "0: No FIFO selected"]
    NONE = 0,
    #[doc = "1: FIFO message lost"]
    LOST = 1,
    #[doc = "2: Message stored in FIFO 0"]
    FIFO0 = 2,
    #[doc = "3: Message stored in FIFO 1"]
    FIFO1 = 3,
}
impl From<MSISELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: MSISELECT_A) -> Self {
        variant as _
    }
}
impl MSI_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSISELECT_A {
        match self.bits {
            0 => MSISELECT_A::NONE,
            1 => MSISELECT_A::LOST,
            2 => MSISELECT_A::FIFO0,
            3 => MSISELECT_A::FIFO1,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == MSISELECT_A::NONE
    }
    #[doc = "Checks if the value of the field is `LOST`"]
    #[inline(always)]
    pub fn is_lost(&self) -> bool {
        *self == MSISELECT_A::LOST
    }
    #[doc = "Checks if the value of the field is `FIFO0`"]
    #[inline(always)]
    pub fn is_fifo0(&self) -> bool {
        *self == MSISELECT_A::FIFO0
    }
    #[doc = "Checks if the value of the field is `FIFO1`"]
    #[inline(always)]
    pub fn is_fifo1(&self) -> bool {
        *self == MSISELECT_A::FIFO1
    }
}
#[doc = "Field `FIDX` reader - Filter Index"]
pub type FIDX_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FLST` reader - Filter List"]
pub type FLST_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 0:5 - Buffer Index"]
    #[inline(always)]
    pub fn bidx(&self) -> BIDX_R {
        BIDX_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:7 - Message Storage Indicator"]
    #[inline(always)]
    pub fn msi(&self) -> MSI_R {
        MSI_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:14 - Filter Index"]
    #[inline(always)]
    pub fn fidx(&self) -> FIDX_R {
        FIDX_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bit 15 - Filter List"]
    #[inline(always)]
    pub fn flst(&self) -> FLST_R {
        FLST_R::new(((self.bits >> 15) & 1) != 0)
    }
}
#[doc = "High Priority Message Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hpms](index.html) module"]
pub struct HPMS_SPEC;
impl crate::RegisterSpec for HPMS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hpms::R](R) reader structure"]
impl crate::Readable for HPMS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets HPMS to value 0"]
impl crate::Resettable for HPMS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
