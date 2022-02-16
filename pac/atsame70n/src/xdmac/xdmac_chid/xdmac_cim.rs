#[doc = "Register `XDMAC_CIM` reader"]
pub struct R(crate::R<XDMAC_CIM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<XDMAC_CIM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<XDMAC_CIM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<XDMAC_CIM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `BIM` reader - End of Block Interrupt Mask Bit"]
pub struct BIM_R(crate::FieldReader<bool, bool>);
impl BIM_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BIM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LIM` reader - End of Linked List Interrupt Mask Bit"]
pub struct LIM_R(crate::FieldReader<bool, bool>);
impl LIM_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LIM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIM` reader - End of Disable Interrupt Mask Bit"]
pub struct DIM_R(crate::FieldReader<bool, bool>);
impl DIM_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FIM` reader - End of Flush Interrupt Mask Bit"]
pub struct FIM_R(crate::FieldReader<bool, bool>);
impl FIM_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FIM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RBEIM` reader - Read Bus Error Interrupt Mask Bit"]
pub struct RBEIM_R(crate::FieldReader<bool, bool>);
impl RBEIM_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RBEIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RBEIM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WBEIM` reader - Write Bus Error Interrupt Mask Bit"]
pub struct WBEIM_R(crate::FieldReader<bool, bool>);
impl WBEIM_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WBEIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WBEIM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ROIM` reader - Request Overflow Error Interrupt Mask Bit"]
pub struct ROIM_R(crate::FieldReader<bool, bool>);
impl ROIM_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ROIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ROIM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - End of Block Interrupt Mask Bit"]
    #[inline(always)]
    pub fn bim(&self) -> BIM_R {
        BIM_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - End of Linked List Interrupt Mask Bit"]
    #[inline(always)]
    pub fn lim(&self) -> LIM_R {
        LIM_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - End of Disable Interrupt Mask Bit"]
    #[inline(always)]
    pub fn dim(&self) -> DIM_R {
        DIM_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - End of Flush Interrupt Mask Bit"]
    #[inline(always)]
    pub fn fim(&self) -> FIM_R {
        FIM_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Read Bus Error Interrupt Mask Bit"]
    #[inline(always)]
    pub fn rbeim(&self) -> RBEIM_R {
        RBEIM_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Write Bus Error Interrupt Mask Bit"]
    #[inline(always)]
    pub fn wbeim(&self) -> WBEIM_R {
        WBEIM_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Request Overflow Error Interrupt Mask Bit"]
    #[inline(always)]
    pub fn roim(&self) -> ROIM_R {
        ROIM_R::new(((self.bits >> 6) & 0x01) != 0)
    }
}
#[doc = "Channel Interrupt Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [xdmac_cim](index.html) module"]
pub struct XDMAC_CIM_SPEC;
impl crate::RegisterSpec for XDMAC_CIM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [xdmac_cim::R](R) reader structure"]
impl crate::Readable for XDMAC_CIM_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets XDMAC_CIM to value 0"]
impl crate::Resettable for XDMAC_CIM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
