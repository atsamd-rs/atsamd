#[doc = "Reader of register VTOR"]
pub type R = crate::R<u32, super::VTOR>;
#[doc = "Writer for register VTOR"]
pub type W = crate::W<u32, super::VTOR>;
#[doc = "Register VTOR `reset()`'s with value 0"]
impl crate::ResetValue for super::VTOR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TBLOFF`"]
pub type TBLOFF_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `TBLOFF`"]
pub struct TBLOFF_W<'a> {
    w: &'a mut W,
}
impl<'a> TBLOFF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01ff_ffff << 7)) | (((value as u32) & 0x01ff_ffff) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bits 7:31 - Vector table base offset"]
    #[inline(always)]
    pub fn tbloff(&self) -> TBLOFF_R {
        TBLOFF_R::new(((self.bits >> 7) & 0x01ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 7:31 - Vector table base offset"]
    #[inline(always)]
    pub fn tbloff(&mut self) -> TBLOFF_W {
        TBLOFF_W { w: self }
    }
}
