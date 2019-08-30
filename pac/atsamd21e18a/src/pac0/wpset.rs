#[doc = "Reader of register WPSET"]
pub type R = crate::R<u32, super::WPSET>;
#[doc = "Writer for register WPSET"]
pub type W = crate::W<u32, super::WPSET>;
#[doc = "Register WPSET `reset()`'s with value 0"]
impl crate::ResetValue for super::WPSET {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `WP`"]
pub type WP_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `WP`"]
pub struct WP_W<'a> {
    w: &'a mut W,
}
impl<'a> WP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7fff_ffff << 1)) | (((value as u32) & 0x7fff_ffff) << 1);
        self.w
    }
}
impl R {
    #[doc = "Bits 1:31 - Write Protection Set"]
    #[inline(always)]
    pub fn wp(&self) -> WP_R {
        WP_R::new(((self.bits >> 1) & 0x7fff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 1:31 - Write Protection Set"]
    #[inline(always)]
    pub fn wp(&mut self) -> WP_W {
        WP_W { w: self }
    }
}
