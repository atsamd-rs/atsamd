#[doc = "Register `XDMAC_CID` writer"]
pub struct W(crate::W<XDMAC_CID_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<XDMAC_CID_SPEC>;
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
impl From<crate::W<XDMAC_CID_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<XDMAC_CID_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BID` writer - End of Block Interrupt Disable Bit"]
pub struct BID_W<'a> {
    w: &'a mut W,
}
impl<'a> BID_W<'a> {
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
#[doc = "Field `LID` writer - End of Linked List Interrupt Disable Bit"]
pub struct LID_W<'a> {
    w: &'a mut W,
}
impl<'a> LID_W<'a> {
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
#[doc = "Field `DID` writer - End of Disable Interrupt Disable Bit"]
pub struct DID_W<'a> {
    w: &'a mut W,
}
impl<'a> DID_W<'a> {
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
#[doc = "Field `FID` writer - End of Flush Interrupt Disable Bit"]
pub struct FID_W<'a> {
    w: &'a mut W,
}
impl<'a> FID_W<'a> {
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
#[doc = "Field `RBEID` writer - Read Bus Error Interrupt Disable Bit"]
pub struct RBEID_W<'a> {
    w: &'a mut W,
}
impl<'a> RBEID_W<'a> {
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
#[doc = "Field `WBEID` writer - Write Bus Error Interrupt Disable Bit"]
pub struct WBEID_W<'a> {
    w: &'a mut W,
}
impl<'a> WBEID_W<'a> {
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
#[doc = "Field `ROID` writer - Request Overflow Error Interrupt Disable Bit"]
pub struct ROID_W<'a> {
    w: &'a mut W,
}
impl<'a> ROID_W<'a> {
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
    #[doc = "Bit 0 - End of Block Interrupt Disable Bit"]
    #[inline(always)]
    pub fn bid(&mut self) -> BID_W {
        BID_W { w: self }
    }
    #[doc = "Bit 1 - End of Linked List Interrupt Disable Bit"]
    #[inline(always)]
    pub fn lid(&mut self) -> LID_W {
        LID_W { w: self }
    }
    #[doc = "Bit 2 - End of Disable Interrupt Disable Bit"]
    #[inline(always)]
    pub fn did(&mut self) -> DID_W {
        DID_W { w: self }
    }
    #[doc = "Bit 3 - End of Flush Interrupt Disable Bit"]
    #[inline(always)]
    pub fn fid(&mut self) -> FID_W {
        FID_W { w: self }
    }
    #[doc = "Bit 4 - Read Bus Error Interrupt Disable Bit"]
    #[inline(always)]
    pub fn rbeid(&mut self) -> RBEID_W {
        RBEID_W { w: self }
    }
    #[doc = "Bit 5 - Write Bus Error Interrupt Disable Bit"]
    #[inline(always)]
    pub fn wbeid(&mut self) -> WBEID_W {
        WBEID_W { w: self }
    }
    #[doc = "Bit 6 - Request Overflow Error Interrupt Disable Bit"]
    #[inline(always)]
    pub fn roid(&mut self) -> ROID_W {
        ROID_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel Interrupt Disable Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [xdmac_cid](index.html) module"]
pub struct XDMAC_CID_SPEC;
impl crate::RegisterSpec for XDMAC_CID_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [xdmac_cid::W](W) writer structure"]
impl crate::Writable for XDMAC_CID_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets XDMAC_CID to value 0"]
impl crate::Resettable for XDMAC_CID_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
