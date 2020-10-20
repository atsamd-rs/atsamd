#[doc = "Reader of register TPSF"]
pub type R = crate::R<u32, super::TPSF>;
#[doc = "Writer for register TPSF"]
pub type W = crate::W<u32, super::TPSF>;
#[doc = "Register TPSF `reset()`'s with value 0x03ff"]
impl crate::ResetValue for super::TPSF {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x03ff
    }
}
#[doc = "Reader of field `TPB1ADR`"]
pub type TPB1ADR_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `TPB1ADR`"]
pub struct TPB1ADR_W<'a> {
    w: &'a mut W,
}
impl<'a> TPB1ADR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | ((value as u32) & 0x03ff);
        self.w
    }
}
#[doc = "Reader of field `ENTXP`"]
pub type ENTXP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENTXP`"]
pub struct ENTXP_W<'a> {
    w: &'a mut W,
}
impl<'a> ENTXP_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:9 - TX packet buffer address"]
    #[inline(always)]
    pub fn tpb1adr(&self) -> TPB1ADR_R {
        TPB1ADR_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 31 - Enable TX partial store and forward operation"]
    #[inline(always)]
    pub fn entxp(&self) -> ENTXP_R {
        ENTXP_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9 - TX packet buffer address"]
    #[inline(always)]
    pub fn tpb1adr(&mut self) -> TPB1ADR_W {
        TPB1ADR_W { w: self }
    }
    #[doc = "Bit 31 - Enable TX partial store and forward operation"]
    #[inline(always)]
    pub fn entxp(&mut self) -> ENTXP_W {
        ENTXP_W { w: self }
    }
}
