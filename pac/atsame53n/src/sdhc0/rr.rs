#[doc = "Reader of register RR[%s]"]
pub type R = crate::R<u32, super::RR>;
#[doc = "Reader of field `CMDRESP`"]
pub type CMDRESP_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Command Response"]
    #[inline(always)]
    pub fn cmdresp(&self) -> CMDRESP_R {
        CMDRESP_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
