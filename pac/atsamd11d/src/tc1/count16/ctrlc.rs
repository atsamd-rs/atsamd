#[doc = "Register `CTRLC` reader"]
pub struct R(crate::R<CTRLC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRLC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRLC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRLC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRLC` writer"]
pub struct W(crate::W<CTRLC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRLC_SPEC>;
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
impl From<crate::W<CTRLC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRLC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INVEN0` reader - Output Waveform 0 Invert Enable"]
pub type INVEN0_R = crate::BitReader<bool>;
#[doc = "Field `INVEN0` writer - Output Waveform 0 Invert Enable"]
pub type INVEN0_W<'a, const O: u8> = crate::BitWriter<'a, u8, CTRLC_SPEC, bool, O>;
#[doc = "Field `INVEN1` reader - Output Waveform 1 Invert Enable"]
pub type INVEN1_R = crate::BitReader<bool>;
#[doc = "Field `INVEN1` writer - Output Waveform 1 Invert Enable"]
pub type INVEN1_W<'a, const O: u8> = crate::BitWriter<'a, u8, CTRLC_SPEC, bool, O>;
#[doc = "Field `CPTEN0` reader - Capture Channel 0 Enable"]
pub type CPTEN0_R = crate::BitReader<bool>;
#[doc = "Field `CPTEN0` writer - Capture Channel 0 Enable"]
pub type CPTEN0_W<'a, const O: u8> = crate::BitWriter<'a, u8, CTRLC_SPEC, bool, O>;
#[doc = "Field `CPTEN1` reader - Capture Channel 1 Enable"]
pub type CPTEN1_R = crate::BitReader<bool>;
#[doc = "Field `CPTEN1` writer - Capture Channel 1 Enable"]
pub type CPTEN1_W<'a, const O: u8> = crate::BitWriter<'a, u8, CTRLC_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Output Waveform 0 Invert Enable"]
    #[inline(always)]
    pub fn inven0(&self) -> INVEN0_R {
        INVEN0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Output Waveform 1 Invert Enable"]
    #[inline(always)]
    pub fn inven1(&self) -> INVEN1_R {
        INVEN1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - Capture Channel 0 Enable"]
    #[inline(always)]
    pub fn cpten0(&self) -> CPTEN0_R {
        CPTEN0_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Capture Channel 1 Enable"]
    #[inline(always)]
    pub fn cpten1(&self) -> CPTEN1_R {
        CPTEN1_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Output Waveform 0 Invert Enable"]
    #[inline(always)]
    #[must_use]
    pub fn inven0(&mut self) -> INVEN0_W<0> {
        INVEN0_W::new(self)
    }
    #[doc = "Bit 1 - Output Waveform 1 Invert Enable"]
    #[inline(always)]
    #[must_use]
    pub fn inven1(&mut self) -> INVEN1_W<1> {
        INVEN1_W::new(self)
    }
    #[doc = "Bit 4 - Capture Channel 0 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cpten0(&mut self) -> CPTEN0_W<4> {
        CPTEN0_W::new(self)
    }
    #[doc = "Bit 5 - Capture Channel 1 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cpten1(&mut self) -> CPTEN1_W<5> {
        CPTEN1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control C\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrlc](index.html) module"]
pub struct CTRLC_SPEC;
impl crate::RegisterSpec for CTRLC_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [ctrlc::R](R) reader structure"]
impl crate::Readable for CTRLC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrlc::W](W) writer structure"]
impl crate::Writable for CTRLC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRLC to value 0"]
impl crate::Resettable for CTRLC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
