#[doc = "Register `WEXCTRL` reader"]
pub struct R(crate::R<WEXCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WEXCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WEXCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WEXCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WEXCTRL` writer"]
pub struct W(crate::W<WEXCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WEXCTRL_SPEC>;
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
impl From<crate::W<WEXCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WEXCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OTMX` reader - Output Matrix"]
pub type OTMX_R = crate::FieldReader<u8, u8>;
#[doc = "Field `OTMX` writer - Output Matrix"]
pub type OTMX_W<'a, const O: u8> = crate::FieldWriter<'a, u32, WEXCTRL_SPEC, u8, u8, 2, O>;
#[doc = "Field `DTIEN0` reader - Dead-time Insertion Generator 0 Enable"]
pub type DTIEN0_R = crate::BitReader<bool>;
#[doc = "Field `DTIEN0` writer - Dead-time Insertion Generator 0 Enable"]
pub type DTIEN0_W<'a, const O: u8> = crate::BitWriter<'a, u32, WEXCTRL_SPEC, bool, O>;
#[doc = "Field `DTIEN1` reader - Dead-time Insertion Generator 1 Enable"]
pub type DTIEN1_R = crate::BitReader<bool>;
#[doc = "Field `DTIEN1` writer - Dead-time Insertion Generator 1 Enable"]
pub type DTIEN1_W<'a, const O: u8> = crate::BitWriter<'a, u32, WEXCTRL_SPEC, bool, O>;
#[doc = "Field `DTIEN2` reader - Dead-time Insertion Generator 2 Enable"]
pub type DTIEN2_R = crate::BitReader<bool>;
#[doc = "Field `DTIEN2` writer - Dead-time Insertion Generator 2 Enable"]
pub type DTIEN2_W<'a, const O: u8> = crate::BitWriter<'a, u32, WEXCTRL_SPEC, bool, O>;
#[doc = "Field `DTIEN3` reader - Dead-time Insertion Generator 3 Enable"]
pub type DTIEN3_R = crate::BitReader<bool>;
#[doc = "Field `DTIEN3` writer - Dead-time Insertion Generator 3 Enable"]
pub type DTIEN3_W<'a, const O: u8> = crate::BitWriter<'a, u32, WEXCTRL_SPEC, bool, O>;
#[doc = "Field `DTLS` reader - Dead-time Low Side Outputs Value"]
pub type DTLS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DTLS` writer - Dead-time Low Side Outputs Value"]
pub type DTLS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, WEXCTRL_SPEC, u8, u8, 8, O>;
#[doc = "Field `DTHS` reader - Dead-time High Side Outputs Value"]
pub type DTHS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DTHS` writer - Dead-time High Side Outputs Value"]
pub type DTHS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, WEXCTRL_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:1 - Output Matrix"]
    #[inline(always)]
    pub fn otmx(&self) -> OTMX_R {
        OTMX_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 8 - Dead-time Insertion Generator 0 Enable"]
    #[inline(always)]
    pub fn dtien0(&self) -> DTIEN0_R {
        DTIEN0_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Dead-time Insertion Generator 1 Enable"]
    #[inline(always)]
    pub fn dtien1(&self) -> DTIEN1_R {
        DTIEN1_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Dead-time Insertion Generator 2 Enable"]
    #[inline(always)]
    pub fn dtien2(&self) -> DTIEN2_R {
        DTIEN2_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Dead-time Insertion Generator 3 Enable"]
    #[inline(always)]
    pub fn dtien3(&self) -> DTIEN3_R {
        DTIEN3_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 16:23 - Dead-time Low Side Outputs Value"]
    #[inline(always)]
    pub fn dtls(&self) -> DTLS_R {
        DTLS_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Dead-time High Side Outputs Value"]
    #[inline(always)]
    pub fn dths(&self) -> DTHS_R {
        DTHS_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Output Matrix"]
    #[inline(always)]
    #[must_use]
    pub fn otmx(&mut self) -> OTMX_W<0> {
        OTMX_W::new(self)
    }
    #[doc = "Bit 8 - Dead-time Insertion Generator 0 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dtien0(&mut self) -> DTIEN0_W<8> {
        DTIEN0_W::new(self)
    }
    #[doc = "Bit 9 - Dead-time Insertion Generator 1 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dtien1(&mut self) -> DTIEN1_W<9> {
        DTIEN1_W::new(self)
    }
    #[doc = "Bit 10 - Dead-time Insertion Generator 2 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dtien2(&mut self) -> DTIEN2_W<10> {
        DTIEN2_W::new(self)
    }
    #[doc = "Bit 11 - Dead-time Insertion Generator 3 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dtien3(&mut self) -> DTIEN3_W<11> {
        DTIEN3_W::new(self)
    }
    #[doc = "Bits 16:23 - Dead-time Low Side Outputs Value"]
    #[inline(always)]
    #[must_use]
    pub fn dtls(&mut self) -> DTLS_W<16> {
        DTLS_W::new(self)
    }
    #[doc = "Bits 24:31 - Dead-time High Side Outputs Value"]
    #[inline(always)]
    #[must_use]
    pub fn dths(&mut self) -> DTHS_W<24> {
        DTHS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Waveform Extension Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wexctrl](index.html) module"]
pub struct WEXCTRL_SPEC;
impl crate::RegisterSpec for WEXCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wexctrl::R](R) reader structure"]
impl crate::Readable for WEXCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wexctrl::W](W) writer structure"]
impl crate::Writable for WEXCTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WEXCTRL to value 0"]
impl crate::Resettable for WEXCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
