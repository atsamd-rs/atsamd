#[doc = "Register `STATUSB` reader"]
pub struct R(crate::R<STATUSB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STATUSB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STATUSB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STATUSB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `USB_` reader - USB APB Protect Enable"]
pub struct USB__R(crate::FieldReader<bool, bool>);
impl USB__R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        USB__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USB__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DSU_` reader - DSU APB Protect Enable"]
pub struct DSU__R(crate::FieldReader<bool, bool>);
impl DSU__R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DSU__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DSU__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NVMCTRL_` reader - NVMCTRL APB Protect Enable"]
pub struct NVMCTRL__R(crate::FieldReader<bool, bool>);
impl NVMCTRL__R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        NVMCTRL__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NVMCTRL__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMCC_` reader - CMCC APB Protect Enable"]
pub struct CMCC__R(crate::FieldReader<bool, bool>);
impl CMCC__R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CMCC__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMCC__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PORT_` reader - PORT APB Protect Enable"]
pub struct PORT__R(crate::FieldReader<bool, bool>);
impl PORT__R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PORT__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PORT__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMAC_` reader - DMAC APB Protect Enable"]
pub struct DMAC__R(crate::FieldReader<bool, bool>);
impl DMAC__R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DMAC__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMAC__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HMATRIX_` reader - HMATRIX APB Protect Enable"]
pub struct HMATRIX__R(crate::FieldReader<bool, bool>);
impl HMATRIX__R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        HMATRIX__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HMATRIX__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EVSYS_` reader - EVSYS APB Protect Enable"]
pub struct EVSYS__R(crate::FieldReader<bool, bool>);
impl EVSYS__R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EVSYS__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EVSYS__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SERCOM2_` reader - SERCOM2 APB Protect Enable"]
pub struct SERCOM2__R(crate::FieldReader<bool, bool>);
impl SERCOM2__R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SERCOM2__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SERCOM2__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SERCOM3_` reader - SERCOM3 APB Protect Enable"]
pub struct SERCOM3__R(crate::FieldReader<bool, bool>);
impl SERCOM3__R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SERCOM3__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SERCOM3__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TCC0_` reader - TCC0 APB Protect Enable"]
pub struct TCC0__R(crate::FieldReader<bool, bool>);
impl TCC0__R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TCC0__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TCC0__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TCC1_` reader - TCC1 APB Protect Enable"]
pub struct TCC1__R(crate::FieldReader<bool, bool>);
impl TCC1__R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TCC1__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TCC1__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TC2_` reader - TC2 APB Protect Enable"]
pub struct TC2__R(crate::FieldReader<bool, bool>);
impl TC2__R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TC2__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TC2__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TC3_` reader - TC3 APB Protect Enable"]
pub struct TC3__R(crate::FieldReader<bool, bool>);
impl TC3__R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TC3__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TC3__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RAMECC_` reader - RAMECC APB Protect Enable"]
pub struct RAMECC__R(crate::FieldReader<bool, bool>);
impl RAMECC__R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RAMECC__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RAMECC__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - USB APB Protect Enable"]
    #[inline(always)]
    pub fn usb_(&self) -> USB__R {
        USB__R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - DSU APB Protect Enable"]
    #[inline(always)]
    pub fn dsu_(&self) -> DSU__R {
        DSU__R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - NVMCTRL APB Protect Enable"]
    #[inline(always)]
    pub fn nvmctrl_(&self) -> NVMCTRL__R {
        NVMCTRL__R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - CMCC APB Protect Enable"]
    #[inline(always)]
    pub fn cmcc_(&self) -> CMCC__R {
        CMCC__R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - PORT APB Protect Enable"]
    #[inline(always)]
    pub fn port_(&self) -> PORT__R {
        PORT__R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - DMAC APB Protect Enable"]
    #[inline(always)]
    pub fn dmac_(&self) -> DMAC__R {
        DMAC__R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - HMATRIX APB Protect Enable"]
    #[inline(always)]
    pub fn hmatrix_(&self) -> HMATRIX__R {
        HMATRIX__R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - EVSYS APB Protect Enable"]
    #[inline(always)]
    pub fn evsys_(&self) -> EVSYS__R {
        EVSYS__R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 9 - SERCOM2 APB Protect Enable"]
    #[inline(always)]
    pub fn sercom2_(&self) -> SERCOM2__R {
        SERCOM2__R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - SERCOM3 APB Protect Enable"]
    #[inline(always)]
    pub fn sercom3_(&self) -> SERCOM3__R {
        SERCOM3__R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - TCC0 APB Protect Enable"]
    #[inline(always)]
    pub fn tcc0_(&self) -> TCC0__R {
        TCC0__R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - TCC1 APB Protect Enable"]
    #[inline(always)]
    pub fn tcc1_(&self) -> TCC1__R {
        TCC1__R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - TC2 APB Protect Enable"]
    #[inline(always)]
    pub fn tc2_(&self) -> TC2__R {
        TC2__R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - TC3 APB Protect Enable"]
    #[inline(always)]
    pub fn tc3_(&self) -> TC3__R {
        TC3__R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 16 - RAMECC APB Protect Enable"]
    #[inline(always)]
    pub fn ramecc_(&self) -> RAMECC__R {
        RAMECC__R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
#[doc = "Peripheral write protection status - Bridge B\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [statusb](index.html) module"]
pub struct STATUSB_SPEC;
impl crate::RegisterSpec for STATUSB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [statusb::R](R) reader structure"]
impl crate::Readable for STATUSB_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets STATUSB to value 0x02"]
impl crate::Resettable for STATUSB_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x02
    }
}
