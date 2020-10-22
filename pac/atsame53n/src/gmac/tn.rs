#[doc = "Reader of register TN"]
pub type R = crate::R<u32, super::TN>;
#[doc = "Writer for register TN"]
pub type W = crate::W<u32, super::TN>;
#[doc = "Register TN `reset()`'s with value 0"]
impl crate::ResetValue for super::TN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TNS`"]
pub type TNS_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `TNS`"]
pub struct TNS_W<'a> {
    w: &'a mut W,
}
impl<'a> TNS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3fff_ffff) | ((value as u32) & 0x3fff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:29 - Timer Count in Nanoseconds"]
    #[inline(always)]
    pub fn tns(&self) -> TNS_R {
        TNS_R::new((self.bits & 0x3fff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:29 - Timer Count in Nanoseconds"]
    #[inline(always)]
    pub fn tns(&mut self) -> TNS_W {
        TNS_W { w: self }
    }
}
