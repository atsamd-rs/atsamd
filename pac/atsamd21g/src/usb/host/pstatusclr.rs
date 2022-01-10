#[doc = "Register `PSTATUSCLR%s` writer"]
pub struct W(crate::W<PSTATUSCLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PSTATUSCLR_SPEC>;
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
impl From<crate::W<PSTATUSCLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PSTATUSCLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CURBK` writer - Curren Bank clear"]
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
#[doc = "Field `PFREEZE` writer - Pipe Freeze Clear"]
pub struct PFREEZE_W<'a> {
    w: &'a mut W,
}
impl<'a> PFREEZE_W<'a> {
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
    #[doc = "Bit 2 - Curren Bank clear"]
    #[inline(always)]
    pub fn curbk(&mut self) -> CURBK_W {
        CURBK_W { w: self }
    }
    #[doc = "Bit 4 - Pipe Freeze Clear"]
    #[inline(always)]
    pub fn pfreeze(&mut self) -> PFREEZE_W {
        PFREEZE_W { w: self }
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
#[doc = "HOST End Point Pipe Status Clear\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pstatusclr](index.html) module"]
pub struct PSTATUSCLR_SPEC;
impl crate::RegisterSpec for PSTATUSCLR_SPEC {
    type Ux = u8;
}
#[doc = "`write(|w| ..)` method takes [pstatusclr::W](W) writer structure"]
impl crate::Writable for PSTATUSCLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PSTATUSCLR%s to value 0"]
impl crate::Resettable for PSTATUSCLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
