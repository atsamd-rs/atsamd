#[doc = "Register `CHPRILVL` reader"]
pub struct R(crate::R<CHPRILVL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CHPRILVL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CHPRILVL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CHPRILVL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CHPRILVL` writer"]
pub struct W(crate::W<CHPRILVL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CHPRILVL_SPEC>;
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
impl From<crate::W<CHPRILVL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CHPRILVL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRILVL` reader - Channel Priority Level"]
pub type PRILVL_R = crate::FieldReader<u8, PRILVLSELECT_A>;
#[doc = "Channel Priority Level\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PRILVLSELECT_A {
    #[doc = "0: Channel Priority Level 0 (Lowest Level)"]
    LVL0 = 0,
    #[doc = "1: Channel Priority Level 1"]
    LVL1 = 1,
    #[doc = "2: Channel Priority Level 2"]
    LVL2 = 2,
    #[doc = "3: Channel Priority Level 3 (Highest Level)"]
    LVL3 = 3,
}
impl From<PRILVLSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: PRILVLSELECT_A) -> Self {
        variant as _
    }
}
impl PRILVL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PRILVLSELECT_A {
        match self.bits {
            0 => PRILVLSELECT_A::LVL0,
            1 => PRILVLSELECT_A::LVL1,
            2 => PRILVLSELECT_A::LVL2,
            3 => PRILVLSELECT_A::LVL3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LVL0`"]
    #[inline(always)]
    pub fn is_lvl0(&self) -> bool {
        *self == PRILVLSELECT_A::LVL0
    }
    #[doc = "Checks if the value of the field is `LVL1`"]
    #[inline(always)]
    pub fn is_lvl1(&self) -> bool {
        *self == PRILVLSELECT_A::LVL1
    }
    #[doc = "Checks if the value of the field is `LVL2`"]
    #[inline(always)]
    pub fn is_lvl2(&self) -> bool {
        *self == PRILVLSELECT_A::LVL2
    }
    #[doc = "Checks if the value of the field is `LVL3`"]
    #[inline(always)]
    pub fn is_lvl3(&self) -> bool {
        *self == PRILVLSELECT_A::LVL3
    }
}
#[doc = "Field `PRILVL` writer - Channel Priority Level"]
pub type PRILVL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u8, CHPRILVL_SPEC, u8, PRILVLSELECT_A, 2, O>;
impl<'a, const O: u8> PRILVL_W<'a, O> {
    #[doc = "Channel Priority Level 0 (Lowest Level)"]
    #[inline(always)]
    pub fn lvl0(self) -> &'a mut W {
        self.variant(PRILVLSELECT_A::LVL0)
    }
    #[doc = "Channel Priority Level 1"]
    #[inline(always)]
    pub fn lvl1(self) -> &'a mut W {
        self.variant(PRILVLSELECT_A::LVL1)
    }
    #[doc = "Channel Priority Level 2"]
    #[inline(always)]
    pub fn lvl2(self) -> &'a mut W {
        self.variant(PRILVLSELECT_A::LVL2)
    }
    #[doc = "Channel Priority Level 3 (Highest Level)"]
    #[inline(always)]
    pub fn lvl3(self) -> &'a mut W {
        self.variant(PRILVLSELECT_A::LVL3)
    }
}
impl R {
    #[doc = "Bits 0:1 - Channel Priority Level"]
    #[inline(always)]
    pub fn prilvl(&self) -> PRILVL_R {
        PRILVL_R::new(self.bits & 3)
    }
}
impl W {
    #[doc = "Bits 0:1 - Channel Priority Level"]
    #[inline(always)]
    #[must_use]
    pub fn prilvl(&mut self) -> PRILVL_W<0> {
        PRILVL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel n Priority Level\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chprilvl](index.html) module"]
pub struct CHPRILVL_SPEC;
impl crate::RegisterSpec for CHPRILVL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [chprilvl::R](R) reader structure"]
impl crate::Readable for CHPRILVL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [chprilvl::W](W) writer structure"]
impl crate::Writable for CHPRILVL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CHPRILVL to value 0"]
impl crate::Resettable for CHPRILVL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
