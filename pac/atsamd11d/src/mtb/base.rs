#[doc = "Register `BASE` reader"]
pub type R = crate::R<BASE_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<BASE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "MTB Base\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`base::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BASE_SPEC;
impl crate::RegisterSpec for BASE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`base::R`](R) reader structure"]
impl crate::Readable for BASE_SPEC {}
#[doc = "`reset()` method sets BASE to value 0"]
impl crate::Resettable for BASE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
