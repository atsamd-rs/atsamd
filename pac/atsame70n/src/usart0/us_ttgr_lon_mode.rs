#[doc = "Register `US_TTGR_LON_MODE` reader"]
pub struct R(crate::R<US_TTGR_LON_MODE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<US_TTGR_LON_MODE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<US_TTGR_LON_MODE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<US_TTGR_LON_MODE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `US_TTGR_LON_MODE` writer"]
pub struct W(crate::W<US_TTGR_LON_MODE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<US_TTGR_LON_MODE_SPEC>;
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
impl From<crate::W<US_TTGR_LON_MODE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<US_TTGR_LON_MODE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PCYCLE` reader - LON PCYCLE Length"]
pub struct PCYCLE_R(crate::FieldReader<u32, u32>);
impl PCYCLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        PCYCLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PCYCLE_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PCYCLE` writer - LON PCYCLE Length"]
pub struct PCYCLE_W<'a> {
    w: &'a mut W,
}
impl<'a> PCYCLE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x00ff_ffff) | (value as u32 & 0x00ff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:23 - LON PCYCLE Length"]
    #[inline(always)]
    pub fn pcycle(&self) -> PCYCLE_R {
        PCYCLE_R::new((self.bits & 0x00ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:23 - LON PCYCLE Length"]
    #[inline(always)]
    pub fn pcycle(&mut self) -> PCYCLE_W {
        PCYCLE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Transmitter Timeguard Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [us_ttgr_lon_mode](index.html) module"]
pub struct US_TTGR_LON_MODE_SPEC;
impl crate::RegisterSpec for US_TTGR_LON_MODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [us_ttgr_lon_mode::R](R) reader structure"]
impl crate::Readable for US_TTGR_LON_MODE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [us_ttgr_lon_mode::W](W) writer structure"]
impl crate::Writable for US_TTGR_LON_MODE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets US_TTGR_LON_MODE to value 0"]
impl crate::Resettable for US_TTGR_LON_MODE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
