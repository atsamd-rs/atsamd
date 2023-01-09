#[doc = "Register `NMICTRL` reader"]
pub struct R(crate::R<NMICTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NMICTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NMICTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NMICTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `NMICTRL` writer"]
pub struct W(crate::W<NMICTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<NMICTRL_SPEC>;
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
impl From<crate::W<NMICTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<NMICTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `NMISENSE` reader - NMI Input Sense Configuration"]
pub type NMISENSE_R = crate::FieldReader<u8, NMISENSESELECT_A>;
#[doc = "NMI Input Sense Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum NMISENSESELECT_A {
    #[doc = "0: No detection"]
    NONE = 0,
    #[doc = "1: Rising edge detection"]
    RISE = 1,
    #[doc = "2: Falling edge detection"]
    FALL = 2,
    #[doc = "3: Both edges detection"]
    BOTH = 3,
    #[doc = "4: High level detection"]
    HIGH = 4,
    #[doc = "5: Low level detection"]
    LOW = 5,
}
impl From<NMISENSESELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: NMISENSESELECT_A) -> Self {
        variant as _
    }
}
impl NMISENSE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<NMISENSESELECT_A> {
        match self.bits {
            0 => Some(NMISENSESELECT_A::NONE),
            1 => Some(NMISENSESELECT_A::RISE),
            2 => Some(NMISENSESELECT_A::FALL),
            3 => Some(NMISENSESELECT_A::BOTH),
            4 => Some(NMISENSESELECT_A::HIGH),
            5 => Some(NMISENSESELECT_A::LOW),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == NMISENSESELECT_A::NONE
    }
    #[doc = "Checks if the value of the field is `RISE`"]
    #[inline(always)]
    pub fn is_rise(&self) -> bool {
        *self == NMISENSESELECT_A::RISE
    }
    #[doc = "Checks if the value of the field is `FALL`"]
    #[inline(always)]
    pub fn is_fall(&self) -> bool {
        *self == NMISENSESELECT_A::FALL
    }
    #[doc = "Checks if the value of the field is `BOTH`"]
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        *self == NMISENSESELECT_A::BOTH
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == NMISENSESELECT_A::HIGH
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == NMISENSESELECT_A::LOW
    }
}
#[doc = "Field `NMISENSE` writer - NMI Input Sense Configuration"]
pub type NMISENSE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u8, NMICTRL_SPEC, u8, NMISENSESELECT_A, 3, O>;
impl<'a, const O: u8> NMISENSE_W<'a, O> {
    #[doc = "No detection"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(NMISENSESELECT_A::NONE)
    }
    #[doc = "Rising edge detection"]
    #[inline(always)]
    pub fn rise(self) -> &'a mut W {
        self.variant(NMISENSESELECT_A::RISE)
    }
    #[doc = "Falling edge detection"]
    #[inline(always)]
    pub fn fall(self) -> &'a mut W {
        self.variant(NMISENSESELECT_A::FALL)
    }
    #[doc = "Both edges detection"]
    #[inline(always)]
    pub fn both(self) -> &'a mut W {
        self.variant(NMISENSESELECT_A::BOTH)
    }
    #[doc = "High level detection"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(NMISENSESELECT_A::HIGH)
    }
    #[doc = "Low level detection"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(NMISENSESELECT_A::LOW)
    }
}
#[doc = "Field `NMIFILTEN` reader - NMI Filter Enable"]
pub type NMIFILTEN_R = crate::BitReader<bool>;
#[doc = "Field `NMIFILTEN` writer - NMI Filter Enable"]
pub type NMIFILTEN_W<'a, const O: u8> = crate::BitWriter<'a, u8, NMICTRL_SPEC, bool, O>;
#[doc = "Field `NMIASYNCH` reader - NMI Asynchronous edge Detection Enable"]
pub type NMIASYNCH_R = crate::BitReader<bool>;
#[doc = "Field `NMIASYNCH` writer - NMI Asynchronous edge Detection Enable"]
pub type NMIASYNCH_W<'a, const O: u8> = crate::BitWriter<'a, u8, NMICTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:2 - NMI Input Sense Configuration"]
    #[inline(always)]
    pub fn nmisense(&self) -> NMISENSE_R {
        NMISENSE_R::new(self.bits & 7)
    }
    #[doc = "Bit 3 - NMI Filter Enable"]
    #[inline(always)]
    pub fn nmifilten(&self) -> NMIFILTEN_R {
        NMIFILTEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - NMI Asynchronous edge Detection Enable"]
    #[inline(always)]
    pub fn nmiasynch(&self) -> NMIASYNCH_R {
        NMIASYNCH_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - NMI Input Sense Configuration"]
    #[inline(always)]
    #[must_use]
    pub fn nmisense(&mut self) -> NMISENSE_W<0> {
        NMISENSE_W::new(self)
    }
    #[doc = "Bit 3 - NMI Filter Enable"]
    #[inline(always)]
    #[must_use]
    pub fn nmifilten(&mut self) -> NMIFILTEN_W<3> {
        NMIFILTEN_W::new(self)
    }
    #[doc = "Bit 4 - NMI Asynchronous edge Detection Enable"]
    #[inline(always)]
    #[must_use]
    pub fn nmiasynch(&mut self) -> NMIASYNCH_W<4> {
        NMIASYNCH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "NMI Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nmictrl](index.html) module"]
pub struct NMICTRL_SPEC;
impl crate::RegisterSpec for NMICTRL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [nmictrl::R](R) reader structure"]
impl crate::Readable for NMICTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [nmictrl::W](W) writer structure"]
impl crate::Writable for NMICTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets NMICTRL to value 0"]
impl crate::Resettable for NMICTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
