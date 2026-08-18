#[doc = "Register `STATUS` reader"]
pub struct R(crate::R<STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `XOSCRDY` reader - XOSC Ready"]
pub struct XOSCRDY_R(crate::FieldReader<bool, bool>);
impl XOSCRDY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        XOSCRDY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for XOSCRDY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `XOSCFAIL` reader - XOSC Clock Failure Detector"]
pub struct XOSCFAIL_R(crate::FieldReader<bool, bool>);
impl XOSCFAIL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        XOSCFAIL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for XOSCFAIL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `XOSCCKSW` reader - XOSC Clock Switch"]
pub struct XOSCCKSW_R(crate::FieldReader<bool, bool>);
impl XOSCCKSW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        XOSCCKSW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for XOSCCKSW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OSC48MRDY` reader - OSC48M Ready"]
pub struct OSC48MRDY_R(crate::FieldReader<bool, bool>);
impl OSC48MRDY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OSC48MRDY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OSC48MRDY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DPLLLCKR` reader - DPLL Lock Rise"]
pub struct DPLLLCKR_R(crate::FieldReader<bool, bool>);
impl DPLLLCKR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DPLLLCKR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DPLLLCKR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DPLLLCKF` reader - DPLL Lock Fall"]
pub struct DPLLLCKF_R(crate::FieldReader<bool, bool>);
impl DPLLLCKF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DPLLLCKF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DPLLLCKF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DPLLTO` reader - DPLL Timeout"]
pub struct DPLLTO_R(crate::FieldReader<bool, bool>);
impl DPLLTO_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DPLLTO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DPLLTO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DPLLLDRTO` reader - DPLL Ratio Ready"]
pub struct DPLLLDRTO_R(crate::FieldReader<bool, bool>);
impl DPLLLDRTO_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DPLLLDRTO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DPLLLDRTO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - XOSC Ready"]
    #[inline(always)]
    pub fn xoscrdy(&self) -> XOSCRDY_R {
        XOSCRDY_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - XOSC Clock Failure Detector"]
    #[inline(always)]
    pub fn xoscfail(&self) -> XOSCFAIL_R {
        XOSCFAIL_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - XOSC Clock Switch"]
    #[inline(always)]
    pub fn xosccksw(&self) -> XOSCCKSW_R {
        XOSCCKSW_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 4 - OSC48M Ready"]
    #[inline(always)]
    pub fn osc48mrdy(&self) -> OSC48MRDY_R {
        OSC48MRDY_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 8 - DPLL Lock Rise"]
    #[inline(always)]
    pub fn dplllckr(&self) -> DPLLLCKR_R {
        DPLLLCKR_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - DPLL Lock Fall"]
    #[inline(always)]
    pub fn dplllckf(&self) -> DPLLLCKF_R {
        DPLLLCKF_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - DPLL Timeout"]
    #[inline(always)]
    pub fn dpllto(&self) -> DPLLTO_R {
        DPLLTO_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - DPLL Ratio Ready"]
    #[inline(always)]
    pub fn dpllldrto(&self) -> DPLLLDRTO_R {
        DPLLLDRTO_R::new(((self.bits >> 11) & 0x01) != 0)
    }
}
#[doc = "Power and Clocks Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status](index.html) module"]
pub struct STATUS_SPEC;
impl crate::RegisterSpec for STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [status::R](R) reader structure"]
impl crate::Readable for STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
