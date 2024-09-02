#[doc = "Register `RLPITR` reader"]
pub type R = crate::R<RlpitrSpec>;
#[doc = "Field `RLPITR` reader - Count number of times transition from rx normal idle to low power idle"]
pub type RlpitrR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Count number of times transition from rx normal idle to low power idle"]
    #[inline(always)]
    pub fn rlpitr(&self) -> RlpitrR {
        RlpitrR::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Receive LPI transition Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rlpitr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RlpitrSpec;
impl crate::RegisterSpec for RlpitrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rlpitr::R`](R) reader structure"]
impl crate::Readable for RlpitrSpec {}
#[doc = "`reset()` method sets RLPITR to value 0"]
impl crate::Resettable for RlpitrSpec {
    const RESET_VALUE: u32 = 0;
}
