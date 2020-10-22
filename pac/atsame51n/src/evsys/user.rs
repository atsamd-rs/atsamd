#[doc = "Reader of register USER[%s]"]
pub type R = crate::R<u32, super::USER>;
#[doc = "Writer for register USER[%s]"]
pub type W = crate::W<u32, super::USER>;
#[doc = "Register USER[%s]
`reset()`'s with value 0"]
impl crate::ResetValue for super::USER {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
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
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - Channel Event Selection"]
    #[inline(always)]
    pub fn channel(&self) -> CHANNEL_R {
        CHANNEL_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Channel Event Selection"]
    #[inline(always)]
    pub fn channel(&mut self) -> CHANNEL_W {
        CHANNEL_W { w: self }
    }
}
