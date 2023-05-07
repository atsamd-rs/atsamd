#[doc = "Register `CTRLBSET` reader"]
pub struct R(crate::R<CTRLBSET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRLBSET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRLBSET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRLBSET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRLBSET` writer"]
pub struct W(crate::W<CTRLBSET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRLBSET_SPEC>;
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
impl From<crate::W<CTRLBSET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRLBSET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DIR` reader - Counter Direction"]
pub type DIR_R = crate::BitReader<bool>;
#[doc = "Field `DIR` writer - Counter Direction"]
pub type DIR_W<'a, const O: u8> = crate::BitWriter<'a, u8, CTRLBSET_SPEC, bool, O>;
#[doc = "Field `ONESHOT` reader - One-Shot"]
pub type ONESHOT_R = crate::BitReader<bool>;
#[doc = "Field `ONESHOT` writer - One-Shot"]
pub type ONESHOT_W<'a, const O: u8> = crate::BitWriter<'a, u8, CTRLBSET_SPEC, bool, O>;
#[doc = "Field `CMD` reader - Command"]
pub type CMD_R = crate::FieldReader<u8, CMDSELECT_A>;
#[doc = "Command\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CMDSELECT_A {
    #[doc = "0: No action"]
    NONE = 0,
    #[doc = "1: Force a start, restart or retrigger"]
    RETRIGGER = 1,
    #[doc = "2: Force a stop"]
    STOP = 2,
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
            0 => Some(CMDSELECT_A::NONE),
            1 => Some(CMDSELECT_A::RETRIGGER),
            2 => Some(CMDSELECT_A::STOP),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == CMDSELECT_A::NONE
    }
    #[doc = "Checks if the value of the field is `RETRIGGER`"]
    #[inline(always)]
    pub fn is_retrigger(&self) -> bool {
        *self == CMDSELECT_A::RETRIGGER
    }
    #[doc = "Checks if the value of the field is `STOP`"]
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        *self == CMDSELECT_A::STOP
    }
}
#[doc = "Field `CMD` writer - Command"]
pub type CMD_W<'a, const O: u8> = crate::FieldWriter<'a, u8, CTRLBSET_SPEC, u8, CMDSELECT_A, 2, O>;
impl<'a, const O: u8> CMD_W<'a, O> {
    #[doc = "No action"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(CMDSELECT_A::NONE)
    }
    #[doc = "Force a start, restart or retrigger"]
    #[inline(always)]
    pub fn retrigger(self) -> &'a mut W {
        self.variant(CMDSELECT_A::RETRIGGER)
    }
    #[doc = "Force a stop"]
    #[inline(always)]
    pub fn stop(self) -> &'a mut W {
        self.variant(CMDSELECT_A::STOP)
    }
}
impl R {
    #[doc = "Bit 0 - Counter Direction"]
    #[inline(always)]
    pub fn dir(&self) -> DIR_R {
        DIR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - One-Shot"]
    #[inline(always)]
    pub fn oneshot(&self) -> ONESHOT_R {
        ONESHOT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Command"]
    #[inline(always)]
    pub fn cmd(&self) -> CMD_R {
        CMD_R::new((self.bits >> 6) & 3)
    }
}
impl W {
    #[doc = "Bit 0 - Counter Direction"]
    #[inline(always)]
    #[must_use]
    pub fn dir(&mut self) -> DIR_W<0> {
        DIR_W::new(self)
    }
    #[doc = "Bit 2 - One-Shot"]
    #[inline(always)]
    #[must_use]
    pub fn oneshot(&mut self) -> ONESHOT_W<2> {
        ONESHOT_W::new(self)
    }
    #[doc = "Bits 6:7 - Command"]
    #[inline(always)]
    #[must_use]
    pub fn cmd(&mut self) -> CMD_W<6> {
        CMD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control B Set\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrlbset](index.html) module"]
pub struct CTRLBSET_SPEC;
impl crate::RegisterSpec for CTRLBSET_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [ctrlbset::R](R) reader structure"]
impl crate::Readable for CTRLBSET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrlbset::W](W) writer structure"]
impl crate::Writable for CTRLBSET_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRLBSET to value 0"]
impl crate::Resettable for CTRLBSET_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
