#[doc = "Reader of register FIFO0"]
pub type R = crate::R<u32, super::FIFO0>;
#[doc = "Reader of field `ETM0`"]
pub type ETM0_R = crate::R<u8, u8>;
#[doc = "Reader of field `ETM1`"]
pub type ETM1_R = crate::R<u8, u8>;
#[doc = "Reader of field `ETM2`"]
pub type ETM2_R = crate::R<u8, u8>;
#[doc = "Reader of field `ETM_bytecount`"]
pub type ETM_BYTECOUNT_R = crate::R<u8, u8>;
#[doc = "Reader of field `ETM_ATVALID`"]
pub type ETM_ATVALID_R = crate::R<bool, bool>;
#[doc = "Reader of field `ITM_bytecount`"]
pub type ITM_BYTECOUNT_R = crate::R<u8, u8>;
#[doc = "Reader of field `ITM_ATVALID`"]
pub type ITM_ATVALID_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn etm0(&self) -> ETM0_R {
        ETM0_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn etm1(&self) -> ETM1_R {
        ETM1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn etm2(&self) -> ETM2_R {
        ETM2_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn etm_bytecount(&self) -> ETM_BYTECOUNT_R {
        ETM_BYTECOUNT_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn etm_atvalid(&self) -> ETM_ATVALID_R {
        ETM_ATVALID_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bits 27:28"]
    #[inline(always)]
    pub fn itm_bytecount(&self) -> ITM_BYTECOUNT_R {
        ITM_BYTECOUNT_R::new(((self.bits >> 27) & 0x03) as u8)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn itm_atvalid(&self) -> ITM_ATVALID_R {
        ITM_ATVALID_R::new(((self.bits >> 29) & 0x01) != 0)
    }
}
