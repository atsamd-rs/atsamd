#[doc = "Register `PINSTATE` reader"]
pub type R = crate::R<PINSTATE_SPEC>;
#[doc = "Field `PINSTATE` reader - Pin State"]
pub type PINSTATE_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Pin State"]
    #[inline(always)]
    pub fn pinstate(&self) -> PINSTATE_R {
        PINSTATE_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Pin State\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pinstate::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PINSTATE_SPEC;
impl crate::RegisterSpec for PINSTATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pinstate::R`](R) reader structure"]
impl crate::Readable for PINSTATE_SPEC {}
#[doc = "`reset()` method sets PINSTATE to value 0"]
impl crate::Resettable for PINSTATE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
