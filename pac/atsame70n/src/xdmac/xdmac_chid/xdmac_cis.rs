#[doc = "Register `XDMAC_CIS` reader"]
pub struct R(crate::R<XDMAC_CIS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<XDMAC_CIS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<XDMAC_CIS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<XDMAC_CIS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `BIS` reader - End of Block Interrupt Status Bit"]
pub struct BIS_R(crate::FieldReader<bool, bool>);
impl BIS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LIS` reader - End of Linked List Interrupt Status Bit"]
pub struct LIS_R(crate::FieldReader<bool, bool>);
impl LIS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIS` reader - End of Disable Interrupt Status Bit"]
pub struct DIS_R(crate::FieldReader<bool, bool>);
impl DIS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FIS` reader - End of Flush Interrupt Status Bit"]
pub struct FIS_R(crate::FieldReader<bool, bool>);
impl FIS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RBEIS` reader - Read Bus Error Interrupt Status Bit"]
pub struct RBEIS_R(crate::FieldReader<bool, bool>);
impl RBEIS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RBEIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RBEIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WBEIS` reader - Write Bus Error Interrupt Status Bit"]
pub struct WBEIS_R(crate::FieldReader<bool, bool>);
impl WBEIS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WBEIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WBEIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ROIS` reader - Request Overflow Error Interrupt Status Bit"]
pub struct ROIS_R(crate::FieldReader<bool, bool>);
impl ROIS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ROIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ROIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - End of Block Interrupt Status Bit"]
    #[inline(always)]
    pub fn bis(&self) -> BIS_R {
        BIS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - End of Linked List Interrupt Status Bit"]
    #[inline(always)]
    pub fn lis(&self) -> LIS_R {
        LIS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - End of Disable Interrupt Status Bit"]
    #[inline(always)]
    pub fn dis(&self) -> DIS_R {
        DIS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - End of Flush Interrupt Status Bit"]
    #[inline(always)]
    pub fn fis(&self) -> FIS_R {
        FIS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Read Bus Error Interrupt Status Bit"]
    #[inline(always)]
    pub fn rbeis(&self) -> RBEIS_R {
        RBEIS_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Write Bus Error Interrupt Status Bit"]
    #[inline(always)]
    pub fn wbeis(&self) -> WBEIS_R {
        WBEIS_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Request Overflow Error Interrupt Status Bit"]
    #[inline(always)]
    pub fn rois(&self) -> ROIS_R {
        ROIS_R::new(((self.bits >> 6) & 0x01) != 0)
    }
}
#[doc = "Channel Interrupt Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [xdmac_cis](index.html) module"]
pub struct XDMAC_CIS_SPEC;
impl crate::RegisterSpec for XDMAC_CIS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [xdmac_cis::R](R) reader structure"]
impl crate::Readable for XDMAC_CIS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets XDMAC_CIS to value 0"]
impl crate::Resettable for XDMAC_CIS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
