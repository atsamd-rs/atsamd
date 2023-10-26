#[doc = "Register `PSR` reader"]
pub type R = crate::R<PSR_SPEC>;
#[doc = "Field `LEC` reader - Last Error Code"]
pub type LEC_R = crate::FieldReader<LECSELECT_A>;
#[doc = "Last Error Code\n\nValue on reset: 7"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LECSELECT_A {
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
impl From<LECSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: LECSELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for LECSELECT_A {
    type Ux = u8;
}
impl LEC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LECSELECT_A {
        match self.bits {
            0 => LECSELECT_A::NONE,
            1 => LECSELECT_A::STUFF,
            2 => LECSELECT_A::FORM,
            3 => LECSELECT_A::ACK,
            4 => LECSELECT_A::BIT1,
            5 => LECSELECT_A::BIT0,
            6 => LECSELECT_A::CRC,
            7 => LECSELECT_A::NC,
            _ => unreachable!(),
        }
    }
    #[doc = "No Error"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == LECSELECT_A::NONE
    }
    #[doc = "Stuff Error"]
    #[inline(always)]
    pub fn is_stuff(&self) -> bool {
        *self == LECSELECT_A::STUFF
    }
    #[doc = "Form Error"]
    #[inline(always)]
    pub fn is_form(&self) -> bool {
        *self == LECSELECT_A::FORM
    }
    #[doc = "Ack Error"]
    #[inline(always)]
    pub fn is_ack(&self) -> bool {
        *self == LECSELECT_A::ACK
    }
    #[doc = "Bit1 Error"]
    #[inline(always)]
    pub fn is_bit1(&self) -> bool {
        *self == LECSELECT_A::BIT1
    }
    #[doc = "Bit0 Error"]
    #[inline(always)]
    pub fn is_bit0(&self) -> bool {
        *self == LECSELECT_A::BIT0
    }
    #[doc = "CRC Error"]
    #[inline(always)]
    pub fn is_crc(&self) -> bool {
        *self == LECSELECT_A::CRC
    }
    #[doc = "No Change"]
    #[inline(always)]
    pub fn is_nc(&self) -> bool {
        *self == LECSELECT_A::NC
    }
}
#[doc = "Field `ACT` reader - Activity"]
pub type ACT_R = crate::FieldReader<ACTSELECT_A>;
#[doc = "Activity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ACTSELECT_A {
    #[doc = "0: Node is synchronizing on CAN communication"]
    SYNC = 0,
    #[doc = "1: Node is neither receiver nor transmitter"]
    IDLE = 1,
    #[doc = "2: Node is operating as receiver"]
    RX = 2,
    #[doc = "3: Node is operating as transmitter"]
    TX = 3,
}
impl From<ACTSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: ACTSELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ACTSELECT_A {
    type Ux = u8;
}
impl ACT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ACTSELECT_A {
        match self.bits {
            0 => ACTSELECT_A::SYNC,
            1 => ACTSELECT_A::IDLE,
            2 => ACTSELECT_A::RX,
            3 => ACTSELECT_A::TX,
            _ => unreachable!(),
        }
    }
    #[doc = "Node is synchronizing on CAN communication"]
    #[inline(always)]
    pub fn is_sync(&self) -> bool {
        *self == ACTSELECT_A::SYNC
    }
    #[doc = "Node is neither receiver nor transmitter"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == ACTSELECT_A::IDLE
    }
    #[doc = "Node is operating as receiver"]
    #[inline(always)]
    pub fn is_rx(&self) -> bool {
        *self == ACTSELECT_A::RX
    }
    #[doc = "Node is operating as transmitter"]
    #[inline(always)]
    pub fn is_tx(&self) -> bool {
        *self == ACTSELECT_A::TX
    }
}
#[doc = "Field `EP` reader - Error Passive"]
pub type EP_R = crate::BitReader;
#[doc = "Field `EW` reader - Warning Status"]
pub type EW_R = crate::BitReader;
#[doc = "Field `BO` reader - Bus_Off Status"]
pub type BO_R = crate::BitReader;
#[doc = "Field `DLEC` reader - Data Phase Last Error Code"]
pub type DLEC_R = crate::FieldReader<DLECSELECT_A>;
#[doc = "Data Phase Last Error Code\n\nValue on reset: 7"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DLECSELECT_A {
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
impl From<DLECSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: DLECSELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DLECSELECT_A {
    type Ux = u8;
}
impl DLEC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DLECSELECT_A {
        match self.bits {
            0 => DLECSELECT_A::NONE,
            1 => DLECSELECT_A::STUFF,
            2 => DLECSELECT_A::FORM,
            3 => DLECSELECT_A::ACK,
            4 => DLECSELECT_A::BIT1,
            5 => DLECSELECT_A::BIT0,
            6 => DLECSELECT_A::CRC,
            7 => DLECSELECT_A::NC,
            _ => unreachable!(),
        }
    }
    #[doc = "No Error"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == DLECSELECT_A::NONE
    }
    #[doc = "Stuff Error"]
    #[inline(always)]
    pub fn is_stuff(&self) -> bool {
        *self == DLECSELECT_A::STUFF
    }
    #[doc = "Form Error"]
    #[inline(always)]
    pub fn is_form(&self) -> bool {
        *self == DLECSELECT_A::FORM
    }
    #[doc = "Ack Error"]
    #[inline(always)]
    pub fn is_ack(&self) -> bool {
        *self == DLECSELECT_A::ACK
    }
    #[doc = "Bit1 Error"]
    #[inline(always)]
    pub fn is_bit1(&self) -> bool {
        *self == DLECSELECT_A::BIT1
    }
    #[doc = "Bit0 Error"]
    #[inline(always)]
    pub fn is_bit0(&self) -> bool {
        *self == DLECSELECT_A::BIT0
    }
    #[doc = "CRC Error"]
    #[inline(always)]
    pub fn is_crc(&self) -> bool {
        *self == DLECSELECT_A::CRC
    }
    #[doc = "No Change"]
    #[inline(always)]
    pub fn is_nc(&self) -> bool {
        *self == DLECSELECT_A::NC
    }
}
#[doc = "Field `RESI` reader - ESI flag of last received CAN FD Message"]
pub type RESI_R = crate::BitReader;
#[doc = "Field `RBRS` reader - BRS flag of last received CAN FD Message"]
pub type RBRS_R = crate::BitReader;
#[doc = "Field `RFDF` reader - Received a CAN FD Message"]
pub type RFDF_R = crate::BitReader;
#[doc = "Field `PXE` reader - Protocol Exception Event"]
pub type PXE_R = crate::BitReader;
#[doc = "Field `TDCV` reader - Transmitter Delay Compensation Value"]
pub type TDCV_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:2 - Last Error Code"]
    #[inline(always)]
    pub fn lec(&self) -> LEC_R {
        LEC_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:4 - Activity"]
    #[inline(always)]
    pub fn act(&self) -> ACT_R {
        ACT_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 5 - Error Passive"]
    #[inline(always)]
    pub fn ep(&self) -> EP_R {
        EP_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Warning Status"]
    #[inline(always)]
    pub fn ew(&self) -> EW_R {
        EW_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Bus_Off Status"]
    #[inline(always)]
    pub fn bo(&self) -> BO_R {
        BO_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - Data Phase Last Error Code"]
    #[inline(always)]
    pub fn dlec(&self) -> DLEC_R {
        DLEC_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11 - ESI flag of last received CAN FD Message"]
    #[inline(always)]
    pub fn resi(&self) -> RESI_R {
        RESI_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - BRS flag of last received CAN FD Message"]
    #[inline(always)]
    pub fn rbrs(&self) -> RBRS_R {
        RBRS_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Received a CAN FD Message"]
    #[inline(always)]
    pub fn rfdf(&self) -> RFDF_R {
        RFDF_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Protocol Exception Event"]
    #[inline(always)]
    pub fn pxe(&self) -> PXE_R {
        PXE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 16:22 - Transmitter Delay Compensation Value"]
    #[inline(always)]
    pub fn tdcv(&self) -> TDCV_R {
        TDCV_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
}
#[doc = "Protocol Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`psr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PSR_SPEC;
impl crate::RegisterSpec for PSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`psr::R`](R) reader structure"]
impl crate::Readable for PSR_SPEC {}
#[doc = "`reset()` method sets PSR to value 0x0707"]
impl crate::Resettable for PSR_SPEC {
    const RESET_VALUE: Self::Ux = 0x0707;
}
