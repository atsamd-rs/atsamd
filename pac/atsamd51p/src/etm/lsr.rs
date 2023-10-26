#[doc = "Register `LSR` reader"]
pub type R = crate::R<LSR_SPEC>;
#[doc = "Field `Present` reader - "]
pub type PRESENT_R = crate::BitReader;
#[doc = "Field `Access` reader - "]
pub type ACCESS_R = crate::BitReader;
#[doc = "Field `ByteAcc` reader - "]
pub type BYTE_ACC_R = crate::BitReader;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn present(&self) -> PRESENT_R {
        PRESENT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn access(&self) -> ACCESS_R {
        ACCESS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn byte_acc(&self) -> BYTE_ACC_R {
        BYTE_ACC_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[doc = "ETM Lock Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lsr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LSR_SPEC;
impl crate::RegisterSpec for LSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lsr::R`](R) reader structure"]
impl crate::Readable for LSR_SPEC {}
#[doc = "`reset()` method sets LSR to value 0"]
impl crate::Resettable for LSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
