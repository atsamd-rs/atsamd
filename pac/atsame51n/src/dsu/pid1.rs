#[doc = "Register `PID1` reader"]
pub type R = crate::R<PID1_SPEC>;
#[doc = "Field `PARTNBH` reader - Part Number High"]
pub type PARTNBH_R = crate::FieldReader;
#[doc = "Field `JEPIDCL` reader - Low part of the JEP-106 Identity Code"]
pub type JEPIDCL_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - Part Number High"]
    #[inline(always)]
    pub fn partnbh(&self) -> PARTNBH_R {
        PARTNBH_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Low part of the JEP-106 Identity Code"]
    #[inline(always)]
    pub fn jepidcl(&self) -> JEPIDCL_R {
        JEPIDCL_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
#[doc = "Peripheral Identification 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pid1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PID1_SPEC;
impl crate::RegisterSpec for PID1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pid1::R`](R) reader structure"]
impl crate::Readable for PID1_SPEC {}
#[doc = "`reset()` method sets PID1 to value 0xfc"]
impl crate::Resettable for PID1_SPEC {
    const RESET_VALUE: Self::Ux = 0xfc;
}
