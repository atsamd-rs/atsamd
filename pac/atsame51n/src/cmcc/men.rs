#[doc = "Register `MEN` reader"]
pub struct R(crate::R<MEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MEN` writer"]
pub struct W(crate::W<MEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MEN_SPEC>;
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
impl From<crate::W<MEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MENABLE` reader - Cache Controller Monitor Enable"]
pub struct MENABLE_R(crate::FieldReader<bool, bool>);
impl MENABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MENABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MENABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MENABLE` writer - Cache Controller Monitor Enable"]
pub struct MENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> MENABLE_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Cache Controller Monitor Enable"]
    #[inline(always)]
    pub fn menable(&self) -> MENABLE_R {
        MENABLE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Cache Controller Monitor Enable"]
    #[inline(always)]
    pub fn menable(&mut self) -> MENABLE_W {
        MENABLE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Cache Monitor Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [men](index.html) module"]
pub struct MEN_SPEC;
impl crate::RegisterSpec for MEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [men::R](R) reader structure"]
impl crate::Readable for MEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [men::W](W) writer structure"]
impl crate::Writable for MEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MEN to value 0"]
impl crate::Resettable for MEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
