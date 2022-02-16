#[doc = "Register `XDMAC_CNDA` reader"]
pub struct R(crate::R<XDMAC_CNDA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<XDMAC_CNDA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<XDMAC_CNDA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<XDMAC_CNDA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `XDMAC_CNDA` writer"]
pub struct W(crate::W<XDMAC_CNDA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<XDMAC_CNDA_SPEC>;
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
impl From<crate::W<XDMAC_CNDA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<XDMAC_CNDA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `NDAIF` reader - Channel x Next Descriptor Interface"]
pub struct NDAIF_R(crate::FieldReader<bool, bool>);
impl NDAIF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        NDAIF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NDAIF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NDAIF` writer - Channel x Next Descriptor Interface"]
pub struct NDAIF_W<'a> {
    w: &'a mut W,
}
impl<'a> NDAIF_W<'a> {
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
#[doc = "Field `NDA` reader - Channel x Next Descriptor Address"]
pub struct NDA_R(crate::FieldReader<u32, u32>);
impl NDA_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        NDA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NDA_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NDA` writer - Channel x Next Descriptor Address"]
pub struct NDA_W<'a> {
    w: &'a mut W,
}
impl<'a> NDA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3fff_ffff << 2)) | ((value as u32 & 0x3fff_ffff) << 2);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Channel x Next Descriptor Interface"]
    #[inline(always)]
    pub fn ndaif(&self) -> NDAIF_R {
        NDAIF_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 2:31 - Channel x Next Descriptor Address"]
    #[inline(always)]
    pub fn nda(&self) -> NDA_R {
        NDA_R::new(((self.bits >> 2) & 0x3fff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bit 0 - Channel x Next Descriptor Interface"]
    #[inline(always)]
    pub fn ndaif(&mut self) -> NDAIF_W {
        NDAIF_W { w: self }
    }
    #[doc = "Bits 2:31 - Channel x Next Descriptor Address"]
    #[inline(always)]
    pub fn nda(&mut self) -> NDA_W {
        NDA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel Next Descriptor Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [xdmac_cnda](index.html) module"]
pub struct XDMAC_CNDA_SPEC;
impl crate::RegisterSpec for XDMAC_CNDA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [xdmac_cnda::R](R) reader structure"]
impl crate::Readable for XDMAC_CNDA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [xdmac_cnda::W](W) writer structure"]
impl crate::Writable for XDMAC_CNDA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets XDMAC_CNDA to value 0"]
impl crate::Resettable for XDMAC_CNDA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
