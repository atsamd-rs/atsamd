#[doc = "Register `CTRLD` reader"]
pub struct R(crate::R<CTRLD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRLD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRLD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRLD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRLD` writer"]
pub struct W(crate::W<CTRLD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRLD_SPEC>;
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
impl From<crate::W<CTRLD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRLD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BLANK` reader - Blank LCD"]
pub type BLANK_R = crate::BitReader<bool>;
#[doc = "Field `BLANK` writer - Blank LCD"]
pub type BLANK_W<'a, const O: u8> = crate::BitWriter<'a, u8, CTRLD_SPEC, bool, O>;
#[doc = "Field `BLINK` reader - Blinking Enable"]
pub type BLINK_R = crate::BitReader<bool>;
#[doc = "Field `BLINK` writer - Blinking Enable"]
pub type BLINK_W<'a, const O: u8> = crate::BitWriter<'a, u8, CTRLD_SPEC, bool, O>;
#[doc = "Field `CSREN` reader - Circular Shift Register Enable"]
pub type CSREN_R = crate::BitReader<bool>;
#[doc = "Field `CSREN` writer - Circular Shift Register Enable"]
pub type CSREN_W<'a, const O: u8> = crate::BitWriter<'a, u8, CTRLD_SPEC, bool, O>;
#[doc = "Field `FC0EN` reader - Frame Counter 0 Enable"]
pub type FC0EN_R = crate::BitReader<bool>;
#[doc = "Field `FC0EN` writer - Frame Counter 0 Enable"]
pub type FC0EN_W<'a, const O: u8> = crate::BitWriter<'a, u8, CTRLD_SPEC, bool, O>;
#[doc = "Field `FC1EN` reader - Frame Counter 1 Enable"]
pub type FC1EN_R = crate::BitReader<bool>;
#[doc = "Field `FC1EN` writer - Frame Counter 1 Enable"]
pub type FC1EN_W<'a, const O: u8> = crate::BitWriter<'a, u8, CTRLD_SPEC, bool, O>;
#[doc = "Field `FC2EN` reader - Frame Counter 2 Enable"]
pub type FC2EN_R = crate::BitReader<bool>;
#[doc = "Field `FC2EN` writer - Frame Counter 2 Enable"]
pub type FC2EN_W<'a, const O: u8> = crate::BitWriter<'a, u8, CTRLD_SPEC, bool, O>;
#[doc = "Field `DISPEN` reader - Display enable"]
pub type DISPEN_R = crate::BitReader<bool>;
#[doc = "Field `DISPEN` writer - Display enable"]
pub type DISPEN_W<'a, const O: u8> = crate::BitWriter<'a, u8, CTRLD_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Blank LCD"]
    #[inline(always)]
    pub fn blank(&self) -> BLANK_R {
        BLANK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Blinking Enable"]
    #[inline(always)]
    pub fn blink(&self) -> BLINK_R {
        BLINK_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Circular Shift Register Enable"]
    #[inline(always)]
    pub fn csren(&self) -> CSREN_R {
        CSREN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Frame Counter 0 Enable"]
    #[inline(always)]
    pub fn fc0en(&self) -> FC0EN_R {
        FC0EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Frame Counter 1 Enable"]
    #[inline(always)]
    pub fn fc1en(&self) -> FC1EN_R {
        FC1EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Frame Counter 2 Enable"]
    #[inline(always)]
    pub fn fc2en(&self) -> FC2EN_R {
        FC2EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Display enable"]
    #[inline(always)]
    pub fn dispen(&self) -> DISPEN_R {
        DISPEN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Blank LCD"]
    #[inline(always)]
    #[must_use]
    pub fn blank(&mut self) -> BLANK_W<0> {
        BLANK_W::new(self)
    }
    #[doc = "Bit 1 - Blinking Enable"]
    #[inline(always)]
    #[must_use]
    pub fn blink(&mut self) -> BLINK_W<1> {
        BLINK_W::new(self)
    }
    #[doc = "Bit 2 - Circular Shift Register Enable"]
    #[inline(always)]
    #[must_use]
    pub fn csren(&mut self) -> CSREN_W<2> {
        CSREN_W::new(self)
    }
    #[doc = "Bit 4 - Frame Counter 0 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn fc0en(&mut self) -> FC0EN_W<4> {
        FC0EN_W::new(self)
    }
    #[doc = "Bit 5 - Frame Counter 1 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn fc1en(&mut self) -> FC1EN_W<5> {
        FC1EN_W::new(self)
    }
    #[doc = "Bit 6 - Frame Counter 2 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn fc2en(&mut self) -> FC2EN_W<6> {
        FC2EN_W::new(self)
    }
    #[doc = "Bit 7 - Display enable"]
    #[inline(always)]
    #[must_use]
    pub fn dispen(&mut self) -> DISPEN_W<7> {
        DISPEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control D\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrld](index.html) module"]
pub struct CTRLD_SPEC;
impl crate::RegisterSpec for CTRLD_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [ctrld::R](R) reader structure"]
impl crate::Readable for CTRLD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrld::W](W) writer structure"]
impl crate::Writable for CTRLD_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRLD to value 0x80"]
impl crate::Resettable for CTRLD_SPEC {
    const RESET_VALUE: Self::Ux = 0x80;
}
