#[doc = "Register `MCFG[%s]` reader"]
pub struct R(crate::R<MCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MCFG[%s]` writer"]
pub struct W(crate::W<MCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MCFG_SPEC>;
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
impl From<crate::W<MCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Undefined Length Burst Type\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ULBT_A {
    #[doc = "0: Infinite Length"]
    INFINITE = 0,
    #[doc = "1: Single Access"]
    SINGLE = 1,
    #[doc = "2: Four Beat Burst"]
    FOUR_BEAT = 2,
    #[doc = "3: Eight Beat Burst"]
    EIGHT_BEAT = 3,
    #[doc = "4: Sixteen Beat Burst"]
    SIXTEEN_BEAT = 4,
}
impl From<ULBT_A> for u8 {
    #[inline(always)]
    fn from(variant: ULBT_A) -> Self {
        variant as _
    }
}
#[doc = "Field `ULBT` reader - Undefined Length Burst Type"]
pub struct ULBT_R(crate::FieldReader<u8, ULBT_A>);
impl ULBT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        ULBT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ULBT_A> {
        match self.bits {
            0 => Some(ULBT_A::INFINITE),
            1 => Some(ULBT_A::SINGLE),
            2 => Some(ULBT_A::FOUR_BEAT),
            3 => Some(ULBT_A::EIGHT_BEAT),
            4 => Some(ULBT_A::SIXTEEN_BEAT),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `INFINITE`"]
    #[inline(always)]
    pub fn is_infinite(&self) -> bool {
        **self == ULBT_A::INFINITE
    }
    #[doc = "Checks if the value of the field is `SINGLE`"]
    #[inline(always)]
    pub fn is_single(&self) -> bool {
        **self == ULBT_A::SINGLE
    }
    #[doc = "Checks if the value of the field is `FOUR_BEAT`"]
    #[inline(always)]
    pub fn is_four_beat(&self) -> bool {
        **self == ULBT_A::FOUR_BEAT
    }
    #[doc = "Checks if the value of the field is `EIGHT_BEAT`"]
    #[inline(always)]
    pub fn is_eight_beat(&self) -> bool {
        **self == ULBT_A::EIGHT_BEAT
    }
    #[doc = "Checks if the value of the field is `SIXTEEN_BEAT`"]
    #[inline(always)]
    pub fn is_sixteen_beat(&self) -> bool {
        **self == ULBT_A::SIXTEEN_BEAT
    }
}
impl core::ops::Deref for ULBT_R {
    type Target = crate::FieldReader<u8, ULBT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ULBT` writer - Undefined Length Burst Type"]
pub struct ULBT_W<'a> {
    w: &'a mut W,
}
impl<'a> ULBT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ULBT_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Infinite Length"]
    #[inline(always)]
    pub fn infinite(self) -> &'a mut W {
        self.variant(ULBT_A::INFINITE)
    }
    #[doc = "Single Access"]
    #[inline(always)]
    pub fn single(self) -> &'a mut W {
        self.variant(ULBT_A::SINGLE)
    }
    #[doc = "Four Beat Burst"]
    #[inline(always)]
    pub fn four_beat(self) -> &'a mut W {
        self.variant(ULBT_A::FOUR_BEAT)
    }
    #[doc = "Eight Beat Burst"]
    #[inline(always)]
    pub fn eight_beat(self) -> &'a mut W {
        self.variant(ULBT_A::EIGHT_BEAT)
    }
    #[doc = "Sixteen Beat Burst"]
    #[inline(always)]
    pub fn sixteen_beat(self) -> &'a mut W {
        self.variant(ULBT_A::SIXTEEN_BEAT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Undefined Length Burst Type"]
    #[inline(always)]
    pub fn ulbt(&self) -> ULBT_R {
        ULBT_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Undefined Length Burst Type"]
    #[inline(always)]
    pub fn ulbt(&mut self) -> ULBT_W {
        ULBT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Master Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcfg](index.html) module"]
pub struct MCFG_SPEC;
impl crate::RegisterSpec for MCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mcfg::R](R) reader structure"]
impl crate::Readable for MCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mcfg::W](W) writer structure"]
impl crate::Writable for MCFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MCFG[%s]
to value 0x02"]
impl crate::Resettable for MCFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x02
    }
}
