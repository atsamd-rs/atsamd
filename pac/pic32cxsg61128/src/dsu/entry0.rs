#[doc = "Register `ENTRY0` reader"]
pub type R = crate::R<Entry0Spec>;
#[doc = "Field `EPRES` reader - Entry Present"]
pub type EpresR = crate::BitReader;
#[doc = "Field `FMT` reader - Format"]
pub type FmtR = crate::BitReader;
#[doc = "Field `ADDOFF` reader - Address Offset"]
pub type AddoffR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bit 0 - Entry Present"]
    #[inline(always)]
    pub fn epres(&self) -> EpresR {
        EpresR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Format"]
    #[inline(always)]
    pub fn fmt(&self) -> FmtR {
        FmtR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 12:31 - Address Offset"]
    #[inline(always)]
    pub fn addoff(&self) -> AddoffR {
        AddoffR::new((self.bits >> 12) & 0x000f_ffff)
    }
}
#[doc = "CoreSight ROM Table Entry 0\n\nYou can [`read`](crate::Reg::read) this register and get [`entry0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Entry0Spec;
impl crate::RegisterSpec for Entry0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`entry0::R`](R) reader structure"]
impl crate::Readable for Entry0Spec {}
#[doc = "`reset()` method sets ENTRY0 to value 0x9f0f_c002"]
impl crate::Resettable for Entry0Spec {
    const RESET_VALUE: u32 = 0x9f0f_c002;
}
