#[doc = "Reader of register SSAR_CMD23_MODE"]
pub type R = crate::R<u32, super::SSAR_CMD23_MODE>;
#[doc = "Writer for register SSAR_CMD23_MODE"]
pub type W = crate::W<u32, super::SSAR_CMD23_MODE>;
#[doc = "Register SSAR_CMD23_MODE `reset()`'s with value 0"]
impl crate::ResetValue for super::SSAR_CMD23_MODE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ARG2`"]
pub type ARG2_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `ARG2`"]
pub struct ARG2_W<'a> {
    w: &'a mut W,
}
impl<'a> ARG2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Argument 2"]
    #[inline(always)]
    pub fn arg2(&self) -> ARG2_R {
        ARG2_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Argument 2"]
    #[inline(always)]
    pub fn arg2(&mut self) -> ARG2_W {
        ARG2_W { w: self }
    }
}
