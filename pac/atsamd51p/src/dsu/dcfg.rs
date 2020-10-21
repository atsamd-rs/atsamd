#[doc = "Reader of register DCFG[%s]"]
pub type R = crate::R<u32, super::DCFG>;
#[doc = "Writer for register DCFG[%s]"]
pub type W = crate::W<u32, super::DCFG>;
#[doc = "Register DCFG[%s]
`reset()`'s with value 0"]
impl crate::ResetValue for super::DCFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DCFG`"]
pub type DCFG_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `DCFG`"]
pub struct DCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> DCFG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Device Configuration"]
    #[inline(always)]
    pub fn dcfg(&self) -> DCFG_R {
        DCFG_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Device Configuration"]
    #[inline(always)]
    pub fn dcfg(&mut self) -> DCFG_W {
        DCFG_W { w: self }
    }
}
