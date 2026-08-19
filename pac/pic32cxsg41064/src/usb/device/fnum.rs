#[doc = "Register `FNUM` reader"]
pub type R = crate::R<FnumSpec>;
#[doc = "Field `FNUM` reader - Frame Number"]
pub type FnumR = crate::FieldReader<u16>;
#[doc = "Field `FNCERR` reader - Frame Number CRC Error"]
pub type FncerrR = crate::BitReader;
impl R {
    #[doc = "Bits 3:13 - Frame Number"]
    #[inline(always)]
    pub fn fnum(&self) -> FnumR {
        FnumR::new((self.bits >> 3) & 0x07ff)
    }
    #[doc = "Bit 15 - Frame Number CRC Error"]
    #[inline(always)]
    pub fn fncerr(&self) -> FncerrR {
        FncerrR::new(((self.bits >> 15) & 1) != 0)
    }
}
#[doc = "DEVICE Device Frame Number\n\nYou can [`read`](crate::Reg::read) this register and get [`fnum::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FnumSpec;
impl crate::RegisterSpec for FnumSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`fnum::R`](R) reader structure"]
impl crate::Readable for FnumSpec {}
#[doc = "`reset()` method sets FNUM to value 0"]
impl crate::Resettable for FnumSpec {
    const RESET_VALUE: u16 = 0;
}
