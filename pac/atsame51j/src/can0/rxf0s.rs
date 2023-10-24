#[doc = "Register `RXF0S` reader"]
pub type R = crate::R<RXF0S_SPEC>;
#[doc = "Field `F0FL` reader - Rx FIFO 0 Fill Level"]
pub type F0FL_R = crate::FieldReader;
#[doc = "Field `F0GI` reader - Rx FIFO 0 Get Index"]
pub type F0GI_R = crate::FieldReader;
#[doc = "Field `F0PI` reader - Rx FIFO 0 Put Index"]
pub type F0PI_R = crate::FieldReader;
#[doc = "Field `F0F` reader - Rx FIFO 0 Full"]
pub type F0F_R = crate::BitReader;
#[doc = "Field `RF0L` reader - Rx FIFO 0 Message Lost"]
pub type RF0L_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:6 - Rx FIFO 0 Fill Level"]
    #[inline(always)]
    pub fn f0fl(&self) -> F0FL_R {
        F0FL_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:13 - Rx FIFO 0 Get Index"]
    #[inline(always)]
    pub fn f0gi(&self) -> F0GI_R {
        F0GI_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - Rx FIFO 0 Put Index"]
    #[inline(always)]
    pub fn f0pi(&self) -> F0PI_R {
        F0PI_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bit 24 - Rx FIFO 0 Full"]
    #[inline(always)]
    pub fn f0f(&self) -> F0F_R {
        F0F_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Rx FIFO 0 Message Lost"]
    #[inline(always)]
    pub fn rf0l(&self) -> RF0L_R {
        RF0L_R::new(((self.bits >> 25) & 1) != 0)
    }
}
#[doc = "Rx FIFO 0 Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxf0s::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RXF0S_SPEC;
impl crate::RegisterSpec for RXF0S_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxf0s::R`](R) reader structure"]
impl crate::Readable for RXF0S_SPEC {}
#[doc = "`reset()` method sets RXF0S to value 0"]
impl crate::Resettable for RXF0S_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
