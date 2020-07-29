#[doc = "Reader of register DEMCR"]
pub type R = crate::R<u32, super::DEMCR>;
#[doc = "Writer for register DEMCR"]
pub type W = crate::W<u32, super::DEMCR>;
#[doc = "Register DEMCR `reset()`'s with value 0"]
impl crate::ResetValue for super::DEMCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `VC_CORERESET`"]
pub type VC_CORERESET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VC_CORERESET`"]
pub struct VC_CORERESET_W<'a> {
    w: &'a mut W,
}
impl<'a> VC_CORERESET_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `VC_MMERR`"]
pub type VC_MMERR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VC_MMERR`"]
pub struct VC_MMERR_W<'a> {
    w: &'a mut W,
}
impl<'a> VC_MMERR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `VC_NOCPERR`"]
pub type VC_NOCPERR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VC_NOCPERR`"]
pub struct VC_NOCPERR_W<'a> {
    w: &'a mut W,
}
impl<'a> VC_NOCPERR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `VC_CHKERR`"]
pub type VC_CHKERR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VC_CHKERR`"]
pub struct VC_CHKERR_W<'a> {
    w: &'a mut W,
}
impl<'a> VC_CHKERR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `VC_STATERR`"]
pub type VC_STATERR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VC_STATERR`"]
pub struct VC_STATERR_W<'a> {
    w: &'a mut W,
}
impl<'a> VC_STATERR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `VC_BUSERR`"]
pub type VC_BUSERR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VC_BUSERR`"]
pub struct VC_BUSERR_W<'a> {
    w: &'a mut W,
}
impl<'a> VC_BUSERR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `VC_INTERR`"]
pub type VC_INTERR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VC_INTERR`"]
pub struct VC_INTERR_W<'a> {
    w: &'a mut W,
}
impl<'a> VC_INTERR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `VC_HARDERR`"]
pub type VC_HARDERR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VC_HARDERR`"]
pub struct VC_HARDERR_W<'a> {
    w: &'a mut W,
}
impl<'a> VC_HARDERR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `MON_EN`"]
pub type MON_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MON_EN`"]
pub struct MON_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> MON_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `MON_PEND`"]
pub type MON_PEND_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MON_PEND`"]
pub struct MON_PEND_W<'a> {
    w: &'a mut W,
}
impl<'a> MON_PEND_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Reader of field `MON_STEP`"]
pub type MON_STEP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MON_STEP`"]
pub struct MON_STEP_W<'a> {
    w: &'a mut W,
}
impl<'a> MON_STEP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Reader of field `MON_REQ`"]
pub type MON_REQ_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MON_REQ`"]
pub struct MON_REQ_W<'a> {
    w: &'a mut W,
}
impl<'a> MON_REQ_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Reader of field `TRCENA`"]
pub type TRCENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TRCENA`"]
pub struct TRCENA_W<'a> {
    w: &'a mut W,
}
impl<'a> TRCENA_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn vc_corereset(&self) -> VC_CORERESET_R {
        VC_CORERESET_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn vc_mmerr(&self) -> VC_MMERR_R {
        VC_MMERR_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn vc_nocperr(&self) -> VC_NOCPERR_R {
        VC_NOCPERR_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn vc_chkerr(&self) -> VC_CHKERR_R {
        VC_CHKERR_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn vc_staterr(&self) -> VC_STATERR_R {
        VC_STATERR_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn vc_buserr(&self) -> VC_BUSERR_R {
        VC_BUSERR_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn vc_interr(&self) -> VC_INTERR_R {
        VC_INTERR_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn vc_harderr(&self) -> VC_HARDERR_R {
        VC_HARDERR_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn mon_en(&self) -> MON_EN_R {
        MON_EN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn mon_pend(&self) -> MON_PEND_R {
        MON_PEND_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn mon_step(&self) -> MON_STEP_R {
        MON_STEP_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn mon_req(&self) -> MON_REQ_R {
        MON_REQ_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn trcena(&self) -> TRCENA_R {
        TRCENA_R::new(((self.bits >> 24) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn vc_corereset(&mut self) -> VC_CORERESET_W {
        VC_CORERESET_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn vc_mmerr(&mut self) -> VC_MMERR_W {
        VC_MMERR_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn vc_nocperr(&mut self) -> VC_NOCPERR_W {
        VC_NOCPERR_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn vc_chkerr(&mut self) -> VC_CHKERR_W {
        VC_CHKERR_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn vc_staterr(&mut self) -> VC_STATERR_W {
        VC_STATERR_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn vc_buserr(&mut self) -> VC_BUSERR_W {
        VC_BUSERR_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn vc_interr(&mut self) -> VC_INTERR_W {
        VC_INTERR_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn vc_harderr(&mut self) -> VC_HARDERR_W {
        VC_HARDERR_W { w: self }
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn mon_en(&mut self) -> MON_EN_W {
        MON_EN_W { w: self }
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn mon_pend(&mut self) -> MON_PEND_W {
        MON_PEND_W { w: self }
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn mon_step(&mut self) -> MON_STEP_W {
        MON_STEP_W { w: self }
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn mon_req(&mut self) -> MON_REQ_W {
        MON_REQ_W { w: self }
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn trcena(&mut self) -> TRCENA_W {
        TRCENA_W { w: self }
    }
}
