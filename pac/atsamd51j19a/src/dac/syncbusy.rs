#[doc = "Reader of register SYNCBUSY"]
pub type R = crate::R<u32, super::SYNCBUSY>;
#[doc = "Reader of field `SWRST`"]
pub type SWRST_R = crate::R<bool, bool>;
#[doc = "Reader of field `ENABLE`"]
pub type ENABLE_R = crate::R<bool, bool>;
#[doc = "Reader of field `DATA0`"]
pub type DATA0_R = crate::R<bool, bool>;
#[doc = "Reader of field `DATA1`"]
pub type DATA1_R = crate::R<bool, bool>;
#[doc = "Reader of field `DATABUF0`"]
pub type DATABUF0_R = crate::R<bool, bool>;
#[doc = "Reader of field `DATABUF1`"]
pub type DATABUF1_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Software Reset"]
    #[inline(always)]
    pub fn swrst(&self) -> SWRST_R {
        SWRST_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - DAC Enable Status"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Data DAC 0"]
    #[inline(always)]
    pub fn data0(&self) -> DATA0_R {
        DATA0_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Data DAC 1"]
    #[inline(always)]
    pub fn data1(&self) -> DATA1_R {
        DATA1_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Data Buffer DAC 0"]
    #[inline(always)]
    pub fn databuf0(&self) -> DATABUF0_R {
        DATABUF0_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Data Buffer DAC 1"]
    #[inline(always)]
    pub fn databuf1(&self) -> DATABUF1_R {
        DATABUF1_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
