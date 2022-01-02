#[doc = "Register `GENDIV` reader"]
pub struct R(crate::R<GENDIV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GENDIV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GENDIV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GENDIV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GENDIV` writer"]
pub struct W(crate::W<GENDIV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GENDIV_SPEC>;
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
impl From<crate::W<GENDIV_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GENDIV_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ID` reader - Generic Clock Generator Selection"]
pub struct ID_R(crate::FieldReader<u8, u8>);
impl ID_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        ID_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ID_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ID` writer - Generic Clock Generator Selection"]
pub struct ID_W<'a> {
    w: &'a mut W,
}
impl<'a> ID_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
#[doc = "Field `DIV` reader - Division Factor"]
pub struct DIV_R(crate::FieldReader<u16, u16>);
impl DIV_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        DIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIV_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIV` writer - Division Factor"]
pub struct DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> DIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 8)) | ((value as u32 & 0xffff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Generic Clock Generator Selection"]
    #[inline(always)]
    pub fn id(&self) -> ID_R {
        ID_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:23 - Division Factor"]
    #[inline(always)]
    pub fn div(&self) -> DIV_R {
        DIV_R::new(((self.bits >> 8) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:3 - Generic Clock Generator Selection"]
    #[inline(always)]
    pub fn id(&mut self) -> ID_W {
        ID_W { w: self }
    }
    #[doc = "Bits 8:23 - Division Factor"]
    #[inline(always)]
    pub fn div(&mut self) -> DIV_W {
        DIV_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Generic Clock Generator Division\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gendiv](index.html) module"]
pub struct GENDIV_SPEC;
impl crate::RegisterSpec for GENDIV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gendiv::R](R) reader structure"]
impl crate::Readable for GENDIV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gendiv::W](W) writer structure"]
impl crate::Writable for GENDIV_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GENDIV to value 0"]
impl crate::Resettable for GENDIV_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
