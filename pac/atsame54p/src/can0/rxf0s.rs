#[doc = "Reader of register RXF0S"]
pub type R = crate::R<u32, super::RXF0S>;
#[doc = "Reader of field `F0FL`"]
pub type F0FL_R = crate::R<u8, u8>;
#[doc = "Reader of field `F0GI`"]
pub type F0GI_R = crate::R<u8, u8>;
#[doc = "Reader of field `F0PI`"]
pub type F0PI_R = crate::R<u8, u8>;
#[doc = "Reader of field `F0F`"]
pub type F0F_R = crate::R<bool, bool>;
#[doc = "Reader of field `RF0L`"]
pub type RF0L_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bits 0:6 - Rx FIFO 0 Fill Level"]
    #[inline(always)]
    pub fn f0fl(&self) -> F0FL_R {
        F0FL_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:13 - Rx FIFO 0 Get Index"]
    #[inline(always)]
    pub fn f0gi(&self) -> F0GI_R {
        F0GI_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - Rx FIFO 0 Put Index"]
    #[inline(always)]
    pub fn f0pi(&self) -> F0PI_R {
        F0PI_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bit 24 - Rx FIFO 0 Full"]
    #[inline(always)]
    pub fn f0f(&self) -> F0F_R {
        F0F_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Rx FIFO 0 Message Lost"]
    #[inline(always)]
    pub fn rf0l(&self) -> RF0L_R {
        RF0L_R::new(((self.bits >> 25) & 0x01) != 0)
    }
}
