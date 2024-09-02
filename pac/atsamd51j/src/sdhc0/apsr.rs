#[doc = "Register `APSR` reader"]
pub type R = crate::R<ApsrSpec>;
#[doc = "Field `HDATLL` reader - High Line Level"]
pub type HdatllR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - High Line Level"]
    #[inline(always)]
    pub fn hdatll(&self) -> HdatllR {
        HdatllR::new((self.bits & 0x0f) as u8)
    }
}
#[doc = "Additional Present State Register\n\nYou can [`read`](crate::Reg::read) this register and get [`apsr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ApsrSpec;
impl crate::RegisterSpec for ApsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apsr::R`](R) reader structure"]
impl crate::Readable for ApsrSpec {}
#[doc = "`reset()` method sets APSR to value 0x0f"]
impl crate::Resettable for ApsrSpec {
    const RESET_VALUE: u32 = 0x0f;
}
