#[doc = "Reader of register TI"]
pub type R = crate::R<u32, super::TI>;
#[doc = "Writer for register TI"]
pub type W = crate::W<u32, super::TI>;
#[doc = "Register TI `reset()`'s with value 0"]
impl crate::ResetValue for super::TI {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CNS`"]
pub type CNS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CNS`"]
pub struct CNS_W<'a> {
    w: &'a mut W,
}
impl<'a> CNS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `ACNS`"]
pub type ACNS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ACNS`"]
pub struct ACNS_W<'a> {
    w: &'a mut W,
}
impl<'a> ACNS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `NIT`"]
pub type NIT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `NIT`"]
pub struct NIT_W<'a> {
    w: &'a mut W,
}
impl<'a> NIT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Count Nanoseconds"]
    #[inline(always)]
    pub fn cns(&self) -> CNS_R {
        CNS_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Alternative Count Nanoseconds"]
    #[inline(always)]
    pub fn acns(&self) -> ACNS_R {
        ACNS_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Number of Increments"]
    #[inline(always)]
    pub fn nit(&self) -> NIT_R {
        NIT_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Count Nanoseconds"]
    #[inline(always)]
    pub fn cns(&mut self) -> CNS_W {
        CNS_W { w: self }
    }
    #[doc = "Bits 8:15 - Alternative Count Nanoseconds"]
    #[inline(always)]
    pub fn acns(&mut self) -> ACNS_W {
        ACNS_W { w: self }
    }
    #[doc = "Bits 16:23 - Number of Increments"]
    #[inline(always)]
    pub fn nit(&mut self) -> NIT_W {
        NIT_W { w: self }
    }
}
