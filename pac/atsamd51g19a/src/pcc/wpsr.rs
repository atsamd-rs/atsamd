#[doc = "Reader of register WPSR"]
pub type R = crate::R<u32, super::WPSR>;
#[doc = "Reader of field `WPVS`"]
pub type WPVS_R = crate::R<bool, bool>;
#[doc = "Reader of field `WPVSRC`"]
pub type WPVSRC_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bit 0 - Write Protection Violation Source"]
    #[inline(always)]
    pub fn wpvs(&self) -> WPVS_R {
        WPVS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 8:23 - Write Protection Violation Status"]
    #[inline(always)]
    pub fn wpvsrc(&self) -> WPVSRC_R {
        WPVSRC_R::new(((self.bits >> 8) & 0xffff) as u16)
    }
}
