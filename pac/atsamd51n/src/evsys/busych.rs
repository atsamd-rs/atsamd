#[doc = "Reader of register BUSYCH"]
pub type R = crate::R<u32, super::BUSYCH>;
#[doc = "Reader of field `BUSYCH0`"]
pub type BUSYCH0_R = crate::R<bool, bool>;
#[doc = "Reader of field `BUSYCH1`"]
pub type BUSYCH1_R = crate::R<bool, bool>;
#[doc = "Reader of field `BUSYCH2`"]
pub type BUSYCH2_R = crate::R<bool, bool>;
#[doc = "Reader of field `BUSYCH3`"]
pub type BUSYCH3_R = crate::R<bool, bool>;
#[doc = "Reader of field `BUSYCH4`"]
pub type BUSYCH4_R = crate::R<bool, bool>;
#[doc = "Reader of field `BUSYCH5`"]
pub type BUSYCH5_R = crate::R<bool, bool>;
#[doc = "Reader of field `BUSYCH6`"]
pub type BUSYCH6_R = crate::R<bool, bool>;
#[doc = "Reader of field `BUSYCH7`"]
pub type BUSYCH7_R = crate::R<bool, bool>;
#[doc = "Reader of field `BUSYCH8`"]
pub type BUSYCH8_R = crate::R<bool, bool>;
#[doc = "Reader of field `BUSYCH9`"]
pub type BUSYCH9_R = crate::R<bool, bool>;
#[doc = "Reader of field `BUSYCH10`"]
pub type BUSYCH10_R = crate::R<bool, bool>;
#[doc = "Reader of field `BUSYCH11`"]
pub type BUSYCH11_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Busy Channel 0"]
    #[inline(always)]
    pub fn busych0(&self) -> BUSYCH0_R {
        BUSYCH0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Busy Channel 1"]
    #[inline(always)]
    pub fn busych1(&self) -> BUSYCH1_R {
        BUSYCH1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Busy Channel 2"]
    #[inline(always)]
    pub fn busych2(&self) -> BUSYCH2_R {
        BUSYCH2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Busy Channel 3"]
    #[inline(always)]
    pub fn busych3(&self) -> BUSYCH3_R {
        BUSYCH3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Busy Channel 4"]
    #[inline(always)]
    pub fn busych4(&self) -> BUSYCH4_R {
        BUSYCH4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Busy Channel 5"]
    #[inline(always)]
    pub fn busych5(&self) -> BUSYCH5_R {
        BUSYCH5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Busy Channel 6"]
    #[inline(always)]
    pub fn busych6(&self) -> BUSYCH6_R {
        BUSYCH6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Busy Channel 7"]
    #[inline(always)]
    pub fn busych7(&self) -> BUSYCH7_R {
        BUSYCH7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Busy Channel 8"]
    #[inline(always)]
    pub fn busych8(&self) -> BUSYCH8_R {
        BUSYCH8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Busy Channel 9"]
    #[inline(always)]
    pub fn busych9(&self) -> BUSYCH9_R {
        BUSYCH9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Busy Channel 10"]
    #[inline(always)]
    pub fn busych10(&self) -> BUSYCH10_R {
        BUSYCH10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Busy Channel 11"]
    #[inline(always)]
    pub fn busych11(&self) -> BUSYCH11_R {
        BUSYCH11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
}
