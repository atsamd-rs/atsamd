#[doc = "Register `AES_ISR` reader"]
pub struct R(crate::R<AES_ISR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AES_ISR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AES_ISR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AES_ISR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DATRDY` reader - Data Ready (cleared by setting bit START or bit SWRST in AES_CR or by reading AES_ODATARx)"]
pub struct DATRDY_R(crate::FieldReader<bool, bool>);
impl DATRDY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DATRDY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DATRDY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `URAD` reader - Unspecified Register Access Detection Status (cleared by writing SWRST in AES_CR)"]
pub struct URAD_R(crate::FieldReader<bool, bool>);
impl URAD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        URAD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for URAD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Unspecified Register Access (cleared by writing SWRST in AES_CR)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum URAT_A {
    #[doc = "0: Input Data Register written during the data processing when SMOD = 0x2 mode."]
    IDR_WR_PROCESSING = 0,
    #[doc = "1: Output Data Register read during the data processing."]
    ODR_RD_PROCESSING = 1,
    #[doc = "2: Mode Register written during the data processing."]
    MR_WR_PROCESSING = 2,
    #[doc = "3: Output Data Register read during the sub-keys generation."]
    ODR_RD_SUBKGEN = 3,
    #[doc = "4: Mode Register written during the sub-keys generation."]
    MR_WR_SUBKGEN = 4,
    #[doc = "5: Write-only register read access."]
    WOR_RD_ACCESS = 5,
}
impl From<URAT_A> for u8 {
    #[inline(always)]
    fn from(variant: URAT_A) -> Self {
        variant as _
    }
}
#[doc = "Field `URAT` reader - Unspecified Register Access (cleared by writing SWRST in AES_CR)"]
pub struct URAT_R(crate::FieldReader<u8, URAT_A>);
impl URAT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        URAT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<URAT_A> {
        match self.bits {
            0 => Some(URAT_A::IDR_WR_PROCESSING),
            1 => Some(URAT_A::ODR_RD_PROCESSING),
            2 => Some(URAT_A::MR_WR_PROCESSING),
            3 => Some(URAT_A::ODR_RD_SUBKGEN),
            4 => Some(URAT_A::MR_WR_SUBKGEN),
            5 => Some(URAT_A::WOR_RD_ACCESS),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `IDR_WR_PROCESSING`"]
    #[inline(always)]
    pub fn is_idr_wr_processing(&self) -> bool {
        **self == URAT_A::IDR_WR_PROCESSING
    }
    #[doc = "Checks if the value of the field is `ODR_RD_PROCESSING`"]
    #[inline(always)]
    pub fn is_odr_rd_processing(&self) -> bool {
        **self == URAT_A::ODR_RD_PROCESSING
    }
    #[doc = "Checks if the value of the field is `MR_WR_PROCESSING`"]
    #[inline(always)]
    pub fn is_mr_wr_processing(&self) -> bool {
        **self == URAT_A::MR_WR_PROCESSING
    }
    #[doc = "Checks if the value of the field is `ODR_RD_SUBKGEN`"]
    #[inline(always)]
    pub fn is_odr_rd_subkgen(&self) -> bool {
        **self == URAT_A::ODR_RD_SUBKGEN
    }
    #[doc = "Checks if the value of the field is `MR_WR_SUBKGEN`"]
    #[inline(always)]
    pub fn is_mr_wr_subkgen(&self) -> bool {
        **self == URAT_A::MR_WR_SUBKGEN
    }
    #[doc = "Checks if the value of the field is `WOR_RD_ACCESS`"]
    #[inline(always)]
    pub fn is_wor_rd_access(&self) -> bool {
        **self == URAT_A::WOR_RD_ACCESS
    }
}
impl core::ops::Deref for URAT_R {
    type Target = crate::FieldReader<u8, URAT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TAGRDY` reader - GCM Tag Ready"]
pub struct TAGRDY_R(crate::FieldReader<bool, bool>);
impl TAGRDY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TAGRDY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TAGRDY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Data Ready (cleared by setting bit START or bit SWRST in AES_CR or by reading AES_ODATARx)"]
    #[inline(always)]
    pub fn datrdy(&self) -> DATRDY_R {
        DATRDY_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 8 - Unspecified Register Access Detection Status (cleared by writing SWRST in AES_CR)"]
    #[inline(always)]
    pub fn urad(&self) -> URAD_R {
        URAD_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 12:15 - Unspecified Register Access (cleared by writing SWRST in AES_CR)"]
    #[inline(always)]
    pub fn urat(&self) -> URAT_R {
        URAT_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bit 16 - GCM Tag Ready"]
    #[inline(always)]
    pub fn tagrdy(&self) -> TAGRDY_R {
        TAGRDY_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
#[doc = "Interrupt Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aes_isr](index.html) module"]
pub struct AES_ISR_SPEC;
impl crate::RegisterSpec for AES_ISR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [aes_isr::R](R) reader structure"]
impl crate::Readable for AES_ISR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets AES_ISR to value 0"]
impl crate::Resettable for AES_ISR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
