#[doc = "Register `STATUSB` reader"]
pub struct R(crate::R<STATUSB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STATUSB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STATUSB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STATUSB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PROT` reader - Protected"]
pub struct PROT_R(crate::FieldReader<bool, bool>);
impl PROT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PROT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PROT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DBGPRES` reader - Debugger Present"]
pub struct DBGPRES_R(crate::FieldReader<bool, bool>);
impl DBGPRES_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DBGPRES_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DBGPRES_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DCCD0` reader - Debug Communication Channel 0 Dirty"]
pub struct DCCD0_R(crate::FieldReader<bool, bool>);
impl DCCD0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DCCD0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DCCD0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DCCD1` reader - Debug Communication Channel 1 Dirty"]
pub struct DCCD1_R(crate::FieldReader<bool, bool>);
impl DCCD1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DCCD1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DCCD1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HPE` reader - Hot-Plugging Enable"]
pub struct HPE_R(crate::FieldReader<bool, bool>);
impl HPE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        HPE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HPE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Protected"]
    #[inline(always)]
    pub fn prot(&self) -> PROT_R {
        PROT_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Debugger Present"]
    #[inline(always)]
    pub fn dbgpres(&self) -> DBGPRES_R {
        DBGPRES_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Debug Communication Channel 0 Dirty"]
    #[inline(always)]
    pub fn dccd0(&self) -> DCCD0_R {
        DCCD0_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Debug Communication Channel 1 Dirty"]
    #[inline(always)]
    pub fn dccd1(&self) -> DCCD1_R {
        DCCD1_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Hot-Plugging Enable"]
    #[inline(always)]
    pub fn hpe(&self) -> HPE_R {
        HPE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
#[doc = "Status B\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [statusb](index.html) module"]
pub struct STATUSB_SPEC;
impl crate::RegisterSpec for STATUSB_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [statusb::R](R) reader structure"]
impl crate::Readable for STATUSB_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets STATUSB to value 0x10"]
impl crate::Resettable for STATUSB_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x10
    }
}
