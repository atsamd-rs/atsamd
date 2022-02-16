#[doc = "Register `MCAN_CCCR` reader"]
pub struct R(crate::R<MCAN_CCCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MCAN_CCCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MCAN_CCCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MCAN_CCCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MCAN_CCCR` writer"]
pub struct W(crate::W<MCAN_CCCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MCAN_CCCR_SPEC>;
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
impl From<crate::W<MCAN_CCCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MCAN_CCCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Initialization (read/write)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INIT_A {
    #[doc = "0: Normal operation."]
    DISABLED = 0,
    #[doc = "1: Initialization is started."]
    ENABLED = 1,
}
impl From<INIT_A> for bool {
    #[inline(always)]
    fn from(variant: INIT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INIT` reader - Initialization (read/write)"]
pub struct INIT_R(crate::FieldReader<bool, INIT_A>);
impl INIT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        INIT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INIT_A {
        match self.bits {
            false => INIT_A::DISABLED,
            true => INIT_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == INIT_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == INIT_A::ENABLED
    }
}
impl core::ops::Deref for INIT_R {
    type Target = crate::FieldReader<bool, INIT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INIT` writer - Initialization (read/write)"]
pub struct INIT_W<'a> {
    w: &'a mut W,
}
impl<'a> INIT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INIT_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Normal operation."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(INIT_A::DISABLED)
    }
    #[doc = "Initialization is started."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(INIT_A::ENABLED)
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Configuration Change Enable (read/write, write protection)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCE_A {
    #[doc = "0: The processor has no write access to the protected configuration registers."]
    PROTECTED = 0,
    #[doc = "1: The processor has write access to the protected configuration registers (while MCAN_CCCR.INIT = '1')."]
    CONFIGURABLE = 1,
}
impl From<CCE_A> for bool {
    #[inline(always)]
    fn from(variant: CCE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCE` reader - Configuration Change Enable (read/write, write protection)"]
pub struct CCE_R(crate::FieldReader<bool, CCE_A>);
impl CCE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CCE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CCE_A {
        match self.bits {
            false => CCE_A::PROTECTED,
            true => CCE_A::CONFIGURABLE,
        }
    }
    #[doc = "Checks if the value of the field is `PROTECTED`"]
    #[inline(always)]
    pub fn is_protected(&self) -> bool {
        **self == CCE_A::PROTECTED
    }
    #[doc = "Checks if the value of the field is `CONFIGURABLE`"]
    #[inline(always)]
    pub fn is_configurable(&self) -> bool {
        **self == CCE_A::CONFIGURABLE
    }
}
impl core::ops::Deref for CCE_R {
    type Target = crate::FieldReader<bool, CCE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CCE` writer - Configuration Change Enable (read/write, write protection)"]
pub struct CCE_W<'a> {
    w: &'a mut W,
}
impl<'a> CCE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CCE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The processor has no write access to the protected configuration registers."]
    #[inline(always)]
    pub fn protected(self) -> &'a mut W {
        self.variant(CCE_A::PROTECTED)
    }
    #[doc = "The processor has write access to the protected configuration registers (while MCAN_CCCR.INIT = '1')."]
    #[inline(always)]
    pub fn configurable(self) -> &'a mut W {
        self.variant(CCE_A::CONFIGURABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Restricted Operation Mode (read/write, write protection against '1')\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ASM_A {
    #[doc = "0: Normal CAN operation."]
    NORMAL = 0,
    #[doc = "1: Restricted Operation mode active."]
    RESTRICTED = 1,
}
impl From<ASM_A> for bool {
    #[inline(always)]
    fn from(variant: ASM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ASM` reader - Restricted Operation Mode (read/write, write protection against '1')"]
pub struct ASM_R(crate::FieldReader<bool, ASM_A>);
impl ASM_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ASM_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ASM_A {
        match self.bits {
            false => ASM_A::NORMAL,
            true => ASM_A::RESTRICTED,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        **self == ASM_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `RESTRICTED`"]
    #[inline(always)]
    pub fn is_restricted(&self) -> bool {
        **self == ASM_A::RESTRICTED
    }
}
impl core::ops::Deref for ASM_R {
    type Target = crate::FieldReader<bool, ASM_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ASM` writer - Restricted Operation Mode (read/write, write protection against '1')"]
pub struct ASM_W<'a> {
    w: &'a mut W,
}
impl<'a> ASM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ASM_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Normal CAN operation."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(ASM_A::NORMAL)
    }
    #[doc = "Restricted Operation mode active."]
    #[inline(always)]
    pub fn restricted(self) -> &'a mut W {
        self.variant(ASM_A::RESTRICTED)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `CSA` reader - Clock Stop Acknowledge (read-only)"]
pub struct CSA_R(crate::FieldReader<bool, bool>);
impl CSA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CSA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CSA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CSA` writer - Clock Stop Acknowledge (read-only)"]
pub struct CSA_W<'a> {
    w: &'a mut W,
}
impl<'a> CSA_W<'a> {
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
#[doc = "Clock Stop Request (read/write)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CSR_A {
    #[doc = "0: No clock stop is requested."]
    NO_CLOCK_STOP = 0,
    #[doc = "1: Clock stop requested. When clock stop is requested, first INIT and then CSA will be set after all pend-ing transfer requests have been completed and the CAN bus reached idle."]
    CLOCK_STOP = 1,
}
impl From<CSR_A> for bool {
    #[inline(always)]
    fn from(variant: CSR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CSR` reader - Clock Stop Request (read/write)"]
pub struct CSR_R(crate::FieldReader<bool, CSR_A>);
impl CSR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CSR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CSR_A {
        match self.bits {
            false => CSR_A::NO_CLOCK_STOP,
            true => CSR_A::CLOCK_STOP,
        }
    }
    #[doc = "Checks if the value of the field is `NO_CLOCK_STOP`"]
    #[inline(always)]
    pub fn is_no_clock_stop(&self) -> bool {
        **self == CSR_A::NO_CLOCK_STOP
    }
    #[doc = "Checks if the value of the field is `CLOCK_STOP`"]
    #[inline(always)]
    pub fn is_clock_stop(&self) -> bool {
        **self == CSR_A::CLOCK_STOP
    }
}
impl core::ops::Deref for CSR_R {
    type Target = crate::FieldReader<bool, CSR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CSR` writer - Clock Stop Request (read/write)"]
pub struct CSR_W<'a> {
    w: &'a mut W,
}
impl<'a> CSR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CSR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No clock stop is requested."]
    #[inline(always)]
    pub fn no_clock_stop(self) -> &'a mut W {
        self.variant(CSR_A::NO_CLOCK_STOP)
    }
    #[doc = "Clock stop requested. When clock stop is requested, first INIT and then CSA will be set after all pend-ing transfer requests have been completed and the CAN bus reached idle."]
    #[inline(always)]
    pub fn clock_stop(self) -> &'a mut W {
        self.variant(CSR_A::CLOCK_STOP)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Bus Monitoring Mode (read/write, write protection against '1')\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MON_A {
    #[doc = "0: Bus Monitoring mode is disabled."]
    DISABLED = 0,
    #[doc = "1: Bus Monitoring mode is enabled."]
    ENABLED = 1,
}
impl From<MON_A> for bool {
    #[inline(always)]
    fn from(variant: MON_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MON` reader - Bus Monitoring Mode (read/write, write protection against '1')"]
pub struct MON_R(crate::FieldReader<bool, MON_A>);
impl MON_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MON_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MON_A {
        match self.bits {
            false => MON_A::DISABLED,
            true => MON_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == MON_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == MON_A::ENABLED
    }
}
impl core::ops::Deref for MON_R {
    type Target = crate::FieldReader<bool, MON_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MON` writer - Bus Monitoring Mode (read/write, write protection against '1')"]
pub struct MON_W<'a> {
    w: &'a mut W,
}
impl<'a> MON_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MON_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Bus Monitoring mode is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MON_A::DISABLED)
    }
    #[doc = "Bus Monitoring mode is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(MON_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Disable Automatic Retransmission (read/write, write protection)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DAR_A {
    #[doc = "0: Automatic retransmission of messages not transmitted successfully enabled."]
    AUTO_RETX = 0,
    #[doc = "1: Automatic retransmission disabled."]
    NO_AUTO_RETX = 1,
}
impl From<DAR_A> for bool {
    #[inline(always)]
    fn from(variant: DAR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DAR` reader - Disable Automatic Retransmission (read/write, write protection)"]
pub struct DAR_R(crate::FieldReader<bool, DAR_A>);
impl DAR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DAR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DAR_A {
        match self.bits {
            false => DAR_A::AUTO_RETX,
            true => DAR_A::NO_AUTO_RETX,
        }
    }
    #[doc = "Checks if the value of the field is `AUTO_RETX`"]
    #[inline(always)]
    pub fn is_auto_retx(&self) -> bool {
        **self == DAR_A::AUTO_RETX
    }
    #[doc = "Checks if the value of the field is `NO_AUTO_RETX`"]
    #[inline(always)]
    pub fn is_no_auto_retx(&self) -> bool {
        **self == DAR_A::NO_AUTO_RETX
    }
}
impl core::ops::Deref for DAR_R {
    type Target = crate::FieldReader<bool, DAR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DAR` writer - Disable Automatic Retransmission (read/write, write protection)"]
pub struct DAR_W<'a> {
    w: &'a mut W,
}
impl<'a> DAR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DAR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Automatic retransmission of messages not transmitted successfully enabled."]
    #[inline(always)]
    pub fn auto_retx(self) -> &'a mut W {
        self.variant(DAR_A::AUTO_RETX)
    }
    #[doc = "Automatic retransmission disabled."]
    #[inline(always)]
    pub fn no_auto_retx(self) -> &'a mut W {
        self.variant(DAR_A::NO_AUTO_RETX)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Test Mode Enable (read/write, write protection against '1')\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TEST_A {
    #[doc = "0: Normal operation, MCAN_TEST register holds reset values."]
    DISABLED = 0,
    #[doc = "1: Test mode, write access to MCAN_TEST register enabled."]
    ENABLED = 1,
}
impl From<TEST_A> for bool {
    #[inline(always)]
    fn from(variant: TEST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TEST` reader - Test Mode Enable (read/write, write protection against '1')"]
pub struct TEST_R(crate::FieldReader<bool, TEST_A>);
impl TEST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TEST_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TEST_A {
        match self.bits {
            false => TEST_A::DISABLED,
            true => TEST_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == TEST_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == TEST_A::ENABLED
    }
}
impl core::ops::Deref for TEST_R {
    type Target = crate::FieldReader<bool, TEST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TEST` writer - Test Mode Enable (read/write, write protection against '1')"]
pub struct TEST_W<'a> {
    w: &'a mut W,
}
impl<'a> TEST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TEST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Normal operation, MCAN_TEST register holds reset values."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TEST_A::DISABLED)
    }
    #[doc = "Test mode, write access to MCAN_TEST register enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TEST_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "CAN FD Operation Enable (read/write, write protection)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FDOE_A {
    #[doc = "0: FD operation disabled."]
    DISABLED = 0,
    #[doc = "1: FD operation enabled."]
    ENABLED = 1,
}
impl From<FDOE_A> for bool {
    #[inline(always)]
    fn from(variant: FDOE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FDOE` reader - CAN FD Operation Enable (read/write, write protection)"]
pub struct FDOE_R(crate::FieldReader<bool, FDOE_A>);
impl FDOE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FDOE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FDOE_A {
        match self.bits {
            false => FDOE_A::DISABLED,
            true => FDOE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == FDOE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == FDOE_A::ENABLED
    }
}
impl core::ops::Deref for FDOE_R {
    type Target = crate::FieldReader<bool, FDOE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FDOE` writer - CAN FD Operation Enable (read/write, write protection)"]
pub struct FDOE_W<'a> {
    w: &'a mut W,
}
impl<'a> FDOE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FDOE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "FD operation disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(FDOE_A::DISABLED)
    }
    #[doc = "FD operation enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(FDOE_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Bit Rate Switching Enable (read/write, write protection)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BRSE_A {
    #[doc = "0: Bit rate switching for transmissions disabled."]
    DISABLED = 0,
    #[doc = "1: Bit rate switching for transmissions enabled."]
    ENABLED = 1,
}
impl From<BRSE_A> for bool {
    #[inline(always)]
    fn from(variant: BRSE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BRSE` reader - Bit Rate Switching Enable (read/write, write protection)"]
pub struct BRSE_R(crate::FieldReader<bool, BRSE_A>);
impl BRSE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BRSE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BRSE_A {
        match self.bits {
            false => BRSE_A::DISABLED,
            true => BRSE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == BRSE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == BRSE_A::ENABLED
    }
}
impl core::ops::Deref for BRSE_R {
    type Target = crate::FieldReader<bool, BRSE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BRSE` writer - Bit Rate Switching Enable (read/write, write protection)"]
pub struct BRSE_W<'a> {
    w: &'a mut W,
}
impl<'a> BRSE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BRSE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Bit rate switching for transmissions disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(BRSE_A::DISABLED)
    }
    #[doc = "Bit rate switching for transmissions enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(BRSE_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Field `PXHD` reader - Protocol Exception Event Handling (read/write, write protection)"]
