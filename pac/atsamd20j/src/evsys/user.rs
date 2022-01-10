#[doc = "Reader of register USER"]
pub type R = crate::R<u16, super::USER>;
#[doc = "Writer for register USER"]
pub type W = crate::W<u16, super::USER>;
#[doc = "Register USER `reset()`'s with value 0"]
impl crate::ResetValue for super::USER {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `USER`"]
pub type USER_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `USER`"]
pub struct USER_W<'a> {
    w: &'a mut W,
}
impl<'a> USER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u16) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `CHANNEL`"]
pub type CHANNEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CHANNEL`"]
pub struct CHANNEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CHANNEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u16) & 0x0f) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - User Multiplexer Selection"]
    #[inline(always)]
    pub fn user(&self) -> USER_R {
        USER_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Channel Event Selection"]
    #[inline(always)]
    pub fn channel(&self) -> CHANNEL_R {
        CHANNEL_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - User Multiplexer Selection"]
    #[inline(always)]
    pub fn user(&mut self) -> USER_W {
        USER_W { w: self }
    }
    #[doc = "Bits 8:11 - Channel Event Selection"]
    #[inline(always)]
    pub fn channel(&mut self) -> CHANNEL_W {
        CHANNEL_W { w: self }
    }
}
