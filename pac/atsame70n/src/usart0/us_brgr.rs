#[doc = "Register `US_BRGR` reader"]
pub struct R(crate::R<US_BRGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<US_BRGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<US_BRGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<US_BRGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `US_BRGR` writer"]
pub struct W(crate::W<US_BRGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<US_BRGR_SPEC>;
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
impl From<crate::W<US_BRGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<US_BRGR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CD` reader - Clock Divider"]
pub struct CD_R(crate::FieldReader<u16, u16>);
impl CD_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        CD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CD_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CD` writer - Clock Divider"]
pub struct CD_W<'a> {
    w: &'a mut W,
}
impl<'a> CD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
#[doc = "Field `FP` reader - Fractional Part"]
pub struct FP_R(crate::FieldReader<u8, u8>);
impl FP_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        FP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FP` writer - Fractional Part"]
pub struct FP_W<'a> {
    w: &'a mut W,
}
impl<'a> FP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | ((value as u32 & 0x07) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Clock Divider"]
    #[inline(always)]
    pub fn cd(&self) -> CD_R {
        CD_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:18 - Fractional Part"]
    #[inline(always)]
    pub fn fp(&self) -> FP_R {
        FP_R::new(((self.bits >> 16) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - Clock Divider"]
    #[inline(always)]
    pub fn cd(&mut self) -> CD_W {
        CD_W { w: self }
    }
    #[doc = "Bits 16:18 - Fractional Part"]
    #[inline(always)]
    pub fn fp(&mut self) -> FP_W {
        FP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Baud Rate Generator Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [us_brgr](index.html) module"]
pub struct US_BRGR_SPEC;
impl crate::RegisterSpec for US_BRGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [us_brgr::R](R) reader structure"]
impl crate::Readable for US_BRGR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [us_brgr::W](W) writer structure"]
impl crate::Writable for US_BRGR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets US_BRGR to value 0"]
impl crate::Resettable for US_BRGR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
