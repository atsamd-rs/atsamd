#[doc = "Reader of register DEVTYPE"]
pub type R = crate::R<u32, super::DEVTYPE>;
#[doc = "Reader of field `SubType`"]
pub type SUBTYPE_R = crate::R<u8, u8>;
#[doc = "Reader of field `MajorType`"]
pub type MAJORTYPE_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn sub_type(&self) -> SUBTYPE_R {
        SUBTYPE_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn major_type(&self) -> MAJORTYPE_R {
        MAJORTYPE_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
