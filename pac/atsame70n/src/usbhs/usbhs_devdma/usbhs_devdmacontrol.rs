#[doc = "Register `USBHS_DEVDMACONTROL` reader"]
pub struct R(crate::R<USBHS_DEVDMACONTROL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USBHS_DEVDMACONTROL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USBHS_DEVDMACONTROL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USBHS_DEVDMACONTROL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USBHS_DEVDMACONTROL` writer"]
pub struct W(crate::W<USBHS_DEVDMACONTROL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USBHS_DEVDMACONTROL_SPEC>;
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
impl From<crate::W<USBHS_DEVDMACONTROL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USBHS_DEVDMACONTROL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CHANN_ENB` reader - Channel Enable Command"]
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
#[doc = "Field `CHANN_ENB` writer - Channel Enable Command"]
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
#[doc = "Field `LDNXT_DSC` reader - Load Next Channel Transfer Descriptor Enable Command"]
pub struct LDNXT_DSC_R(crate::FieldReader<bool, bool>);
impl LDNXT_DSC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LDNXT_DSC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LDNXT_DSC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LDNXT_DSC` writer - Load Next Channel Transfer Descriptor Enable Command"]
pub struct LDNXT_DSC_W<'a> {
    w: &'a mut W,
}
impl<'a> LDNXT_DSC_W<'a> {
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
#[doc = "Field `END_TR_EN` reader - End of Transfer Enable Control (OUT transfers only)"]
pub struct END_TR_EN_R(crate::FieldReader<bool, bool>);
impl END_TR_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        END_TR_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for END_TR_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `END_TR_EN` writer - End of Transfer Enable Control (OUT transfers only)"]
pub struct END_TR_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> END_TR_EN_W<'a> {
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
#[doc = "Field `END_B_EN` reader - End of Buffer Enable Control"]
pub struct END_B_EN_R(crate::FieldReader<bool, bool>);
impl END_B_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        END_B_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for END_B_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `END_B_EN` writer - End of Buffer Enable Control"]
pub struct END_B_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> END_B_EN_W<'a> {
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
#[doc = "Field `END_TR_IT` reader - End of Transfer Interrupt Enable"]
pub struct END_TR_IT_R(crate::FieldReader<bool, bool>);
impl END_TR_IT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        END_TR_IT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for END_TR_IT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `END_TR_IT` writer - End of Transfer Interrupt Enable"]
pub struct END_TR_IT_W<'a> {
    w: &'a mut W,
}
impl<'a> END_TR_IT_W<'a> {
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
#[doc = "Field `END_BUFFIT` reader - End of Buffer Interrupt Enable"]
pub struct END_BUFFIT_R(crate::FieldReader<bool, bool>);
impl END_BUFFIT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        END_BUFFIT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for END_BUFFIT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `END_BUFFIT` writer - End of Buffer Interrupt Enable"]
pub struct END_BUFFIT_W<'a> {
    w: &'a mut W,
}
impl<'a> END_BUFFIT_W<'a> {
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
#[doc = "Field `DESC_LD_IT` reader - Descriptor Loaded Interrupt Enable"]
pub struct DESC_LD_IT_R(crate::FieldReader<bool, bool>);
impl DESC_LD_IT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DESC_LD_IT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DESC_LD_IT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DESC_LD_IT` writer - Descriptor Loaded Interrupt Enable"]
pub struct DESC_LD_IT_W<'a> {
    w: &'a mut W,
}
impl<'a> DESC_LD_IT_W<'a> {
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
#[doc = "Field `BURST_LCK` reader - Burst Lock Enable"]
pub struct BURST_LCK_R(crate::FieldReader<bool, bool>);
impl BURST_LCK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BURST_LCK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BURST_LCK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BURST_LCK` writer - Burst Lock Enable"]
pub struct BURST_LCK_W<'a> {
    w: &'a mut W,
}
impl<'a> BURST_LCK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Field `BUFF_LENGTH` reader - Buffer Byte Length (Write-only)"]
pub struct BUFF_LENGTH_R(crate::FieldReader<u16, u16>);
impl BUFF_LENGTH_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        BUFF_LENGTH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BUFF_LENGTH_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BUFF_LENGTH` writer - Buffer Byte Length (Write-only)"]
pub struct BUFF_LENGTH_W<'a> {
    w: &'a mut W,
}
impl<'a> BUFF_LENGTH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | ((value as u32 & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Channel Enable Command"]
    #[inline(always)]
    pub fn chann_enb(&self) -> CHANN_ENB_R {
        CHANN_ENB_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Load Next Channel Transfer Descriptor Enable Command"]
    #[inline(always)]
    pub fn ldnxt_dsc(&self) -> LDNXT_DSC_R {
        LDNXT_DSC_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - End of Transfer Enable Control (OUT transfers only)"]
    #[inline(always)]
    pub fn end_tr_en(&self) -> END_TR_EN_R {
        END_TR_EN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - End of Buffer Enable Control"]
    #[inline(always)]
    pub fn end_b_en(&self) -> END_B_EN_R {
        END_B_EN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - End of Transfer Interrupt Enable"]
    #[inline(always)]
    pub fn end_tr_it(&self) -> END_TR_IT_R {
        END_TR_IT_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - End of Buffer Interrupt Enable"]
    #[inline(always)]
    pub fn end_buffit(&self) -> END_BUFFIT_R {
        END_BUFFIT_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Descriptor Loaded Interrupt Enable"]
    #[inline(always)]
    pub fn desc_ld_it(&self) -> DESC_LD_IT_R {
        DESC_LD_IT_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Burst Lock Enable"]
    #[inline(always)]
    pub fn burst_lck(&self) -> BURST_LCK_R {
        BURST_LCK_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 16:31 - Buffer Byte Length (Write-only)"]
    #[inline(always)]
    pub fn buff_length(&self) -> BUFF_LENGTH_R {
        BUFF_LENGTH_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Channel Enable Command"]
    #[inline(always)]
    pub fn chann_enb(&mut self) -> CHANN_ENB_W {
        CHANN_ENB_W { w: self }
    }
    #[doc = "Bit 1 - Load Next Channel Transfer Descriptor Enable Command"]
    #[inline(always)]
    pub fn ldnxt_dsc(&mut self) -> LDNXT_DSC_W {
        LDNXT_DSC_W { w: self }
    }
    #[doc = "Bit 2 - End of Transfer Enable Control (OUT transfers only)"]
    #[inline(always)]
    pub fn end_tr_en(&mut self) -> END_TR_EN_W {
        END_TR_EN_W { w: self }
    }
    #[doc = "Bit 3 - End of Buffer Enable Control"]
    #[inline(always)]
    pub fn end_b_en(&mut self) -> END_B_EN_W {
        END_B_EN_W { w: self }
    }
    #[doc = "Bit 4 - End of Transfer Interrupt Enable"]
    #[inline(always)]
    pub fn end_tr_it(&mut self) -> END_TR_IT_W {
        END_TR_IT_W { w: self }
    }
    #[doc = "Bit 5 - End of Buffer Interrupt Enable"]
    #[inline(always)]
    pub fn end_buffit(&mut self) -> END_BUFFIT_W {
        END_BUFFIT_W { w: self }
    }
    #[doc = "Bit 6 - Descriptor Loaded Interrupt Enable"]
    #[inline(always)]
    pub fn desc_ld_it(&mut self) -> DESC_LD_IT_W {
        DESC_LD_IT_W { w: self }
    }
    #[doc = "Bit 7 - Burst Lock Enable"]
    #[inline(always)]
    pub fn burst_lck(&mut self) -> BURST_LCK_W {
        BURST_LCK_W { w: self }
    }
    #[doc = "Bits 16:31 - Buffer Byte Length (Write-only)"]
    #[inline(always)]
    pub fn buff_length(&mut self) -> BUFF_LENGTH_W {
        BUFF_LENGTH_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Device DMA Channel Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbhs_devdmacontrol](index.html) module"]
pub struct USBHS_DEVDMACONTROL_SPEC;
impl crate::RegisterSpec for USBHS_DEVDMACONTROL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [usbhs_devdmacontrol::R](R) reader structure"]
impl crate::Readable for USBHS_DEVDMACONTROL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usbhs_devdmacontrol::W](W) writer structure"]
impl crate::Writable for USBHS_DEVDMACONTROL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets USBHS_DEVDMACONTROL to value 0"]
impl crate::Resettable for USBHS_DEVDMACONTROL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
