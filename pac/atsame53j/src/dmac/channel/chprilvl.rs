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
#[doc = "Channel Priority Level\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PRILVL_A {
    #[doc = "0: Channel Priority Level 0 (Lowest Level)"]
    LVL0 = 0,
    #[doc = "1: Channel Priority Level 1"]
    LVL1 = 1,
    #[doc = "2: Channel Priority Level 2"]
    LVL2 = 2,
    #[doc = "3: Channel Priority Level 3 (Highest Level)"]
    LVL3 = 3,
}
impl From<PRILVL_A> for u8 {
    #[inline(always)]
    fn from(variant: PRILVL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PRILVL` reader - Channel Priority Level"]
pub struct PRILVL_R(crate::FieldReader<u8, PRILVL_A>);
impl PRILVL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PRILVL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PRILVL_A {
        match self.bits {
            0 => PRILVL_A::LVL0,
            1 => PRILVL_A::LVL1,
            2 => PRILVL_A::LVL2,
            3 => PRILVL_A::LVL3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LVL0`"]
    #[inline(always)]
    pub fn is_lvl0(&self) -> bool {
        **self == PRILVL_A::LVL0
    }
    #[doc = "Checks if the value of the field is `LVL1`"]
    #[inline(always)]
    pub fn is_lvl1(&self) -> bool {
        **self == PRILVL_A::LVL1
    }
    #[doc = "Checks if the value of the field is `LVL2`"]
    #[inline(always)]
    pub fn is_lvl2(&self) -> bool {
        **self == PRILVL_A::LVL2
    }
    #[doc = "Checks if the value of the field is `LVL3`"]
    #[inline(always)]
    pub fn is_lvl3(&self) -> bool {
        **self == PRILVL_A::LVL3
    }
}
impl core::ops::Deref for PRILVL_R {
    type Target = crate::FieldReader<u8, PRILVL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRILVL` writer - Channel Priority Level"]
pub struct PRILVL_W<'a> {
    w: &'a mut W,
}
impl<'a> PRILVL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PRILVL_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Channel Priority Level 0 (Lowest Level)"]
    #[inline(always)]
    pub fn lvl0(self) -> &'a mut W {
        self.variant(PRILVL_A::LVL0)
    }
    #[doc = "Channel Priority Level 1"]
    #[inline(always)]
    pub fn lvl1(self) -> &'a mut W {
        self.variant(PRILVL_A::LVL1)
    }
    #[doc = "Channel Priority Level 2"]
    #[inline(always)]
    pub fn lvl2(self) -> &'a mut W {
        self.variant(PRILVL_A::LVL2)
    }
    #[doc = "Channel Priority Level 3 (Highest Level)"]
    #[inline(always)]
    pub fn lvl3(self) -> &'a mut W {
        self.variant(PRILVL_A::LVL3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u8 & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Channel Priority Level"]
    #[inline(always)]
    pub fn prilvl(&self) -> PRILVL_R {
        PRILVL_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Channel Priority Level"]
    #[inline(always)]
    pub fn prilvl(&mut self) -> PRILVL_W {
        PRILVL_W { w: self }
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
}
#[doc = "`reset()` method sets CHPRILVL to value 0"]
impl crate::Resettable for CHPRILVL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
