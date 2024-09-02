#[doc = "Register `ENTRY` reader"]
pub type R = crate::R<EntrySpec>;
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
#[doc = "CoreSight ROM Table Entry 0\n\nYou can [`read`](crate::Reg::read) this register and get [`entry::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EntrySpec;
impl crate::RegisterSpec for EntrySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`entry::R`](R) reader structure"]
impl crate::Readable for EntrySpec {}
#[doc = "`reset()` method sets ENTRY to value 0x9f9f_c002"]
impl crate::Resettable for EntrySpec {
    const RESET_VALUE: u32 = 0x9f9f_c002;
}
