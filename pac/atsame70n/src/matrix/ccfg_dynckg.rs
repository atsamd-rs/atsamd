#[doc = "Register `CCFG_DYNCKG` reader"]
pub struct R(crate::R<CCFG_DYNCKG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCFG_DYNCKG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CCFG_DYNCKG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CCFG_DYNCKG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CCFG_DYNCKG` writer"]
pub struct W(crate::W<CCFG_DYNCKG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CCFG_DYNCKG_SPEC>;
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
impl From<crate::W<CCFG_DYNCKG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CCFG_DYNCKG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MATCKG` reader - MATRIX Dynamic Clock Gating"]
pub struct MATCKG_R(crate::FieldReader<bool, bool>);
impl MATCKG_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MATCKG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MATCKG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MATCKG` writer - MATRIX Dynamic Clock Gating"]
pub struct MATCKG_W<'a> {
    w: &'a mut W,
}
impl<'a> MATCKG_W<'a> {
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
#[doc = "Field `BRIDCKG` reader - Bridge Dynamic Clock Gating Enable"]
pub struct BRIDCKG_R(crate::FieldReader<bool, bool>);
impl BRIDCKG_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BRIDCKG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BRIDCKG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BRIDCKG` writer - Bridge Dynamic Clock Gating Enable"]
pub struct BRIDCKG_W<'a> {
    w: &'a mut W,
}
impl<'a> BRIDCKG_W<'a> {
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
#[doc = "Field `EFCCKG` reader - EFC Dynamic Clock Gating Enable"]
pub struct EFCCKG_R(crate::FieldReader<bool, bool>);
impl EFCCKG_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EFCCKG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EFCCKG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EFCCKG` writer - EFC Dynamic Clock Gating Enable"]
pub struct EFCCKG_W<'a> {
    w: &'a mut W,
}
impl<'a> EFCCKG_W<'a> {
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
impl R {
    #[doc = "Bit 0 - MATRIX Dynamic Clock Gating"]
    #[inline(always)]
    pub fn matckg(&self) -> MATCKG_R {
        MATCKG_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Bridge Dynamic Clock Gating Enable"]
    #[inline(always)]
    pub fn bridckg(&self) -> BRIDCKG_R {
        BRIDCKG_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - EFC Dynamic Clock Gating Enable"]
    #[inline(always)]
    pub fn efcckg(&self) -> EFCCKG_R {
        EFCCKG_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - MATRIX Dynamic Clock Gating"]
    #[inline(always)]
    pub fn matckg(&mut self) -> MATCKG_W {
        MATCKG_W { w: self }
    }
    #[doc = "Bit 1 - Bridge Dynamic Clock Gating Enable"]
    #[inline(always)]
    pub fn bridckg(&mut self) -> BRIDCKG_W {
        BRIDCKG_W { w: self }
    }
    #[doc = "Bit 2 - EFC Dynamic Clock Gating Enable"]
    #[inline(always)]
    pub fn efcckg(&mut self) -> EFCCKG_W {
        EFCCKG_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Dynamic Clock Gating Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccfg_dynckg](index.html) module"]
pub struct CCFG_DYNCKG_SPEC;
impl crate::RegisterSpec for CCFG_DYNCKG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ccfg_dynckg::R](R) reader structure"]
impl crate::Readable for CCFG_DYNCKG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ccfg_dynckg::W](W) writer structure"]
impl crate::Writable for CCFG_DYNCKG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CCFG_DYNCKG to value 0"]
impl crate::Resettable for CCFG_DYNCKG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
