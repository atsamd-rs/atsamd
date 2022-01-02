#[doc = "Register `CHINTENSET` reader"]
pub struct R(crate::R<CHINTENSET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CHINTENSET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CHINTENSET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CHINTENSET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CHINTENSET` writer"]
pub struct W(crate::W<CHINTENSET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CHINTENSET_SPEC>;
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
impl From<crate::W<CHINTENSET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CHINTENSET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TERR` reader - Channel Transfer Error Interrupt Enable"]
pub struct TERR_R(crate::FieldReader<bool, bool>);
impl TERR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TERR` writer - Channel Transfer Error Interrupt Enable"]
pub struct TERR_W<'a> {
    w: &'a mut W,
}
impl<'a> TERR_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | (value as u8 & 0x01);
        self.w
    }
}
#[doc = "Field `TCMPL` reader - Channel Transfer Complete Interrupt Enable"]
pub struct TCMPL_R(crate::FieldReader<bool, bool>);
impl TCMPL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TCMPL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TCMPL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TCMPL` writer - Channel Transfer Complete Interrupt Enable"]
pub struct TCMPL_W<'a> {
    w: &'a mut W,
}
impl<'a> TCMPL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u8 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `SUSP` reader - Channel Suspend Interrupt Enable"]
pub struct SUSP_R(crate::FieldReader<bool, bool>);
impl SUSP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SUSP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SUSP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SUSP` writer - Channel Suspend Interrupt Enable"]
pub struct SUSP_W<'a> {
    w: &'a mut W,
}
impl<'a> SUSP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u8 & 0x01) << 2);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Channel Transfer Error Interrupt Enable"]
    #[inline(always)]
    pub fn terr(&self) -> TERR_R {
        TERR_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Channel Transfer Complete Interrupt Enable"]
    #[inline(always)]
    pub fn tcmpl(&self) -> TCMPL_R {
        TCMPL_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Channel Suspend Interrupt Enable"]
    #[inline(always)]
    pub fn susp(&self) -> SUSP_R {
        SUSP_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Channel Transfer Error Interrupt Enable"]
    #[inline(always)]
    pub fn terr(&mut self) -> TERR_W {
        TERR_W { w: self }
    }
    #[doc = "Bit 1 - Channel Transfer Complete Interrupt Enable"]
    #[inline(always)]
    pub fn tcmpl(&mut self) -> TCMPL_W {
        TCMPL_W { w: self }
    }
    #[doc = "Bit 2 - Channel Suspend Interrupt Enable"]
    #[inline(always)]
    pub fn susp(&mut self) -> SUSP_W {
        SUSP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel n Interrupt Enable Set\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chintenset](index.html) module"]
pub struct CHINTENSET_SPEC;
impl crate::RegisterSpec for CHINTENSET_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [chintenset::R](R) reader structure"]
impl crate::Readable for CHINTENSET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [chintenset::W](W) writer structure"]
impl crate::Writable for CHINTENSET_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CHINTENSET to value 0"]
impl crate::Resettable for CHINTENSET_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
