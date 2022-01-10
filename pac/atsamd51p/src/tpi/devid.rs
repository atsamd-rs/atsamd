#[doc = "Register `DEVID` reader"]
pub struct R(crate::R<DEVID_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DEVID_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DEVID_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DEVID_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `NrTraceInput` reader - "]
pub struct NRTRACEINPUT_R(crate::FieldReader<bool, bool>);
impl NRTRACEINPUT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        NRTRACEINPUT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NRTRACEINPUT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AsynClkIn` reader - "]
pub struct ASYNCLKIN_R(crate::FieldReader<bool, bool>);
impl ASYNCLKIN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ASYNCLKIN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ASYNCLKIN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MinBufSz` reader - "]
pub struct MINBUFSZ_R(crate::FieldReader<u8, u8>);
impl MINBUFSZ_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        MINBUFSZ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MINBUFSZ_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PTINVALID` reader - "]
pub struct PTINVALID_R(crate::FieldReader<bool, bool>);
impl PTINVALID_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PTINVALID_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PTINVALID_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MANCVALID` reader - "]
pub struct MANCVALID_R(crate::FieldReader<bool, bool>);
impl MANCVALID_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MANCVALID_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MANCVALID_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NRZVALID` reader - "]
pub struct NRZVALID_R(crate::FieldReader<bool, bool>);
impl NRZVALID_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        NRZVALID_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NRZVALID_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn nr_trace_input(&self) -> NRTRACEINPUT_R {
        NRTRACEINPUT_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn asyn_clk_in(&self) -> ASYNCLKIN_R {
        ASYNCLKIN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bits 6:8"]
    #[inline(always)]
    pub fn min_buf_sz(&self) -> MINBUFSZ_R {
        MINBUFSZ_R::new(((self.bits >> 6) & 0x07) as u8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn ptinvalid(&self) -> PTINVALID_R {
        PTINVALID_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn mancvalid(&self) -> MANCVALID_R {
        MANCVALID_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn nrzvalid(&self) -> NRZVALID_R {
        NRZVALID_R::new(((self.bits >> 11) & 0x01) != 0)
    }
}
#[doc = "TPIU_DEVID\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [devid](index.html) module"]
pub struct DEVID_SPEC;
impl crate::RegisterSpec for DEVID_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [devid::R](R) reader structure"]
impl crate::Readable for DEVID_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DEVID to value 0"]
impl crate::Resettable for DEVID_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
