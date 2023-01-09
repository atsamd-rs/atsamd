#[doc = "Register `STATUS` reader"]
pub struct R(crate::R<STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `BOD33RDY` reader - BOD33 Ready"]
pub type BOD33RDY_R = crate::BitReader<bool>;
#[doc = "Field `BOD33DET` reader - BOD33 Detection"]
pub type BOD33DET_R = crate::BitReader<bool>;
#[doc = "Field `B33SRDY` reader - BOD33 Synchronization Ready"]
pub type B33SRDY_R = crate::BitReader<bool>;
#[doc = "Field `BOD12RDY` reader - BOD12 Ready"]
pub type BOD12RDY_R = crate::BitReader<bool>;
#[doc = "Field `BOD12DET` reader - BOD12 Detection"]
pub type BOD12DET_R = crate::BitReader<bool>;
#[doc = "Field `B12SRDY` reader - BOD12 Synchronization Ready"]
pub type B12SRDY_R = crate::BitReader<bool>;
#[doc = "Field `VREGRDY` reader - Voltage Regulator Ready"]
pub type VREGRDY_R = crate::BitReader<bool>;
#[doc = "Field `APWSRDY` reader - Automatic Power Switch Ready"]
pub type APWSRDY_R = crate::BitReader<bool>;
#[doc = "Field `VCORERDY` reader - VDDCORE Ready"]
pub type VCORERDY_R = crate::BitReader<bool>;
#[doc = "Field `BBPS` reader - Battery Backup Power Switch"]
pub type BBPS_R = crate::BitReader<bool>;
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
    #[doc = "Bit 3 - BOD12 Ready"]
    #[inline(always)]
    pub fn bod12rdy(&self) -> BOD12RDY_R {
        BOD12RDY_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - BOD12 Detection"]
    #[inline(always)]
    pub fn bod12det(&self) -> BOD12DET_R {
        BOD12DET_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - BOD12 Synchronization Ready"]
    #[inline(always)]
    pub fn b12srdy(&self) -> B12SRDY_R {
        B12SRDY_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - Voltage Regulator Ready"]
    #[inline(always)]
    pub fn vregrdy(&self) -> VREGRDY_R {
        VREGRDY_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Automatic Power Switch Ready"]
    #[inline(always)]
    pub fn apwsrdy(&self) -> APWSRDY_R {
        APWSRDY_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - VDDCORE Ready"]
    #[inline(always)]
    pub fn vcorerdy(&self) -> VCORERDY_R {
        VCORERDY_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Battery Backup Power Switch"]
    #[inline(always)]
    pub fn bbps(&self) -> BBPS_R {
        BBPS_R::new(((self.bits >> 11) & 1) != 0)
    }
}
#[doc = "Power and Clocks Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status](index.html) module"]
pub struct STATUS_SPEC;
impl crate::RegisterSpec for STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [status::R](R) reader structure"]
impl crate::Readable for STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
