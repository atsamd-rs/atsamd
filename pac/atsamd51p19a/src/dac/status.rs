#[doc = "Reader of register STATUS"]
pub type R = crate::R<u8, super::STATUS>;
#[doc = "Reader of field `READY0`"]
pub type READY0_R = crate::R<bool, bool>;
#[doc = "Reader of field `READY1`"]
pub type READY1_R = crate::R<bool, bool>;
#[doc = "Reader of field `EOC0`"]
pub type EOC0_R = crate::R<bool, bool>;
#[doc = "Reader of field `EOC1`"]
pub type EOC1_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - DAC 0 Startup Ready"]
    #[inline(always)]
    pub fn ready0(&self) -> READY0_R {
        READY0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - DAC 1 Startup Ready"]
    #[inline(always)]
    pub fn ready1(&self) -> READY1_R {
        READY1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - DAC 0 End of Conversion"]
    #[inline(always)]
    pub fn eoc0(&self) -> EOC0_R {
        EOC0_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - DAC 1 End of Conversion"]
    #[inline(always)]
    pub fn eoc1(&self) -> EOC1_R {
        EOC1_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
