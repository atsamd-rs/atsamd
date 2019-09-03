#[doc = "Reader of register PID0"]
pub type R = crate::R<u32, super::PID0>;
#[doc = "Reader of field `PARTNBL`"]
pub type PARTNBL_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Part Number Low"]
    #[inline(always)]
    pub fn partnbl(&self) -> PARTNBL_R {
        PARTNBL_R::new((self.bits & 0xff) as u8)
    }
}
