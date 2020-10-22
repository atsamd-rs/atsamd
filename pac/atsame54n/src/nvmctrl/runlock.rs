#[doc = "Reader of register RUNLOCK"]
pub type R = crate::R<u32, super::RUNLOCK>;
#[doc = "Reader of field `RUNLOCK`"]
pub type RUNLOCK_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Region Un-Lock Bits"]
    #[inline(always)]
    pub fn runlock(&self) -> RUNLOCK_R {
        RUNLOCK_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
