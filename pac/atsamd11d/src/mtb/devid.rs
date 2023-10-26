#[doc = "Register `DEVID` reader"]
pub type R = crate::R<DEVID_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<DEVID_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "MTB Device Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`devid::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DEVID_SPEC;
impl crate::RegisterSpec for DEVID_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`devid::R`](R) reader structure"]
impl crate::Readable for DEVID_SPEC {}
#[doc = "`reset()` method sets DEVID to value 0"]
impl crate::Resettable for DEVID_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
