#[doc = "Register `DSEQSTAT` reader"]
pub struct R(crate::R<DSEQSTAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DSEQSTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DSEQSTAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DSEQSTAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `INPUTCTRL` reader - Input Control"]
pub struct INPUTCTRL_R(crate::FieldReader<bool, bool>);
impl INPUTCTRL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        INPUTCTRL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INPUTCTRL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTRLB` reader - Control B"]
pub struct CTRLB_R(crate::FieldReader<bool, bool>);
impl CTRLB_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CTRLB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTRLB_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REFCTRL` reader - Reference Control"]
pub struct REFCTRL_R(crate::FieldReader<bool, bool>);
impl REFCTRL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        REFCTRL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REFCTRL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AVGCTRL` reader - Average Control"]
pub struct AVGCTRL_R(crate::FieldReader<bool, bool>);
impl AVGCTRL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        AVGCTRL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AVGCTRL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SAMPCTRL` reader - Sampling Time Control"]
pub struct SAMPCTRL_R(crate::FieldReader<bool, bool>);
impl SAMPCTRL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SAMPCTRL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SAMPCTRL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WINLT` reader - Window Monitor Lower Threshold"]
pub struct WINLT_R(crate::FieldReader<bool, bool>);
impl WINLT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WINLT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WINLT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WINUT` reader - Window Monitor Upper Threshold"]
pub struct WINUT_R(crate::FieldReader<bool, bool>);
impl WINUT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WINUT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WINUT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GAINCORR` reader - Gain Correction"]
pub struct GAINCORR_R(crate::FieldReader<bool, bool>);
impl GAINCORR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GAINCORR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GAINCORR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OFFSETCORR` reader - Offset Correction"]
pub struct OFFSETCORR_R(crate::FieldReader<bool, bool>);
impl OFFSETCORR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OFFSETCORR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OFFSETCORR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BUSY` reader - DMA Sequencing Busy"]
pub struct BUSY_R(crate::FieldReader<bool, bool>);
impl BUSY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BUSY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BUSY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Input Control"]
    #[inline(always)]
    pub fn inputctrl(&self) -> INPUTCTRL_R {
        INPUTCTRL_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Control B"]
    #[inline(always)]
    pub fn ctrlb(&self) -> CTRLB_R {
        CTRLB_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Reference Control"]
    #[inline(always)]
    pub fn refctrl(&self) -> REFCTRL_R {
        REFCTRL_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Average Control"]
    #[inline(always)]
    pub fn avgctrl(&self) -> AVGCTRL_R {
        AVGCTRL_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Sampling Time Control"]
    #[inline(always)]
    pub fn sampctrl(&self) -> SAMPCTRL_R {
        SAMPCTRL_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Window Monitor Lower Threshold"]
    #[inline(always)]
    pub fn winlt(&self) -> WINLT_R {
        WINLT_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Window Monitor Upper Threshold"]
    #[inline(always)]
    pub fn winut(&self) -> WINUT_R {
        WINUT_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Gain Correction"]
    #[inline(always)]
    pub fn gaincorr(&self) -> GAINCORR_R {
        GAINCORR_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Offset Correction"]
    #[inline(always)]
    pub fn offsetcorr(&self) -> OFFSETCORR_R {
        OFFSETCORR_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 31 - DMA Sequencing Busy"]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
#[doc = "DMA Sequencial Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dseqstat](index.html) module"]
pub struct DSEQSTAT_SPEC;
impl crate::RegisterSpec for DSEQSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dseqstat::R](R) reader structure"]
impl crate::Readable for DSEQSTAT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DSEQSTAT to value 0"]
impl crate::Resettable for DSEQSTAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
