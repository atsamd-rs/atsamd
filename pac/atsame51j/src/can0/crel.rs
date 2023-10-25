#[doc = "Register `CREL` reader"]
pub type R = crate::R<CREL_SPEC>;
#[doc = "Field `SUBSTEP` reader - Sub-step of Core Release"]
pub type SUBSTEP_R = crate::FieldReader;
#[doc = "Field `STEP` reader - Step of Core Release"]
pub type STEP_R = crate::FieldReader;
#[doc = "Field `REL` reader - Core Release"]
pub type REL_R = crate::FieldReader;
impl R {
    #[doc = "Bits 20:23 - Sub-step of Core Release"]
    #[inline(always)]
    pub fn substep(&self) -> SUBSTEP_R {
        SUBSTEP_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Step of Core Release"]
    #[inline(always)]
    pub fn step(&self) -> STEP_R {
        STEP_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Core Release"]
    #[inline(always)]
    pub fn rel(&self) -> REL_R {
        REL_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
#[doc = "Core Release\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`crel::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CREL_SPEC;
impl crate::RegisterSpec for CREL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`crel::R`](R) reader structure"]
impl crate::Readable for CREL_SPEC {}
#[doc = "`reset()` method sets CREL to value 0x3210_0000"]
impl crate::Resettable for CREL_SPEC {
    const RESET_VALUE: Self::Ux = 0x3210_0000;
}
