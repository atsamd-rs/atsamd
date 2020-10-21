#[doc = "Reader of register PID1"]
pub type R = crate::R<u32, super::PID1>;
#[doc = "Reader of field `PARTNBH`"]
pub type PARTNBH_R = crate::R<u8, u8>;
#[doc = "Reader of field `JEPIDCL`"]
pub type JEPIDCL_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:3 - Part Number High"]
    #[inline(always)]
    pub fn partnbh(&self) -> PARTNBH_R {
        PARTNBH_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Low part of the JEP-106 Identity Code"]
    #[inline(always)]
    pub fn jepidcl(&self) -> JEPIDCL_R {
        JEPIDCL_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
