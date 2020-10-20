#[doc = "Reader of register HCVR"]
pub type R = crate::R<u16, super::HCVR>;
#[doc = "Reader of field `SVER`"]
pub type SVER_R = crate::R<u8, u8>;
#[doc = "Reader of field `VVER`"]
pub type VVER_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Spec Version"]
    #[inline(always)]
    pub fn sver(&self) -> SVER_R {
        SVER_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Vendor Version"]
    #[inline(always)]
    pub fn vver(&self) -> VVER_R {
        VVER_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
