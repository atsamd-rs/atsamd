#[doc = "Register `EVCTRL` reader"]
pub struct R(crate::R<EVCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EVCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EVCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EVCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EVCTRL` writer"]
pub struct W(crate::W<EVCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EVCTRL_SPEC>;
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
impl From<crate::W<EVCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EVCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COMPEO0` reader - Comparator 0 Event Output Enable"]
pub type COMPEO0_R = crate::BitReader<bool>;
#[doc = "Field `COMPEO0` writer - Comparator 0 Event Output Enable"]
pub type COMPEO0_W<'a, const O: u8> = crate::BitWriter<'a, u16, EVCTRL_SPEC, bool, O>;
#[doc = "Field `COMPEO1` reader - Comparator 1 Event Output Enable"]
pub type COMPEO1_R = crate::BitReader<bool>;
#[doc = "Field `COMPEO1` writer - Comparator 1 Event Output Enable"]
pub type COMPEO1_W<'a, const O: u8> = crate::BitWriter<'a, u16, EVCTRL_SPEC, bool, O>;
#[doc = "Field `WINEO0` reader - Window 0 Event Output Enable"]
pub type WINEO0_R = crate::BitReader<bool>;
#[doc = "Field `WINEO0` writer - Window 0 Event Output Enable"]
pub type WINEO0_W<'a, const O: u8> = crate::BitWriter<'a, u16, EVCTRL_SPEC, bool, O>;
#[doc = "Field `COMPEI0` reader - Comparator 0 Event Input Enable"]
pub type COMPEI0_R = crate::BitReader<bool>;
#[doc = "Field `COMPEI0` writer - Comparator 0 Event Input Enable"]
pub type COMPEI0_W<'a, const O: u8> = crate::BitWriter<'a, u16, EVCTRL_SPEC, bool, O>;
#[doc = "Field `COMPEI1` reader - Comparator 1 Event Input Enable"]
pub type COMPEI1_R = crate::BitReader<bool>;
#[doc = "Field `COMPEI1` writer - Comparator 1 Event Input Enable"]
pub type COMPEI1_W<'a, const O: u8> = crate::BitWriter<'a, u16, EVCTRL_SPEC, bool, O>;
#[doc = "Field `INVEI0` reader - Comparator 0 Input Event Invert Enable"]
pub type INVEI0_R = crate::BitReader<bool>;
#[doc = "Field `INVEI0` writer - Comparator 0 Input Event Invert Enable"]
pub type INVEI0_W<'a, const O: u8> = crate::BitWriter<'a, u16, EVCTRL_SPEC, bool, O>;
#[doc = "Field `INVEI1` reader - Comparator 1 Input Event Invert Enable"]
pub type INVEI1_R = crate::BitReader<bool>;
#[doc = "Field `INVEI1` writer - Comparator 1 Input Event Invert Enable"]
pub type INVEI1_W<'a, const O: u8> = crate::BitWriter<'a, u16, EVCTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Comparator 0 Event Output Enable"]
    #[inline(always)]
    pub fn compeo0(&self) -> COMPEO0_R {
        COMPEO0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Comparator 1 Event Output Enable"]
    #[inline(always)]
    pub fn compeo1(&self) -> COMPEO1_R {
        COMPEO1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - Window 0 Event Output Enable"]
    #[inline(always)]
    pub fn wineo0(&self) -> WINEO0_R {
        WINEO0_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - Comparator 0 Event Input Enable"]
    #[inline(always)]
    pub fn compei0(&self) -> COMPEI0_R {
        COMPEI0_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Comparator 1 Event Input Enable"]
    #[inline(always)]
    pub fn compei1(&self) -> COMPEI1_R {
        COMPEI1_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12 - Comparator 0 Input Event Invert Enable"]
    #[inline(always)]
    pub fn invei0(&self) -> INVEI0_R {
        INVEI0_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Comparator 1 Input Event Invert Enable"]
    #[inline(always)]
    pub fn invei1(&self) -> INVEI1_R {
        INVEI1_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Comparator 0 Event Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn compeo0(&mut self) -> COMPEO0_W<0> {
        COMPEO0_W::new(self)
    }
    #[doc = "Bit 1 - Comparator 1 Event Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn compeo1(&mut self) -> COMPEO1_W<1> {
        COMPEO1_W::new(self)
    }
    #[doc = "Bit 4 - Window 0 Event Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn wineo0(&mut self) -> WINEO0_W<4> {
        WINEO0_W::new(self)
    }
    #[doc = "Bit 8 - Comparator 0 Event Input Enable"]
    #[inline(always)]
    #[must_use]
    pub fn compei0(&mut self) -> COMPEI0_W<8> {
        COMPEI0_W::new(self)
    }
    #[doc = "Bit 9 - Comparator 1 Event Input Enable"]
    #[inline(always)]
    #[must_use]
    pub fn compei1(&mut self) -> COMPEI1_W<9> {
        COMPEI1_W::new(self)
    }
    #[doc = "Bit 12 - Comparator 0 Input Event Invert Enable"]
    #[inline(always)]
    #[must_use]
    pub fn invei0(&mut self) -> INVEI0_W<12> {
        INVEI0_W::new(self)
    }
    #[doc = "Bit 13 - Comparator 1 Input Event Invert Enable"]
    #[inline(always)]
    #[must_use]
    pub fn invei1(&mut self) -> INVEI1_W<13> {
        INVEI1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Event Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [evctrl](index.html) module"]
pub struct EVCTRL_SPEC;
impl crate::RegisterSpec for EVCTRL_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [evctrl::R](R) reader structure"]
impl crate::Readable for EVCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [evctrl::W](W) writer structure"]
impl crate::Writable for EVCTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EVCTRL to value 0"]
impl crate::Resettable for EVCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
