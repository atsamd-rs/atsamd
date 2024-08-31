#[doc = "Register `MCF` reader"]
pub type R = crate::R<McfSpec>;
#[doc = "Field `MCOL` reader - Multiple Collision"]
pub type McolR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:17 - Multiple Collision"]
    #[inline(always)]
    pub fn mcol(&self) -> McolR {
        McolR::new(self.bits & 0x0003_ffff)
    }
}
#[doc = "Multiple Collision Frames Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mcf::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct McfSpec;
impl crate::RegisterSpec for McfSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcf::R`](R) reader structure"]
impl crate::Readable for McfSpec {}
#[doc = "`reset()` method sets MCF to value 0"]
impl crate::Resettable for McfSpec {
    const RESET_VALUE: u32 = 0;
}
