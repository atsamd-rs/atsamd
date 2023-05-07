#[doc = "Register `ACTIVE` reader"]
pub struct R(crate::R<ACTIVE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ACTIVE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ACTIVE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ACTIVE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `LVLEX0` reader - Level 0 Channel Trigger Request Executing"]
pub type LVLEX0_R = crate::BitReader<bool>;
#[doc = "Field `LVLEX1` reader - Level 1 Channel Trigger Request Executing"]
pub type LVLEX1_R = crate::BitReader<bool>;
#[doc = "Field `LVLEX2` reader - Level 2 Channel Trigger Request Executing"]
pub type LVLEX2_R = crate::BitReader<bool>;
#[doc = "Field `LVLEX3` reader - Level 3 Channel Trigger Request Executing"]
pub type LVLEX3_R = crate::BitReader<bool>;
#[doc = "Field `ID` reader - Active Channel ID"]
pub type ID_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ABUSY` reader - Active Channel Busy"]
pub type ABUSY_R = crate::BitReader<bool>;
#[doc = "Field `BTCNT` reader - Active Channel Block Transfer Count"]
pub type BTCNT_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bit 0 - Level 0 Channel Trigger Request Executing"]
    #[inline(always)]
    pub fn lvlex0(&self) -> LVLEX0_R {
        LVLEX0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Level 1 Channel Trigger Request Executing"]
    #[inline(always)]
    pub fn lvlex1(&self) -> LVLEX1_R {
        LVLEX1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Level 2 Channel Trigger Request Executing"]
    #[inline(always)]
    pub fn lvlex2(&self) -> LVLEX2_R {
        LVLEX2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Level 3 Channel Trigger Request Executing"]
    #[inline(always)]
    pub fn lvlex3(&self) -> LVLEX3_R {
        LVLEX3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 8:12 - Active Channel ID"]
    #[inline(always)]
    pub fn id(&self) -> ID_R {
        ID_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bit 15 - Active Channel Busy"]
    #[inline(always)]
    pub fn abusy(&self) -> ABUSY_R {
        ABUSY_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:31 - Active Channel Block Transfer Count"]
    #[inline(always)]
    pub fn btcnt(&self) -> BTCNT_R {
        BTCNT_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "Active Channel and Levels\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [active](index.html) module"]
pub struct ACTIVE_SPEC;
impl crate::RegisterSpec for ACTIVE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [active::R](R) reader structure"]
impl crate::Readable for ACTIVE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ACTIVE to value 0"]
impl crate::Resettable for ACTIVE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
