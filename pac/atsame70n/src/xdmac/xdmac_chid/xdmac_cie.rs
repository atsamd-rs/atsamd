#[doc = "Register `XDMAC_CIE` writer"]
pub struct W(crate::W<XDMAC_CIE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<XDMAC_CIE_SPEC>;
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
impl From<crate::W<XDMAC_CIE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<XDMAC_CIE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BIE` writer - End of Block Interrupt Enable Bit"]
pub struct BIE_W<'a> {
    w: &'a mut W,
}
impl<'a> BIE_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Field `LIE` writer - End of Linked List Interrupt Enable Bit"]
pub struct LIE_W<'a> {
    w: &'a mut W,
}
impl<'a> LIE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `DIE` writer - End of Disable Interrupt Enable Bit"]
pub struct DIE_W<'a> {
    w: &'a mut W,
}
impl<'a> DIE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `FIE` writer - End of Flush Interrupt Enable Bit"]
pub struct FIE_W<'a> {
    w: &'a mut W,
}
impl<'a> FIE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `RBIE` writer - Read Bus Error Interrupt Enable Bit"]
pub struct RBIE_W<'a> {
    w: &'a mut W,
}
impl<'a> RBIE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `WBIE` writer - Write Bus Error Interrupt Enable Bit"]
pub struct WBIE_W<'a> {
    w: &'a mut W,
}
impl<'a> WBIE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `ROIE` writer - Request Overflow Error Interrupt Enable Bit"]
pub struct ROIE_W<'a> {
    w: &'a mut W,
}
impl<'a> ROIE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
impl W {
    #[doc = "Bit 0 - End of Block Interrupt Enable Bit"]
    #[inline(always)]
    pub fn bie(&mut self) -> BIE_W {
        BIE_W { w: self }
    }
    #[doc = "Bit 1 - End of Linked List Interrupt Enable Bit"]
    #[inline(always)]
    pub fn lie(&mut self) -> LIE_W {
        LIE_W { w: self }
    }
    #[doc = "Bit 2 - End of Disable Interrupt Enable Bit"]
    #[inline(always)]
    pub fn die(&mut self) -> DIE_W {
        DIE_W { w: self }
    }
    #[doc = "Bit 3 - End of Flush Interrupt Enable Bit"]
    #[inline(always)]
    pub fn fie(&mut self) -> FIE_W {
        FIE_W { w: self }
    }
    #[doc = "Bit 4 - Read Bus Error Interrupt Enable Bit"]
    #[inline(always)]
    pub fn rbie(&mut self) -> RBIE_W {
        RBIE_W { w: self }
    }
    #[doc = "Bit 5 - Write Bus Error Interrupt Enable Bit"]
    #[inline(always)]
    pub fn wbie(&mut self) -> WBIE_W {
        WBIE_W { w: self }
    }
    #[doc = "Bit 6 - Request Overflow Error Interrupt Enable Bit"]
    #[inline(always)]
    pub fn roie(&mut self) -> ROIE_W {
        ROIE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel Interrupt Enable Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [xdmac_cie](index.html) module"]
pub struct XDMAC_CIE_SPEC;
impl crate::RegisterSpec for XDMAC_CIE_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [xdmac_cie::W](W) writer structure"]
impl crate::Writable for XDMAC_CIE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets XDMAC_CIE to value 0"]
impl crate::Resettable for XDMAC_CIE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
