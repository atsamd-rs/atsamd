#[doc = "Register `CTRLA` reader"]
pub struct R(crate::R<CTRLA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRLA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRLA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRLA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRLA` writer"]
pub struct W(crate::W<CTRLA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRLA_SPEC>;
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
impl From<crate::W<CTRLA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRLA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AUTOWS` reader - Auto Wait State Enable"]
pub struct AUTOWS_R(crate::FieldReader<bool, bool>);
impl AUTOWS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        AUTOWS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AUTOWS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AUTOWS` writer - Auto Wait State Enable"]
pub struct AUTOWS_W<'a> {
    w: &'a mut W,
}
impl<'a> AUTOWS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u16 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `SUSPEN` reader - Suspend Enable"]
pub struct SUSPEN_R(crate::FieldReader<bool, bool>);
impl SUSPEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SUSPEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SUSPEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SUSPEN` writer - Suspend Enable"]
pub struct SUSPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SUSPEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u16 & 0x01) << 3);
        self.w
    }
}
#[doc = "Write Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum WMODE_A {
    #[doc = "0: Manual Write"]
    MAN = 0,
    #[doc = "1: Automatic Double Word Write"]
    ADW = 1,
    #[doc = "2: Automatic Quad Word"]
    AQW = 2,
    #[doc = "3: Automatic Page Write"]
    AP = 3,
}
impl From<WMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: WMODE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `WMODE` reader - Write Mode"]
pub struct WMODE_R(crate::FieldReader<u8, WMODE_A>);
impl WMODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        WMODE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WMODE_A {
        match self.bits {
            0 => WMODE_A::MAN,
            1 => WMODE_A::ADW,
            2 => WMODE_A::AQW,
            3 => WMODE_A::AP,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `MAN`"]
    #[inline(always)]
    pub fn is_man(&self) -> bool {
        **self == WMODE_A::MAN
    }
    #[doc = "Checks if the value of the field is `ADW`"]
    #[inline(always)]
    pub fn is_adw(&self) -> bool {
        **self == WMODE_A::ADW
    }
    #[doc = "Checks if the value of the field is `AQW`"]
    #[inline(always)]
    pub fn is_aqw(&self) -> bool {
        **self == WMODE_A::AQW
    }
    #[doc = "Checks if the value of the field is `AP`"]
    #[inline(always)]
    pub fn is_ap(&self) -> bool {
        **self == WMODE_A::AP
    }
}
impl core::ops::Deref for WMODE_R {
    type Target = crate::FieldReader<u8, WMODE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WMODE` writer - Write Mode"]
pub struct WMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> WMODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WMODE_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Manual Write"]
    #[inline(always)]
    pub fn man(self) -> &'a mut W {
        self.variant(WMODE_A::MAN)
    }
    #[doc = "Automatic Double Word Write"]
    #[inline(always)]
    pub fn adw(self) -> &'a mut W {
        self.variant(WMODE_A::ADW)
    }
    #[doc = "Automatic Quad Word"]
    #[inline(always)]
    pub fn aqw(self) -> &'a mut W {
        self.variant(WMODE_A::AQW)
    }
    #[doc = "Automatic Page Write"]
    #[inline(always)]
    pub fn ap(self) -> &'a mut W {
        self.variant(WMODE_A::AP)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u16 & 0x03) << 4);
        self.w
    }
}
#[doc = "Power Reduction Mode during Sleep\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PRM_A {
    #[doc = "0: NVM block enters low-power mode when entering standby mode. NVM block enters low-power mode when SPRM command is issued. NVM block exits low-power mode upon first access."]
    SEMIAUTO = 0,
    #[doc = "1: NVM block enters low-power mode when entering standby mode. NVM block enters low-power mode when SPRM command is issued. NVM block exits low-power mode when system is not in standby mode."]
    FULLAUTO = 1,
    #[doc = "3: NVM block does not enter low-power mode when entering standby mode. NVM block enters low-power mode when SPRM command is issued. NVM block exits low-power mode upon first access."]
    MANUAL = 3,
}
impl From<PRM_A> for u8 {
    #[inline(always)]
    fn from(variant: PRM_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PRM` reader - Power Reduction Mode during Sleep"]
pub struct PRM_R(crate::FieldReader<u8, PRM_A>);
impl PRM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PRM_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PRM_A> {
        match self.bits {
            0 => Some(PRM_A::SEMIAUTO),
            1 => Some(PRM_A::FULLAUTO),
            3 => Some(PRM_A::MANUAL),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SEMIAUTO`"]
    #[inline(always)]
    pub fn is_semiauto(&self) -> bool {
        **self == PRM_A::SEMIAUTO
    }
    #[doc = "Checks if the value of the field is `FULLAUTO`"]
    #[inline(always)]
    pub fn is_fullauto(&self) -> bool {
        **self == PRM_A::FULLAUTO
    }
    #[doc = "Checks if the value of the field is `MANUAL`"]
    #[inline(always)]
    pub fn is_manual(&self) -> bool {
        **self == PRM_A::MANUAL
    }
}
impl core::ops::Deref for PRM_R {
    type Target = crate::FieldReader<u8, PRM_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRM` writer - Power Reduction Mode during Sleep"]
pub struct PRM_W<'a> {
    w: &'a mut W,
}
impl<'a> PRM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PRM_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "NVM block enters low-power mode when entering standby mode. NVM block enters low-power mode when SPRM command is issued. NVM block exits low-power mode upon first access."]
    #[inline(always)]
    pub fn semiauto(self) -> &'a mut W {
        self.variant(PRM_A::SEMIAUTO)
    }
    #[doc = "NVM block enters low-power mode when entering standby mode. NVM block enters low-power mode when SPRM command is issued. NVM block exits low-power mode when system is not in standby mode."]
    #[inline(always)]
    pub fn fullauto(self) -> &'a mut W {
        self.variant(PRM_A::FULLAUTO)
    }
    #[doc = "NVM block does not enter low-power mode when entering standby mode. NVM block enters low-power mode when SPRM command is issued. NVM block exits low-power mode upon first access."]
    #[inline(always)]
    pub fn manual(self) -> &'a mut W {
        self.variant(PRM_A::MANUAL)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | ((value as u16 & 0x03) << 6);
        self.w
    }
}
#[doc = "Field `RWS` reader - NVM Read Wait States"]
pub struct RWS_R(crate::FieldReader<u8, u8>);
impl RWS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RWS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RWS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RWS` writer - NVM Read Wait States"]
pub struct RWS_W<'a> {
    w: &'a mut W,
}
impl<'a> RWS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u16 & 0x0f) << 8);
        self.w
    }
}
#[doc = "Field `AHBNS0` reader - Force AHB0 access to NONSEQ, burst transfers are continuously rearbitrated"]
pub struct AHBNS0_R(crate::FieldReader<bool, bool>);
impl AHBNS0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        AHBNS0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AHBNS0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AHBNS0` writer - Force AHB0 access to NONSEQ, burst transfers are continuously rearbitrated"]
pub struct AHBNS0_W<'a> {
    w: &'a mut W,
}
impl<'a> AHBNS0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u16 & 0x01) << 12);
        self.w
    }
}
#[doc = "Field `AHBNS1` reader - Force AHB1 access to NONSEQ, burst transfers are continuously rearbitrated"]
pub struct AHBNS1_R(crate::FieldReader<bool, bool>);
impl AHBNS1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        AHBNS1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AHBNS1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AHBNS1` writer - Force AHB1 access to NONSEQ, burst transfers are continuously rearbitrated"]
pub struct AHBNS1_W<'a> {
    w: &'a mut W,
}
impl<'a> AHBNS1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u16 & 0x01) << 13);
        self.w
    }
}
#[doc = "Field `CACHEDIS0` reader - AHB0 Cache Disable"]
pub struct CACHEDIS0_R(crate::FieldReader<bool, bool>);
impl CACHEDIS0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CACHEDIS0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CACHEDIS0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CACHEDIS0` writer - AHB0 Cache Disable"]
pub struct CACHEDIS0_W<'a> {
    w: &'a mut W,
}
impl<'a> CACHEDIS0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u16 & 0x01) << 14);
        self.w
    }
}
#[doc = "Field `CACHEDIS1` reader - AHB1 Cache Disable"]
pub struct CACHEDIS1_R(crate::FieldReader<bool, bool>);
impl CACHEDIS1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CACHEDIS1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CACHEDIS1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CACHEDIS1` writer - AHB1 Cache Disable"]
pub struct CACHEDIS1_W<'a> {
    w: &'a mut W,
}
impl<'a> CACHEDIS1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u16 & 0x01) << 15);
        self.w
    }
}
impl R {
    #[doc = "Bit 2 - Auto Wait State Enable"]
    #[inline(always)]
    pub fn autows(&self) -> AUTOWS_R {
        AUTOWS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Suspend Enable"]
    #[inline(always)]
    pub fn suspen(&self) -> SUSPEN_R {
        SUSPEN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 4:5 - Write Mode"]
    #[inline(always)]
    pub fn wmode(&self) -> WMODE_R {
        WMODE_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - Power Reduction Mode during Sleep"]
    #[inline(always)]
    pub fn prm(&self) -> PRM_R {
        PRM_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 8:11 - NVM Read Wait States"]
    #[inline(always)]
    pub fn rws(&self) -> RWS_R {
        RWS_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - Force AHB0 access to NONSEQ, burst transfers are continuously rearbitrated"]
    #[inline(always)]
    pub fn ahbns0(&self) -> AHBNS0_R {
        AHBNS0_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Force AHB1 access to NONSEQ, burst transfers are continuously rearbitrated"]
    #[inline(always)]
    pub fn ahbns1(&self) -> AHBNS1_R {
        AHBNS1_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - AHB0 Cache Disable"]
    #[inline(always)]
    pub fn cachedis0(&self) -> CACHEDIS0_R {
        CACHEDIS0_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - AHB1 Cache Disable"]
    #[inline(always)]
    pub fn cachedis1(&self) -> CACHEDIS1_R {
        CACHEDIS1_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Auto Wait State Enable"]
    #[inline(always)]
    pub fn autows(&mut self) -> AUTOWS_W {
        AUTOWS_W { w: self }
    }
    #[doc = "Bit 3 - Suspend Enable"]
    #[inline(always)]
    pub fn suspen(&mut self) -> SUSPEN_W {
        SUSPEN_W { w: self }
    }
    #[doc = "Bits 4:5 - Write Mode"]
    #[inline(always)]
    pub fn wmode(&mut self) -> WMODE_W {
        WMODE_W { w: self }
    }
    #[doc = "Bits 6:7 - Power Reduction Mode during Sleep"]
    #[inline(always)]
    pub fn prm(&mut self) -> PRM_W {
        PRM_W { w: self }
    }
    #[doc = "Bits 8:11 - NVM Read Wait States"]
    #[inline(always)]
    pub fn rws(&mut self) -> RWS_W {
        RWS_W { w: self }
    }
    #[doc = "Bit 12 - Force AHB0 access to NONSEQ, burst transfers are continuously rearbitrated"]
    #[inline(always)]
    pub fn ahbns0(&mut self) -> AHBNS0_W {
        AHBNS0_W { w: self }
    }
    #[doc = "Bit 13 - Force AHB1 access to NONSEQ, burst transfers are continuously rearbitrated"]
    #[inline(always)]
    pub fn ahbns1(&mut self) -> AHBNS1_W {
        AHBNS1_W { w: self }
    }
    #[doc = "Bit 14 - AHB0 Cache Disable"]
    #[inline(always)]
    pub fn cachedis0(&mut self) -> CACHEDIS0_W {
        CACHEDIS0_W { w: self }
    }
    #[doc = "Bit 15 - AHB1 Cache Disable"]
    #[inline(always)]
    pub fn cachedis1(&mut self) -> CACHEDIS1_W {
        CACHEDIS1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control A\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrla](index.html) module"]
pub struct CTRLA_SPEC;
impl crate::RegisterSpec for CTRLA_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [ctrla::R](R) reader structure"]
impl crate::Readable for CTRLA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrla::W](W) writer structure"]
impl crate::Writable for CTRLA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTRLA to value 0x04"]
impl crate::Resettable for CTRLA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x04
    }
}
