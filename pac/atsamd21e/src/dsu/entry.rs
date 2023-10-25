#[doc = "Register `ENTRY` reader"]
pub type R = crate::R<ENTRY_SPEC>;
#[doc = "Field `EPRES` reader - Entry Present"]
pub type EPRES_R = crate::BitReader;
#[doc = "Field `FMT` reader - Format"]
pub type FMT_R = crate::BitReader;
#[doc = "Field `ADDOFF` reader - Address Offset"]
pub type ADDOFF_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bit 0 - Entry Present"]
    #[inline(always)]
    pub fn epres(&self) -> EPRES_R {
        EPRES_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Format"]
    #[inline(always)]
    pub fn fmt(&self) -> FMT_R {
        FMT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 12:31 - Address Offset"]
    #[inline(always)]
    pub fn addoff(&self) -> ADDOFF_R {
        ADDOFF_R::new((self.bits >> 12) & 0x000f_ffff)
    }
}
#[doc = "CoreSight ROM Table Entry 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`entry::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ENTRY_SPEC;
impl crate::RegisterSpec for ENTRY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`entry::R`](R) reader structure"]
impl crate::Readable for ENTRY_SPEC {}
#[doc = "`reset()` method sets ENTRY to value 0x9f9f_c002"]
impl crate::Resettable for ENTRY_SPEC {
    const RESET_VALUE: Self::Ux = 0x9f9f_c002;
}
