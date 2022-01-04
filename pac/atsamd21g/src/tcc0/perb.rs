#[doc = "Register `PERB` reader"]
pub struct R(crate::R<PERB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PERB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PERB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PERB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PERB` writer"]
pub struct W(crate::W<PERB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PERB_SPEC>;
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
impl From<crate::W<PERB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PERB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PERB` reader - Period Buffer Value"]
pub struct PERB_R(crate::FieldReader<u32, u32>);
impl PERB_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        PERB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PERB_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PERB` writer - Period Buffer Value"]
pub struct PERB_W<'a> {
    w: &'a mut W,
}
impl<'a> PERB_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x00ff_ffff) | (value as u32 & 0x00ff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:23 - Period Buffer Value"]
    #[inline(always)]
    pub fn perb(&self) -> PERB_R {
        PERB_R::new((self.bits & 0x00ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:23 - Period Buffer Value"]
    #[inline(always)]
    pub fn perb(&mut self) -> PERB_W {
        PERB_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Period Buffer\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [perb](index.html) module"]
pub struct PERB_SPEC;
impl crate::RegisterSpec for PERB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [perb::R](R) reader structure"]
impl crate::Readable for PERB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [perb::W](W) writer structure"]
impl crate::Writable for PERB_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PERB to value 0xffff_ffff"]
impl crate::Resettable for PERB_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
