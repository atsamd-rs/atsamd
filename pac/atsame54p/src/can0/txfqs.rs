#[doc = "Register `TXFQS` reader"]
pub type R = crate::R<TXFQS_SPEC>;
#[doc = "Field `TFFL` reader - Tx FIFO Free Level"]
pub type TFFL_R = crate::FieldReader;
#[doc = "Field `TFGI` reader - Tx FIFO Get Index"]
pub type TFGI_R = crate::FieldReader;
#[doc = "Field `TFQPI` reader - Tx FIFO/Queue Put Index"]
pub type TFQPI_R = crate::FieldReader;
#[doc = "Field `TFQF` reader - Tx FIFO/Queue Full"]
pub type TFQF_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:5 - Tx FIFO Free Level"]
    #[inline(always)]
    pub fn tffl(&self) -> TFFL_R {
        TFFL_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:12 - Tx FIFO Get Index"]
    #[inline(always)]
    pub fn tfgi(&self) -> TFGI_R {
        TFGI_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - Tx FIFO/Queue Put Index"]
    #[inline(always)]
    pub fn tfqpi(&self) -> TFQPI_R {
        TFQPI_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bit 21 - Tx FIFO/Queue Full"]
    #[inline(always)]
    pub fn tfqf(&self) -> TFQF_R {
        TFQF_R::new(((self.bits >> 21) & 1) != 0)
    }
}
#[doc = "Tx FIFO / Queue Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txfqs::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TXFQS_SPEC;
impl crate::RegisterSpec for TXFQS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txfqs::R`](R) reader structure"]
impl crate::Readable for TXFQS_SPEC {}
#[doc = "`reset()` method sets TXFQS to value 0"]
impl crate::Resettable for TXFQS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
