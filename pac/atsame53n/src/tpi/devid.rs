#[doc = "Register `DEVID` reader"]
pub type R = crate::R<DevidSpec>;
#[doc = "Field `NrTraceInput` reader - "]
pub type NrTraceInputR = crate::BitReader;
#[doc = "Field `AsynClkIn` reader - "]
pub type AsynClkInR = crate::BitReader;
#[doc = "Field `MinBufSz` reader - "]
pub type MinBufSzR = crate::FieldReader;
#[doc = "Field `PTINVALID` reader - "]
pub type PtinvalidR = crate::BitReader;
#[doc = "Field `MANCVALID` reader - "]
pub type MancvalidR = crate::BitReader;
#[doc = "Field `NRZVALID` reader - "]
pub type NrzvalidR = crate::BitReader;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn nr_trace_input(&self) -> NrTraceInputR {
        NrTraceInputR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn asyn_clk_in(&self) -> AsynClkInR {
        AsynClkInR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:8"]
    #[inline(always)]
    pub fn min_buf_sz(&self) -> MinBufSzR {
        MinBufSzR::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn ptinvalid(&self) -> PtinvalidR {
        PtinvalidR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn mancvalid(&self) -> MancvalidR {
        MancvalidR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn nrzvalid(&self) -> NrzvalidR {
        NrzvalidR::new(((self.bits >> 11) & 1) != 0)
    }
}
#[doc = "TPIU_DEVID\n\nYou can [`read`](crate::Reg::read) this register and get [`devid::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DevidSpec;
impl crate::RegisterSpec for DevidSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`devid::R`](R) reader structure"]
impl crate::Readable for DevidSpec {}
#[doc = "`reset()` method sets DEVID to value 0"]
impl crate::Resettable for DevidSpec {
    const RESET_VALUE: u32 = 0;
}
