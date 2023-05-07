#[doc = "Register `CHCTRLB` reader"]
pub struct R(crate::R<CHCTRLB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CHCTRLB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CHCTRLB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CHCTRLB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CHCTRLB` writer"]
pub struct W(crate::W<CHCTRLB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CHCTRLB_SPEC>;
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
impl From<crate::W<CHCTRLB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CHCTRLB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CMD` reader - Software Command"]
pub type CMD_R = crate::FieldReader<u8, CMDSELECT_A>;
#[doc = "Software Command\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CMDSELECT_A {
    #[doc = "0: No action"]
    NOACT = 0,
    #[doc = "1: Channel suspend operation"]
    SUSPEND = 1,
    #[doc = "2: Channel resume operation"]
    RESUME = 2,
}
impl From<CMDSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: CMDSELECT_A) -> Self {
        variant as _
    }
}
impl CMD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CMDSELECT_A> {
        match self.bits {
            0 => Some(CMDSELECT_A::NOACT),
            1 => Some(CMDSELECT_A::SUSPEND),
            2 => Some(CMDSELECT_A::RESUME),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NOACT`"]
    #[inline(always)]
    pub fn is_noact(&self) -> bool {
        *self == CMDSELECT_A::NOACT
    }
    #[doc = "Checks if the value of the field is `SUSPEND`"]
    #[inline(always)]
    pub fn is_suspend(&self) -> bool {
        *self == CMDSELECT_A::SUSPEND
    }
    #[doc = "Checks if the value of the field is `RESUME`"]
    #[inline(always)]
    pub fn is_resume(&self) -> bool {
        *self == CMDSELECT_A::RESUME
    }
}
#[doc = "Field `CMD` writer - Software Command"]
pub type CMD_W<'a, const O: u8> = crate::FieldWriter<'a, u8, CHCTRLB_SPEC, u8, CMDSELECT_A, 2, O>;
impl<'a, const O: u8> CMD_W<'a, O> {
    #[doc = "No action"]
    #[inline(always)]
    pub fn noact(self) -> &'a mut W {
        self.variant(CMDSELECT_A::NOACT)
    }
    #[doc = "Channel suspend operation"]
    #[inline(always)]
    pub fn suspend(self) -> &'a mut W {
        self.variant(CMDSELECT_A::SUSPEND)
    }
    #[doc = "Channel resume operation"]
    #[inline(always)]
    pub fn resume(self) -> &'a mut W {
        self.variant(CMDSELECT_A::RESUME)
    }
}
impl R {
    #[doc = "Bits 0:1 - Software Command"]
    #[inline(always)]
    pub fn cmd(&self) -> CMD_R {
        CMD_R::new(self.bits & 3)
    }
}
impl W {
    #[doc = "Bits 0:1 - Software Command"]
    #[inline(always)]
    #[must_use]
    pub fn cmd(&mut self) -> CMD_W<0> {
        CMD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel n Control B\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chctrlb](index.html) module"]
pub struct CHCTRLB_SPEC;
impl crate::RegisterSpec for CHCTRLB_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [chctrlb::R](R) reader structure"]
impl crate::Readable for CHCTRLB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [chctrlb::W](W) writer structure"]
impl crate::Writable for CHCTRLB_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CHCTRLB to value 0"]
impl crate::Resettable for CHCTRLB_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
