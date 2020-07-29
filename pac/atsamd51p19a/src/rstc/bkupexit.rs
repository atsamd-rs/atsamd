#[doc = "Reader of register BKUPEXIT"]
pub type R = crate::R<u8, super::BKUPEXIT>;
#[doc = "Reader of field `RTC`"]
pub type RTC_R = crate::R<bool, bool>;
#[doc = "Reader of field `BBPS`"]
pub type BBPS_R = crate::R<bool, bool>;
#[doc = "Reader of field `HIB`"]
pub type HIB_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 1 - Real Timer Counter Interrupt"]
    #[inline(always)]
    pub fn rtc(&self) -> RTC_R {
        RTC_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Battery Backup Power Switch"]
    #[inline(always)]
    pub fn bbps(&self) -> BBPS_R {
        BBPS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Hibernate"]
    #[inline(always)]
    pub fn hib(&self) -> HIB_R {
        HIB_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
