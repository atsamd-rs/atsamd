#[doc = "Reader of register NCR"]
pub type R = crate::R<u32, super::NCR>;
#[doc = "Writer for register NCR"]
pub type W = crate::W<u32, super::NCR>;
#[doc = "Register NCR `reset()`'s with value 0"]
impl crate::ResetValue for super::NCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `LBL`"]
pub type LBL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LBL`"]
pub struct LBL_W<'a> {
    w: &'a mut W,
}
impl<'a> LBL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `RXEN`"]
pub type RXEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXEN`"]
pub struct RXEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RXEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `TXEN`"]
pub type TXEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXEN`"]
pub struct TXEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TXEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `MPE`"]
pub type MPE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MPE`"]
pub struct MPE_W<'a> {
    w: &'a mut W,
}
impl<'a> MPE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `CLRSTAT`"]
pub type CLRSTAT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLRSTAT`"]
pub struct CLRSTAT_W<'a> {
    w: &'a mut W,
}
impl<'a> CLRSTAT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `INCSTAT`"]
pub type INCSTAT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INCSTAT`"]
pub struct INCSTAT_W<'a> {
    w: &'a mut W,
}
impl<'a> INCSTAT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `WESTAT`"]
pub type WESTAT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WESTAT`"]
pub struct WESTAT_W<'a> {
    w: &'a mut W,
}
impl<'a> WESTAT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `BP`"]
pub type BP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BP`"]
pub struct BP_W<'a> {
    w: &'a mut W,
}
impl<'a> BP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `TSTART`"]
pub type TSTART_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TSTART`"]
pub struct TSTART_W<'a> {
    w: &'a mut W,
}
impl<'a> TSTART_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `THALT`"]
pub type THALT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `THALT`"]
pub struct THALT_W<'a> {
    w: &'a mut W,
}
impl<'a> THALT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `TXPF`"]
pub type TXPF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXPF`"]
pub struct TXPF_W<'a> {
    w: &'a mut W,
}
impl<'a> TXPF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Reader of field `TXZQPF`"]
pub type TXZQPF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXZQPF`"]
pub struct TXZQPF_W<'a> {
    w: &'a mut W,
}
impl<'a> TXZQPF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `SRTSM`"]
pub type SRTSM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SRTSM`"]
pub struct SRTSM_W<'a> {
    w: &'a mut W,
}
impl<'a> SRTSM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Reader of field `ENPBPR`"]
pub type ENPBPR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENPBPR`"]
pub struct ENPBPR_W<'a> {
    w: &'a mut W,
}
impl<'a> ENPBPR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `TXPBPF`"]
pub type TXPBPF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXPBPF`"]
pub struct TXPBPF_W<'a> {
    w: &'a mut W,
}
impl<'a> TXPBPF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Reader of field `FNP`"]
pub type FNP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FNP`"]
pub struct FNP_W<'a> {
    w: &'a mut W,
}
impl<'a> FNP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Reader of field `LPI`"]
pub type LPI_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LPI`"]
pub struct LPI_W<'a> {
    w: &'a mut W,
}
impl<'a> LPI_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
impl R {
    #[doc = "Bit 1 - Loop Back Local"]
    #[inline(always)]
    pub fn lbl(&self) -> LBL_R {
        LBL_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Receive Enable"]
    #[inline(always)]
    pub fn rxen(&self) -> RXEN_R {
        RXEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Transmit Enable"]
    #[inline(always)]
    pub fn txen(&self) -> TXEN_R {
        TXEN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Management Port Enable"]
    #[inline(always)]
    pub fn mpe(&self) -> MPE_R {
        MPE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Clear Statistics Registers"]
    #[inline(always)]
    pub fn clrstat(&self) -> CLRSTAT_R {
        CLRSTAT_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Increment Statistics Registers"]
    #[inline(always)]
    pub fn incstat(&self) -> INCSTAT_R {
        INCSTAT_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Write Enable for Statistics Registers"]
    #[inline(always)]
    pub fn westat(&self) -> WESTAT_R {
        WESTAT_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Back pressure"]
    #[inline(always)]
    pub fn bp(&self) -> BP_R {
        BP_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Start Transmission"]
    #[inline(always)]
    pub fn tstart(&self) -> TSTART_R {
        TSTART_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Transmit Halt"]
    #[inline(always)]
    pub fn thalt(&self) -> THALT_R {
        THALT_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Transmit Pause Frame"]
    #[inline(always)]
    pub fn txpf(&self) -> TXPF_R {
        TXPF_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Transmit Zero Quantum Pause Frame"]
    #[inline(always)]
    pub fn txzqpf(&self) -> TXZQPF_R {
        TXZQPF_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Store Receive Time Stamp to Memory"]
    #[inline(always)]
    pub fn srtsm(&self) -> SRTSM_R {
        SRTSM_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Enable PFC Priority-based Pause Reception"]
    #[inline(always)]
    pub fn enpbpr(&self) -> ENPBPR_R {
        ENPBPR_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Transmit PFC Priority-based Pause Frame"]
    #[inline(always)]
    pub fn txpbpf(&self) -> TXPBPF_R {
        TXPBPF_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Flush Next Packet"]
    #[inline(always)]
    pub fn fnp(&self) -> FNP_R {
        FNP_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Low Power Idle Enable"]
    #[inline(always)]
    pub fn lpi(&self) -> LPI_R {
        LPI_R::new(((self.bits >> 19) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Loop Back Local"]
    #[inline(always)]
    pub fn lbl(&mut self) -> LBL_W {
        LBL_W { w: self }
    }
    #[doc = "Bit 2 - Receive Enable"]
    #[inline(always)]
    pub fn rxen(&mut self) -> RXEN_W {
        RXEN_W { w: self }
    }
    #[doc = "Bit 3 - Transmit Enable"]
    #[inline(always)]
    pub fn txen(&mut self) -> TXEN_W {
        TXEN_W { w: self }
    }
    #[doc = "Bit 4 - Management Port Enable"]
    #[inline(always)]
    pub fn mpe(&mut self) -> MPE_W {
        MPE_W { w: self }
    }
    #[doc = "Bit 5 - Clear Statistics Registers"]
    #[inline(always)]
    pub fn clrstat(&mut self) -> CLRSTAT_W {
        CLRSTAT_W { w: self }
    }
    #[doc = "Bit 6 - Increment Statistics Registers"]
    #[inline(always)]
    pub fn incstat(&mut self) -> INCSTAT_W {
        INCSTAT_W { w: self }
    }
    #[doc = "Bit 7 - Write Enable for Statistics Registers"]
    #[inline(always)]
    pub fn westat(&mut self) -> WESTAT_W {
        WESTAT_W { w: self }
    }
    #[doc = "Bit 8 - Back pressure"]
    #[inline(always)]
    pub fn bp(&mut self) -> BP_W {
        BP_W { w: self }
    }
    #[doc = "Bit 9 - Start Transmission"]
    #[inline(always)]
    pub fn tstart(&mut self) -> TSTART_W {
        TSTART_W { w: self }
    }
    #[doc = "Bit 10 - Transmit Halt"]
    #[inline(always)]
    pub fn thalt(&mut self) -> THALT_W {
        THALT_W { w: self }
    }
    #[doc = "Bit 11 - Transmit Pause Frame"]
    #[inline(always)]
    pub fn txpf(&mut self) -> TXPF_W {
        TXPF_W { w: self }
    }
    #[doc = "Bit 12 - Transmit Zero Quantum Pause Frame"]
    #[inline(always)]
    pub fn txzqpf(&mut self) -> TXZQPF_W {
        TXZQPF_W { w: self }
    }
    #[doc = "Bit 15 - Store Receive Time Stamp to Memory"]
    #[inline(always)]
    pub fn srtsm(&mut self) -> SRTSM_W {
        SRTSM_W { w: self }
    }
    #[doc = "Bit 16 - Enable PFC Priority-based Pause Reception"]
    #[inline(always)]
    pub fn enpbpr(&mut self) -> ENPBPR_W {
        ENPBPR_W { w: self }
    }
    #[doc = "Bit 17 - Transmit PFC Priority-based Pause Frame"]
    #[inline(always)]
    pub fn txpbpf(&mut self) -> TXPBPF_W {
        TXPBPF_W { w: self }
    }
    #[doc = "Bit 18 - Flush Next Packet"]
    #[inline(always)]
    pub fn fnp(&mut self) -> FNP_W {
        FNP_W { w: self }
    }
    #[doc = "Bit 19 - Low Power Idle Enable"]
    #[inline(always)]
    pub fn lpi(&mut self) -> LPI_W {
        LPI_W { w: self }
    }
}
