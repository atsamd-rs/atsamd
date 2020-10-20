#[doc = "Reader of register STATUS"]
pub type R = crate::R<u32, super::STATUS>;
#[doc = "Reader of field `XOSC32KRDY`"]
pub type XOSC32KRDY_R = crate::R<bool, bool>;
#[doc = "Reader of field `XOSC32KFAIL`"]
pub type XOSC32KFAIL_R = crate::R<bool, bool>;
#[doc = "Reader of field `XOSC32KSW`"]
pub type XOSC32KSW_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - XOSC32K Ready"]
    #[inline(always)]
    pub fn xosc32krdy(&self) -> XOSC32KRDY_R {
        XOSC32KRDY_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 2 - XOSC32K Clock Failure Detector"]
    #[inline(always)]
    pub fn xosc32kfail(&self) -> XOSC32KFAIL_R {
        XOSC32KFAIL_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - XOSC32K Clock switch"]
    #[inline(always)]
    pub fn xosc32ksw(&self) -> XOSC32KSW_R {
        XOSC32KSW_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
