#[doc = "Register `DEVID` reader"]
pub type R = crate::R<DEVID_SPEC>;
#[doc = "Field `NrTraceInput` reader - "]
pub type NR_TRACE_INPUT_R = crate::BitReader;
#[doc = "Field `AsynClkIn` reader - "]
pub type ASYN_CLK_IN_R = crate::BitReader;
#[doc = "Field `MinBufSz` reader - "]
pub type MIN_BUF_SZ_R = crate::FieldReader;
#[doc = "Field `PTINVALID` reader - "]
pub type PTINVALID_R = crate::BitReader;
#[doc = "Field `MANCVALID` reader - "]
pub type MANCVALID_R = crate::BitReader;
#[doc = "Field `NRZVALID` reader - "]
pub type NRZVALID_R = crate::BitReader;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn nr_trace_input(&self) -> NR_TRACE_INPUT_R {
        NR_TRACE_INPUT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn asyn_clk_in(&self) -> ASYN_CLK_IN_R {
        ASYN_CLK_IN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:8"]
    #[inline(always)]
    pub fn min_buf_sz(&self) -> MIN_BUF_SZ_R {
        MIN_BUF_SZ_R::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn ptinvalid(&self) -> PTINVALID_R {
        PTINVALID_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn mancvalid(&self) -> MANCVALID_R {
        MANCVALID_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn nrzvalid(&self) -> NRZVALID_R {
        NRZVALID_R::new(((self.bits >> 11) & 1) != 0)
    }
}
#[doc = "TPIU_DEVID\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`devid::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DEVID_SPEC;
impl crate::RegisterSpec for DEVID_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`devid::R`](R) reader structure"]
impl crate::Readable for DEVID_SPEC {}
#[doc = "`reset()` method sets DEVID to value 0"]
impl crate::Resettable for DEVID_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
