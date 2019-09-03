#[doc = "Reader of register FNUM"]
pub type R = crate::R<u16, super::FNUM>;
#[doc = "Reader of field `MFNUM`"]
pub type MFNUM_R = crate::R<u8, u8>;
#[doc = "Reader of field `FNUM`"]
pub type FNUM_R = crate::R<u16, u16>;
#[doc = "Reader of field `FNCERR`"]
pub type FNCERR_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bits 0:2 - Micro Frame Number"]
    #[inline(always)]
    pub fn mfnum(&self) -> MFNUM_R {
        MFNUM_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 3:13 - Frame Number"]
    #[inline(always)]
    pub fn fnum(&self) -> FNUM_R {
        FNUM_R::new(((self.bits >> 3) & 0x07ff) as u16)
    }
    #[doc = "Bit 15 - Frame Number CRC Error"]
    #[inline(always)]
    pub fn fncerr(&self) -> FNCERR_R {
        FNCERR_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
