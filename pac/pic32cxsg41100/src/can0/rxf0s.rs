#[doc = "Register `RXF0S` reader"]
pub type R = crate::R<Rxf0sSpec>;
#[doc = "Field `F0FL` reader - Rx FIFO 0 Fill Level"]
pub type F0flR = crate::FieldReader;
#[doc = "Field `F0GI` reader - Rx FIFO 0 Get Index"]
pub type F0giR = crate::FieldReader;
#[doc = "Field `F0PI` reader - Rx FIFO 0 Put Index"]
pub type F0piR = crate::FieldReader;
#[doc = "Field `F0F` reader - Rx FIFO 0 Full"]
pub type F0fR = crate::BitReader;
#[doc = "Field `RF0L` reader - Rx FIFO 0 Message Lost"]
pub type Rf0lR = crate::BitReader;
impl R {
    #[doc = "Bits 0:6 - Rx FIFO 0 Fill Level"]
    #[inline(always)]
    pub fn f0fl(&self) -> F0flR {
        F0flR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:13 - Rx FIFO 0 Get Index"]
    #[inline(always)]
    pub fn f0gi(&self) -> F0giR {
        F0giR::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - Rx FIFO 0 Put Index"]
    #[inline(always)]
    pub fn f0pi(&self) -> F0piR {
        F0piR::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bit 24 - Rx FIFO 0 Full"]
    #[inline(always)]
    pub fn f0f(&self) -> F0fR {
        F0fR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Rx FIFO 0 Message Lost"]
    #[inline(always)]
    pub fn rf0l(&self) -> Rf0lR {
        Rf0lR::new(((self.bits >> 25) & 1) != 0)
    }
}
#[doc = "Rx FIFO 0 Status\n\nYou can [`read`](crate::Reg::read) this register and get [`rxf0s::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rxf0sSpec;
impl crate::RegisterSpec for Rxf0sSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxf0s::R`](R) reader structure"]
impl crate::Readable for Rxf0sSpec {}
#[doc = "`reset()` method sets RXF0S to value 0"]
impl crate::Resettable for Rxf0sSpec {
    const RESET_VALUE: u32 = 0;
}
