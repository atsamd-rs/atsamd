#[doc = "Register `USER` reader"]
pub struct R(crate::R<USER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USER` writer"]
pub struct W(crate::W<USER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USER_SPEC>;
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
impl From<crate::W<USER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USER` reader - User Multiplexer Selection"]
pub struct USER_R(crate::FieldReader<u8, u8>);
impl USER_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        USER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USER_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USER` writer - User Multiplexer Selection"]
pub struct USER_W<'a> {
    w: &'a mut W,
}
impl<'a> USER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | (value as u16 & 0x1f);
        self.w
    }
}
#[doc = "Channel Event Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CHANNEL_A {
    #[doc = "0: No Channel Output Selected"]
    _0 = 0,
}
impl From<CHANNEL_A> for u8 {
    #[inline(always)]
    fn from(variant: CHANNEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CHANNEL` reader - Channel Event Selection"]
pub struct CHANNEL_R(crate::FieldReader<u8, CHANNEL_A>);
impl CHANNEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CHANNEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CHANNEL_A> {
        match self.bits {
            0 => Some(CHANNEL_A::_0),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CHANNEL_A::_0
    }
}
impl core::ops::Deref for CHANNEL_R {
    type Target = crate::FieldReader<u8, CHANNEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHANNEL` writer - Channel Event Selection"]
pub struct CHANNEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CHANNEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHANNEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "No Channel Output Selected"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CHANNEL_A::_0)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 8)) | ((value as u16 & 0x1f) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - User Multiplexer Selection"]
    #[inline(always)]
    pub fn user(&self) -> USER_R {
        USER_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - Channel Event Selection"]
    #[inline(always)]
    pub fn channel(&self) -> CHANNEL_R {
        CHANNEL_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - User Multiplexer Selection"]
    #[inline(always)]
    pub fn user(&mut self) -> USER_W {
        USER_W { w: self }
    }
    #[doc = "Bits 8:12 - Channel Event Selection"]
    #[inline(always)]
    pub fn channel(&mut self) -> CHANNEL_W {
        CHANNEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "User Multiplexer\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [user](index.html) module"]
pub struct USER_SPEC;
impl crate::RegisterSpec for USER_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [user::R](R) reader structure"]
impl crate::Readable for USER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [user::W](W) writer structure"]
impl crate::Writable for USER_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets USER to value 0"]
impl crate::Resettable for USER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
