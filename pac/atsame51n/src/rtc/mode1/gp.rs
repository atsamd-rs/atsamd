#[doc = "Reader of register GP[%s]"]
pub type R = crate::R<u32, super::GP>;
#[doc = "Writer for register GP[%s]"]
pub type W = crate::W<u32, super::GP>;
#[doc = "Register GP[%s]
`reset()`'s with value 0"]
impl crate::ResetValue for super::GP {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `GP`"]
pub type GP_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `GP`"]
pub struct GP_W<'a> {
    w: &'a mut W,
}
impl<'a> GP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - General Purpose"]
    #[inline(always)]
    pub fn gp(&self) -> GP_R {
        GP_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - General Purpose"]
    #[inline(always)]
    pub fn gp(&mut self) -> GP_W {
        GP_W { w: self }
    }
}
