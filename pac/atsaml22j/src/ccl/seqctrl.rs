#[doc = "Register `SEQCTRL%s` reader"]
pub struct R(crate::R<SEQCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SEQCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SEQCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SEQCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SEQCTRL%s` writer"]
pub struct W(crate::W<SEQCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SEQCTRL_SPEC>;
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
impl From<crate::W<SEQCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SEQCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEQSEL` reader - Sequential Selection"]
pub type SEQSEL_R = crate::FieldReader<u8, SEQSELSELECT_A>;
#[doc = "Sequential Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SEQSELSELECT_A {
    #[doc = "0: Sequential logic is disabled"]
    DISABLE = 0,
    #[doc = "1: D flip flop"]
    DFF = 1,
    #[doc = "2: JK flip flop"]
    JK = 2,
    #[doc = "3: D latch"]
    LATCH = 3,
    #[doc = "4: RS latch"]
    RS = 4,
}
impl From<SEQSELSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: SEQSELSELECT_A) -> Self {
        variant as _
    }
}
impl SEQSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SEQSELSELECT_A> {
        match self.bits {
            0 => Some(SEQSELSELECT_A::DISABLE),
            1 => Some(SEQSELSELECT_A::DFF),
            2 => Some(SEQSELSELECT_A::JK),
            3 => Some(SEQSELSELECT_A::LATCH),
            4 => Some(SEQSELSELECT_A::RS),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == SEQSELSELECT_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `DFF`"]
    #[inline(always)]
    pub fn is_dff(&self) -> bool {
        *self == SEQSELSELECT_A::DFF
    }
    #[doc = "Checks if the value of the field is `JK`"]
    #[inline(always)]
    pub fn is_jk(&self) -> bool {
        *self == SEQSELSELECT_A::JK
    }
    #[doc = "Checks if the value of the field is `LATCH`"]
    #[inline(always)]
    pub fn is_latch(&self) -> bool {
        *self == SEQSELSELECT_A::LATCH
    }
    #[doc = "Checks if the value of the field is `RS`"]
    #[inline(always)]
    pub fn is_rs(&self) -> bool {
        *self == SEQSELSELECT_A::RS
    }
}
#[doc = "Field `SEQSEL` writer - Sequential Selection"]
pub type SEQSEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u8, SEQCTRL_SPEC, u8, SEQSELSELECT_A, 4, O>;
impl<'a, const O: u8> SEQSEL_W<'a, O> {
    #[doc = "Sequential logic is disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(SEQSELSELECT_A::DISABLE)
    }
    #[doc = "D flip flop"]
    #[inline(always)]
    pub fn dff(self) -> &'a mut W {
        self.variant(SEQSELSELECT_A::DFF)
    }
    #[doc = "JK flip flop"]
    #[inline(always)]
    pub fn jk(self) -> &'a mut W {
        self.variant(SEQSELSELECT_A::JK)
    }
    #[doc = "D latch"]
    #[inline(always)]
    pub fn latch(self) -> &'a mut W {
        self.variant(SEQSELSELECT_A::LATCH)
    }
    #[doc = "RS latch"]
    #[inline(always)]
    pub fn rs(self) -> &'a mut W {
        self.variant(SEQSELSELECT_A::RS)
    }
}
impl R {
    #[doc = "Bits 0:3 - Sequential Selection"]
    #[inline(always)]
    pub fn seqsel(&self) -> SEQSEL_R {
        SEQSEL_R::new(self.bits & 0x0f)
    }
}
impl W {
    #[doc = "Bits 0:3 - Sequential Selection"]
    #[inline(always)]
    #[must_use]
    pub fn seqsel(&mut self) -> SEQSEL_W<0> {
        SEQSEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SEQ Control x\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [seqctrl](index.html) module"]
pub struct SEQCTRL_SPEC;
impl crate::RegisterSpec for SEQCTRL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [seqctrl::R](R) reader structure"]
impl crate::Readable for SEQCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [seqctrl::W](W) writer structure"]
impl crate::Writable for SEQCTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SEQCTRL%s to value 0"]
impl crate::Resettable for SEQCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
