#[doc = "Reader of register PERB"]
pub type R = crate::R<u32, super::PERB>;
#[doc = "Writer for register PERB"]
pub type W = crate::W<u32, super::PERB>;
#[doc = "Register PERB `reset()`'s with value 0xffff_ffff"]
impl crate::ResetValue for super::PERB {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff_ffff
    }
}
#[doc = "Reader of field `PERB`"]
pub type PERB_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `PERB`"]
pub struct PERB_W<'a> {
    w: &'a mut W,
}
impl<'a> PERB_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x00ff_ffff) | ((value as u32) & 0x00ff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:23 - Period Buffer Value"]
    #[inline(always)]
    pub fn perb(&self) -> PERB_R {
        PERB_R::new((self.bits & 0x00ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:23 - Period Buffer Value"]
    #[inline(always)]
    pub fn perb(&mut self) -> PERB_W {
        PERB_W { w: self }
    }
}
