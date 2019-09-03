#[doc = "Reader of register CID1"]
pub type R = crate::R<u32, super::CID1>;
#[doc = "Reader of field `PREAMBLE`"]
pub type PREAMBLE_R = crate::R<u8, u8>;
#[doc = "Reader of field `CCLASS`"]
pub type CCLASS_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:3 - Preamble"]
    #[inline(always)]
    pub fn preamble(&self) -> PREAMBLE_R {
        PREAMBLE_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Component Class"]
    #[inline(always)]
    pub fn cclass(&self) -> CCLASS_R {
        CCLASS_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
