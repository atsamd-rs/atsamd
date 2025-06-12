#[doc = "Register `JR` reader"]
pub type R = crate::R<JrSpec>;
#[doc = "Field `JRX` reader - Jabbers Received"]
pub type JrxR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:9 - Jabbers Received"]
    #[inline(always)]
    pub fn jrx(&self) -> JrxR {
        JrxR::new((self.bits & 0x03ff) as u16)
    }
}
#[doc = "Jabbers Received Register\n\nYou can [`read`](crate::Reg::read) this register and get [`jr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct JrSpec;
impl crate::RegisterSpec for JrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`jr::R`](R) reader structure"]
impl crate::Readable for JrSpec {}
#[doc = "`reset()` method sets JR to value 0"]
impl crate::Resettable for JrSpec {}
