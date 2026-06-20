#[doc = "Register `GAIN` reader"]
pub struct R(crate::R<GAIN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GAIN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GAIN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GAIN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GAIN` writer"]
pub struct W(crate::W<GAIN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GAIN_SPEC>;
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
impl From<crate::W<GAIN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GAIN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GAIN` reader - Time Amplifier Gain"]
pub struct GAIN_R(crate::FieldReader<u32, u32>);
impl GAIN_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        GAIN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GAIN_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GAIN` writer - Time Amplifier Gain"]
pub struct GAIN_W<'a> {
    w: &'a mut W,
}
impl<'a> GAIN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x00ff_ffff) | (value as u32 & 0x00ff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:23 - Time Amplifier Gain"]
    #[inline(always)]
    pub fn gain(&self) -> GAIN_R {
        GAIN_R::new((self.bits & 0x00ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:23 - Time Amplifier Gain"]
    #[inline(always)]
    pub fn gain(&mut self) -> GAIN_W {
        GAIN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Gain Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gain](index.html) module"]
pub struct GAIN_SPEC;
impl crate::RegisterSpec for GAIN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gain::R](R) reader structure"]
impl crate::Readable for GAIN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gain::W](W) writer structure"]
impl crate::Writable for GAIN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GAIN to value 0"]
impl crate::Resettable for GAIN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
