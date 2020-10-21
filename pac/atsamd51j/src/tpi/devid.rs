#[doc = "Reader of register DEVID"]
pub type R = crate::R<u32, super::DEVID>;
#[doc = "Reader of field `NrTraceInput`"]
pub type NRTRACEINPUT_R = crate::R<bool, bool>;
#[doc = "Reader of field `AsynClkIn`"]
pub type ASYNCLKIN_R = crate::R<bool, bool>;
#[doc = "Reader of field `MinBufSz`"]
pub type MINBUFSZ_R = crate::R<u8, u8>;
#[doc = "Reader of field `PTINVALID`"]
pub type PTINVALID_R = crate::R<bool, bool>;
#[doc = "Reader of field `MANCVALID`"]
pub type MANCVALID_R = crate::R<bool, bool>;
#[doc = "Reader of field `NRZVALID`"]
pub type NRZVALID_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn nr_trace_input(&self) -> NRTRACEINPUT_R {
        NRTRACEINPUT_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn asyn_clk_in(&self) -> ASYNCLKIN_R {
        ASYNCLKIN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bits 6:8"]
    #[inline(always)]
    pub fn min_buf_sz(&self) -> MINBUFSZ_R {
        MINBUFSZ_R::new(((self.bits >> 6) & 0x07) as u8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn ptinvalid(&self) -> PTINVALID_R {
        PTINVALID_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn mancvalid(&self) -> MANCVALID_R {
        MANCVALID_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn nrzvalid(&self) -> NRZVALID_R {
        NRZVALID_R::new(((self.bits >> 11) & 0x01) != 0)
    }
}
