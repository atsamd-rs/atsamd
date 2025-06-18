#[doc = "Register `ISAR[%s]` reader"]
pub type R = crate::R<IsarSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "Instruction Set Attributes Register\n\nYou can [`read`](crate::Reg::read) this register and get [`isar::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IsarSpec;
impl crate::RegisterSpec for IsarSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`isar::R`](R) reader structure"]
impl crate::Readable for IsarSpec {}
#[doc = "`reset()` method sets ISAR[%s] to value 0"]
impl crate::Resettable for IsarSpec {}
