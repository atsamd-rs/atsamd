#[doc = "Register `BKIN` reader"]
pub type R = crate::R<BKIN_SPEC>;
#[doc = "Field `BKIN0` reader - Backup Input 0"]
pub type BKIN0_R = crate::BitReader;
#[doc = "Field `BKIN1` reader - Backup Input 1"]
pub type BKIN1_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Backup Input 0"]
    #[inline(always)]
    pub fn bkin0(&self) -> BKIN0_R {
        BKIN0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Backup Input 1"]
    #[inline(always)]
    pub fn bkin1(&self) -> BKIN1_R {
        BKIN1_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "Backup Input Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bkin::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BKIN_SPEC;
impl crate::RegisterSpec for BKIN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bkin::R`](R) reader structure"]
impl crate::Readable for BKIN_SPEC {}
#[doc = "`reset()` method sets BKIN to value 0"]
impl crate::Resettable for BKIN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
