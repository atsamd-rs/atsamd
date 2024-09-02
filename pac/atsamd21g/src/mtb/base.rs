#[doc = "Register `BASE` reader"]
pub type R = crate::R<BaseSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "MTB Base\n\nYou can [`read`](crate::Reg::read) this register and get [`base::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BaseSpec;
impl crate::RegisterSpec for BaseSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`base::R`](R) reader structure"]
impl crate::Readable for BaseSpec {}
#[doc = "`reset()` method sets BASE to value 0"]
impl crate::Resettable for BaseSpec {
    const RESET_VALUE: u32 = 0;
}
