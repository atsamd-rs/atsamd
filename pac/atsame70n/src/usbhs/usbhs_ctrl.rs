#[doc = "Register `USBHS_CTRL` reader"]
pub struct R(crate::R<USBHS_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USBHS_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USBHS_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USBHS_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USBHS_CTRL` writer"]
pub struct W(crate::W<USBHS_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USBHS_CTRL_SPEC>;
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
impl From<crate::W<USBHS_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USBHS_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RDERRE` reader - Remote Device Connection Error Interrupt Enable"]
pub struct RDERRE_R(crate::FieldReader<bool, bool>);
impl RDERRE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RDERRE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RDERRE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RDERRE` writer - Remote Device Connection Error Interrupt Enable"]
pub struct RDERRE_W<'a> {
    w: &'a mut W,
}
impl<'a> RDERRE_W<'a> {
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
#[doc = "Field `VBUSHWC` reader - VBUS Hardware Control"]
pub struct VBUSHWC_R(crate::FieldReader<bool, bool>);
impl VBUSHWC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        VBUSHWC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VBUSHWC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VBUSHWC` writer - VBUS Hardware Control"]
pub struct VBUSHWC_W<'a> {
    w: &'a mut W,
}
impl<'a> VBUSHWC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `FRZCLK` reader - Freeze USB Clock"]
pub struct FRZCLK_R(crate::FieldReader<bool, bool>);
impl FRZCLK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FRZCLK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FRZCLK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FRZCLK` writer - Freeze USB Clock"]
pub struct FRZCLK_W<'a> {
    w: &'a mut W,
}
impl<'a> FRZCLK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
#[doc = "Field `USBE` reader - USBHS Enable"]
pub struct USBE_R(crate::FieldReader<bool, bool>);
impl USBE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        USBE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USBE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USBE` writer - USBHS Enable"]
pub struct USBE_W<'a> {
    w: &'a mut W,
}
impl<'a> USBE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
#[doc = "Field `UID` reader - UID Pin Enable"]
pub struct UID_R(crate::FieldReader<bool, bool>);
impl UID_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        UID_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UID_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UID` writer - UID Pin Enable"]
pub struct UID_W<'a> {
    w: &'a mut W,
}
impl<'a> UID_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
#[doc = "USBHS Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UIMOD_A {
    #[doc = "0: The module is in USB Host mode."]
    HOST = 0,
    #[doc = "1: The module is in USB Device mode."]
    DEVICE = 1,
}
impl From<UIMOD_A> for bool {
    #[inline(always)]
    fn from(variant: UIMOD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UIMOD` reader - USBHS Mode"]
pub struct UIMOD_R(crate::FieldReader<bool, UIMOD_A>);
impl UIMOD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        UIMOD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UIMOD_A {
        match self.bits {
            false => UIMOD_A::HOST,
            true => UIMOD_A::DEVICE,
        }
    }
    #[doc = "Checks if the value of the field is `HOST`"]
    #[inline(always)]
    pub fn is_host(&self) -> bool {
        **self == UIMOD_A::HOST
    }
    #[doc = "Checks if the value of the field is `DEVICE`"]
    #[inline(always)]
    pub fn is_device(&self) -> bool {
        **self == UIMOD_A::DEVICE
    }
}
impl core::ops::Deref for UIMOD_R {
    type Target = crate::FieldReader<bool, UIMOD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UIMOD` writer - USBHS Mode"]
pub struct UIMOD_W<'a> {
    w: &'a mut W,
}
impl<'a> UIMOD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UIMOD_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The module is in USB Host mode."]
    #[inline(always)]
    pub fn host(self) -> &'a mut W {
        self.variant(UIMOD_A::HOST)
    }
    #[doc = "The module is in USB Device mode."]
    #[inline(always)]
    pub fn device(self) -> &'a mut W {
        self.variant(UIMOD_A::DEVICE)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | ((value as u32 & 0x01) << 25);
        self.w
    }
}
impl R {
    #[doc = "Bit 4 - Remote Device Connection Error Interrupt Enable"]
    #[inline(always)]
    pub fn rderre(&self) -> RDERRE_R {
        RDERRE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 8 - VBUS Hardware Control"]
    #[inline(always)]
    pub fn vbushwc(&self) -> VBUSHWC_R {
        VBUSHWC_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Freeze USB Clock"]
    #[inline(always)]
    pub fn frzclk(&self) -> FRZCLK_R {
        FRZCLK_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - USBHS Enable"]
    #[inline(always)]
    pub fn usbe(&self) -> USBE_R {
        USBE_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 24 - UID Pin Enable"]
    #[inline(always)]
    pub fn uid(&self) -> UID_R {
        UID_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - USBHS Mode"]
    #[inline(always)]
    pub fn uimod(&self) -> UIMOD_R {
        UIMOD_R::new(((self.bits >> 25) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - Remote Device Connection Error Interrupt Enable"]
    #[inline(always)]
    pub fn rderre(&mut self) -> RDERRE_W {
        RDERRE_W { w: self }
    }
    #[doc = "Bit 8 - VBUS Hardware Control"]
    #[inline(always)]
    pub fn vbushwc(&mut self) -> VBUSHWC_W {
        VBUSHWC_W { w: self }
    }
    #[doc = "Bit 14 - Freeze USB Clock"]
    #[inline(always)]
    pub fn frzclk(&mut self) -> FRZCLK_W {
        FRZCLK_W { w: self }
    }
    #[doc = "Bit 15 - USBHS Enable"]
    #[inline(always)]
    pub fn usbe(&mut self) -> USBE_W {
        USBE_W { w: self }
    }
    #[doc = "Bit 24 - UID Pin Enable"]
    #[inline(always)]
    pub fn uid(&mut self) -> UID_W {
        UID_W { w: self }
    }
    #[doc = "Bit 25 - USBHS Mode"]
    #[inline(always)]
    pub fn uimod(&mut self) -> UIMOD_W {
        UIMOD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "General Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbhs_ctrl](index.html) module"]
pub struct USBHS_CTRL_SPEC;
impl crate::RegisterSpec for USBHS_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [usbhs_ctrl::R](R) reader structure"]
impl crate::Readable for USBHS_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usbhs_ctrl::W](W) writer structure"]
impl crate::Writable for USBHS_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets USBHS_CTRL to value 0"]
impl crate::Resettable for USBHS_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
