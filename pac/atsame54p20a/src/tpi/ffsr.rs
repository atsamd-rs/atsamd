#[doc = "Reader of register FFSR"]
pub type R = crate::R<u32, super::FFSR>;
#[doc = "Reader of field `FlInProg`"]
pub type FLINPROG_R = crate::R<bool, bool>;
#[doc = "Reader of field `FtStopped`"]
pub type FTSTOPPED_R = crate::R<bool, bool>;
#[doc = "Reader of field `TCPresent`"]
pub type TCPRESENT_R = crate::R<bool, bool>;
#[doc = "Reader of field `FtNonStop`"]
pub type FTNONSTOP_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn fl_in_prog(&self) -> FLINPROG_R {
        FLINPROG_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn ft_stopped(&self) -> FTSTOPPED_R {
        FTSTOPPED_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn tcpresent(&self) -> TCPRESENT_R {
        TCPRESENT_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn ft_non_stop(&self) -> FTNONSTOP_R {
        FTNONSTOP_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
