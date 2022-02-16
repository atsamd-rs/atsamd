#[doc = "Register `MCAN_PSR` reader"]
pub struct R(crate::R<MCAN_PSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MCAN_PSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MCAN_PSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MCAN_PSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Last Error Code (set to 111 on read)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LEC_A {
    #[doc = "0: No error occurred since LEC has been reset by successful reception or transmission."]
    NO_ERROR = 0,
    #[doc = "1: More than 5 equal bits in a sequence have occurred in a part of a received message where this is not allowed."]
    STUFF_ERROR = 1,
    #[doc = "2: A fixed format part of a received frame has the wrong format."]
    FORM_ERROR = 2,
    #[doc = "3: The message transmitted by the MCAN was not acknowledged by another node."]
    ACK_ERROR = 3,
    #[doc = "4: During transmission of a message (with the exception of the arbitration field), the device tried to send a recessive level (bit of logical value '1'), but the monitored bus value was dominant."]
    BIT1_ERROR = 4,
    #[doc = "5: During transmission of a message (or acknowledge bit, or active error flag, or overload flag), the device tried to send a dominant level (data or identifier bit logical value '0'), but the monitored bus value was recessive. During Bus_Off recovery, this status is set each time a sequence of 11 recessive bits has been monitored. This enables the processor to monitor the proceeding of the Bus_Off recovery sequence (indicating the bus is not stuck at dominant or continuously disturbed)."]
    BIT0_ERROR = 5,
    #[doc = "6: The CRC check sum of a received message was incorrect. The CRC of an incoming message does not match the CRC calculated from the received data."]
    CRC_ERROR = 6,
    #[doc = "7: Any read access to the Protocol Status Register re-initializes the LEC to '7'. When the LEC shows value '7', no CAN bus event was detected since the last processor read access to the Protocol Status Register."]
    NO_CHANGE = 7,
}
impl From<LEC_A> for u8 {
    #[inline(always)]
    fn from(variant: LEC_A) -> Self {
        variant as _
    }
}
#[doc = "Field `LEC` reader - Last Error Code (set to 111 on read)"]
pub struct LEC_R(crate::FieldReader<u8, LEC_A>);
impl LEC_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        LEC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LEC_A {
        match self.bits {
            0 => LEC_A::NO_ERROR,
            1 => LEC_A::STUFF_ERROR,
            2 => LEC_A::FORM_ERROR,
            3 => LEC_A::ACK_ERROR,
            4 => LEC_A::BIT1_ERROR,
            5 => LEC_A::BIT0_ERROR,
            6 => LEC_A::CRC_ERROR,
            7 => LEC_A::NO_CHANGE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NO_ERROR`"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        **self == LEC_A::NO_ERROR
    }
    #[doc = "Checks if the value of the field is `STUFF_ERROR`"]
    #[inline(always)]
    pub fn is_stuff_error(&self) -> bool {
        **self == LEC_A::STUFF_ERROR
    }
    #[doc = "Checks if the value of the field is `FORM_ERROR`"]
    #[inline(always)]
    pub fn is_form_error(&self) -> bool {
        **self == LEC_A::FORM_ERROR
    }
    #[doc = "Checks if the value of the field is `ACK_ERROR`"]
    #[inline(always)]
    pub fn is_ack_error(&self) -> bool {
        **self == LEC_A::ACK_ERROR
    }
    #[doc = "Checks if the value of the field is `BIT1_ERROR`"]
    #[inline(always)]
    pub fn is_bit1_error(&self) -> bool {
        **self == LEC_A::BIT1_ERROR
    }
    #[doc = "Checks if the value of the field is `BIT0_ERROR`"]
    #[inline(always)]
    pub fn is_bit0_error(&self) -> bool {
        **self == LEC_A::BIT0_ERROR
    }
    #[doc = "Checks if the value of the field is `CRC_ERROR`"]
    #[inline(always)]
    pub fn is_crc_error(&self) -> bool {
        **self == LEC_A::CRC_ERROR
    }
    #[doc = "Checks if the value of the field is `NO_CHANGE`"]
    #[inline(always)]
    pub fn is_no_change(&self) -> bool {
        **self == LEC_A::NO_CHANGE
    }
}
impl core::ops::Deref for LEC_R {
    type Target = crate::FieldReader<u8, LEC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Activity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ACT_A {
    #[doc = "0: Node is synchronizing on CAN communication"]
    SYNCHRONIZING = 0,
    #[doc = "1: Node is neither receiver nor transmitter"]
    IDLE = 1,
    #[doc = "2: Node is operating as receiver"]
    RECEIVER = 2,
    #[doc = "3: Node is operating as transmitter"]
    TRANSMITTER = 3,
}
impl From<ACT_A> for u8 {
    #[inline(always)]
    fn from(variant: ACT_A) -> Self {
        variant as _
    }
}
#[doc = "Field `ACT` reader - Activity"]
pub struct ACT_R(crate::FieldReader<u8, ACT_A>);
impl ACT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        ACT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACT_A {
        match self.bits {
            0 => ACT_A::SYNCHRONIZING,
            1 => ACT_A::IDLE,
            2 => ACT_A::RECEIVER,
            3 => ACT_A::TRANSMITTER,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SYNCHRONIZING`"]
    #[inline(always)]
    pub fn is_synchronizing(&self) -> bool {
        **self == ACT_A::SYNCHRONIZING
    }
    #[doc = "Checks if the value of the field is `IDLE`"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        **self == ACT_A::IDLE
    }
    #[doc = "Checks if the value of the field is `RECEIVER`"]
    #[inline(always)]
    pub fn is_receiver(&self) -> bool {
        **self == ACT_A::RECEIVER
    }
    #[doc = "Checks if the value of the field is `TRANSMITTER`"]
    #[inline(always)]
    pub fn is_transmitter(&self) -> bool {
        **self == ACT_A::TRANSMITTER
    }
}
impl core::ops::Deref for ACT_R {
    type Target = crate::FieldReader<u8, ACT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EP` reader - Error Passive"]
pub struct EP_R(crate::FieldReader<bool, bool>);
impl EP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EW` reader - Warning Status"]
pub struct EW_R(crate::FieldReader<bool, bool>);
impl EW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BO` reader - Bus_Off Status"]
pub struct BO_R(crate::FieldReader<bool, bool>);
impl BO_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DLEC` reader - Data Phase Last Error Code (set to 111 on read)"]
pub struct DLEC_R(crate::FieldReader<u8, u8>);
impl DLEC_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DLEC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DLEC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESI` reader - ESI Flag of Last Received CAN FD Message (cleared on read)"]
pub struct RESI_R(crate::FieldReader<bool, bool>);
impl RESI_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RESI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RBRS` reader - BRS Flag of Last Received CAN FD Message (cleared on read)"]
pub struct RBRS_R(crate::FieldReader<bool, bool>);
impl RBRS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RBRS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RBRS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RFDF` reader - Received a CAN FD Message (cleared on read)"]
pub struct RFDF_R(crate::FieldReader<bool, bool>);
impl RFDF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RFDF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RFDF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PXE` reader - Protocol Exception Event (cleared on read)"]
pub struct PXE_R(crate::FieldReader<bool, bool>);
impl PXE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PXE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PXE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TDCV` reader - Transmitter Delay Compensation Value"]
pub struct TDCV_R(crate::FieldReader<u8, u8>);
impl TDCV_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TDCV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TDCV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:2 - Last Error Code (set to 111 on read)"]
    #[inline(always)]
    pub fn lec(&self) -> LEC_R {
        LEC_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 3:4 - Activity"]
    #[inline(always)]
    pub fn act(&self) -> ACT_R {
        ACT_R::new(((self.bits >> 3) & 0x03) as u8)
    }
    #[doc = "Bit 5 - Error Passive"]
    #[inline(always)]
    pub fn ep(&self) -> EP_R {
        EP_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Warning Status"]
    #[inline(always)]
    pub fn ew(&self) -> EW_R {
        EW_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Bus_Off Status"]
    #[inline(always)]
    pub fn bo(&self) -> BO_R {
        BO_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:10 - Data Phase Last Error Code (set to 111 on read)"]
    #[inline(always)]
    pub fn dlec(&self) -> DLEC_R {
        DLEC_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bit 11 - ESI Flag of Last Received CAN FD Message (cleared on read)"]
    #[inline(always)]
    pub fn resi(&self) -> RESI_R {
        RESI_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - BRS Flag of Last Received CAN FD Message (cleared on read)"]
    #[inline(always)]
    pub fn rbrs(&self) -> RBRS_R {
        RBRS_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Received a CAN FD Message (cleared on read)"]
    #[inline(always)]
    pub fn rfdf(&self) -> RFDF_R {
        RFDF_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Protocol Exception Event (cleared on read)"]
    #[inline(always)]
    pub fn pxe(&self) -> PXE_R {
        PXE_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bits 16:22 - Transmitter Delay Compensation Value"]
    #[inline(always)]
    pub fn tdcv(&self) -> TDCV_R {
        TDCV_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
}
#[doc = "Protocol Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcan_psr](index.html) module"]
pub struct MCAN_PSR_SPEC;
impl crate::RegisterSpec for MCAN_PSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mcan_psr::R](R) reader structure"]
impl crate::Readable for MCAN_PSR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets MCAN_PSR to value 0"]
impl crate::Resettable for MCAN_PSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
