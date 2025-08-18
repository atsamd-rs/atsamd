#[doc = "Register `INTFLAGB` reader"]
pub struct R(crate::R<INTFLAGB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTFLAGB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTFLAGB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTFLAGB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTFLAGB` writer"]
pub struct W(crate::W<INTFLAGB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTFLAGB_SPEC>;
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
impl From<crate::W<INTFLAGB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTFLAGB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PORT_` reader - PORT"]
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
#[doc = "Field `PORT_` writer - PORT"]
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Field `DSU_` reader - DSU"]
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
#[doc = "Field `DSU_` writer - DSU"]
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
#[doc = "Field `NVMCTRL_` reader - NVMCTRL"]
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
#[doc = "Field `NVMCTRL_` writer - NVMCTRL"]
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
#[doc = "Field `DMAC_` reader - DMAC"]
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
#[doc = "Field `DMAC_` writer - DMAC"]
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `MTB_` reader - MTB"]
pub struct MTB__R(crate::FieldReader<bool, bool>);
impl MTB__R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MTB__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MTB__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MTB_` writer - MTB"]
pub struct MTB__W<'a> {
    w: &'a mut W,
}
impl<'a> MTB__W<'a> {
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
#[doc = "Field `HMATRIXHS_` reader - HMATRIXHS"]
pub struct HMATRIXHS__R(crate::FieldReader<bool, bool>);
impl HMATRIXHS__R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        HMATRIXHS__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HMATRIXHS__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HMATRIXHS_` writer - HMATRIXHS"]
pub struct HMATRIXHS__W<'a> {
    w: &'a mut W,
}
impl<'a> HMATRIXHS__W<'a> {
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
impl R {
    #[doc = "Bit 0 - PORT"]
    #[inline(always)]
    pub fn port_(&self) -> PORT__R {
        PORT__R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - DSU"]
    #[inline(always)]
    pub fn dsu_(&self) -> DSU__R {
        DSU__R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - NVMCTRL"]
    #[inline(always)]
    pub fn nvmctrl_(&self) -> NVMCTRL__R {
        NVMCTRL__R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - DMAC"]
    #[inline(always)]
    pub fn dmac_(&self) -> DMAC__R {
        DMAC__R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - MTB"]
    #[inline(always)]
    pub fn mtb_(&self) -> MTB__R {
        MTB__R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - HMATRIXHS"]
    #[inline(always)]
    pub fn hmatrixhs_(&self) -> HMATRIXHS__R {
        HMATRIXHS__R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PORT"]
    #[inline(always)]
    pub fn port_(&mut self) -> PORT__W {
        PORT__W { w: self }
    }
    #[doc = "Bit 1 - DSU"]
    #[inline(always)]
    pub fn dsu_(&mut self) -> DSU__W {
        DSU__W { w: self }
    }
    #[doc = "Bit 2 - NVMCTRL"]
    #[inline(always)]
    pub fn nvmctrl_(&mut self) -> NVMCTRL__W {
        NVMCTRL__W { w: self }
    }
    #[doc = "Bit 3 - DMAC"]
    #[inline(always)]
    pub fn dmac_(&mut self) -> DMAC__W {
        DMAC__W { w: self }
    }
    #[doc = "Bit 4 - MTB"]
    #[inline(always)]
    pub fn mtb_(&mut self) -> MTB__W {
        MTB__W { w: self }
    }
    #[doc = "Bit 5 - HMATRIXHS"]
    #[inline(always)]
    pub fn hmatrixhs_(&mut self) -> HMATRIXHS__W {
        HMATRIXHS__W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Peripheral interrupt flag status - Bridge B\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intflagb](index.html) module"]
pub struct INTFLAGB_SPEC;
impl crate::RegisterSpec for INTFLAGB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intflagb::R](R) reader structure"]
impl crate::Readable for INTFLAGB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intflagb::W](W) writer structure"]
impl crate::Writable for INTFLAGB_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTFLAGB to value 0"]
impl crate::Resettable for INTFLAGB_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
