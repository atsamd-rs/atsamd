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
pub struct DIR_R(crate::FieldReader<bool, bool>);
impl DIR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DIR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIR` writer - Counter Direction"]
pub struct DIR_W<'a> {
    w: &'a mut W,
}
impl<'a> DIR_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | (value as u8 & 0x01);
        self.w
    }
}
#[doc = "Field `ONESHOT` reader - One-Shot"]
pub struct ONESHOT_R(crate::FieldReader<bool, bool>);
impl ONESHOT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ONESHOT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ONESHOT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ONESHOT` writer - One-Shot"]
pub struct ONESHOT_W<'a> {
    w: &'a mut W,
}
impl<'a> ONESHOT_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u8 & 0x01) << 2);
        self.w
    }
}
#[doc = "Command\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CMD_A {
    #[doc = "0: No action"]
    NONE = 0,
    #[doc = "1: Force a start, restart or retrigger"]
    RETRIGGER = 1,
    #[doc = "2: Force a stop"]
    STOP = 2,
}
impl From<CMD_A> for u8 {
    #[inline(always)]
    fn from(variant: CMD_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CMD` reader - Command"]
pub struct CMD_R(crate::FieldReader<u8, CMD_A>);
impl CMD_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CMD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CMD_A> {
        match self.bits {
            0 => Some(CMD_A::NONE),
            1 => Some(CMD_A::RETRIGGER),
            2 => Some(CMD_A::STOP),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        **self == CMD_A::NONE
    }
    #[doc = "Checks if the value of the field is `RETRIGGER`"]
    #[inline(always)]
    pub fn is_retrigger(&self) -> bool {
        **self == CMD_A::RETRIGGER
    }
    #[doc = "Checks if the value of the field is `STOP`"]
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        **self == CMD_A::STOP
    }
}
impl core::ops::Deref for CMD_R {
    type Target = crate::FieldReader<u8, CMD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMD` writer - Command"]
pub struct CMD_W<'a> {
    w: &'a mut W,
}
impl<'a> CMD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CMD_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "No action"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(CMD_A::NONE)
    }
    #[doc = "Force a start, restart or retrigger"]
    #[inline(always)]
    pub fn retrigger(self) -> &'a mut W {
        self.variant(CMD_A::RETRIGGER)
    }
    #[doc = "Force a stop"]
    #[inline(always)]
    pub fn stop(self) -> &'a mut W {
        self.variant(CMD_A::STOP)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | ((value as u8 & 0x03) << 6);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Counter Direction"]
    #[inline(always)]
    pub fn dir(&self) -> DIR_R {
        DIR_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 2 - One-Shot"]
    #[inline(always)]
    pub fn oneshot(&self) -> ONESHOT_R {
        ONESHOT_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 6:7 - Command"]
    #[inline(always)]
    pub fn cmd(&self) -> CMD_R {
        CMD_R::new(((self.bits >> 6) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Counter Direction"]
    #[inline(always)]
    pub fn dir(&mut self) -> DIR_W {
        DIR_W { w: self }
    }
    #[doc = "Bit 2 - One-Shot"]
    #[inline(always)]
    pub fn oneshot(&mut self) -> ONESHOT_W {
        ONESHOT_W { w: self }
    }
    #[doc = "Bits 6:7 - Command"]
    #[inline(always)]
    pub fn cmd(&mut self) -> CMD_W {
        CMD_W { w: self }
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
}
#[doc = "`reset()` method sets CTRLBSET to value 0"]
impl crate::Resettable for CTRLBSET_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
