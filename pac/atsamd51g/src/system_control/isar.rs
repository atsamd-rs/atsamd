#[doc = "Register `ISAR[%s]` reader"]
pub type R = crate::R<ISAR_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<ISAR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Instruction Set Attributes Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isar::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ISAR_SPEC;
impl crate::RegisterSpec for ISAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`isar::R`](R) reader structure"]
impl crate::Readable for ISAR_SPEC {}
#[doc = "`reset()` method sets ISAR[%s]
to value 0"]
impl crate::Resettable for ISAR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
