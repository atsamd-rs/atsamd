#[doc = "Register `DPLLSYNCBUSY` reader"]
pub type R = crate::R<DPLLSYNCBUSY_SPEC>;
#[doc = "Field `ENABLE` reader - DPLL Enable Synchronization Status"]
pub type ENABLE_R = crate::BitReader;
#[doc = "Field `DPLLRATIO` reader - DPLL Loop Divider Ratio Synchronization Status"]
pub type DPLLRATIO_R = crate::BitReader;
impl R {
    #[doc = "Bit 1 - DPLL Enable Synchronization Status"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DPLL Loop Divider Ratio Synchronization Status"]
    #[inline(always)]
    pub fn dpllratio(&self) -> DPLLRATIO_R {
        DPLLRATIO_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[doc = "DPLL Synchronization Busy\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dpllsyncbusy::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DPLLSYNCBUSY_SPEC;
impl crate::RegisterSpec for DPLLSYNCBUSY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dpllsyncbusy::R`](R) reader structure"]
impl crate::Readable for DPLLSYNCBUSY_SPEC {}
#[doc = "`reset()` method sets DPLLSYNCBUSY to value 0"]
impl crate::Resettable for DPLLSYNCBUSY_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
