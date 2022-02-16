#[doc = "Register `MCAN_XIDAM` reader"]
pub struct R(crate::R<MCAN_XIDAM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MCAN_XIDAM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MCAN_XIDAM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MCAN_XIDAM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MCAN_XIDAM` writer"]
pub struct W(crate::W<MCAN_XIDAM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MCAN_XIDAM_SPEC>;
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
impl From<crate::W<MCAN_XIDAM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MCAN_XIDAM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EIDM` reader - Extended ID Mask"]
pub struct EIDM_R(crate::FieldReader<u32, u32>);
impl EIDM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        EIDM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EIDM_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EIDM` writer - Extended ID Mask"]
pub struct EIDM_W<'a> {
    w: &'a mut W,
}
impl<'a> EIDM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1fff_ffff) | (value as u32 & 0x1fff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:28 - Extended ID Mask"]
    #[inline(always)]
    pub fn eidm(&self) -> EIDM_R {
        EIDM_R::new((self.bits & 0x1fff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:28 - Extended ID Mask"]
    #[inline(always)]
    pub fn eidm(&mut self) -> EIDM_W {
        EIDM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Extended ID AND Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcan_xidam](index.html) module"]
pub struct MCAN_XIDAM_SPEC;
impl crate::RegisterSpec for MCAN_XIDAM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mcan_xidam::R](R) reader structure"]
impl crate::Readable for MCAN_XIDAM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mcan_xidam::W](W) writer structure"]
impl crate::Writable for MCAN_XIDAM_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MCAN_XIDAM to value 0"]
impl crate::Resettable for MCAN_XIDAM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
