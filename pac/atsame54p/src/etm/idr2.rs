#[doc = "Register `IDR2` reader"]
pub type R = crate::R<IDR2_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<IDR2_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "ETM ID Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`idr2::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IDR2_SPEC;
impl crate::RegisterSpec for IDR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`idr2::R`](R) reader structure"]
impl crate::Readable for IDR2_SPEC {}
#[doc = "`reset()` method sets IDR2 to value 0"]
impl crate::Resettable for IDR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
