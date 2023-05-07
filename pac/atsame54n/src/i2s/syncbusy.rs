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
#[doc = "Field `TXEN` reader - Tx Serializer Enable Synchronization Status"]
pub type TXEN_R = crate::BitReader<bool>;
#[doc = "Field `RXEN` reader - Rx Serializer Enable Synchronization Status"]
pub type RXEN_R = crate::BitReader<bool>;
#[doc = "Field `TXDATA` reader - Tx Data Synchronization Status"]
pub type TXDATA_R = crate::BitReader<bool>;
#[doc = "Field `RXDATA` reader - Rx Data Synchronization Status"]
pub type RXDATA_R = crate::BitReader<bool>;
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
    #[doc = "Bit 4 - Tx Serializer Enable Synchronization Status"]
    #[inline(always)]
    pub fn txen(&self) -> TXEN_R {
        TXEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Rx Serializer Enable Synchronization Status"]
    #[inline(always)]
    pub fn rxen(&self) -> RXEN_R {
        RXEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - Tx Data Synchronization Status"]
    #[inline(always)]
    pub fn txdata(&self) -> TXDATA_R {
        TXDATA_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Rx Data Synchronization Status"]
    #[inline(always)]
    pub fn rxdata(&self) -> RXDATA_R {
        RXDATA_R::new(((self.bits >> 9) & 1) != 0)
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
