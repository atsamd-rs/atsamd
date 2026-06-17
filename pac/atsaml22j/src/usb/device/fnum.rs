#[doc = "Register `FNUM` reader"]
pub struct R(crate::R<FNUM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FNUM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FNUM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FNUM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `MFNUM` reader - Micro Frame Number"]
pub type MFNUM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FNUM` reader - Frame Number"]
pub type FNUM_R = crate::FieldReader<u16, u16>;
#[doc = "Field `FNCERR` reader - Frame Number CRC Error"]
pub type FNCERR_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 0:2 - Micro Frame Number"]
    #[inline(always)]
    pub fn mfnum(&self) -> MFNUM_R {
        MFNUM_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:13 - Frame Number"]
    #[inline(always)]
    pub fn fnum(&self) -> FNUM_R {
        FNUM_R::new((self.bits >> 3) & 0x07ff)
    }
    #[doc = "Bit 15 - Frame Number CRC Error"]
    #[inline(always)]
    pub fn fncerr(&self) -> FNCERR_R {
        FNCERR_R::new(((self.bits >> 15) & 1) != 0)
    }
}
#[doc = "DEVICE Device Frame Number\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fnum](index.html) module"]
pub struct FNUM_SPEC;
impl crate::RegisterSpec for FNUM_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [fnum::R](R) reader structure"]
impl crate::Readable for FNUM_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FNUM to value 0"]
impl crate::Resettable for FNUM_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
