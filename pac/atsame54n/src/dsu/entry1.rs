#[doc = "Register `ENTRY1` reader"]
pub type R = crate::R<Entry1Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "CoreSight ROM Table Entry 1\n\nYou can [`read`](crate::Reg::read) this register and get [`entry1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Entry1Spec;
impl crate::RegisterSpec for Entry1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`entry1::R`](R) reader structure"]
impl crate::Readable for Entry1Spec {}
#[doc = "`reset()` method sets ENTRY1 to value 0"]
impl crate::Resettable for Entry1Spec {}
