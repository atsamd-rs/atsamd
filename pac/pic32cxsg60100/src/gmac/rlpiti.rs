#[doc = "Register `RLPITI` reader"]
pub type R = crate::R<RlpitiSpec>;
#[doc = "Field `RLPITI` reader - Increment once over 16 ahb clock when LPI indication bit 20 is set in rx mode"]
pub type RlpitiR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:23 - Increment once over 16 ahb clock when LPI indication bit 20 is set in rx mode"]
    #[inline(always)]
    pub fn rlpiti(&self) -> RlpitiR {
        RlpitiR::new(self.bits & 0x00ff_ffff)
    }
}
#[doc = "Receive LPI Time Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rlpiti::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RlpitiSpec;
impl crate::RegisterSpec for RlpitiSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rlpiti::R`](R) reader structure"]
impl crate::Readable for RlpitiSpec {}
#[doc = "`reset()` method sets RLPITI to value 0"]
impl crate::Resettable for RlpitiSpec {
    const RESET_VALUE: u32 = 0;
}
