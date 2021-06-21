#[doc = "Reader of register CHSTATUS"]
pub type R = crate::R<u32, super::CHSTATUS>;
#[doc = "Reader of field `USRRDY0`"]
pub type USRRDY0_R = crate::R<bool, bool>;
#[doc = "Reader of field `USRRDY1`"]
pub type USRRDY1_R = crate::R<bool, bool>;
#[doc = "Reader of field `USRRDY2`"]
pub type USRRDY2_R = crate::R<bool, bool>;
#[doc = "Reader of field `USRRDY3`"]
pub type USRRDY3_R = crate::R<bool, bool>;
#[doc = "Reader of field `USRRDY4`"]
pub type USRRDY4_R = crate::R<bool, bool>;
#[doc = "Reader of field `USRRDY5`"]
pub type USRRDY5_R = crate::R<bool, bool>;
#[doc = "Reader of field `CHBUSY0`"]
pub type CHBUSY0_R = crate::R<bool, bool>;
#[doc = "Reader of field `CHBUSY1`"]
pub type CHBUSY1_R = crate::R<bool, bool>;
#[doc = "Reader of field `CHBUSY2`"]
pub type CHBUSY2_R = crate::R<bool, bool>;
#[doc = "Reader of field `CHBUSY3`"]
pub type CHBUSY3_R = crate::R<bool, bool>;
#[doc = "Reader of field `CHBUSY4`"]
pub type CHBUSY4_R = crate::R<bool, bool>;
#[doc = "Reader of field `CHBUSY5`"]
pub type CHBUSY5_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Channel 0 User Ready"]
    #[inline(always)]
    pub fn usrrdy0(&self) -> USRRDY0_R {
        USRRDY0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Channel 1 User Ready"]
    #[inline(always)]
    pub fn usrrdy1(&self) -> USRRDY1_R {
        USRRDY1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Channel 2 User Ready"]
    #[inline(always)]
    pub fn usrrdy2(&self) -> USRRDY2_R {
        USRRDY2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Channel 3 User Ready"]
    #[inline(always)]
    pub fn usrrdy3(&self) -> USRRDY3_R {
        USRRDY3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Channel 4 User Ready"]
    #[inline(always)]
    pub fn usrrdy4(&self) -> USRRDY4_R {
        USRRDY4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Channel 5 User Ready"]
    #[inline(always)]
    pub fn usrrdy5(&self) -> USRRDY5_R {
        USRRDY5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Channel 0 Busy"]
    #[inline(always)]
    pub fn chbusy0(&self) -> CHBUSY0_R {
        CHBUSY0_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Channel 1 Busy"]
    #[inline(always)]
    pub fn chbusy1(&self) -> CHBUSY1_R {
        CHBUSY1_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Channel 2 Busy"]
    #[inline(always)]
    pub fn chbusy2(&self) -> CHBUSY2_R {
        CHBUSY2_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Channel 3 Busy"]
    #[inline(always)]
    pub fn chbusy3(&self) -> CHBUSY3_R {
        CHBUSY3_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Channel 4 Busy"]
    #[inline(always)]
    pub fn chbusy4(&self) -> CHBUSY4_R {
        CHBUSY4_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Channel 5 Busy"]
    #[inline(always)]
    pub fn chbusy5(&self) -> CHBUSY5_R {
        CHBUSY5_R::new(((self.bits >> 13) & 0x01) != 0)
    }
}
