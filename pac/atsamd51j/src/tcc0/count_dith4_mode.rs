#[doc = "Reader of register COUNT_DITH4_MODE"]
pub type R = crate::R<u32, super::COUNT_DITH4_MODE>;
#[doc = "Writer for register COUNT_DITH4_MODE"]
pub type W = crate::W<u32, super::COUNT_DITH4_MODE>;
#[doc = "Register COUNT_DITH4_MODE `reset()`'s with value 0"]
impl crate::ResetValue for super::COUNT_DITH4_MODE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `COUNT`"]
pub type COUNT_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `COUNT`"]
pub struct COUNT_W<'a> {
    w: &'a mut W,
}
impl<'a> COUNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x000f_ffff << 4)) | (((value as u32) & 0x000f_ffff) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bits 4:23 - Counter Value"]
    #[inline(always)]
    pub fn count(&self) -> COUNT_R {
        COUNT_R::new(((self.bits >> 4) & 0x000f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 4:23 - Counter Value"]
    #[inline(always)]
    pub fn count(&mut self) -> COUNT_W {
        COUNT_W { w: self }
    }
}
