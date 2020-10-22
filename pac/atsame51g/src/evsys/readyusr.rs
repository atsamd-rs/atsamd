#[doc = "Reader of register READYUSR"]
pub type R = crate::R<u32, super::READYUSR>;
#[doc = "Reader of field `READYUSR0`"]
pub type READYUSR0_R = crate::R<bool, bool>;
#[doc = "Reader of field `READYUSR1`"]
pub type READYUSR1_R = crate::R<bool, bool>;
#[doc = "Reader of field `READYUSR2`"]
pub type READYUSR2_R = crate::R<bool, bool>;
#[doc = "Reader of field `READYUSR3`"]
pub type READYUSR3_R = crate::R<bool, bool>;
#[doc = "Reader of field `READYUSR4`"]
pub type READYUSR4_R = crate::R<bool, bool>;
#[doc = "Reader of field `READYUSR5`"]
pub type READYUSR5_R = crate::R<bool, bool>;
#[doc = "Reader of field `READYUSR6`"]
pub type READYUSR6_R = crate::R<bool, bool>;
#[doc = "Reader of field `READYUSR7`"]
pub type READYUSR7_R = crate::R<bool, bool>;
#[doc = "Reader of field `READYUSR8`"]
pub type READYUSR8_R = crate::R<bool, bool>;
#[doc = "Reader of field `READYUSR9`"]
pub type READYUSR9_R = crate::R<bool, bool>;
#[doc = "Reader of field `READYUSR10`"]
pub type READYUSR10_R = crate::R<bool, bool>;
#[doc = "Reader of field `READYUSR11`"]
pub type READYUSR11_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Ready User for Channel 0"]
    #[inline(always)]
    pub fn readyusr0(&self) -> READYUSR0_R {
        READYUSR0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Ready User for Channel 1"]
    #[inline(always)]
    pub fn readyusr1(&self) -> READYUSR1_R {
        READYUSR1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Ready User for Channel 2"]
    #[inline(always)]
    pub fn readyusr2(&self) -> READYUSR2_R {
        READYUSR2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Ready User for Channel 3"]
    #[inline(always)]
    pub fn readyusr3(&self) -> READYUSR3_R {
        READYUSR3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Ready User for Channel 4"]
    #[inline(always)]
    pub fn readyusr4(&self) -> READYUSR4_R {
        READYUSR4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Ready User for Channel 5"]
    #[inline(always)]
    pub fn readyusr5(&self) -> READYUSR5_R {
        READYUSR5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Ready User for Channel 6"]
    #[inline(always)]
    pub fn readyusr6(&self) -> READYUSR6_R {
        READYUSR6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Ready User for Channel 7"]
    #[inline(always)]
    pub fn readyusr7(&self) -> READYUSR7_R {
        READYUSR7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Ready User for Channel 8"]
    #[inline(always)]
    pub fn readyusr8(&self) -> READYUSR8_R {
        READYUSR8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Ready User for Channel 9"]
    #[inline(always)]
    pub fn readyusr9(&self) -> READYUSR9_R {
        READYUSR9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Ready User for Channel 10"]
    #[inline(always)]
    pub fn readyusr10(&self) -> READYUSR10_R {
        READYUSR10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Ready User for Channel 11"]
    #[inline(always)]
    pub fn readyusr11(&self) -> READYUSR11_R {
        READYUSR11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
}
