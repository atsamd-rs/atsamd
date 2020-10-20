#[doc = "Reader of register DPLLSYNCBUSY"]
pub type R = crate::R<u32, super::DPLLSYNCBUSY>;
#[doc = "Reader of field `ENABLE`"]
pub type ENABLE_R = crate::R<bool, bool>;
#[doc = "Reader of field `DPLLRATIO`"]
pub type DPLLRATIO_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 1 - DPLL Enable Synchronization Status"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - DPLL Loop Divider Ratio Synchronization Status"]
    #[inline(always)]
    pub fn dpllratio(&self) -> DPLLRATIO_R {
        DPLLRATIO_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
