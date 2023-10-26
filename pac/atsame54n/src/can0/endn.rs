#[doc = "Register `ENDN` reader"]
pub type R = crate::R<ENDN_SPEC>;
#[doc = "Field `ETV` reader - Endianness Test Value"]
pub type ETV_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Endianness Test Value"]
    #[inline(always)]
    pub fn etv(&self) -> ETV_R {
        ETV_R::new(self.bits)
    }
}
#[doc = "Endian\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`endn::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ENDN_SPEC;
impl crate::RegisterSpec for ENDN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`endn::R`](R) reader structure"]
impl crate::Readable for ENDN_SPEC {}
#[doc = "`reset()` method sets ENDN to value 0x8765_4321"]
impl crate::Resettable for ENDN_SPEC {
    const RESET_VALUE: Self::Ux = 0x8765_4321;
}
