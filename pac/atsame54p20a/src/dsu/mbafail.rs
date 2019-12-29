#[doc = "Reader of register MBAFAIL"]
pub type R = crate::R<u32, super::MBAFAIL>;
#[doc = "Reader of field `ADDR`"]
pub type ADDR_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:13 - Error Address"]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new((self.bits & 0x3fff) as u16)
    }
}
