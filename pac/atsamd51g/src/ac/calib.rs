#[doc = "Register `CALIB` reader"]
pub struct R(crate::R<CALIB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CALIB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CALIB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CALIB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CALIB` writer"]
pub struct W(crate::W<CALIB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CALIB_SPEC>;
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
impl From<crate::W<CALIB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CALIB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BIAS0` reader - COMP0/1 Bias Scaling"]
pub struct BIAS0_R(crate::FieldReader<u8, u8>);
impl BIAS0_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        BIAS0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BIAS0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BIAS0` writer - COMP0/1 Bias Scaling"]
pub struct BIAS0_W<'a> {
    w: &'a mut W,
}
impl<'a> BIAS0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u16 & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - COMP0/1 Bias Scaling"]
    #[inline(always)]
    pub fn bias0(&self) -> BIAS0_R {
        BIAS0_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - COMP0/1 Bias Scaling"]
    #[inline(always)]
    pub fn bias0(&mut self) -> BIAS0_W {
        BIAS0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Calibration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [calib](index.html) module"]
pub struct CALIB_SPEC;
impl crate::RegisterSpec for CALIB_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [calib::R](R) reader structure"]
impl crate::Readable for CALIB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [calib::W](W) writer structure"]
impl crate::Writable for CALIB_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CALIB to value 0x0101"]
impl crate::Resettable for CALIB_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0101
    }
}
