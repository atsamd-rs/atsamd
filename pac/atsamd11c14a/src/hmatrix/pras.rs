#[doc = "Reader of register PRAS%s"]
pub type R = crate::R<u32, super::PRAS>;
#[doc = "Writer for register PRAS%s"]
pub type W = crate::W<u32, super::PRAS>;
#[doc = "Register PRAS%s `reset()`'s with value 0"]
impl crate::ResetValue for super::PRAS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `M0PR`"]
pub type M0PR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `M0PR`"]
pub struct M0PR_W<'a> {
    w: &'a mut W,
}
impl<'a> M0PR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `M1PR`"]
pub type M1PR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `M1PR`"]
pub struct M1PR_W<'a> {
    w: &'a mut W,
}
impl<'a> M1PR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Reader of field `M2PR`"]
pub type M2PR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `M2PR`"]
pub struct M2PR_W<'a> {
    w: &'a mut W,
}
impl<'a> M2PR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Reader of field `M3PR`"]
pub type M3PR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `M3PR`"]
pub struct M3PR_W<'a> {
    w: &'a mut W,
}
impl<'a> M3PR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u32) & 0x0f) << 12);
        self.w
    }
}
#[doc = "Reader of field `M4PR`"]
pub type M4PR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `M4PR`"]
pub struct M4PR_W<'a> {
    w: &'a mut W,
}
impl<'a> M4PR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "Reader of field `M5PR`"]
pub type M5PR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `M5PR`"]
pub struct M5PR_W<'a> {
    w: &'a mut W,
}
impl<'a> M5PR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | (((value as u32) & 0x0f) << 20);
        self.w
    }
}
#[doc = "Reader of field `M6PR`"]
pub type M6PR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `M6PR`"]
pub struct M6PR_W<'a> {
    w: &'a mut W,
}
impl<'a> M6PR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | (((value as u32) & 0x0f) << 24);
        self.w
    }
}
#[doc = "Reader of field `M7PR`"]
pub type M7PR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `M7PR`"]
pub struct M7PR_W<'a> {
    w: &'a mut W,
}
impl<'a> M7PR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | (((value as u32) & 0x0f) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Master 0 Priority"]
    #[inline(always)]
    pub fn m0pr(&self) -> M0PR_R {
        M0PR_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Master 1 Priority"]
    #[inline(always)]
    pub fn m1pr(&self) -> M1PR_R {
        M1PR_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Master 2 Priority"]
    #[inline(always)]
    pub fn m2pr(&self) -> M2PR_R {
        M2PR_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Master 3 Priority"]
    #[inline(always)]
    pub fn m3pr(&self) -> M3PR_R {
        M3PR_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Master 4 Priority"]
    #[inline(always)]
    pub fn m4pr(&self) -> M4PR_R {
        M4PR_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Master 5 Priority"]
    #[inline(always)]
    pub fn m5pr(&self) -> M5PR_R {
        M5PR_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Master 6 Priority"]
    #[inline(always)]
    pub fn m6pr(&self) -> M6PR_R {
        M6PR_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Master 7 Priority"]
    #[inline(always)]
    pub fn m7pr(&self) -> M7PR_R {
        M7PR_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Master 0 Priority"]
    #[inline(always)]
    pub fn m0pr(&mut self) -> M0PR_W {
        M0PR_W { w: self }
    }
    #[doc = "Bits 4:7 - Master 1 Priority"]
    #[inline(always)]
    pub fn m1pr(&mut self) -> M1PR_W {
        M1PR_W { w: self }
    }
    #[doc = "Bits 8:11 - Master 2 Priority"]
    #[inline(always)]
    pub fn m2pr(&mut self) -> M2PR_W {
        M2PR_W { w: self }
    }
    #[doc = "Bits 12:15 - Master 3 Priority"]
    #[inline(always)]
    pub fn m3pr(&mut self) -> M3PR_W {
        M3PR_W { w: self }
    }
    #[doc = "Bits 16:19 - Master 4 Priority"]
    #[inline(always)]
    pub fn m4pr(&mut self) -> M4PR_W {
        M4PR_W { w: self }
    }
    #[doc = "Bits 20:23 - Master 5 Priority"]
    #[inline(always)]
    pub fn m5pr(&mut self) -> M5PR_W {
        M5PR_W { w: self }
    }
    #[doc = "Bits 24:27 - Master 6 Priority"]
    #[inline(always)]
    pub fn m6pr(&mut self) -> M6PR_W {
        M6PR_W { w: self }
    }
    #[doc = "Bits 28:31 - Master 7 Priority"]
    #[inline(always)]
    pub fn m7pr(&mut self) -> M7PR_W {
        M7PR_W { w: self }
    }
}
