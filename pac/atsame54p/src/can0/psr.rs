#[doc = "Reader of register PSR"]
pub type R = crate::R<u32, super::PSR>;
#[doc = "Last Error Code\n\nValue on reset: 7"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LEC_A {
    #[doc = "0: No Error"]
    NONE = 0,
    #[doc = "1: Stuff Error"]
    STUFF = 1,
    #[doc = "2: Form Error"]
    FORM = 2,
    #[doc = "3: Ack Error"]
    ACK = 3,
    #[doc = "4: Bit1 Error"]
    BIT1 = 4,
    #[doc = "5: Bit0 Error"]
    BIT0 = 5,
    #[doc = "6: CRC Error"]
    CRC = 6,
    #[doc = "7: No Change"]
    NC = 7,
}
impl From<LEC_A> for u8 {
    #[inline(always)]
    fn from(variant: LEC_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `LEC`"]
pub type LEC_R = crate::R<u8, LEC_A>;
impl LEC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LEC_A {
        match self.bits {
            0 => LEC_A::NONE,
            1 => LEC_A::STUFF,
            2 => LEC_A::FORM,
            3 => LEC_A::ACK,
            4 => LEC_A::BIT1,
            5 => LEC_A::BIT0,
            6 => LEC_A::CRC,
            7 => LEC_A::NC,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == LEC_A::NONE
    }
    #[doc = "Checks if the value of the field is `STUFF`"]
    #[inline(always)]
    pub fn is_stuff(&self) -> bool {
        *self == LEC_A::STUFF
    }
    #[doc = "Checks if the value of the field is `FORM`"]
    #[inline(always)]
    pub fn is_form(&self) -> bool {
        *self == LEC_A::FORM
    }
    #[doc = "Checks if the value of the field is `ACK`"]
    #[inline(always)]
    pub fn is_ack(&self) -> bool {
        *self == LEC_A::ACK
    }
    #[doc = "Checks if the value of the field is `BIT1`"]
    #[inline(always)]
    pub fn is_bit1(&self) -> bool {
        *self == LEC_A::BIT1
    }
    #[doc = "Checks if the value of the field is `BIT0`"]
    #[inline(always)]
    pub fn is_bit0(&self) -> bool {
        *self == LEC_A::BIT0
    }
    #[doc = "Checks if the value of the field is `CRC`"]
    #[inline(always)]
    pub fn is_crc(&self) -> bool {
        *self == LEC_A::CRC
    }
    #[doc = "Checks if the value of the field is `NC`"]
    #[inline(always)]
    pub fn is_nc(&self) -> bool {
        *self == LEC_A::NC
    }
}
#[doc = "Activity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ACT_A {
    #[doc = "0: Node is synchronizing on CAN communication"]
    SYNC = 0,
    #[doc = "1: Node is neither receiver nor transmitter"]
    IDLE = 1,
    #[doc = "2: Node is operating as receiver"]
    RX = 2,
    #[doc = "3: Node is operating as transmitter"]
    TX = 3,
}
impl From<ACT_A> for u8 {
    #[inline(always)]
    fn from(variant: ACT_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `ACT`"]
pub type ACT_R = crate::R<u8, ACT_A>;
impl ACT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACT_A {
        match self.bits {
            0 => ACT_A::SYNC,
            1 => ACT_A::IDLE,
            2 => ACT_A::RX,
            3 => ACT_A::TX,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SYNC`"]
    #[inline(always)]
    pub fn is_sync(&self) -> bool {
        *self == ACT_A::SYNC
    }
    #[doc = "Checks if the value of the field is `IDLE`"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == ACT_A::IDLE
    }
    #[doc = "Checks if the value of the field is `RX`"]
    #[inline(always)]
    pub fn is_rx(&self) -> bool {
        *self == ACT_A::RX
    }
    #[doc = "Checks if the value of the field is `TX`"]
    #[inline(always)]
    pub fn is_tx(&self) -> bool {
        *self == ACT_A::TX
    }
}
#[doc = "Reader of field `EP`"]
pub type EP_R = crate::R<bool, bool>;
#[doc = "Reader of field `EW`"]
pub type EW_R = crate::R<bool, bool>;
#[doc = "Reader of field `BO`"]
pub type BO_R = crate::R<bool, bool>;
#[doc = "Data Phase Last Error Code\n\nValue on reset: 7"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DLEC_A {
    #[doc = "0: No Error"]
    NONE = 0,
    #[doc = "1: Stuff Error"]
    STUFF = 1,
    #[doc = "2: Form Error"]
    FORM = 2,
    #[doc = "3: Ack Error"]
    ACK = 3,
    #[doc = "4: Bit1 Error"]
    BIT1 = 4,
    #[doc = "5: Bit0 Error"]
    BIT0 = 5,
    #[doc = "6: CRC Error"]
    CRC = 6,
    #[doc = "7: No Change"]
    NC = 7,
}
impl From<DLEC_A> for u8 {
    #[inline(always)]
    fn from(variant: DLEC_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `DLEC`"]
pub type DLEC_R = crate::R<u8, DLEC_A>;
impl DLEC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DLEC_A {
        match self.bits {
            0 => DLEC_A::NONE,
            1 => DLEC_A::STUFF,
            2 => DLEC_A::FORM,
            3 => DLEC_A::ACK,
            4 => DLEC_A::BIT1,
            5 => DLEC_A::BIT0,
            6 => DLEC_A::CRC,
            7 => DLEC_A::NC,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == DLEC_A::NONE
    }
    #[doc = "Checks if the value of the field is `STUFF`"]
    #[inline(always)]
    pub fn is_stuff(&self) -> bool {
        *self == DLEC_A::STUFF
    }
    #[doc = "Checks if the value of the field is `FORM`"]
    #[inline(always)]
    pub fn is_form(&self) -> bool {
        *self == DLEC_A::FORM
    }
    #[doc = "Checks if the value of the field is `ACK`"]
    #[inline(always)]
    pub fn is_ack(&self) -> bool {
        *self == DLEC_A::ACK
    }
    #[doc = "Checks if the value of the field is `BIT1`"]
    #[inline(always)]
    pub fn is_bit1(&self) -> bool {
        *self == DLEC_A::BIT1
    }
    #[doc = "Checks if the value of the field is `BIT0`"]
    #[inline(always)]
    pub fn is_bit0(&self) -> bool {
        *self == DLEC_A::BIT0
    }
    #[doc = "Checks if the value of the field is `CRC`"]
    #[inline(always)]
    pub fn is_crc(&self) -> bool {
        *self == DLEC_A::CRC
    }
    #[doc = "Checks if the value of the field is `NC`"]
    #[inline(always)]
    pub fn is_nc(&self) -> bool {
        *self == DLEC_A::NC
    }
}
#[doc = "Reader of field `RESI`"]
pub type RESI_R = crate::R<bool, bool>;
#[doc = "Reader of field `RBRS`"]
pub type RBRS_R = crate::R<bool, bool>;
#[doc = "Reader of field `RFDF`"]
pub type RFDF_R = crate::R<bool, bool>;
#[doc = "Reader of field `PXE`"]
pub type PXE_R = crate::R<bool, bool>;
#[doc = "Reader of field `TDCV`"]
pub type TDCV_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:2 - Last Error Code"]
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
    #[doc = "Bits 8:10 - Data Phase Last Error Code"]
    #[inline(always)]
    pub fn dlec(&self) -> DLEC_R {
        DLEC_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bit 11 - ESI flag of last received CAN FD Message"]
    #[inline(always)]
    pub fn resi(&self) -> RESI_R {
        RESI_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - BRS flag of last received CAN FD Message"]
    #[inline(always)]
    pub fn rbrs(&self) -> RBRS_R {
        RBRS_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Received a CAN FD Message"]
    #[inline(always)]
    pub fn rfdf(&self) -> RFDF_R {
        RFDF_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Protocol Exception Event"]
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
