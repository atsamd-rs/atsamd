#[doc = "Reader of register ASYNCH"]
pub type R = crate::R<u32, super::ASYNCH>;
#[doc = "Writer for register ASYNCH"]
pub type W = crate::W<u32, super::ASYNCH>;
#[doc = "Register ASYNCH `reset()`'s with value 0"]
impl crate::ResetValue for super::ASYNCH {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ASYNCH`"]
pub type ASYNCH_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `ASYNCH`"]
pub struct ASYNCH_W<'a> {
    w: &'a mut W,
}
impl<'a> ASYNCH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Asynchronous Edge Detection Mode"]
    #[inline(always)]
    pub fn asynch(&self) -> ASYNCH_R {
        ASYNCH_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Asynchronous Edge Detection Mode"]
    #[inline(always)]
    pub fn asynch(&mut self) -> ASYNCH_W {
        ASYNCH_W { w: self }
    }
}
