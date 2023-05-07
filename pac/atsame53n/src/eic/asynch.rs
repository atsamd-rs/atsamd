#[doc = "Register `ASYNCH` reader"]
pub struct R(crate::R<ASYNCH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ASYNCH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ASYNCH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ASYNCH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ASYNCH` writer"]
pub struct W(crate::W<ASYNCH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ASYNCH_SPEC>;
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
impl From<crate::W<ASYNCH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ASYNCH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ASYNCH` reader - Asynchronous Edge Detection Mode"]
pub type ASYNCH_R = crate::FieldReader<u16, ASYNCHSELECT_A>;
#[doc = "Asynchronous Edge Detection Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum ASYNCHSELECT_A {
    #[doc = "0: Edge detection is clock synchronously operated"]
    SYNC = 0,
    #[doc = "1: Edge detection is clock asynchronously operated"]
    ASYNC = 1,
}
impl From<ASYNCHSELECT_A> for u16 {
    #[inline(always)]
    fn from(variant: ASYNCHSELECT_A) -> Self {
        variant as _
    }
}
impl ASYNCH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ASYNCHSELECT_A> {
        match self.bits {
            0 => Some(ASYNCHSELECT_A::SYNC),
            1 => Some(ASYNCHSELECT_A::ASYNC),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SYNC`"]
    #[inline(always)]
    pub fn is_sync(&self) -> bool {
        *self == ASYNCHSELECT_A::SYNC
    }
    #[doc = "Checks if the value of the field is `ASYNC`"]
    #[inline(always)]
    pub fn is_async(&self) -> bool {
        *self == ASYNCHSELECT_A::ASYNC
    }
}
#[doc = "Field `ASYNCH` writer - Asynchronous Edge Detection Mode"]
pub type ASYNCH_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ASYNCH_SPEC, u16, ASYNCHSELECT_A, 16, O>;
impl<'a, const O: u8> ASYNCH_W<'a, O> {
    #[doc = "Edge detection is clock synchronously operated"]
    #[inline(always)]
    pub fn sync(self) -> &'a mut W {
        self.variant(ASYNCHSELECT_A::SYNC)
    }
    #[doc = "Edge detection is clock asynchronously operated"]
    #[inline(always)]
    pub fn async_(self) -> &'a mut W {
        self.variant(ASYNCHSELECT_A::ASYNC)
    }
}
impl R {
    #[doc = "Bits 0:15 - Asynchronous Edge Detection Mode"]
    #[inline(always)]
    pub fn asynch(&self) -> ASYNCH_R {
        ASYNCH_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Asynchronous Edge Detection Mode"]
    #[inline(always)]
    #[must_use]
    pub fn asynch(&mut self) -> ASYNCH_W<0> {
        ASYNCH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "External Interrupt Asynchronous Mode\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [asynch](index.html) module"]
pub struct ASYNCH_SPEC;
impl crate::RegisterSpec for ASYNCH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [asynch::R](R) reader structure"]
impl crate::Readable for ASYNCH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [asynch::W](W) writer structure"]
impl crate::Writable for ASYNCH_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ASYNCH to value 0"]
impl crate::Resettable for ASYNCH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
