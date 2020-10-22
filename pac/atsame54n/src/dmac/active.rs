#[doc = "Reader of register ACTIVE"]
pub type R = crate::R<u32, super::ACTIVE>;
#[doc = "Reader of field `LVLEX0`"]
pub type LVLEX0_R = crate::R<bool, bool>;
#[doc = "Reader of field `LVLEX1`"]
pub type LVLEX1_R = crate::R<bool, bool>;
#[doc = "Reader of field `LVLEX2`"]
pub type LVLEX2_R = crate::R<bool, bool>;
#[doc = "Reader of field `LVLEX3`"]
pub type LVLEX3_R = crate::R<bool, bool>;
#[doc = "Reader of field `ID`"]
pub type ID_R = crate::R<u8, u8>;
#[doc = "Reader of field `ABUSY`"]
pub type ABUSY_R = crate::R<bool, bool>;
#[doc = "Reader of field `BTCNT`"]
pub type BTCNT_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bit 0 - Level 0 Channel Trigger Request Executing"]
    #[inline(always)]
    pub fn lvlex0(&self) -> LVLEX0_R {
        LVLEX0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Level 1 Channel Trigger Request Executing"]
    #[inline(always)]
    pub fn lvlex1(&self) -> LVLEX1_R {
        LVLEX1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Level 2 Channel Trigger Request Executing"]
    #[inline(always)]
    pub fn lvlex2(&self) -> LVLEX2_R {
        LVLEX2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Level 3 Channel Trigger Request Executing"]
    #[inline(always)]
    pub fn lvlex3(&self) -> LVLEX3_R {
        LVLEX3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 8:12 - Active Channel ID"]
    #[inline(always)]
    pub fn id(&self) -> ID_R {
        ID_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bit 15 - Active Channel Busy"]
    #[inline(always)]
    pub fn abusy(&self) -> ABUSY_R {
        ABUSY_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 16:31 - Active Channel Block Transfer Count"]
    #[inline(always)]
    pub fn btcnt(&self) -> BTCNT_R {
        BTCNT_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
