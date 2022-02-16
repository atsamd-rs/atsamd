#[doc = "Register `USBHS_DEVFNUM` reader"]
pub struct R(crate::R<USBHS_DEVFNUM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USBHS_DEVFNUM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USBHS_DEVFNUM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USBHS_DEVFNUM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `MFNUM` reader - Micro Frame Number"]
pub struct MFNUM_R(crate::FieldReader<u8, u8>);
impl MFNUM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        MFNUM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MFNUM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FNUM` reader - Frame Number"]
pub struct FNUM_R(crate::FieldReader<u16, u16>);
impl FNUM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        FNUM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FNUM_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FNCERR` reader - Frame Number CRC Error"]
pub struct FNCERR_R(crate::FieldReader<bool, bool>);
impl FNCERR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FNCERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FNCERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:2 - Micro Frame Number"]
    #[inline(always)]
    pub fn mfnum(&self) -> MFNUM_R {
        MFNUM_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 3:13 - Frame Number"]
    #[inline(always)]
    pub fn fnum(&self) -> FNUM_R {
        FNUM_R::new(((self.bits >> 3) & 0x07ff) as u16)
    }
    #[doc = "Bit 15 - Frame Number CRC Error"]
    #[inline(always)]
    pub fn fncerr(&self) -> FNCERR_R {
        FNCERR_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
#[doc = "Device Frame Number Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbhs_devfnum](index.html) module"]
pub struct USBHS_DEVFNUM_SPEC;
impl crate::RegisterSpec for USBHS_DEVFNUM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [usbhs_devfnum::R](R) reader structure"]
impl crate::Readable for USBHS_DEVFNUM_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets USBHS_DEVFNUM to value 0"]
impl crate::Resettable for USBHS_DEVFNUM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
