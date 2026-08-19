#[doc = "Register `TLPITR` reader"]
pub type R = crate::R<TlpitrSpec>;
#[doc = "Field `TLPITR` reader - Count number of times enable LPI tx bit 20 goes from low to high"]
pub type TlpitrR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Count number of times enable LPI tx bit 20 goes from low to high"]
    #[inline(always)]
    pub fn tlpitr(&self) -> TlpitrR {
        TlpitrR::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Receive LPI transition Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tlpitr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TlpitrSpec;
impl crate::RegisterSpec for TlpitrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tlpitr::R`](R) reader structure"]
impl crate::Readable for TlpitrSpec {}
#[doc = "`reset()` method sets TLPITR to value 0"]
impl crate::Resettable for TlpitrSpec {
    const RESET_VALUE: u32 = 0;
}
