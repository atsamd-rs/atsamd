#[doc = "Register `LSR` reader"]
pub type R = crate::R<LsrSpec>;
#[doc = "Field `Present` reader - "]
pub type PresentR = crate::BitReader;
#[doc = "Field `Access` reader - "]
pub type AccessR = crate::BitReader;
#[doc = "Field `ByteAcc` reader - "]
pub type ByteAccR = crate::BitReader;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn present(&self) -> PresentR {
        PresentR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn access(&self) -> AccessR {
        AccessR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn byte_acc(&self) -> ByteAccR {
        ByteAccR::new(((self.bits >> 2) & 1) != 0)
    }
}
#[doc = "ETM Lock Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`lsr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LsrSpec;
impl crate::RegisterSpec for LsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lsr::R`](R) reader structure"]
impl crate::Readable for LsrSpec {}
#[doc = "`reset()` method sets LSR to value 0"]
impl crate::Resettable for LsrSpec {}
