#[doc = "Register `APBBMASK` reader"]
pub struct R(crate::R<APBBMASK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APBBMASK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APBBMASK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APBBMASK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `APBBMASK` writer"]
pub struct W(crate::W<APBBMASK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APBBMASK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<APBBMASK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APBBMASK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PAC1_` reader - PAC1 APB Clock Enable"]
pub struct PAC1__R(crate::FieldReader<bool, bool>);
impl PAC1__R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PAC1__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAC1__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAC1_` writer - PAC1 APB Clock Enable"]
pub struct PAC1__W<'a> {
    w: &'a mut W,
}
impl<'a> PAC1__W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Field `DSU_` reader - DSU APB Clock Enable"]
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
#[doc = "Field `DSU_` writer - DSU APB Clock Enable"]
pub struct DSU__W<'a> {
    w: &'a mut W,
}
impl<'a> DSU__W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `NVMCTRL_` reader - NVMCTRL APB Clock Enable"]
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
#[doc = "Field `NVMCTRL_` writer - NVMCTRL APB Clock Enable"]
pub struct NVMCTRL__W<'a> {
    w: &'a mut W,
}
impl<'a> NVMCTRL__W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `PORT_` reader - PORT APB Clock Enable"]
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
#[doc = "Field `PORT_` writer - PORT APB Clock Enable"]
pub struct PORT__W<'a> {
    w: &'a mut W,
}
impl<'a> PORT__W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `DMAC_` reader - DMAC APB Clock Enable"]
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
#[doc = "Field `DMAC_` writer - DMAC APB Clock Enable"]
pub struct DMAC__W<'a> {
    w: &'a mut W,
}
impl<'a> DMAC__W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `USB_` reader - USB APB Clock Enable"]
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
#[doc = "Field `USB_` writer - USB APB Clock Enable"]
pub struct USB__W<'a> {
    w: &'a mut W,
}
impl<'a> USB__W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `HMATRIX_` reader - HMATRIX APB Clock Enable"]
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
#[doc = "Field `HMATRIX_` writer - HMATRIX APB Clock Enable"]
pub struct HMATRIX__W<'a> {
    w: &'a mut W,
}
impl<'a> HMATRIX__W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - PAC1 APB Clock Enable"]
    #[inline(always)]
    pub fn pac1_(&self) -> PAC1__R {
        PAC1__R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - DSU APB Clock Enable"]
    #[inline(always)]
    pub fn dsu_(&self) -> DSU__R {
        DSU__R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - NVMCTRL APB Clock Enable"]
    #[inline(always)]
    pub fn nvmctrl_(&self) -> NVMCTRL__R {
        NVMCTRL__R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - PORT APB Clock Enable"]
    #[inline(always)]
    pub fn port_(&self) -> PORT__R {
        PORT__R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - DMAC APB Clock Enable"]
    #[inline(always)]
    pub fn dmac_(&self) -> DMAC__R {
        DMAC__R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - USB APB Clock Enable"]
    #[inline(always)]
    pub fn usb_(&self) -> USB__R {
        USB__R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - HMATRIX APB Clock Enable"]
    #[inline(always)]
    pub fn hmatrix_(&self) -> HMATRIX__R {
        HMATRIX__R::new(((self.bits >> 6) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PAC1 APB Clock Enable"]
    #[inline(always)]
    pub fn pac1_(&mut self) -> PAC1__W {
        PAC1__W { w: self }
    }
    #[doc = "Bit 1 - DSU APB Clock Enable"]
    #[inline(always)]
    pub fn dsu_(&mut self) -> DSU__W {
        DSU__W { w: self }
    }
    #[doc = "Bit 2 - NVMCTRL APB Clock Enable"]
    #[inline(always)]
    pub fn nvmctrl_(&mut self) -> NVMCTRL__W {
        NVMCTRL__W { w: self }
    }
    #[doc = "Bit 3 - PORT APB Clock Enable"]
    #[inline(always)]
    pub fn port_(&mut self) -> PORT__W {
        PORT__W { w: self }
    }
    #[doc = "Bit 4 - DMAC APB Clock Enable"]
    #[inline(always)]
    pub fn dmac_(&mut self) -> DMAC__W {
        DMAC__W { w: self }
    }
    #[doc = "Bit 5 - USB APB Clock Enable"]
    #[inline(always)]
    pub fn usb_(&mut self) -> USB__W {
        USB__W { w: self }
    }
    #[doc = "Bit 6 - HMATRIX APB Clock Enable"]
    #[inline(always)]
    pub fn hmatrix_(&mut self) -> HMATRIX__W {
        HMATRIX__W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "APBB Mask\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apbbmask](index.html) module"]
pub struct APBBMASK_SPEC;
impl crate::RegisterSpec for APBBMASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [apbbmask::R](R) reader structure"]
impl crate::Readable for APBBMASK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [apbbmask::W](W) writer structure"]
impl crate::Writable for APBBMASK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets APBBMASK to value 0x7f"]
impl crate::Resettable for APBBMASK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x7f
    }
}
