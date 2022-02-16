#[doc = "Register `USBHS_HSTDMASTATUS` reader"]
pub struct R(crate::R<USBHS_HSTDMASTATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USBHS_HSTDMASTATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USBHS_HSTDMASTATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USBHS_HSTDMASTATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USBHS_HSTDMASTATUS` writer"]
pub struct W(crate::W<USBHS_HSTDMASTATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USBHS_HSTDMASTATUS_SPEC>;
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
impl From<crate::W<USBHS_HSTDMASTATUS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USBHS_HSTDMASTATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CHANN_ENB` reader - Channel Enable Status"]
pub struct CHANN_ENB_R(crate::FieldReader<bool, bool>);
impl CHANN_ENB_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CHANN_ENB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHANN_ENB_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHANN_ENB` writer - Channel Enable Status"]
pub struct CHANN_ENB_W<'a> {
    w: &'a mut W,
}
impl<'a> CHANN_ENB_W<'a> {
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
#[doc = "Field `CHANN_ACT` reader - Channel Active Status"]
pub struct CHANN_ACT_R(crate::FieldReader<bool, bool>);
impl CHANN_ACT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CHANN_ACT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHANN_ACT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHANN_ACT` writer - Channel Active Status"]
pub struct CHANN_ACT_W<'a> {
    w: &'a mut W,
}
impl<'a> CHANN_ACT_W<'a> {
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
#[doc = "Field `END_TR_ST` reader - End of Channel Transfer Status"]
pub struct END_TR_ST_R(crate::FieldReader<bool, bool>);
impl END_TR_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        END_TR_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for END_TR_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `END_TR_ST` writer - End of Channel Transfer Status"]
pub struct END_TR_ST_W<'a> {
    w: &'a mut W,
}
impl<'a> END_TR_ST_W<'a> {
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
#[doc = "Field `END_BF_ST` reader - End of Channel Buffer Status"]
pub struct END_BF_ST_R(crate::FieldReader<bool, bool>);
impl END_BF_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        END_BF_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for END_BF_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `END_BF_ST` writer - End of Channel Buffer Status"]
pub struct END_BF_ST_W<'a> {
    w: &'a mut W,
}
impl<'a> END_BF_ST_W<'a> {
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
#[doc = "Field `DESC_LDST` reader - Descriptor Loaded Status"]
pub struct DESC_LDST_R(crate::FieldReader<bool, bool>);
impl DESC_LDST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DESC_LDST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DESC_LDST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DESC_LDST` writer - Descriptor Loaded Status"]
pub struct DESC_LDST_W<'a> {
    w: &'a mut W,
}
impl<'a> DESC_LDST_W<'a> {
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
#[doc = "Field `BUFF_COUNT` reader - Buffer Byte Count"]
pub struct BUFF_COUNT_R(crate::FieldReader<u16, u16>);
impl BUFF_COUNT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        BUFF_COUNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BUFF_COUNT_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BUFF_COUNT` writer - Buffer Byte Count"]
pub struct BUFF_COUNT_W<'a> {
    w: &'a mut W,
}
impl<'a> BUFF_COUNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | ((value as u32 & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Channel Enable Status"]
    #[inline(always)]
    pub fn chann_enb(&self) -> CHANN_ENB_R {
        CHANN_ENB_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Channel Active Status"]
    #[inline(always)]
    pub fn chann_act(&self) -> CHANN_ACT_R {
        CHANN_ACT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 4 - End of Channel Transfer Status"]
    #[inline(always)]
    pub fn end_tr_st(&self) -> END_TR_ST_R {
        END_TR_ST_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - End of Channel Buffer Status"]
    #[inline(always)]
    pub fn end_bf_st(&self) -> END_BF_ST_R {
        END_BF_ST_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Descriptor Loaded Status"]
    #[inline(always)]
    pub fn desc_ldst(&self) -> DESC_LDST_R {
        DESC_LDST_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bits 16:31 - Buffer Byte Count"]
    #[inline(always)]
    pub fn buff_count(&self) -> BUFF_COUNT_R {
        BUFF_COUNT_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Channel Enable Status"]
    #[inline(always)]
    pub fn chann_enb(&mut self) -> CHANN_ENB_W {
        CHANN_ENB_W { w: self }
    }
    #[doc = "Bit 1 - Channel Active Status"]
    #[inline(always)]
    pub fn chann_act(&mut self) -> CHANN_ACT_W {
        CHANN_ACT_W { w: self }
    }
    #[doc = "Bit 4 - End of Channel Transfer Status"]
    #[inline(always)]
    pub fn end_tr_st(&mut self) -> END_TR_ST_W {
        END_TR_ST_W { w: self }
    }
    #[doc = "Bit 5 - End of Channel Buffer Status"]
    #[inline(always)]
    pub fn end_bf_st(&mut self) -> END_BF_ST_W {
        END_BF_ST_W { w: self }
    }
    #[doc = "Bit 6 - Descriptor Loaded Status"]
    #[inline(always)]
    pub fn desc_ldst(&mut self) -> DESC_LDST_W {
        DESC_LDST_W { w: self }
    }
    #[doc = "Bits 16:31 - Buffer Byte Count"]
    #[inline(always)]
    pub fn buff_count(&mut self) -> BUFF_COUNT_W {
        BUFF_COUNT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Host DMA Channel Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbhs_hstdmastatus](index.html) module"]
pub struct USBHS_HSTDMASTATUS_SPEC;
impl crate::RegisterSpec for USBHS_HSTDMASTATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [usbhs_hstdmastatus::R](R) reader structure"]
impl crate::Readable for USBHS_HSTDMASTATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usbhs_hstdmastatus::W](W) writer structure"]
impl crate::Writable for USBHS_HSTDMASTATUS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets USBHS_HSTDMASTATUS to value 0"]
impl crate::Resettable for USBHS_HSTDMASTATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
