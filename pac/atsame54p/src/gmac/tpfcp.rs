#[doc = "Reader of register TPFCP"]
pub type R = crate::R<u32, super::TPFCP>;
#[doc = "Writer for register TPFCP"]
pub type W = crate::W<u32, super::TPFCP>;
#[doc = "Register TPFCP `reset()`'s with value 0"]
impl crate::ResetValue for super::TPFCP {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PEV`"]
pub type PEV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PEV`"]
pub struct PEV_W<'a> {
    w: &'a mut W,
}
impl<'a> PEV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `PQ`"]
pub type PQ_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PQ`"]
pub struct PQ_W<'a> {
    w: &'a mut W,
}
impl<'a> PQ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Priority Enable Vector"]
    #[inline(always)]
    pub fn pev(&self) -> PEV_R {
        PEV_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Pause Quantum"]
    #[inline(always)]
    pub fn pq(&self) -> PQ_R {
        PQ_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Priority Enable Vector"]
    #[inline(always)]
    pub fn pev(&mut self) -> PEV_W {
        PEV_W { w: self }
    }
    #[doc = "Bits 8:15 - Pause Quantum"]
    #[inline(always)]
    pub fn pq(&mut self) -> PQ_W {
        PQ_W { w: self }
    }
}
