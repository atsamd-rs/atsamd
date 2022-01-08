#[doc = "Register `EPSTATUSCLR` writer"]
pub struct W(crate::W<EPSTATUSCLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EPSTATUSCLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<EPSTATUSCLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EPSTATUSCLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DTGLOUT` writer - Data Toggle OUT Clear"]
pub struct DTGLOUT_W<'a> {
    w: &'a mut W,
}
impl<'a> DTGLOUT_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | (value as u8 & 0x01);
        self.w
    }
}
#[doc = "Field `DTGLIN` writer - Data Toggle IN Clear"]
pub struct DTGLIN_W<'a> {
    w: &'a mut W,
}
impl<'a> DTGLIN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u8 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `CURBK` writer - Current Bank Clear"]
pub struct CURBK_W<'a> {
    w: &'a mut W,
}
impl<'a> CURBK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u8 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `STALLRQ0` writer - Stall 0 Request Clear"]
pub struct STALLRQ0_W<'a> {
    w: &'a mut W,
}
impl<'a> STALLRQ0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u8 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `STALLRQ1` writer - Stall 1 Request Clear"]
pub struct STALLRQ1_W<'a> {
    w: &'a mut W,
}
impl<'a> STALLRQ1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u8 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `BK0RDY` writer - Bank 0 Ready Clear"]
pub struct BK0RDY_W<'a> {
    w: &'a mut W,
}
impl<'a> BK0RDY_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u8 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `BK1RDY` writer - Bank 1 Ready Clear"]
pub struct BK1RDY_W<'a> {
    w: &'a mut W,
}
impl<'a> BK1RDY_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u8 & 0x01) << 7);
        self.w
    }
}
impl W {
    #[doc = "Bit 0 - Data Toggle OUT Clear"]
    #[inline(always)]
    pub fn dtglout(&mut self) -> DTGLOUT_W {
        DTGLOUT_W { w: self }
    }
    #[doc = "Bit 1 - Data Toggle IN Clear"]
    #[inline(always)]
    pub fn dtglin(&mut self) -> DTGLIN_W {
        DTGLIN_W { w: self }
    }
    #[doc = "Bit 2 - Current Bank Clear"]
    #[inline(always)]
    pub fn curbk(&mut self) -> CURBK_W {
        CURBK_W { w: self }
    }
    #[doc = "Bit 4 - Stall 0 Request Clear"]
    #[inline(always)]
    pub fn stallrq0(&mut self) -> STALLRQ0_W {
        STALLRQ0_W { w: self }
    }
    #[doc = "Bit 5 - Stall 1 Request Clear"]
    #[inline(always)]
    pub fn stallrq1(&mut self) -> STALLRQ1_W {
        STALLRQ1_W { w: self }
    }
    #[doc = "Bit 6 - Bank 0 Ready Clear"]
    #[inline(always)]
    pub fn bk0rdy(&mut self) -> BK0RDY_W {
        BK0RDY_W { w: self }
    }
    #[doc = "Bit 7 - Bank 1 Ready Clear"]
    #[inline(always)]
    pub fn bk1rdy(&mut self) -> BK1RDY_W {
        BK1RDY_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DEVICE_ENDPOINT End Point Pipe Status Clear\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [epstatusclr](index.html) module"]
pub struct EPSTATUSCLR_SPEC;
impl crate::RegisterSpec for EPSTATUSCLR_SPEC {
    type Ux = u8;
}
#[doc = "`write(|w| ..)` method takes [epstatusclr::W](W) writer structure"]
impl crate::Writable for EPSTATUSCLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EPSTATUSCLR to value 0"]
impl crate::Resettable for EPSTATUSCLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
