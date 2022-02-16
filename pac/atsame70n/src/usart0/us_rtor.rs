#[doc = "Register `US_RTOR` reader"]
pub struct R(crate::R<US_RTOR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<US_RTOR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<US_RTOR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<US_RTOR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `US_RTOR` writer"]
pub struct W(crate::W<US_RTOR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<US_RTOR_SPEC>;
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
impl From<crate::W<US_RTOR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<US_RTOR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TO` reader - Timeout Value"]
pub struct TO_R(crate::FieldReader<u32, u32>);
impl TO_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        TO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TO_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TO` writer - Timeout Value"]
pub struct TO_W<'a> {
    w: &'a mut W,
}
impl<'a> TO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0001_ffff) | (value as u32 & 0x0001_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:16 - Timeout Value"]
    #[inline(always)]
    pub fn to(&self) -> TO_R {
        TO_R::new((self.bits & 0x0001_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:16 - Timeout Value"]
    #[inline(always)]
    pub fn to(&mut self) -> TO_W {
        TO_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Receiver Timeout Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [us_rtor](index.html) module"]
pub struct US_RTOR_SPEC;
impl crate::RegisterSpec for US_RTOR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [us_rtor::R](R) reader structure"]
impl crate::Readable for US_RTOR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [us_rtor::W](W) writer structure"]
impl crate::Writable for US_RTOR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets US_RTOR to value 0"]
impl crate::Resettable for US_RTOR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
