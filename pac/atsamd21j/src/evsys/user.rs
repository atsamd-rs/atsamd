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
pub type USER_R = crate::FieldReader<u8, u8>;
#[doc = "Field `USER` writer - User Multiplexer Selection"]
pub type USER_W<'a, const O: u8> = crate::FieldWriter<'a, u16, USER_SPEC, u8, u8, 5, O>;
#[doc = "Field `CHANNEL` reader - Channel Event Selection"]
pub type CHANNEL_R = crate::FieldReader<u8, CHANNELSELECT_A>;
#[doc = "Channel Event Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CHANNELSELECT_A {
    #[doc = "0: No Channel Output Selected"]
    _0 = 0,
}
impl From<CHANNELSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: CHANNELSELECT_A) -> Self {
        variant as _
    }
}
impl CHANNEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CHANNELSELECT_A> {
        match self.bits {
            0 => Some(CHANNELSELECT_A::_0),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CHANNELSELECT_A::_0
    }
}
#[doc = "Field `CHANNEL` writer - Channel Event Selection"]
pub type CHANNEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u16, USER_SPEC, u8, CHANNELSELECT_A, 5, O>;
impl<'a, const O: u8> CHANNEL_W<'a, O> {
    #[doc = "No Channel Output Selected"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CHANNELSELECT_A::_0)
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
    #[must_use]
    pub fn user(&mut self) -> USER_W<0> {
        USER_W::new(self)
    }
    #[doc = "Bits 8:12 - Channel Event Selection"]
    #[inline(always)]
    #[must_use]
    pub fn channel(&mut self) -> CHANNEL_W<8> {
        CHANNEL_W::new(self)
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets USER to value 0"]
impl crate::Resettable for USER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
