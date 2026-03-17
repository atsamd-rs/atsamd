#[doc = "Register `TSCV` reader"]
pub type R = crate::R<TscvSpec>;
#[doc = "Field `TSC` reader - Timestamp Counter"]
pub type TscR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Timestamp Counter"]
    #[inline(always)]
    pub fn tsc(&self) -> TscR {
        TscR::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Timestamp Counter Value\n\nYou can [`read`](crate::Reg::read) this register and get [`tscv::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TscvSpec;
impl crate::RegisterSpec for TscvSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tscv::R`](R) reader structure"]
impl crate::Readable for TscvSpec {}
#[doc = "`reset()` method sets TSCV to value 0"]
impl crate::Resettable for TscvSpec {
    const RESET_VALUE: u32 = 0;
}
