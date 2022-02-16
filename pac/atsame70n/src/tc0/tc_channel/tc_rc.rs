#[doc = "Register `TC_RC` reader"]
pub struct R(crate::R<TC_RC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TC_RC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TC_RC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TC_RC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TC_RC` writer"]
pub struct W(crate::W<TC_RC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TC_RC_SPEC>;
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
impl From<crate::W<TC_RC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TC_RC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RC` reader - Register C"]
pub struct RC_R(crate::FieldReader<u32, u32>);
impl RC_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        RC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RC_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RC` writer - Register C"]
pub struct RC_W<'a> {
    w: &'a mut W,
}
impl<'a> RC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value as u32;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Register C"]
    #[inline(always)]
    pub fn rc(&self) -> RC_R {
        RC_R::new(self.bits as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Register C"]
    #[inline(always)]
    pub fn rc(&mut self) -> RC_W {
        RC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Register C (channel = 0)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tc_rc](index.html) module"]
pub struct TC_RC_SPEC;
impl crate::RegisterSpec for TC_RC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tc_rc::R](R) reader structure"]
impl crate::Readable for TC_RC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tc_rc::W](W) writer structure"]
impl crate::Writable for TC_RC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TC_RC to value 0"]
impl crate::Resettable for TC_RC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
