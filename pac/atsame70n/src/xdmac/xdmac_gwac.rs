#[doc = "Register `XDMAC_GWAC` reader"]
pub struct R(crate::R<XDMAC_GWAC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<XDMAC_GWAC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<XDMAC_GWAC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<XDMAC_GWAC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `XDMAC_GWAC` writer"]
pub struct W(crate::W<XDMAC_GWAC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<XDMAC_GWAC_SPEC>;
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
impl From<crate::W<XDMAC_GWAC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<XDMAC_GWAC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PW0` reader - Pool Weight 0"]
pub struct PW0_R(crate::FieldReader<u8, u8>);
impl PW0_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PW0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PW0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PW0` writer - Pool Weight 0"]
pub struct PW0_W<'a> {
    w: &'a mut W,
}
impl<'a> PW0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
#[doc = "Field `PW1` reader - Pool Weight 1"]
pub struct PW1_R(crate::FieldReader<u8, u8>);
impl PW1_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PW1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PW1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PW1` writer - Pool Weight 1"]
pub struct PW1_W<'a> {
    w: &'a mut W,
}
impl<'a> PW1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | ((value as u32 & 0x0f) << 4);
        self.w
    }
}
#[doc = "Field `PW2` reader - Pool Weight 2"]
pub struct PW2_R(crate::FieldReader<u8, u8>);
impl PW2_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PW2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PW2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PW2` writer - Pool Weight 2"]
pub struct PW2_W<'a> {
    w: &'a mut W,
}
impl<'a> PW2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u32 & 0x0f) << 8);
        self.w
    }
}
#[doc = "Field `PW3` reader - Pool Weight 3"]
pub struct PW3_R(crate::FieldReader<u8, u8>);
impl PW3_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PW3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PW3_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PW3` writer - Pool Weight 3"]
pub struct PW3_W<'a> {
    w: &'a mut W,
}
impl<'a> PW3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | ((value as u32 & 0x0f) << 12);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Pool Weight 0"]
    #[inline(always)]
    pub fn pw0(&self) -> PW0_R {
        PW0_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Pool Weight 1"]
    #[inline(always)]
    pub fn pw1(&self) -> PW1_R {
        PW1_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Pool Weight 2"]
    #[inline(always)]
    pub fn pw2(&self) -> PW2_R {
        PW2_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Pool Weight 3"]
    #[inline(always)]
    pub fn pw3(&self) -> PW3_R {
        PW3_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Pool Weight 0"]
    #[inline(always)]
    pub fn pw0(&mut self) -> PW0_W {
        PW0_W { w: self }
    }
    #[doc = "Bits 4:7 - Pool Weight 1"]
    #[inline(always)]
    pub fn pw1(&mut self) -> PW1_W {
        PW1_W { w: self }
    }
    #[doc = "Bits 8:11 - Pool Weight 2"]
    #[inline(always)]
    pub fn pw2(&mut self) -> PW2_W {
        PW2_W { w: self }
    }
    #[doc = "Bits 12:15 - Pool Weight 3"]
    #[inline(always)]
    pub fn pw3(&mut self) -> PW3_W {
        PW3_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Global Weighted Arbiter Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [xdmac_gwac](index.html) module"]
pub struct XDMAC_GWAC_SPEC;
impl crate::RegisterSpec for XDMAC_GWAC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [xdmac_gwac::R](R) reader structure"]
impl crate::Readable for XDMAC_GWAC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [xdmac_gwac::W](W) writer structure"]
impl crate::Writable for XDMAC_GWAC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets XDMAC_GWAC to value 0"]
impl crate::Resettable for XDMAC_GWAC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
