#[doc = "Register `FIFO1` reader"]
pub struct R(crate::R<FIFO1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FIFO1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FIFO1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FIFO1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ITM0` reader - "]
pub struct ITM0_R(crate::FieldReader<u8, u8>);
impl ITM0_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        ITM0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ITM0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ITM1` reader - "]
pub struct ITM1_R(crate::FieldReader<u8, u8>);
impl ITM1_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        ITM1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ITM1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ITM2` reader - "]
pub struct ITM2_R(crate::FieldReader<u8, u8>);
impl ITM2_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        ITM2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ITM2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ETM_bytecount` reader - "]
pub struct ETM_BYTECOUNT_R(crate::FieldReader<u8, u8>);
impl ETM_BYTECOUNT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        ETM_BYTECOUNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ETM_BYTECOUNT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ETM_ATVALID` reader - "]
pub struct ETM_ATVALID_R(crate::FieldReader<bool, bool>);
impl ETM_ATVALID_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ETM_ATVALID_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ETM_ATVALID_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ITM_bytecount` reader - "]
pub struct ITM_BYTECOUNT_R(crate::FieldReader<u8, u8>);
impl ITM_BYTECOUNT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        ITM_BYTECOUNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ITM_BYTECOUNT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ITM_ATVALID` reader - "]
pub struct ITM_ATVALID_R(crate::FieldReader<bool, bool>);
impl ITM_ATVALID_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ITM_ATVALID_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ITM_ATVALID_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn itm0(&self) -> ITM0_R {
        ITM0_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn itm1(&self) -> ITM1_R {
        ITM1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn itm2(&self) -> ITM2_R {
        ITM2_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn etm_bytecount(&self) -> ETM_BYTECOUNT_R {
        ETM_BYTECOUNT_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn etm_atvalid(&self) -> ETM_ATVALID_R {
        ETM_ATVALID_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bits 27:28"]
    #[inline(always)]
    pub fn itm_bytecount(&self) -> ITM_BYTECOUNT_R {
        ITM_BYTECOUNT_R::new(((self.bits >> 27) & 0x03) as u8)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn itm_atvalid(&self) -> ITM_ATVALID_R {
        ITM_ATVALID_R::new(((self.bits >> 29) & 0x01) != 0)
    }
}
#[doc = "Integration ITM Data\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fifo1](index.html) module"]
pub struct FIFO1_SPEC;
impl crate::RegisterSpec for FIFO1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fifo1::R](R) reader structure"]
impl crate::Readable for FIFO1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FIFO1 to value 0"]
impl crate::Resettable for FIFO1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
