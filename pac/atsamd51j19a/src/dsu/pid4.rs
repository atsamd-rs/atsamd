#[doc = "Reader of register PID4"]
pub type R = crate::R<u32, super::PID4>;
#[doc = "Reader of field `JEPCC`"]
pub type JEPCC_R = crate::R<u8, u8>;
#[doc = "Reader of field `FKBC`"]
pub type FKBC_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:3 - JEP-106 Continuation Code"]
    #[inline(always)]
    pub fn jepcc(&self) -> JEPCC_R {
        JEPCC_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - 4KB count"]
    #[inline(always)]
    pub fn fkbc(&self) -> FKBC_R {
        FKBC_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
