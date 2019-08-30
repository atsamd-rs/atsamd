#[doc = "Reader of register TCR"]
pub type R = crate::R<u8, super::TCR>;
#[doc = "Writer for register TCR"]
pub type W = crate::W<u8, super::TCR>;
#[doc = "Register TCR `reset()`'s with value 0"]
impl crate::ResetValue for super::TCR {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DTCVAL`"]
pub type DTCVAL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DTCVAL`"]
pub struct DTCVAL_W<'a> {
    w: &'a mut W,
}
impl<'a> DTCVAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u8) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Data Timeout Counter Value"]
    #[inline(always)]
    pub fn dtcval(&self) -> DTCVAL_R {
        DTCVAL_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Data Timeout Counter Value"]
    #[inline(always)]
    pub fn dtcval(&mut self) -> DTCVAL_W {
        DTCVAL_W { w: self }
    }
}
