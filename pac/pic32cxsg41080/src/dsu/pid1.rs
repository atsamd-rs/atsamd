#[doc = "Register `PID1` reader"]
pub type R = crate::R<Pid1Spec>;
#[doc = "Field `PARTNBH` reader - Part Number High"]
pub type PartnbhR = crate::FieldReader;
#[doc = "Field `JEPIDCL` reader - Low part of the JEP-106 Identity Code"]
pub type JepidclR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - Part Number High"]
    #[inline(always)]
    pub fn partnbh(&self) -> PartnbhR {
        PartnbhR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Low part of the JEP-106 Identity Code"]
    #[inline(always)]
    pub fn jepidcl(&self) -> JepidclR {
        JepidclR::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
#[doc = "Peripheral Identification 1\n\nYou can [`read`](crate::Reg::read) this register and get [`pid1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pid1Spec;
impl crate::RegisterSpec for Pid1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pid1::R`](R) reader structure"]
impl crate::Readable for Pid1Spec {}
#[doc = "`reset()` method sets PID1 to value 0xfc"]
impl crate::Resettable for Pid1Spec {
    const RESET_VALUE: u32 = 0xfc;
}
