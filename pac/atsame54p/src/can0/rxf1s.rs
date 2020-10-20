#[doc = "Reader of register RXF1S"]
pub type R = crate::R<u32, super::RXF1S>;
#[doc = "Reader of field `F1FL`"]
pub type F1FL_R = crate::R<u8, u8>;
#[doc = "Reader of field `F1GI`"]
pub type F1GI_R = crate::R<u8, u8>;
#[doc = "Reader of field `F1PI`"]
pub type F1PI_R = crate::R<u8, u8>;
#[doc = "Reader of field `F1F`"]
pub type F1F_R = crate::R<bool, bool>;
#[doc = "Reader of field `RF1L`"]
pub type RF1L_R = crate::R<bool, bool>;
#[doc = "Debug Message Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DMS_A {
    #[doc = "0: Idle state"]
    IDLE = 0,
    #[doc = "1: Debug message A received"]
    DBGA = 1,
    #[doc = "2: Debug message A/B received"]
    DBGB = 2,
    #[doc = "3: Debug message A/B/C received, DMA request set"]
    DBGC = 3,
}
impl From<DMS_A> for u8 {
    #[inline(always)]
    fn from(variant: DMS_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `DMS`"]
pub type DMS_R = crate::R<u8, DMS_A>;
impl DMS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMS_A {
        match self.bits {
            0 => DMS_A::IDLE,
            1 => DMS_A::DBGA,
            2 => DMS_A::DBGB,
            3 => DMS_A::DBGC,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `IDLE`"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == DMS_A::IDLE
    }
    #[doc = "Checks if the value of the field is `DBGA`"]
    #[inline(always)]
    pub fn is_dbga(&self) -> bool {
        *self == DMS_A::DBGA
    }
    #[doc = "Checks if the value of the field is `DBGB`"]
    #[inline(always)]
    pub fn is_dbgb(&self) -> bool {
        *self == DMS_A::DBGB
    }
    #[doc = "Checks if the value of the field is `DBGC`"]
    #[inline(always)]
    pub fn is_dbgc(&self) -> bool {
        *self == DMS_A::DBGC
    }
}
impl R {
    #[doc = "Bits 0:6 - Rx FIFO 1 Fill Level"]
    #[inline(always)]
    pub fn f1fl(&self) -> F1FL_R {
        F1FL_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:13 - Rx FIFO 1 Get Index"]
    #[inline(always)]
    pub fn f1gi(&self) -> F1GI_R {
        F1GI_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - Rx FIFO 1 Put Index"]
    #[inline(always)]
    pub fn f1pi(&self) -> F1PI_R {
        F1PI_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bit 24 - Rx FIFO 1 Full"]
    #[inline(always)]
    pub fn f1f(&self) -> F1F_R {
        F1F_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Rx FIFO 1 Message Lost"]
    #[inline(always)]
    pub fn rf1l(&self) -> RF1L_R {
        RF1L_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bits 30:31 - Debug Message Status"]
    #[inline(always)]
    pub fn dms(&self) -> DMS_R {
        DMS_R::new(((self.bits >> 30) & 0x03) as u8)
    }
}
