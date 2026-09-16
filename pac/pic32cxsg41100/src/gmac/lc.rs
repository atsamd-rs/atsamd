#[doc = "Register `LC` reader"]
pub type R = crate::R<LcSpec>;
#[doc = "Field `LCOL` reader - Late Collisions"]
pub type LcolR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:9 - Late Collisions"]
    #[inline(always)]
    pub fn lcol(&self) -> LcolR {
        LcolR::new((self.bits & 0x03ff) as u16)
    }
}
#[doc = "Late Collisions Register\n\nYou can [`read`](crate::Reg::read) this register and get [`lc::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LcSpec;
impl crate::RegisterSpec for LcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lc::R`](R) reader structure"]
impl crate::Readable for LcSpec {}
#[doc = "`reset()` method sets LC to value 0"]
impl crate::Resettable for LcSpec {
    const RESET_VALUE: u32 = 0;
}
