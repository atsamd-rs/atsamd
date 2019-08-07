#[doc = "Reader of register SYNCBUSY"]
pub type R = crate::R<u16, super::SYNCBUSY>;
#[doc = "Reader of field `SWRST`"]
pub type SWRST_R = crate::R<bool, bool>;
#[doc = "Reader of field `ENABLE`"]
pub type ENABLE_R = crate::R<bool, bool>;
#[doc = "Reader of field `CKEN0`"]
pub type CKEN0_R = crate::R<bool, bool>;
#[doc = "Reader of field `CKEN1`"]
pub type CKEN1_R = crate::R<bool, bool>;
#[doc = "Reader of field `SEREN0`"]
pub type SEREN0_R = crate::R<bool, bool>;
#[doc = "Reader of field `SEREN1`"]
pub type SEREN1_R = crate::R<bool, bool>;
#[doc = "Reader of field `DATA0`"]
pub type DATA0_R = crate::R<bool, bool>;
#[doc = "Reader of field `DATA1`"]
pub type DATA1_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Software Reset Synchronization Status"]
    #[inline(always)]
    pub fn swrst(&self) -> SWRST_R {
        SWRST_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Enable Synchronization Status"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Clock Unit 0 Enable Synchronization Status"]
    #[inline(always)]
    pub fn cken0(&self) -> CKEN0_R {
        CKEN0_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Clock Unit 1 Enable Synchronization Status"]
    #[inline(always)]
    pub fn cken1(&self) -> CKEN1_R {
        CKEN1_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Serializer 0 Enable Synchronization Status"]
    #[inline(always)]
    pub fn seren0(&self) -> SEREN0_R {
        SEREN0_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Serializer 1 Enable Synchronization Status"]
    #[inline(always)]
    pub fn seren1(&self) -> SEREN1_R {
        SEREN1_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Data 0 Synchronization Status"]
    #[inline(always)]
    pub fn data0(&self) -> DATA0_R {
        DATA0_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Data 1 Synchronization Status"]
    #[inline(always)]
    pub fn data1(&self) -> DATA1_R {
        DATA1_R::new(((self.bits >> 9) & 0x01) != 0)
    }
}
