#[doc = "Register `PSR` reader"]
pub type R = crate::R<PsrSpec>;
#[doc = "Last Error Code\n\nValue on reset: 7"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Lecselect {
    #[doc = "0: No Error"]
    None = 0,
    #[doc = "1: Stuff Error"]
    Stuff = 1,
    #[doc = "2: Form Error"]
    Form = 2,
    #[doc = "3: Ack Error"]
    Ack = 3,
    #[doc = "4: Bit1 Error"]
    Bit1 = 4,
    #[doc = "5: Bit0 Error"]
    Bit0 = 5,
    #[doc = "6: CRC Error"]
    Crc = 6,
    #[doc = "7: No Change"]
    Nc = 7,
}
impl From<Lecselect> for u8 {
    #[inline(always)]
    fn from(variant: Lecselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Lecselect {
    type Ux = u8;
}
impl crate::IsEnum for Lecselect {}
#[doc = "Field `LEC` reader - Last Error Code"]
pub type LecR = crate::FieldReader<Lecselect>;
impl LecR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lecselect {
        match self.bits {
            0 => Lecselect::None,
            1 => Lecselect::Stuff,
            2 => Lecselect::Form,
            3 => Lecselect::Ack,
            4 => Lecselect::Bit1,
            5 => Lecselect::Bit0,
            6 => Lecselect::Crc,
            7 => Lecselect::Nc,
            _ => unreachable!(),
        }
    }
    #[doc = "No Error"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Lecselect::None
    }
    #[doc = "Stuff Error"]
    #[inline(always)]
    pub fn is_stuff(&self) -> bool {
        *self == Lecselect::Stuff
    }
    #[doc = "Form Error"]
    #[inline(always)]
    pub fn is_form(&self) -> bool {
        *self == Lecselect::Form
    }
    #[doc = "Ack Error"]
    #[inline(always)]
    pub fn is_ack(&self) -> bool {
        *self == Lecselect::Ack
    }
    #[doc = "Bit1 Error"]
    #[inline(always)]
    pub fn is_bit1(&self) -> bool {
        *self == Lecselect::Bit1
    }
    #[doc = "Bit0 Error"]
    #[inline(always)]
    pub fn is_bit0(&self) -> bool {
        *self == Lecselect::Bit0
    }
    #[doc = "CRC Error"]
    #[inline(always)]
    pub fn is_crc(&self) -> bool {
        *self == Lecselect::Crc
    }
    #[doc = "No Change"]
    #[inline(always)]
    pub fn is_nc(&self) -> bool {
        *self == Lecselect::Nc
    }
}
#[doc = "Activity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Actselect {
    #[doc = "0: Node is synchronizing on CAN communication"]
    Sync = 0,
    #[doc = "1: Node is neither receiver nor transmitter"]
    Idle = 1,
    #[doc = "2: Node is operating as receiver"]
    Rx = 2,
    #[doc = "3: Node is operating as transmitter"]
    Tx = 3,
}
impl From<Actselect> for u8 {
    #[inline(always)]
    fn from(variant: Actselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Actselect {
    type Ux = u8;
}
impl crate::IsEnum for Actselect {}
#[doc = "Field `ACT` reader - Activity"]
pub type ActR = crate::FieldReader<Actselect>;
impl ActR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Actselect {
        match self.bits {
            0 => Actselect::Sync,
            1 => Actselect::Idle,
            2 => Actselect::Rx,
            3 => Actselect::Tx,
            _ => unreachable!(),
        }
    }
    #[doc = "Node is synchronizing on CAN communication"]
    #[inline(always)]
    pub fn is_sync(&self) -> bool {
        *self == Actselect::Sync
    }
    #[doc = "Node is neither receiver nor transmitter"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == Actselect::Idle
    }
    #[doc = "Node is operating as receiver"]
    #[inline(always)]
    pub fn is_rx(&self) -> bool {
        *self == Actselect::Rx
    }
    #[doc = "Node is operating as transmitter"]
    #[inline(always)]
    pub fn is_tx(&self) -> bool {
        *self == Actselect::Tx
    }
}
#[doc = "Field `EP` reader - Error Passive"]
pub type EpR = crate::BitReader;
#[doc = "Field `EW` reader - Warning Status"]
pub type EwR = crate::BitReader;
#[doc = "Field `BO` reader - Bus_Off Status"]
pub type BoR = crate::BitReader;
#[doc = "Data Phase Last Error Code\n\nValue on reset: 7"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dlecselect {
    #[doc = "0: No Error"]
    None = 0,
    #[doc = "1: Stuff Error"]
    Stuff = 1,
    #[doc = "2: Form Error"]
    Form = 2,
    #[doc = "3: Ack Error"]
    Ack = 3,
    #[doc = "4: Bit1 Error"]
    Bit1 = 4,
    #[doc = "5: Bit0 Error"]
    Bit0 = 5,
    #[doc = "6: CRC Error"]
    Crc = 6,
    #[doc = "7: No Change"]
    Nc = 7,
}
impl From<Dlecselect> for u8 {
    #[inline(always)]
    fn from(variant: Dlecselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dlecselect {
    type Ux = u8;
}
impl crate::IsEnum for Dlecselect {}
#[doc = "Field `DLEC` reader - Data Phase Last Error Code"]
pub type DlecR = crate::FieldReader<Dlecselect>;
impl DlecR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dlecselect {
        match self.bits {
            0 => Dlecselect::None,
            1 => Dlecselect::Stuff,
            2 => Dlecselect::Form,
            3 => Dlecselect::Ack,
            4 => Dlecselect::Bit1,
            5 => Dlecselect::Bit0,
            6 => Dlecselect::Crc,
            7 => Dlecselect::Nc,
            _ => unreachable!(),
        }
    }
    #[doc = "No Error"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Dlecselect::None
    }
    #[doc = "Stuff Error"]
    #[inline(always)]
    pub fn is_stuff(&self) -> bool {
        *self == Dlecselect::Stuff
    }
    #[doc = "Form Error"]
    #[inline(always)]
    pub fn is_form(&self) -> bool {
        *self == Dlecselect::Form
    }
    #[doc = "Ack Error"]
    #[inline(always)]
    pub fn is_ack(&self) -> bool {
        *self == Dlecselect::Ack
    }
    #[doc = "Bit1 Error"]
    #[inline(always)]
    pub fn is_bit1(&self) -> bool {
        *self == Dlecselect::Bit1
    }
    #[doc = "Bit0 Error"]
    #[inline(always)]
    pub fn is_bit0(&self) -> bool {
        *self == Dlecselect::Bit0
    }
    #[doc = "CRC Error"]
    #[inline(always)]
    pub fn is_crc(&self) -> bool {
        *self == Dlecselect::Crc
    }
    #[doc = "No Change"]
    #[inline(always)]
    pub fn is_nc(&self) -> bool {
        *self == Dlecselect::Nc
    }
}
#[doc = "Field `RESI` reader - ESI flag of last received CAN FD Message"]
pub type ResiR = crate::BitReader;
#[doc = "Field `RBRS` reader - BRS flag of last received CAN FD Message"]
pub type RbrsR = crate::BitReader;
#[doc = "Field `RFDF` reader - Received a CAN FD Message"]
pub type RfdfR = crate::BitReader;
#[doc = "Field `PXE` reader - Protocol Exception Event"]
pub type PxeR = crate::BitReader;
#[doc = "Field `TDCV` reader - Transmitter Delay Compensation Value"]
pub type TdcvR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:2 - Last Error Code"]
    #[inline(always)]
    pub fn lec(&self) -> LecR {
        LecR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:4 - Activity"]
    #[inline(always)]
    pub fn act(&self) -> ActR {
        ActR::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 5 - Error Passive"]
    #[inline(always)]
    pub fn ep(&self) -> EpR {
        EpR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Warning Status"]
    #[inline(always)]
    pub fn ew(&self) -> EwR {
        EwR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Bus_Off Status"]
    #[inline(always)]
    pub fn bo(&self) -> BoR {
        BoR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - Data Phase Last Error Code"]
    #[inline(always)]
    pub fn dlec(&self) -> DlecR {
        DlecR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11 - ESI flag of last received CAN FD Message"]
    #[inline(always)]
    pub fn resi(&self) -> ResiR {
        ResiR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - BRS flag of last received CAN FD Message"]
    #[inline(always)]
    pub fn rbrs(&self) -> RbrsR {
        RbrsR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Received a CAN FD Message"]
    #[inline(always)]
    pub fn rfdf(&self) -> RfdfR {
        RfdfR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Protocol Exception Event"]
    #[inline(always)]
    pub fn pxe(&self) -> PxeR {
        PxeR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 16:22 - Transmitter Delay Compensation Value"]
    #[inline(always)]
    pub fn tdcv(&self) -> TdcvR {
        TdcvR::new(((self.bits >> 16) & 0x7f) as u8)
    }
}
#[doc = "Protocol Status\n\nYou can [`read`](crate::Reg::read) this register and get [`psr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PsrSpec;
impl crate::RegisterSpec for PsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`psr::R`](R) reader structure"]
impl crate::Readable for PsrSpec {}
#[doc = "`reset()` method sets PSR to value 0x0707"]
impl crate::Resettable for PsrSpec {
    const RESET_VALUE: u32 = 0x0707;
}
