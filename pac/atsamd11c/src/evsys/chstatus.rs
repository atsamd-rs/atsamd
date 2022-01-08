#[doc = "Register `CHSTATUS` reader"]
pub struct R(crate::R<CHSTATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CHSTATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CHSTATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CHSTATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `USRRDY0` reader - Channel 0 User Ready"]
pub struct USRRDY0_R(crate::FieldReader<bool, bool>);
impl USRRDY0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        USRRDY0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USRRDY0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USRRDY1` reader - Channel 1 User Ready"]
pub struct USRRDY1_R(crate::FieldReader<bool, bool>);
impl USRRDY1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        USRRDY1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USRRDY1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USRRDY2` reader - Channel 2 User Ready"]
pub struct USRRDY2_R(crate::FieldReader<bool, bool>);
impl USRRDY2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        USRRDY2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USRRDY2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USRRDY3` reader - Channel 3 User Ready"]
pub struct USRRDY3_R(crate::FieldReader<bool, bool>);
impl USRRDY3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        USRRDY3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USRRDY3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USRRDY4` reader - Channel 4 User Ready"]
pub struct USRRDY4_R(crate::FieldReader<bool, bool>);
impl USRRDY4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        USRRDY4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USRRDY4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USRRDY5` reader - Channel 5 User Ready"]
pub struct USRRDY5_R(crate::FieldReader<bool, bool>);
impl USRRDY5_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        USRRDY5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USRRDY5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHBUSY0` reader - Channel 0 Busy"]
pub struct CHBUSY0_R(crate::FieldReader<bool, bool>);
impl CHBUSY0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CHBUSY0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHBUSY0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHBUSY1` reader - Channel 1 Busy"]
pub struct CHBUSY1_R(crate::FieldReader<bool, bool>);
impl CHBUSY1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CHBUSY1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHBUSY1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHBUSY2` reader - Channel 2 Busy"]
pub struct CHBUSY2_R(crate::FieldReader<bool, bool>);
impl CHBUSY2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CHBUSY2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHBUSY2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHBUSY3` reader - Channel 3 Busy"]
pub struct CHBUSY3_R(crate::FieldReader<bool, bool>);
impl CHBUSY3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CHBUSY3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHBUSY3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHBUSY4` reader - Channel 4 Busy"]
pub struct CHBUSY4_R(crate::FieldReader<bool, bool>);
impl CHBUSY4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CHBUSY4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHBUSY4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHBUSY5` reader - Channel 5 Busy"]
pub struct CHBUSY5_R(crate::FieldReader<bool, bool>);
impl CHBUSY5_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CHBUSY5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHBUSY5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Channel 0 User Ready"]
    #[inline(always)]
    pub fn usrrdy0(&self) -> USRRDY0_R {
        USRRDY0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Channel 1 User Ready"]
    #[inline(always)]
    pub fn usrrdy1(&self) -> USRRDY1_R {
        USRRDY1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Channel 2 User Ready"]
    #[inline(always)]
    pub fn usrrdy2(&self) -> USRRDY2_R {
        USRRDY2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Channel 3 User Ready"]
    #[inline(always)]
    pub fn usrrdy3(&self) -> USRRDY3_R {
        USRRDY3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Channel 4 User Ready"]
    #[inline(always)]
    pub fn usrrdy4(&self) -> USRRDY4_R {
        USRRDY4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Channel 5 User Ready"]
    #[inline(always)]
    pub fn usrrdy5(&self) -> USRRDY5_R {
        USRRDY5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Channel 0 Busy"]
    #[inline(always)]
    pub fn chbusy0(&self) -> CHBUSY0_R {
        CHBUSY0_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Channel 1 Busy"]
    #[inline(always)]
    pub fn chbusy1(&self) -> CHBUSY1_R {
        CHBUSY1_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Channel 2 Busy"]
    #[inline(always)]
    pub fn chbusy2(&self) -> CHBUSY2_R {
        CHBUSY2_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Channel 3 Busy"]
    #[inline(always)]
    pub fn chbusy3(&self) -> CHBUSY3_R {
        CHBUSY3_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Channel 4 Busy"]
    #[inline(always)]
    pub fn chbusy4(&self) -> CHBUSY4_R {
        CHBUSY4_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Channel 5 Busy"]
    #[inline(always)]
    pub fn chbusy5(&self) -> CHBUSY5_R {
        CHBUSY5_R::new(((self.bits >> 13) & 0x01) != 0)
    }
}
#[doc = "Channel Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chstatus](index.html) module"]
pub struct CHSTATUS_SPEC;
impl crate::RegisterSpec for CHSTATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [chstatus::R](R) reader structure"]
impl crate::Readable for CHSTATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CHSTATUS to value 0x3f"]
impl crate::Resettable for CHSTATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x3f
    }
}
