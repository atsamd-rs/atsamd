#[doc = "Register `STATUS` reader"]
pub type R = crate::R<STATUS_SPEC>;
#[doc = "Field `BOD33RDY` reader - BOD33 Ready"]
pub type BOD33RDY_R = crate::BitReader;
#[doc = "Field `BOD33DET` reader - BOD33 Detection"]
pub type BOD33DET_R = crate::BitReader;
#[doc = "Field `B33SRDY` reader - BOD33 Synchronization Ready"]
pub type B33SRDY_R = crate::BitReader;
#[doc = "Field `VREGRDY` reader - Voltage Regulator Ready"]
pub type VREGRDY_R = crate::BitReader;
#[doc = "Field `VCORERDY` reader - VDDCORE Ready"]
pub type VCORERDY_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - BOD33 Ready"]
    #[inline(always)]
    pub fn bod33rdy(&self) -> BOD33RDY_R {
        BOD33RDY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - BOD33 Detection"]
    #[inline(always)]
    pub fn bod33det(&self) -> BOD33DET_R {
        BOD33DET_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - BOD33 Synchronization Ready"]
    #[inline(always)]
    pub fn b33srdy(&self) -> B33SRDY_R {
        B33SRDY_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - Voltage Regulator Ready"]
    #[inline(always)]
    pub fn vregrdy(&self) -> VREGRDY_R {
        VREGRDY_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - VDDCORE Ready"]
    #[inline(always)]
    pub fn vcorerdy(&self) -> VCORERDY_R {
        VCORERDY_R::new(((self.bits >> 10) & 1) != 0)
    }
}
#[doc = "Power and Clocks Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STATUS_SPEC;
impl crate::RegisterSpec for STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for STATUS_SPEC {}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
