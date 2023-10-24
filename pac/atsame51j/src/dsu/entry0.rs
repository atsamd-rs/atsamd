#[doc = "Register `ENTRY0` reader"]
pub type R = crate::R<ENTRY0_SPEC>;
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
#[doc = "CoreSight ROM Table Entry 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`entry0::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ENTRY0_SPEC;
impl crate::RegisterSpec for ENTRY0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`entry0::R`](R) reader structure"]
impl crate::Readable for ENTRY0_SPEC {}
#[doc = "`reset()` method sets ENTRY0 to value 0x9f0f_c002"]
impl crate::Resettable for ENTRY0_SPEC {
    const RESET_VALUE: Self::Ux = 0x9f0f_c002;
}
