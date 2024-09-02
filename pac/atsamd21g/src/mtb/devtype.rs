#[doc = "Register `DEVTYPE` reader"]
pub type R = crate::R<DevtypeSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "MTB Device Type\n\nYou can [`read`](crate::Reg::read) this register and get [`devtype::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DevtypeSpec;
impl crate::RegisterSpec for DevtypeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`devtype::R`](R) reader structure"]
impl crate::Readable for DevtypeSpec {}
#[doc = "`reset()` method sets DEVTYPE to value 0"]
impl crate::Resettable for DevtypeSpec {
    const RESET_VALUE: u32 = 0;
}
