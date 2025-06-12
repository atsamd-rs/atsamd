#[doc = "Register `RESS` reader"]
pub type R = crate::R<RessSpec>;
#[doc = "Field `RESS` reader - Last ADC conversion result"]
pub type RessR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Last ADC conversion result"]
    #[inline(always)]
    pub fn ress(&self) -> RessR {
        RessR::new(self.bits)
    }
}
#[doc = "Last Sample Result\n\nYou can [`read`](crate::Reg::read) this register and get [`ress::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RessSpec;
impl crate::RegisterSpec for RessSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ress::R`](R) reader structure"]
impl crate::Readable for RessSpec {}
#[doc = "`reset()` method sets RESS to value 0"]
impl crate::Resettable for RessSpec {}
