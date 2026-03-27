#[doc = "Register `DPLLSYNCBUSY` reader"]
pub struct R(crate::R<DPLLSYNCBUSY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DPLLSYNCBUSY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DPLLSYNCBUSY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DPLLSYNCBUSY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ENABLE` reader - DPLL Enable Synchronization Status"]
pub type ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `DPLLRATIO` reader - DPLL Ratio Synchronization Status"]
pub type DPLLRATIO_R = crate::BitReader<bool>;
#[doc = "Field `DPLLPRESC` reader - DPLL Prescaler Synchronization Status"]
pub type DPLLPRESC_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 1 - DPLL Enable Synchronization Status"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DPLL Ratio Synchronization Status"]
    #[inline(always)]
    pub fn dpllratio(&self) -> DPLLRATIO_R {
        DPLLRATIO_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DPLL Prescaler Synchronization Status"]
    #[inline(always)]
    pub fn dpllpresc(&self) -> DPLLPRESC_R {
        DPLLPRESC_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[doc = "DPLL Synchronization Busy\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dpllsyncbusy](index.html) module"]
pub struct DPLLSYNCBUSY_SPEC;
impl crate::RegisterSpec for DPLLSYNCBUSY_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [dpllsyncbusy::R](R) reader structure"]
impl crate::Readable for DPLLSYNCBUSY_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DPLLSYNCBUSY to value 0"]
impl crate::Resettable for DPLLSYNCBUSY_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
