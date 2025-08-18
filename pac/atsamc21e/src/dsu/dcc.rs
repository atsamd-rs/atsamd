#[doc = "Register `DCC[%s]` reader"]
pub struct R(crate::R<DCC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DCC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DCC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DCC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DCC[%s]` writer"]
pub struct W(crate::W<DCC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DCC_SPEC>;
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
impl From<crate::W<DCC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DCC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATA` reader - Data"]
pub struct DATA_R(crate::FieldReader<u32, u32>);
impl DATA_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        DATA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DATA_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DATA` writer - Data"]
pub struct DATA_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value as u32;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Data"]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new(self.bits as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Data"]
    #[inline(always)]
    pub fn data(&mut self) -> DATA_W {
        DATA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Debug Communication Channel n\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcc](index.html) module"]
pub struct DCC_SPEC;
impl crate::RegisterSpec for DCC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dcc::R](R) reader structure"]
impl crate::Readable for DCC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dcc::W](W) writer structure"]
impl crate::Writable for DCC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DCC[%s]
to value 0"]
impl crate::Resettable for DCC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
