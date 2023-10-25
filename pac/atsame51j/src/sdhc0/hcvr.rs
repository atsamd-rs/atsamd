#[doc = "Register `HCVR` reader"]
pub type R = crate::R<HCVR_SPEC>;
#[doc = "Field `SVER` reader - Spec Version"]
pub type SVER_R = crate::FieldReader;
#[doc = "Field `VVER` reader - Vendor Version"]
pub type VVER_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Spec Version"]
    #[inline(always)]
    pub fn sver(&self) -> SVER_R {
        SVER_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Vendor Version"]
    #[inline(always)]
    pub fn vver(&self) -> VVER_R {
        VVER_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
#[doc = "Host Controller Version\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcvr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HCVR_SPEC;
impl crate::RegisterSpec for HCVR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`hcvr::R`](R) reader structure"]
impl crate::Readable for HCVR_SPEC {}
#[doc = "`reset()` method sets HCVR to value 0x1802"]
impl crate::Resettable for HCVR_SPEC {
    const RESET_VALUE: Self::Ux = 0x1802;
}
