#[doc = "Register `TPSF` reader"]
pub struct R(crate::R<TPSF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TPSF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TPSF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TPSF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TPSF` writer"]
pub struct W(crate::W<TPSF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TPSF_SPEC>;
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
impl From<crate::W<TPSF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TPSF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TPB1ADR` reader - TX packet buffer address"]
pub struct TPB1ADR_R(crate::FieldReader<u16, u16>);
impl TPB1ADR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        TPB1ADR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TPB1ADR_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TPB1ADR` writer - TX packet buffer address"]
pub struct TPB1ADR_W<'a> {
    w: &'a mut W,
}
impl<'a> TPB1ADR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | (value as u32 & 0x03ff);
        self.w
    }
}
#[doc = "Field `ENTXP` reader - Enable TX partial store and forward operation"]
pub struct ENTXP_R(crate::FieldReader<bool, bool>);
impl ENTXP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ENTXP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENTXP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENTXP` writer - Enable TX partial store and forward operation"]
pub struct ENTXP_W<'a> {
    w: &'a mut W,
}
impl<'a> ENTXP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:9 - TX packet buffer address"]
    #[inline(always)]
    pub fn tpb1adr(&self) -> TPB1ADR_R {
        TPB1ADR_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 31 - Enable TX partial store and forward operation"]
    #[inline(always)]
    pub fn entxp(&self) -> ENTXP_R {
        ENTXP_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9 - TX packet buffer address"]
    #[inline(always)]
    pub fn tpb1adr(&mut self) -> TPB1ADR_W {
        TPB1ADR_W { w: self }
    }
    #[doc = "Bit 31 - Enable TX partial store and forward operation"]
    #[inline(always)]
    pub fn entxp(&mut self) -> ENTXP_W {
        ENTXP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TX partial store and forward Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tpsf](index.html) module"]
pub struct TPSF_SPEC;
impl crate::RegisterSpec for TPSF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tpsf::R](R) reader structure"]
impl crate::Readable for TPSF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tpsf::W](W) writer structure"]
impl crate::Writable for TPSF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TPSF to value 0x03ff"]
impl crate::Resettable for TPSF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x03ff
    }
}
