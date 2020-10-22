#[doc = "Reader of register BKUP[%s]"]
pub type R = crate::R<u32, super::BKUP>;
#[doc = "Writer for register BKUP[%s]"]
pub type W = crate::W<u32, super::BKUP>;
#[doc = "Register BKUP[%s]
`reset()`'s with value 0"]
impl crate::ResetValue for super::BKUP {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `BKUP`"]
pub type BKUP_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `BKUP`"]
pub struct BKUP_W<'a> {
    w: &'a mut W,
}
impl<'a> BKUP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Backup"]
    #[inline(always)]
    pub fn bkup(&self) -> BKUP_R {
        BKUP_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Backup"]
    #[inline(always)]
    pub fn bkup(&mut self) -> BKUP_W {
        BKUP_W { w: self }
    }
}
