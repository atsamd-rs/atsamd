#[doc = "Register `INTENCLR` reader"]
pub struct R(crate::R<INTENCLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTENCLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTENCLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTENCLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTENCLR` writer"]
pub struct W(crate::W<INTENCLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTENCLR_SPEC>;
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
impl From<crate::W<INTENCLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTENCLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UNDERRUN0` reader - Underrun 0 Interrupt Enable"]
pub type UNDERRUN0_R = crate::BitReader<bool>;
#[doc = "Field `UNDERRUN0` writer - Underrun 0 Interrupt Enable"]
pub type UNDERRUN0_W<'a, const O: u8> = crate::BitWriter<'a, u8, INTENCLR_SPEC, bool, O>;
#[doc = "Field `UNDERRUN1` reader - Underrun 1 Interrupt Enable"]
pub type UNDERRUN1_R = crate::BitReader<bool>;
#[doc = "Field `UNDERRUN1` writer - Underrun 1 Interrupt Enable"]
pub type UNDERRUN1_W<'a, const O: u8> = crate::BitWriter<'a, u8, INTENCLR_SPEC, bool, O>;
#[doc = "Field `EMPTY0` reader - Data Buffer 0 Empty Interrupt Enable"]
pub type EMPTY0_R = crate::BitReader<bool>;
#[doc = "Field `EMPTY0` writer - Data Buffer 0 Empty Interrupt Enable"]
pub type EMPTY0_W<'a, const O: u8> = crate::BitWriter<'a, u8, INTENCLR_SPEC, bool, O>;
#[doc = "Field `EMPTY1` reader - Data Buffer 1 Empty Interrupt Enable"]
pub type EMPTY1_R = crate::BitReader<bool>;
#[doc = "Field `EMPTY1` writer - Data Buffer 1 Empty Interrupt Enable"]
pub type EMPTY1_W<'a, const O: u8> = crate::BitWriter<'a, u8, INTENCLR_SPEC, bool, O>;
#[doc = "Field `RESRDY0` reader - Result 0 Ready Interrupt Enable"]
pub type RESRDY0_R = crate::BitReader<bool>;
#[doc = "Field `RESRDY0` writer - Result 0 Ready Interrupt Enable"]
pub type RESRDY0_W<'a, const O: u8> = crate::BitWriter<'a, u8, INTENCLR_SPEC, bool, O>;
#[doc = "Field `RESRDY1` reader - Result 1 Ready Interrupt Enable"]
pub type RESRDY1_R = crate::BitReader<bool>;
#[doc = "Field `RESRDY1` writer - Result 1 Ready Interrupt Enable"]
pub type RESRDY1_W<'a, const O: u8> = crate::BitWriter<'a, u8, INTENCLR_SPEC, bool, O>;
#[doc = "Field `OVERRUN0` reader - Overrun 0 Interrupt Enable"]
pub type OVERRUN0_R = crate::BitReader<bool>;
#[doc = "Field `OVERRUN0` writer - Overrun 0 Interrupt Enable"]
pub type OVERRUN0_W<'a, const O: u8> = crate::BitWriter<'a, u8, INTENCLR_SPEC, bool, O>;
#[doc = "Field `OVERRUN1` reader - Overrun 1 Interrupt Enable"]
pub type OVERRUN1_R = crate::BitReader<bool>;
#[doc = "Field `OVERRUN1` writer - Overrun 1 Interrupt Enable"]
pub type OVERRUN1_W<'a, const O: u8> = crate::BitWriter<'a, u8, INTENCLR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Underrun 0 Interrupt Enable"]
    #[inline(always)]
    pub fn underrun0(&self) -> UNDERRUN0_R {
        UNDERRUN0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Underrun 1 Interrupt Enable"]
    #[inline(always)]
    pub fn underrun1(&self) -> UNDERRUN1_R {
        UNDERRUN1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Data Buffer 0 Empty Interrupt Enable"]
    #[inline(always)]
    pub fn empty0(&self) -> EMPTY0_R {
        EMPTY0_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Data Buffer 1 Empty Interrupt Enable"]
    #[inline(always)]
    pub fn empty1(&self) -> EMPTY1_R {
        EMPTY1_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Result 0 Ready Interrupt Enable"]
    #[inline(always)]
    pub fn resrdy0(&self) -> RESRDY0_R {
        RESRDY0_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Result 1 Ready Interrupt Enable"]
    #[inline(always)]
    pub fn resrdy1(&self) -> RESRDY1_R {
        RESRDY1_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Overrun 0 Interrupt Enable"]
    #[inline(always)]
    pub fn overrun0(&self) -> OVERRUN0_R {
        OVERRUN0_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Overrun 1 Interrupt Enable"]
    #[inline(always)]
    pub fn overrun1(&self) -> OVERRUN1_R {
        OVERRUN1_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Underrun 0 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn underrun0(&mut self) -> UNDERRUN0_W<0> {
        UNDERRUN0_W::new(self)
    }
    #[doc = "Bit 1 - Underrun 1 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn underrun1(&mut self) -> UNDERRUN1_W<1> {
        UNDERRUN1_W::new(self)
    }
    #[doc = "Bit 2 - Data Buffer 0 Empty Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn empty0(&mut self) -> EMPTY0_W<2> {
        EMPTY0_W::new(self)
    }
    #[doc = "Bit 3 - Data Buffer 1 Empty Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn empty1(&mut self) -> EMPTY1_W<3> {
        EMPTY1_W::new(self)
    }
    #[doc = "Bit 4 - Result 0 Ready Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn resrdy0(&mut self) -> RESRDY0_W<4> {
        RESRDY0_W::new(self)
    }
    #[doc = "Bit 5 - Result 1 Ready Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn resrdy1(&mut self) -> RESRDY1_W<5> {
        RESRDY1_W::new(self)
    }
    #[doc = "Bit 6 - Overrun 0 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn overrun0(&mut self) -> OVERRUN0_W<6> {
        OVERRUN0_W::new(self)
    }
    #[doc = "Bit 7 - Overrun 1 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn overrun1(&mut self) -> OVERRUN1_W<7> {
        OVERRUN1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Enable Clear\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intenclr](index.html) module"]
pub struct INTENCLR_SPEC;
impl crate::RegisterSpec for INTENCLR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [intenclr::R](R) reader structure"]
impl crate::Readable for INTENCLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intenclr::W](W) writer structure"]
impl crate::Writable for INTENCLR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTENCLR to value 0"]
impl crate::Resettable for INTENCLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
