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
#[doc = "Field `STARTEI` reader - Start Conversion Event In"]
pub type STARTEI_R = crate::BitReader<bool>;
#[doc = "Field `STARTEI` writer - Start Conversion Event In"]
pub type STARTEI_W<'a, const O: u8> = crate::BitWriter<'a, u8, EVCTRL_SPEC, bool, O>;
#[doc = "Field `SYNCEI` reader - Synchronization Event In"]
pub type SYNCEI_R = crate::BitReader<bool>;
#[doc = "Field `SYNCEI` writer - Synchronization Event In"]
pub type SYNCEI_W<'a, const O: u8> = crate::BitWriter<'a, u8, EVCTRL_SPEC, bool, O>;
#[doc = "Field `RESRDYEO` reader - Result Ready Event Out"]
pub type RESRDYEO_R = crate::BitReader<bool>;
#[doc = "Field `RESRDYEO` writer - Result Ready Event Out"]
pub type RESRDYEO_W<'a, const O: u8> = crate::BitWriter<'a, u8, EVCTRL_SPEC, bool, O>;
#[doc = "Field `WINMONEO` reader - Window Monitor Event Out"]
pub type WINMONEO_R = crate::BitReader<bool>;
#[doc = "Field `WINMONEO` writer - Window Monitor Event Out"]
pub type WINMONEO_W<'a, const O: u8> = crate::BitWriter<'a, u8, EVCTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Start Conversion Event In"]
    #[inline(always)]
    pub fn startei(&self) -> STARTEI_R {
        STARTEI_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Synchronization Event In"]
    #[inline(always)]
    pub fn syncei(&self) -> SYNCEI_R {
        SYNCEI_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - Result Ready Event Out"]
    #[inline(always)]
    pub fn resrdyeo(&self) -> RESRDYEO_R {
        RESRDYEO_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Window Monitor Event Out"]
    #[inline(always)]
    pub fn winmoneo(&self) -> WINMONEO_R {
        WINMONEO_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Start Conversion Event In"]
    #[inline(always)]
    #[must_use]
    pub fn startei(&mut self) -> STARTEI_W<0> {
        STARTEI_W::new(self)
    }
    #[doc = "Bit 1 - Synchronization Event In"]
    #[inline(always)]
    #[must_use]
    pub fn syncei(&mut self) -> SYNCEI_W<1> {
        SYNCEI_W::new(self)
    }
    #[doc = "Bit 4 - Result Ready Event Out"]
    #[inline(always)]
    #[must_use]
    pub fn resrdyeo(&mut self) -> RESRDYEO_W<4> {
        RESRDYEO_W::new(self)
    }
    #[doc = "Bit 5 - Window Monitor Event Out"]
    #[inline(always)]
    #[must_use]
    pub fn winmoneo(&mut self) -> WINMONEO_W<5> {
        WINMONEO_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Event Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [evctrl](index.html) module"]
pub struct EVCTRL_SPEC;
impl crate::RegisterSpec for EVCTRL_SPEC {
    type Ux = u8;
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