pub struct PXHD_R(crate::FieldReader<bool, bool>);
impl PXHD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PXHD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PXHD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PXHD` writer - Protocol Exception Event Handling (read/write, write protection)"]
pub struct PXHD_W<'a> {
    w: &'a mut W,
}
impl<'a> PXHD_W<'a> {
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
#[doc = "Field `EFBI` reader - Edge Filtering during Bus Integration (read/write, write protection)"]
pub struct EFBI_R(crate::FieldReader<bool, bool>);
impl EFBI_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EFBI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EFBI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EFBI` writer - Edge Filtering during Bus Integration (read/write, write protection)"]
pub struct EFBI_W<'a> {
    w: &'a mut W,
}
impl<'a> EFBI_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "Field `TXP` reader - Transmit Pause (read/write, write protection)"]
pub struct TXP_R(crate::FieldReader<bool, bool>);
impl TXP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TXP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXP` writer - Transmit Pause (read/write, write protection)"]
pub struct TXP_W<'a> {
    w: &'a mut W,
}
impl<'a> TXP_W<'a> {
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
#[doc = "Field `NISO` reader - Non-ISO Operation"]
pub struct NISO_R(crate::FieldReader<bool, bool>);
impl NISO_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        NISO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NISO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NISO` writer - Non-ISO Operation"]
pub struct NISO_W<'a> {
    w: &'a mut W,
}
impl<'a> NISO_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Initialization (read/write)"]
    #[inline(always)]
    pub fn init(&self) -> INIT_R {
        INIT_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Configuration Change Enable (read/write, write protection)"]
    #[inline(always)]
    pub fn cce(&self) -> CCE_R {
        CCE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Restricted Operation Mode (read/write, write protection against '1')"]
    #[inline(always)]
    pub fn asm(&self) -> ASM_R {
        ASM_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Clock Stop Acknowledge (read-only)"]
    #[inline(always)]
    pub fn csa(&self) -> CSA_R {
        CSA_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Clock Stop Request (read/write)"]
    #[inline(always)]
    pub fn csr(&self) -> CSR_R {
        CSR_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Bus Monitoring Mode (read/write, write protection against '1')"]
    #[inline(always)]
    pub fn mon(&self) -> MON_R {
        MON_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Disable Automatic Retransmission (read/write, write protection)"]
    #[inline(always)]
    pub fn dar(&self) -> DAR_R {
        DAR_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Test Mode Enable (read/write, write protection against '1')"]
    #[inline(always)]
    pub fn test(&self) -> TEST_R {
        TEST_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - CAN FD Operation Enable (read/write, write protection)"]
    #[inline(always)]
    pub fn fdoe(&self) -> FDOE_R {
        FDOE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Bit Rate Switching Enable (read/write, write protection)"]
    #[inline(always)]
    pub fn brse(&self) -> BRSE_R {
        BRSE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Protocol Exception Event Handling (read/write, write protection)"]
    #[inline(always)]
    pub fn pxhd(&self) -> PXHD_R {
        PXHD_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Edge Filtering during Bus Integration (read/write, write protection)"]
    #[inline(always)]
    pub fn efbi(&self) -> EFBI_R {
        EFBI_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Transmit Pause (read/write, write protection)"]
    #[inline(always)]
    pub fn txp(&self) -> TXP_R {
        TXP_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Non-ISO Operation"]
    #[inline(always)]
    pub fn niso(&self) -> NISO_R {
        NISO_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Initialization (read/write)"]
    #[inline(always)]
    pub fn init(&mut self) -> INIT_W {
        INIT_W { w: self }
    }
    #[doc = "Bit 1 - Configuration Change Enable (read/write, write protection)"]
    #[inline(always)]
    pub fn cce(&mut self) -> CCE_W {
        CCE_W { w: self }
    }
    #[doc = "Bit 2 - Restricted Operation Mode (read/write, write protection against '1')"]
    #[inline(always)]
    pub fn asm(&mut self) -> ASM_W {
        ASM_W { w: self }
    }
    #[doc = "Bit 3 - Clock Stop Acknowledge (read-only)"]
    #[inline(always)]
    pub fn csa(&mut self) -> CSA_W {
        CSA_W { w: self }
    }
    #[doc = "Bit 4 - Clock Stop Request (read/write)"]
    #[inline(always)]
    pub fn csr(&mut self) -> CSR_W {
        CSR_W { w: self }
    }
    #[doc = "Bit 5 - Bus Monitoring Mode (read/write, write protection against '1')"]
    #[inline(always)]
    pub fn mon(&mut self) -> MON_W {
        MON_W { w: self }
    }
    #[doc = "Bit 6 - Disable Automatic Retransmission (read/write, write protection)"]
    #[inline(always)]
    pub fn dar(&mut self) -> DAR_W {
        DAR_W { w: self }
    }
    #[doc = "Bit 7 - Test Mode Enable (read/write, write protection against '1')"]
    #[inline(always)]
    pub fn test(&mut self) -> TEST_W {
        TEST_W { w: self }
    }
    #[doc = "Bit 8 - CAN FD Operation Enable (read/write, write protection)"]
    #[inline(always)]
    pub fn fdoe(&mut self) -> FDOE_W {
        FDOE_W { w: self }
    }
    #[doc = "Bit 9 - Bit Rate Switching Enable (read/write, write protection)"]
    #[inline(always)]
    pub fn brse(&mut self) -> BRSE_W {
        BRSE_W { w: self }
    }
    #[doc = "Bit 12 - Protocol Exception Event Handling (read/write, write protection)"]
    #[inline(always)]
    pub fn pxhd(&mut self) -> PXHD_W {
        PXHD_W { w: self }
    }
    #[doc = "Bit 13 - Edge Filtering during Bus Integration (read/write, write protection)"]
    #[inline(always)]
    pub fn efbi(&mut self) -> EFBI_W {
        EFBI_W { w: self }
    }
    #[doc = "Bit 14 - Transmit Pause (read/write, write protection)"]
    #[inline(always)]
    pub fn txp(&mut self) -> TXP_W {
        TXP_W { w: self }
    }
    #[doc = "Bit 15 - Non-ISO Operation"]
    #[inline(always)]
    pub fn niso(&mut self) -> NISO_W {
        NISO_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CC Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcan_cccr](index.html) module"]
pub struct MCAN_CCCR_SPEC;
impl crate::RegisterSpec for MCAN_CCCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mcan_cccr::R](R) reader structure"]
impl crate::Readable for MCAN_CCCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mcan_cccr::W](W) writer structure"]
impl crate::Writable for MCAN_CCCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MCAN_CCCR to value 0"]
impl crate::Resettable for MCAN_CCCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
