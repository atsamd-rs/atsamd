#[doc = "Register `ENTRY1` reader"]
pub type R = crate::R<ENTRY1_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<ENTRY1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "CoreSight ROM Table Entry 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`entry1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ENTRY1_SPEC;
impl crate::RegisterSpec for ENTRY1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`entry1::R`](R) reader structure"]
impl crate::Readable for ENTRY1_SPEC {}
#[doc = "`reset()` method sets ENTRY1 to value 0x3002"]
impl crate::Resettable for ENTRY1_SPEC {
    const RESET_VALUE: Self::Ux = 0x3002;
}
