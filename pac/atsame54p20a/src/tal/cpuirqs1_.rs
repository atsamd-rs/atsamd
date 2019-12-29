#[doc = "Reader of register CPUIRQS1_%s"]
pub type R = crate::R<u32, super::CPUIRQS1_>;
#[doc = "Reader of field `CPUIRQS`"]
pub type CPUIRQS_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Interrupt Requests for CPU n"]
    #[inline(always)]
    pub fn cpuirqs(&self) -> CPUIRQS_R {
        CPUIRQS_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
