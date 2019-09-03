#[doc = "Reader of register PID2"]
pub type R = crate::R<u32, super::PID2>;
#[doc = "Reader of field `JEPIDCH`"]
pub type JEPIDCH_R = crate::R<u8, u8>;
#[doc = "Reader of field `JEPU`"]
pub type JEPU_R = crate::R<bool, bool>;
#[doc = "Reader of field `REVISION`"]
pub type REVISION_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:2 - JEP-106 Identity Code High"]
    #[inline(always)]
    pub fn jepidch(&self) -> JEPIDCH_R {
        JEPIDCH_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bit 3 - JEP-106 Identity Code is used"]
    #[inline(always)]
    pub fn jepu(&self) -> JEPU_R {
        JEPU_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 4:7 - Revision Number"]
    #[inline(always)]
    pub fn revision(&self) -> REVISION_R {
        REVISION_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
