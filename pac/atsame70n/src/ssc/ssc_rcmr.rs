#[doc = "Register `SSC_RCMR` reader"]
pub struct R(crate::R<SSC_RCMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SSC_RCMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SSC_RCMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SSC_RCMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SSC_RCMR` writer"]
pub struct W(crate::W<SSC_RCMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SSC_RCMR_SPEC>;
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
impl From<crate::W<SSC_RCMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SSC_RCMR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Receive Clock Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CKS_A {
    #[doc = "0: Divided Clock"]
    MCK = 0,
    #[doc = "1: TK Clock signal"]
    TK = 1,
    #[doc = "2: RK pin"]
    RK = 2,
}
impl From<CKS_A> for u8 {
    #[inline(always)]
    fn from(variant: CKS_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CKS` reader - Receive Clock Selection"]
pub struct CKS_R(crate::FieldReader<u8, CKS_A>);
impl CKS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CKS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CKS_A> {
        match self.bits {
            0 => Some(CKS_A::MCK),
            1 => Some(CKS_A::TK),
            2 => Some(CKS_A::RK),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `MCK`"]
    #[inline(always)]
    pub fn is_mck(&self) -> bool {
        **self == CKS_A::MCK
    }
    #[doc = "Checks if the value of the field is `TK`"]
    #[inline(always)]
    pub fn is_tk(&self) -> bool {
        **self == CKS_A::TK
    }
    #[doc = "Checks if the value of the field is `RK`"]
    #[inline(always)]
    pub fn is_rk(&self) -> bool {
        **self == CKS_A::RK
    }
}
impl core::ops::Deref for CKS_R {
    type Target = crate::FieldReader<u8, CKS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CKS` writer - Receive Clock Selection"]
pub struct CKS_W<'a> {
    w: &'a mut W,
}
impl<'a> CKS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CKS_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Divided Clock"]
    #[inline(always)]
    pub fn mck(self) -> &'a mut W {
        self.variant(CKS_A::MCK)
    }
    #[doc = "TK Clock signal"]
    #[inline(always)]
    pub fn tk(self) -> &'a mut W {
        self.variant(CKS_A::TK)
    }
    #[doc = "RK pin"]
    #[inline(always)]
    pub fn rk(self) -> &'a mut W {
        self.variant(CKS_A::RK)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "Receive Clock Output Mode Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CKO_A {
    #[doc = "0: None, RK pin is an input"]
    NONE = 0,
    #[doc = "1: Continuous Receive Clock, RK pin is an output"]
    CONTINUOUS = 1,
    #[doc = "2: Receive Clock only during data transfers, RK pin is an output"]
    TRANSFER = 2,
}
impl From<CKO_A> for u8 {
    #[inline(always)]
    fn from(variant: CKO_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CKO` reader - Receive Clock Output Mode Selection"]
pub struct CKO_R(crate::FieldReader<u8, CKO_A>);
impl CKO_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CKO_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CKO_A> {
        match self.bits {
            0 => Some(CKO_A::NONE),
            1 => Some(CKO_A::CONTINUOUS),
            2 => Some(CKO_A::TRANSFER),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        **self == CKO_A::NONE
    }
    #[doc = "Checks if the value of the field is `CONTINUOUS`"]
    #[inline(always)]
    pub fn is_continuous(&self) -> bool {
        **self == CKO_A::CONTINUOUS
    }
    #[doc = "Checks if the value of the field is `TRANSFER`"]
    #[inline(always)]
    pub fn is_transfer(&self) -> bool {
        **self == CKO_A::TRANSFER
    }
}
impl core::ops::Deref for CKO_R {
    type Target = crate::FieldReader<u8, CKO_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CKO` writer - Receive Clock Output Mode Selection"]
pub struct CKO_W<'a> {
    w: &'a mut W,
}
impl<'a> CKO_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CKO_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "None, RK pin is an input"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(CKO_A::NONE)
    }
    #[doc = "Continuous Receive Clock, RK pin is an output"]
    #[inline(always)]
    pub fn continuous(self) -> &'a mut W {
        self.variant(CKO_A::CONTINUOUS)
    }
    #[doc = "Receive Clock only during data transfers, RK pin is an output"]
    #[inline(always)]
    pub fn transfer(self) -> &'a mut W {
        self.variant(CKO_A::TRANSFER)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 2)) | ((value as u32 & 0x07) << 2);
        self.w
    }
}
#[doc = "Field `CKI` reader - Receive Clock Inversion"]
pub struct CKI_R(crate::FieldReader<bool, bool>);
impl CKI_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CKI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CKI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CKI` writer - Receive Clock Inversion"]
pub struct CKI_W<'a> {
    w: &'a mut W,
}
impl<'a> CKI_W<'a> {
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
#[doc = "Receive Clock Gating Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CKG_A {
    #[doc = "0: None"]
    CONTINUOUS = 0,
    #[doc = "1: Receive Clock enabled only if RF Low"]
    EN_RF_LOW = 1,
    #[doc = "2: Receive Clock enabled only if RF High"]
    EN_RF_HIGH = 2,
}
impl From<CKG_A> for u8 {
    #[inline(always)]
    fn from(variant: CKG_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CKG` reader - Receive Clock Gating Selection"]
pub struct CKG_R(crate::FieldReader<u8, CKG_A>);
impl CKG_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CKG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CKG_A> {
        match self.bits {
            0 => Some(CKG_A::CONTINUOUS),
            1 => Some(CKG_A::EN_RF_LOW),
            2 => Some(CKG_A::EN_RF_HIGH),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `CONTINUOUS`"]
    #[inline(always)]
    pub fn is_continuous(&self) -> bool {
        **self == CKG_A::CONTINUOUS
    }
    #[doc = "Checks if the value of the field is `EN_RF_LOW`"]
    #[inline(always)]
    pub fn is_en_rf_low(&self) -> bool {
        **self == CKG_A::EN_RF_LOW
    }
    #[doc = "Checks if the value of the field is `EN_RF_HIGH`"]
    #[inline(always)]
    pub fn is_en_rf_high(&self) -> bool {
        **self == CKG_A::EN_RF_HIGH
    }
}
impl core::ops::Deref for CKG_R {
    type Target = crate::FieldReader<u8, CKG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CKG` writer - Receive Clock Gating Selection"]
pub struct CKG_W<'a> {
    w: &'a mut W,
}
impl<'a> CKG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CKG_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "None"]
    #[inline(always)]
    pub fn continuous(self) -> &'a mut W {
        self.variant(CKG_A::CONTINUOUS)
    }
    #[doc = "Receive Clock enabled only if RF Low"]
    #[inline(always)]
    pub fn en_rf_low(self) -> &'a mut W {
        self.variant(CKG_A::EN_RF_LOW)
    }
    #[doc = "Receive Clock enabled only if RF High"]
    #[inline(always)]
    pub fn en_rf_high(self) -> &'a mut W {
        self.variant(CKG_A::EN_RF_HIGH)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | ((value as u32 & 0x03) << 6);
        self.w
    }
}
#[doc = "Receive Start Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum START_A {
    #[doc = "0: Continuous, as soon as the receiver is enabled, and immediately after the end of transfer of the previous data."]
    CONTINUOUS = 0,
    #[doc = "1: Transmit start"]
    TRANSMIT = 1,
    #[doc = "2: Detection of a low level on RF signal"]
    RF_LOW = 2,
    #[doc = "3: Detection of a high level on RF signal"]
    RF_HIGH = 3,
    #[doc = "4: Detection of a falling edge on RF signal"]
    RF_FALLING = 4,
    #[doc = "5: Detection of a rising edge on RF signal"]
    RF_RISING = 5,
    #[doc = "6: Detection of any level change on RF signal"]
    RF_LEVEL = 6,
    #[doc = "7: Detection of any edge on RF signal"]
    RF_EDGE = 7,
    #[doc = "8: Compare 0"]
    CMP_0 = 8,
}
impl From<START_A> for u8 {
    #[inline(always)]
    fn from(variant: START_A) -> Self {
        variant as _
    }
}
#[doc = "Field `START` reader - Receive Start Selection"]
pub struct START_R(crate::FieldReader<u8, START_A>);
impl START_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        START_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<START_A> {
        match self.bits {
            0 => Some(START_A::CONTINUOUS),
            1 => Some(START_A::TRANSMIT),
            2 => Some(START_A::RF_LOW),
            3 => Some(START_A::RF_HIGH),
            4 => Some(START_A::RF_FALLING),
            5 => Some(START_A::RF_RISING),
            6 => Some(START_A::RF_LEVEL),
            7 => Some(START_A::RF_EDGE),
            8 => Some(START_A::CMP_0),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `CONTINUOUS`"]
    #[inline(always)]
    pub fn is_continuous(&self) -> bool {
        **self == START_A::CONTINUOUS
    }
    #[doc = "Checks if the value of the field is `TRANSMIT`"]
    #[inline(always)]
    pub fn is_transmit(&self) -> bool {
        **self == START_A::TRANSMIT
    }
    #[doc = "Checks if the value of the field is `RF_LOW`"]
    #[inline(always)]
    pub fn is_rf_low(&self) -> bool {
        **self == START_A::RF_LOW
    }
    #[doc = "Checks if the value of the field is `RF_HIGH`"]
    #[inline(always)]
    pub fn is_rf_high(&self) -> bool {
        **self == START_A::RF_HIGH
    }
    #[doc = "Checks if the value of the field is `RF_FALLING`"]
    #[inline(always)]
    pub fn is_rf_falling(&self) -> bool {
        **self == START_A::RF_FALLING
    }
    #[doc = "Checks if the value of the field is `RF_RISING`"]
    #[inline(always)]
    pub fn is_rf_rising(&self) -> bool {
        **self == START_A::RF_RISING
    }
    #[doc = "Checks if the value of the field is `RF_LEVEL`"]
    #[inline(always)]
    pub fn is_rf_level(&self) -> bool {
        **self == START_A::RF_LEVEL
    }
    #[doc = "Checks if the value of the field is `RF_EDGE`"]
    #[inline(always)]
    pub fn is_rf_edge(&self) -> bool {
        **self == START_A::RF_EDGE
    }
    #[doc = "Checks if the value of the field is `CMP_0`"]
    #[inline(always)]
    pub fn is_cmp_0(&self) -> bool {
        **self == START_A::CMP_0
    }
}
impl core::ops::Deref for START_R {
    type Target = crate::FieldReader<u8, START_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `START` writer - Receive Start Selection"]
pub struct START_W<'a> {
    w: &'a mut W,
}
impl<'a> START_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: START_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Continuous, as soon as the receiver is enabled, and immediately after the end of transfer of the previous data."]
    #[inline(always)]
    pub fn continuous(self) -> &'a mut W {
        self.variant(START_A::CONTINUOUS)
    }
    #[doc = "Transmit start"]
    #[inline(always)]
    pub fn transmit(self) -> &'a mut W {
        self.variant(START_A::TRANSMIT)
    }
    #[doc = "Detection of a low level on RF signal"]
    #[inline(always)]
    pub fn rf_low(self) -> &'a mut W {
        self.variant(START_A::RF_LOW)
    }
    #[doc = "Detection of a high level on RF signal"]
    #[inline(always)]
    pub fn rf_high(self) -> &'a mut W {
        self.variant(START_A::RF_HIGH)
    }
    #[doc = "Detection of a falling edge on RF signal"]
    #[inline(always)]
    pub fn rf_falling(self) -> &'a mut W {
        self.variant(START_A::RF_FALLING)
    }
    #[doc = "Detection of a rising edge on RF signal"]
    #[inline(always)]
    pub fn rf_rising(self) -> &'a mut W {
        self.variant(START_A::RF_RISING)
    }
    #[doc = "Detection of any level change on RF signal"]
    #[inline(always)]
    pub fn rf_level(self) -> &'a mut W {
        self.variant(START_A::RF_LEVEL)
    }
    #[doc = "Detection of any edge on RF signal"]
    #[inline(always)]
    pub fn rf_edge(self) -> &'a mut W {
        self.variant(START_A::RF_EDGE)
    }
    #[doc = "Compare 0"]
    #[inline(always)]
    pub fn cmp_0(self) -> &'a mut W {
        self.variant(START_A::CMP_0)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u32 & 0x0f) << 8);
        self.w
    }
}
#[doc = "Field `STOP` reader - Receive Stop Selection"]
pub struct STOP_R(crate::FieldReader<bool, bool>);
impl STOP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        STOP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STOP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STOP` writer - Receive Stop Selection"]
pub struct STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> STOP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "Field `STTDLY` reader - Receive Start Delay"]
pub struct STTDLY_R(crate::FieldReader<u8, u8>);
impl STTDLY_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        STTDLY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STTDLY_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STTDLY` writer - Receive Start Delay"]
pub struct STTDLY_W<'a> {
    w: &'a mut W,
}
impl<'a> STTDLY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
#[doc = "Field `PERIOD` reader - Receive Period Divider Selection"]
pub struct PERIOD_R(crate::FieldReader<u8, u8>);
impl PERIOD_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PERIOD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PERIOD_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PERIOD` writer - Receive Period Divider Selection"]
pub struct PERIOD_W<'a> {
    w: &'a mut W,
}
impl<'a> PERIOD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | ((value as u32 & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Receive Clock Selection"]
    #[inline(always)]
    pub fn cks(&self) -> CKS_R {
        CKS_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:4 - Receive Clock Output Mode Selection"]
    #[inline(always)]
    pub fn cko(&self) -> CKO_R {
        CKO_R::new(((self.bits >> 2) & 0x07) as u8)
    }
    #[doc = "Bit 5 - Receive Clock Inversion"]
    #[inline(always)]
    pub fn cki(&self) -> CKI_R {
        CKI_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bits 6:7 - Receive Clock Gating Selection"]
    #[inline(always)]
    pub fn ckg(&self) -> CKG_R {
        CKG_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 8:11 - Receive Start Selection"]
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - Receive Stop Selection"]
    #[inline(always)]
    pub fn stop(&self) -> STOP_R {
        STOP_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 16:23 - Receive Start Delay"]
    #[inline(always)]
    pub fn sttdly(&self) -> STTDLY_R {
        STTDLY_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Receive Period Divider Selection"]
    #[inline(always)]
    pub fn period(&self) -> PERIOD_R {
        PERIOD_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Receive Clock Selection"]
    #[inline(always)]
    pub fn cks(&mut self) -> CKS_W {
        CKS_W { w: self }
    }
    #[doc = "Bits 2:4 - Receive Clock Output Mode Selection"]
    #[inline(always)]
    pub fn cko(&mut self) -> CKO_W {
        CKO_W { w: self }
    }
    #[doc = "Bit 5 - Receive Clock Inversion"]
    #[inline(always)]
    pub fn cki(&mut self) -> CKI_W {
        CKI_W { w: self }
    }
    #[doc = "Bits 6:7 - Receive Clock Gating Selection"]
    #[inline(always)]
    pub fn ckg(&mut self) -> CKG_W {
        CKG_W { w: self }
    }
    #[doc = "Bits 8:11 - Receive Start Selection"]
    #[inline(always)]
    pub fn start(&mut self) -> START_W {
        START_W { w: self }
    }
    #[doc = "Bit 12 - Receive Stop Selection"]
    #[inline(always)]
    pub fn stop(&mut self) -> STOP_W {
        STOP_W { w: self }
    }
    #[doc = "Bits 16:23 - Receive Start Delay"]
    #[inline(always)]
    pub fn sttdly(&mut self) -> STTDLY_W {
        STTDLY_W { w: self }
    }
    #[doc = "Bits 24:31 - Receive Period Divider Selection"]
    #[inline(always)]
    pub fn period(&mut self) -> PERIOD_W {
        PERIOD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Receive Clock Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ssc_rcmr](index.html) module"]
pub struct SSC_RCMR_SPEC;
impl crate::RegisterSpec for SSC_RCMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ssc_rcmr::R](R) reader structure"]
impl crate::Readable for SSC_RCMR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ssc_rcmr::W](W) writer structure"]
impl crate::Writable for SSC_RCMR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SSC_RCMR to value 0"]
impl crate::Resettable for SSC_RCMR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
