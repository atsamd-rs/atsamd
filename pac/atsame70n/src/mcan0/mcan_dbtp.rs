#[doc = "Register `MCAN_DBTP` reader"]
pub struct R(crate::R<MCAN_DBTP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MCAN_DBTP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MCAN_DBTP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MCAN_DBTP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MCAN_DBTP` writer"]
pub struct W(crate::W<MCAN_DBTP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MCAN_DBTP_SPEC>;
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
impl From<crate::W<MCAN_DBTP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MCAN_DBTP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DSJW` reader - Data (Re) Synchronization Jump Width"]
pub struct DSJW_R(crate::FieldReader<u8, u8>);
impl DSJW_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DSJW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DSJW_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DSJW` writer - Data (Re) Synchronization Jump Width"]
pub struct DSJW_W<'a> {
    w: &'a mut W,
}
impl<'a> DSJW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
#[doc = "Field `DTSEG2` reader - Data Time Segment After Sample Point"]
pub struct DTSEG2_R(crate::FieldReader<u8, u8>);
impl DTSEG2_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DTSEG2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DTSEG2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DTSEG2` writer - Data Time Segment After Sample Point"]
pub struct DTSEG2_W<'a> {
    w: &'a mut W,
}
impl<'a> DTSEG2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | ((value as u32 & 0x0f) << 4);
        self.w
    }
}
#[doc = "Field `DTSEG1` reader - Data Time Segment Before Sample Point"]
pub struct DTSEG1_R(crate::FieldReader<u8, u8>);
impl DTSEG1_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DTSEG1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DTSEG1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DTSEG1` writer - Data Time Segment Before Sample Point"]
pub struct DTSEG1_W<'a> {
    w: &'a mut W,
}
impl<'a> DTSEG1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 8)) | ((value as u32 & 0x1f) << 8);
        self.w
    }
}
#[doc = "Field `DBRP` reader - Data Bit Rate Prescaler"]
pub struct DBRP_R(crate::FieldReader<u8, u8>);
impl DBRP_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DBRP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DBRP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DBRP` writer - Data Bit Rate Prescaler"]
pub struct DBRP_W<'a> {
    w: &'a mut W,
}
impl<'a> DBRP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 16)) | ((value as u32 & 0x1f) << 16);
        self.w
    }
}
#[doc = "Transmitter Delay Compensation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TDC_A {
    #[doc = "0: Transmitter Delay Compensation disabled."]
    DISABLED = 0,
    #[doc = "1: Transmitter Delay Compensation enabled."]
    ENABLED = 1,
}
impl From<TDC_A> for bool {
    #[inline(always)]
    fn from(variant: TDC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TDC` reader - Transmitter Delay Compensation"]
pub struct TDC_R(crate::FieldReader<bool, TDC_A>);
impl TDC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TDC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TDC_A {
        match self.bits {
            false => TDC_A::DISABLED,
            true => TDC_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == TDC_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == TDC_A::ENABLED
    }
}
impl core::ops::Deref for TDC_R {
    type Target = crate::FieldReader<bool, TDC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TDC` writer - Transmitter Delay Compensation"]
pub struct TDC_W<'a> {
    w: &'a mut W,
}
impl<'a> TDC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TDC_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Transmitter Delay Compensation disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TDC_A::DISABLED)
    }
    #[doc = "Transmitter Delay Compensation enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TDC_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | ((value as u32 & 0x01) << 23);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Data (Re) Synchronization Jump Width"]
    #[inline(always)]
    pub fn dsjw(&self) -> DSJW_R {
        DSJW_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 4:7 - Data Time Segment After Sample Point"]
    #[inline(always)]
    pub fn dtseg2(&self) -> DTSEG2_R {
        DTSEG2_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:12 - Data Time Segment Before Sample Point"]
    #[inline(always)]
    pub fn dtseg1(&self) -> DTSEG1_R {
        DTSEG1_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - Data Bit Rate Prescaler"]
    #[inline(always)]
    pub fn dbrp(&self) -> DBRP_R {
        DBRP_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bit 23 - Transmitter Delay Compensation"]
    #[inline(always)]
    pub fn tdc(&self) -> TDC_R {
        TDC_R::new(((self.bits >> 23) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Data (Re) Synchronization Jump Width"]
    #[inline(always)]
    pub fn dsjw(&mut self) -> DSJW_W {
        DSJW_W { w: self }
    }
    #[doc = "Bits 4:7 - Data Time Segment After Sample Point"]
    #[inline(always)]
    pub fn dtseg2(&mut self) -> DTSEG2_W {
        DTSEG2_W { w: self }
    }
    #[doc = "Bits 8:12 - Data Time Segment Before Sample Point"]
    #[inline(always)]
    pub fn dtseg1(&mut self) -> DTSEG1_W {
        DTSEG1_W { w: self }
    }
    #[doc = "Bits 16:20 - Data Bit Rate Prescaler"]
    #[inline(always)]
    pub fn dbrp(&mut self) -> DBRP_W {
        DBRP_W { w: self }
    }
    #[doc = "Bit 23 - Transmitter Delay Compensation"]
    #[inline(always)]
    pub fn tdc(&mut self) -> TDC_W {
        TDC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Data Bit Timing and Prescaler Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcan_dbtp](index.html) module"]
pub struct MCAN_DBTP_SPEC;
impl crate::RegisterSpec for MCAN_DBTP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mcan_dbtp::R](R) reader structure"]
impl crate::Readable for MCAN_DBTP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mcan_dbtp::W](W) writer structure"]
impl crate::Writable for MCAN_DBTP_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MCAN_DBTP to value 0"]
impl crate::Resettable for MCAN_DBTP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
