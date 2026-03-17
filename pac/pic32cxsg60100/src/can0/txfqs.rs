#[doc = "Register `TXFQS` reader"]
pub type R = crate::R<TxfqsSpec>;
#[doc = "Field `TFFL` reader - Tx FIFO Free Level"]
pub type TfflR = crate::FieldReader;
#[doc = "Field `TFGI` reader - Tx FIFO Get Index"]
pub type TfgiR = crate::FieldReader;
#[doc = "Field `TFQPI` reader - Tx FIFO/Queue Put Index"]
pub type TfqpiR = crate::FieldReader;
#[doc = "Field `TFQF` reader - Tx FIFO/Queue Full"]
pub type TfqfR = crate::BitReader;
impl R {
    #[doc = "Bits 0:5 - Tx FIFO Free Level"]
    #[inline(always)]
    pub fn tffl(&self) -> TfflR {
        TfflR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:12 - Tx FIFO Get Index"]
    #[inline(always)]
    pub fn tfgi(&self) -> TfgiR {
        TfgiR::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - Tx FIFO/Queue Put Index"]
    #[inline(always)]
    pub fn tfqpi(&self) -> TfqpiR {
        TfqpiR::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bit 21 - Tx FIFO/Queue Full"]
    #[inline(always)]
    pub fn tfqf(&self) -> TfqfR {
        TfqfR::new(((self.bits >> 21) & 1) != 0)
    }
}
#[doc = "Tx FIFO / Queue Status\n\nYou can [`read`](crate::Reg::read) this register and get [`txfqs::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxfqsSpec;
impl crate::RegisterSpec for TxfqsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txfqs::R`](R) reader structure"]
impl crate::Readable for TxfqsSpec {}
#[doc = "`reset()` method sets TXFQS to value 0"]
impl crate::Resettable for TxfqsSpec {
    const RESET_VALUE: u32 = 0;
}
