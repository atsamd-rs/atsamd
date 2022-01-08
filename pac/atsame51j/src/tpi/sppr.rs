#[doc = "Register `SPPR` reader"]
pub struct R(crate::R<SPPR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPPR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPPR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPPR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPPR` writer"]
pub struct W(crate::W<SPPR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPPR_SPEC>;
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
impl From<crate::W<SPPR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPPR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TXMODE` reader - "]
pub struct TXMODE_R(crate::FieldReader<u8, u8>);
impl TXMODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TXMODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXMODE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXMODE` writer - "]
pub struct TXMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> TXMODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn txmode(&self) -> TXMODE_R {
        TXMODE_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn txmode(&mut self) -> TXMODE_W {
        TXMODE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Selected Pin Protocol Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sppr](index.html) module"]
pub struct SPPR_SPEC;
impl crate::RegisterSpec for SPPR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sppr::R](R) reader structure"]
impl crate::Readable for SPPR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sppr::W](W) writer structure"]
impl crate::Writable for SPPR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SPPR to value 0"]
impl crate::Resettable for SPPR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
