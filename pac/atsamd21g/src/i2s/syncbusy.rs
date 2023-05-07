#[doc = "Register `SYNCBUSY` reader"]
pub struct R(crate::R<SYNCBUSY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYNCBUSY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYNCBUSY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYNCBUSY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SWRST` reader - Software Reset Synchronization Status"]
pub type SWRST_R = crate::BitReader<bool>;
#[doc = "Field `ENABLE` reader - Enable Synchronization Status"]
pub type ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `CKEN0` reader - Clock Unit 0 Enable Synchronization Status"]
pub type CKEN0_R = crate::BitReader<bool>;
#[doc = "Field `CKEN1` reader - Clock Unit 1 Enable Synchronization Status"]
pub type CKEN1_R = crate::BitReader<bool>;
#[doc = "Field `SEREN0` reader - Serializer 0 Enable Synchronization Status"]
pub type SEREN0_R = crate::BitReader<bool>;
#[doc = "Field `SEREN1` reader - Serializer 1 Enable Synchronization Status"]
pub type SEREN1_R = crate::BitReader<bool>;
#[doc = "Field `DATA0` reader - Data 0 Synchronization Status"]
pub type DATA0_R = crate::BitReader<bool>;
#[doc = "Field `DATA1` reader - Data 1 Synchronization Status"]
pub type DATA1_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Software Reset Synchronization Status"]
    #[inline(always)]
    pub fn swrst(&self) -> SWRST_R {
        SWRST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable Synchronization Status"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Clock Unit 0 Enable Synchronization Status"]
    #[inline(always)]
    pub fn cken0(&self) -> CKEN0_R {
        CKEN0_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Clock Unit 1 Enable Synchronization Status"]
    #[inline(always)]
    pub fn cken1(&self) -> CKEN1_R {
        CKEN1_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Serializer 0 Enable Synchronization Status"]
    #[inline(always)]
    pub fn seren0(&self) -> SEREN0_R {
        SEREN0_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Serializer 1 Enable Synchronization Status"]
    #[inline(always)]
    pub fn seren1(&self) -> SEREN1_R {
        SEREN1_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - Data 0 Synchronization Status"]
    #[inline(always)]
    pub fn data0(&self) -> DATA0_R {
        DATA0_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Data 1 Synchronization Status"]
    #[inline(always)]
    pub fn data1(&self) -> DATA1_R {
        DATA1_R::new(((self.bits >> 9) & 1) != 0)
    }
}
#[doc = "Synchronization Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syncbusy](index.html) module"]
pub struct SYNCBUSY_SPEC;
impl crate::RegisterSpec for SYNCBUSY_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [syncbusy::R](R) reader structure"]
impl crate::Readable for SYNCBUSY_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SYNCBUSY to value 0"]
impl crate::Resettable for SYNCBUSY_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
