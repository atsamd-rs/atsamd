#[doc = "Reader of register MSR"]
pub type R = crate::R<u32, super::MSR>;
#[doc = "Reader of field `EVENT_CNT`"]
pub type EVENT_CNT_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Monitor Event Counter"]
    #[inline(always)]
    pub fn event_cnt(&self) -> EVENT_CNT_R {
        EVENT_CNT_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
