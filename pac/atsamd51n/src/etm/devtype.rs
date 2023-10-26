#[doc = "Register `DEVTYPE` reader"]
pub type R = crate::R<DEVTYPE_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<DEVTYPE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "ETM CoreSight Device Type Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`devtype::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DEVTYPE_SPEC;
impl crate::RegisterSpec for DEVTYPE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`devtype::R`](R) reader structure"]
impl crate::Readable for DEVTYPE_SPEC {}
#[doc = "`reset()` method sets DEVTYPE to value 0x13"]
impl crate::Resettable for DEVTYPE_SPEC {
    const RESET_VALUE: Self::Ux = 0x13;
}
