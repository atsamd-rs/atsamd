#[doc = "Register `SCF` reader"]
pub type R = crate::R<ScfSpec>;
#[doc = "Field `SCOL` reader - Single Collision"]
pub type ScolR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:17 - Single Collision"]
    #[inline(always)]
    pub fn scol(&self) -> ScolR {
        ScolR::new(self.bits & 0x0003_ffff)
    }
}
#[doc = "Single Collision Frames Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scf::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ScfSpec;
impl crate::RegisterSpec for ScfSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scf::R`](R) reader structure"]
impl crate::Readable for ScfSpec {}
#[doc = "`reset()` method sets SCF to value 0"]
impl crate::Resettable for ScfSpec {
    const RESET_VALUE: u32 = 0;
}
