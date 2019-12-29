#[doc = "Reader of register RPSF"]
pub type R = crate::R<u32, super::RPSF>;
#[doc = "Writer for register RPSF"]
pub type W = crate::W<u32, super::RPSF>;
#[doc = "Register RPSF `reset()`'s with value 0x03ff"]
impl crate::ResetValue for super::RPSF {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x03ff
    }
}
#[doc = "Reader of field `RPB1ADR`"]
pub type RPB1ADR_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `RPB1ADR`"]
pub struct RPB1ADR_W<'a> {
    w: &'a mut W,
}
impl<'a> RPB1ADR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | ((value as u32) & 0x03ff);
        self.w
    }
}
#[doc = "Reader of field `ENRXP`"]
pub type ENRXP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENRXP`"]
pub struct ENRXP_W<'a> {
    w: &'a mut W,
}
impl<'a> ENRXP_W<'a> {
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
    #[doc = "Bits 0:9 - RX packet buffer address"]
    #[inline(always)]
    pub fn rpb1adr(&self) -> RPB1ADR_R {
        RPB1ADR_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 31 - Enable RX partial store and forward operation"]
    #[inline(always)]
    pub fn enrxp(&self) -> ENRXP_R {
        ENRXP_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9 - RX packet buffer address"]
    #[inline(always)]
    pub fn rpb1adr(&mut self) -> RPB1ADR_W {
        RPB1ADR_W { w: self }
    }
    #[doc = "Bit 31 - Enable RX partial store and forward operation"]
    #[inline(always)]
    pub fn enrxp(&mut self) -> ENRXP_W {
        ENRXP_W { w: self }
    }
}
