#[doc = "Reader of register FIFO1"]
pub type R = crate::R<u32, super::FIFO1>;
#[doc = "Reader of field `ITM0`"]
pub type ITM0_R = crate::R<u8, u8>;
#[doc = "Reader of field `ITM1`"]
pub type ITM1_R = crate::R<u8, u8>;
#[doc = "Reader of field `ITM2`"]
pub type ITM2_R = crate::R<u8, u8>;
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
    pub fn itm0(&self) -> ITM0_R {
        ITM0_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn itm1(&self) -> ITM1_R {
        ITM1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn itm2(&self) -> ITM2_R {
        ITM2_R::new(((self.bits >> 16) & 0xff) as u8)
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
