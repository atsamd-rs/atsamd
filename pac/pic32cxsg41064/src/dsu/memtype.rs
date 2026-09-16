#[doc = "Register `MEMTYPE` reader"]
pub type R = crate::R<MemtypeSpec>;
#[doc = "Field `SMEMP` reader - System Memory Present"]
pub type SmempR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - System Memory Present"]
    #[inline(always)]
    pub fn smemp(&self) -> SmempR {
        SmempR::new((self.bits & 1) != 0)
    }
}
#[doc = "CoreSight ROM Table Memory Type\n\nYou can [`read`](crate::Reg::read) this register and get [`memtype::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MemtypeSpec;
impl crate::RegisterSpec for MemtypeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`memtype::R`](R) reader structure"]
impl crate::Readable for MemtypeSpec {}
#[doc = "`reset()` method sets MEMTYPE to value 0"]
impl crate::Resettable for MemtypeSpec {
    const RESET_VALUE: u32 = 0;
}
