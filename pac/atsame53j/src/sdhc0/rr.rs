#[doc = "Register `RR[%s]` reader"]
pub type R = crate::R<RrSpec>;
#[doc = "Field `CMDRESP` reader - Command Response"]
pub type CmdrespR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Command Response"]
    #[inline(always)]
    pub fn cmdresp(&self) -> CmdrespR {
        CmdrespR::new(self.bits)
    }
}
#[doc = "Response\n\nYou can [`read`](crate::Reg::read) this register and get [`rr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RrSpec;
impl crate::RegisterSpec for RrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rr::R`](R) reader structure"]
impl crate::Readable for RrSpec {}
#[doc = "`reset()` method sets RR[%s] to value 0"]
impl crate::Resettable for RrSpec {}
