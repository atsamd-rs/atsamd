#[doc = "Reader of register BCR"]
pub type R = crate::R<u16, super::BCR>;
#[doc = "Writer for register BCR"]
pub type W = crate::W<u16, super::BCR>;
#[doc = "Register BCR `reset()`'s with value 0"]
impl crate::ResetValue for super::BCR {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `BCNT`"]
pub type BCNT_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `BCNT`"]
pub struct BCNT_W<'a> {
    w: &'a mut W,
}
impl<'a> BCNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u16) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Blocks Count for Current Transfer"]
    #[inline(always)]
    pub fn bcnt(&self) -> BCNT_R {
        BCNT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Blocks Count for Current Transfer"]
    #[inline(always)]
    pub fn bcnt(&mut self) -> BCNT_W {
        BCNT_W { w: self }
    }
}
