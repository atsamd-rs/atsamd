#[doc = "Register `CCBUF%s_DITH4` reader"]
pub struct R(crate::R<CCBUF_DITH4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCBUF_DITH4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CCBUF_DITH4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CCBUF_DITH4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CCBUF%s_DITH4` writer"]
pub struct W(crate::W<CCBUF_DITH4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CCBUF_DITH4_SPEC>;
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
impl From<crate::W<CCBUF_DITH4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CCBUF_DITH4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CCBUF` reader - Channel Compare/Capture Buffer Value"]
pub type CCBUF_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CCBUF` writer - Channel Compare/Capture Buffer Value"]
pub type CCBUF_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCBUF_DITH4_SPEC, u8, u8, 4, O>;
#[doc = "Field `DITHERBUF` reader - Dithering Buffer Cycle Number"]
pub type DITHERBUF_R = crate::FieldReader<u32, u32>;
#[doc = "Field `DITHERBUF` writer - Dithering Buffer Cycle Number"]
pub type DITHERBUF_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CCBUF_DITH4_SPEC, u32, u32, 20, O>;
impl R {
    #[doc = "Bits 0:3 - Channel Compare/Capture Buffer Value"]
    #[inline(always)]
    pub fn ccbuf(&self) -> CCBUF_R {
        CCBUF_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:23 - Dithering Buffer Cycle Number"]
    #[inline(always)]
    pub fn ditherbuf(&self) -> DITHERBUF_R {
        DITHERBUF_R::new((self.bits >> 4) & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:3 - Channel Compare/Capture Buffer Value"]
    #[inline(always)]
    #[must_use]
    pub fn ccbuf(&mut self) -> CCBUF_W<0> {
        CCBUF_W::new(self)
    }
    #[doc = "Bits 4:23 - Dithering Buffer Cycle Number"]
    #[inline(always)]
    #[must_use]
    pub fn ditherbuf(&mut self) -> DITHERBUF_W<4> {
        DITHERBUF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Compare and Capture Buffer\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccbuf_dith4](index.html) module"]
pub struct CCBUF_DITH4_SPEC;
impl crate::RegisterSpec for CCBUF_DITH4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ccbuf_dith4::R](R) reader structure"]
impl crate::Readable for CCBUF_DITH4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ccbuf_dith4::W](W) writer structure"]
impl crate::Writable for CCBUF_DITH4_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CCBUF%s_DITH4 to value 0"]
impl crate::Resettable for CCBUF_DITH4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
