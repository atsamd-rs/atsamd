#[doc = "Reader of register INTENCLR"]
pub type R = crate::R<u32, super::INTENCLR>;
#[doc = "Writer for register INTENCLR"]
pub type W = crate::W<u32, super::INTENCLR>;
#[doc = "Register INTENCLR `reset()`'s with value 0"]
impl crate::ResetValue for super::INTENCLR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `OVF`"]
pub type OVF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OVF`"]
pub struct OVF_W<'a> {
    w: &'a mut W,
}
impl<'a> OVF_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `TRG`"]
pub type TRG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TRG`"]
pub struct TRG_W<'a> {
    w: &'a mut W,
}
impl<'a> TRG_W<'a> {
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
#[doc = "Reader of field `CNT`"]
pub type CNT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CNT`"]
pub struct CNT_W<'a> {
    w: &'a mut W,
}
impl<'a> CNT_W<'a> {
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
#[doc = "Reader of field `ERR`"]
pub type ERR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ERR`"]
pub struct ERR_W<'a> {
    w: &'a mut W,
}
impl<'a> ERR_W<'a> {
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
#[doc = "Reader of field `UFS`"]
pub type UFS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UFS`"]
pub struct UFS_W<'a> {
    w: &'a mut W,
}
impl<'a> UFS_W<'a> {
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
#[doc = "Reader of field `DFS`"]
pub type DFS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DFS`"]
pub struct DFS_W<'a> {
    w: &'a mut W,
}
impl<'a> DFS_W<'a> {
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
#[doc = "Reader of field `FAULTA`"]
pub type FAULTA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FAULTA`"]
pub struct FAULTA_W<'a> {
    w: &'a mut W,
}
impl<'a> FAULTA_W<'a> {
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
#[doc = "Reader of field `FAULTB`"]
pub type FAULTB_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FAULTB`"]
pub struct FAULTB_W<'a> {
    w: &'a mut W,
}
impl<'a> FAULTB_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Reader of field `FAULT0`"]
pub type FAULT0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FAULT0`"]
pub struct FAULT0_W<'a> {
    w: &'a mut W,
}
impl<'a> FAULT0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Reader of field `FAULT1`"]
pub type FAULT1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FAULT1`"]
pub struct FAULT1_W<'a> {
    w: &'a mut W,
}
impl<'a> FAULT1_W<'a> {
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
#[doc = "Reader of field `MC0`"]
pub type MC0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MC0`"]
pub struct MC0_W<'a> {
    w: &'a mut W,
}
impl<'a> MC0_W<'a> {
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
#[doc = "Reader of field `MC1`"]
pub type MC1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MC1`"]
pub struct MC1_W<'a> {
    w: &'a mut W,
}
impl<'a> MC1_W<'a> {
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
#[doc = "Reader of field `MC2`"]
pub type MC2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MC2`"]
pub struct MC2_W<'a> {
    w: &'a mut W,
}
impl<'a> MC2_W<'a> {
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
#[doc = "Reader of field `MC3`"]
pub type MC3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MC3`"]
pub struct MC3_W<'a> {
    w: &'a mut W,
}
impl<'a> MC3_W<'a> {
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
#[doc = "Reader of field `MC4`"]
pub type MC4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MC4`"]
pub struct MC4_W<'a> {
    w: &'a mut W,
}
impl<'a> MC4_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Reader of field `MC5`"]
pub type MC5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MC5`"]
pub struct MC5_W<'a> {
    w: &'a mut W,
}
impl<'a> MC5_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn ovf(&self) -> OVF_R {
        OVF_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Retrigger Interrupt Enable"]
    #[inline(always)]
    pub fn trg(&self) -> TRG_R {
        TRG_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Counter Interrupt Enable"]
    #[inline(always)]
    pub fn cnt(&self) -> CNT_R {
        CNT_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Error Interrupt Enable"]
    #[inline(always)]
    pub fn err(&self) -> ERR_R {
        ERR_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Non-Recoverable Update Fault Interrupt Enable"]
    #[inline(always)]
    pub fn ufs(&self) -> UFS_R {
        UFS_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Non-Recoverable Debug Fault Interrupt Enable"]
    #[inline(always)]
    pub fn dfs(&self) -> DFS_R {
        DFS_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Recoverable Fault A Interrupt Enable"]
    #[inline(always)]
    pub fn faulta(&self) -> FAULTA_R {
        FAULTA_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Recoverable Fault B Interrupt Enable"]
    #[inline(always)]
    pub fn faultb(&self) -> FAULTB_R {
        FAULTB_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Non-Recoverable Fault 0 Interrupt Enable"]
    #[inline(always)]
    pub fn fault0(&self) -> FAULT0_R {
        FAULT0_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Non-Recoverable Fault 1 Interrupt Enable"]
    #[inline(always)]
    pub fn fault1(&self) -> FAULT1_R {
        FAULT1_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Match or Capture Channel 0 Interrupt Enable"]
    #[inline(always)]
    pub fn mc0(&self) -> MC0_R {
        MC0_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Match or Capture Channel 1 Interrupt Enable"]
    #[inline(always)]
    pub fn mc1(&self) -> MC1_R {
        MC1_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Match or Capture Channel 2 Interrupt Enable"]
    #[inline(always)]
    pub fn mc2(&self) -> MC2_R {
        MC2_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Match or Capture Channel 3 Interrupt Enable"]
    #[inline(always)]
    pub fn mc3(&self) -> MC3_R {
        MC3_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Match or Capture Channel 4 Interrupt Enable"]
    #[inline(always)]
    pub fn mc4(&self) -> MC4_R {
        MC4_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Match or Capture Channel 5 Interrupt Enable"]
    #[inline(always)]
    pub fn mc5(&self) -> MC5_R {
        MC5_R::new(((self.bits >> 21) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn ovf(&mut self) -> OVF_W {
        OVF_W { w: self }
    }
    #[doc = "Bit 1 - Retrigger Interrupt Enable"]
    #[inline(always)]
    pub fn trg(&mut self) -> TRG_W {
        TRG_W { w: self }
    }
    #[doc = "Bit 2 - Counter Interrupt Enable"]
    #[inline(always)]
    pub fn cnt(&mut self) -> CNT_W {
        CNT_W { w: self }
    }
    #[doc = "Bit 3 - Error Interrupt Enable"]
    #[inline(always)]
    pub fn err(&mut self) -> ERR_W {
        ERR_W { w: self }
    }
    #[doc = "Bit 10 - Non-Recoverable Update Fault Interrupt Enable"]
    #[inline(always)]
    pub fn ufs(&mut self) -> UFS_W {
        UFS_W { w: self }
    }
    #[doc = "Bit 11 - Non-Recoverable Debug Fault Interrupt Enable"]
    #[inline(always)]
    pub fn dfs(&mut self) -> DFS_W {
        DFS_W { w: self }
    }
    #[doc = "Bit 12 - Recoverable Fault A Interrupt Enable"]
    #[inline(always)]
    pub fn faulta(&mut self) -> FAULTA_W {
        FAULTA_W { w: self }
    }
    #[doc = "Bit 13 - Recoverable Fault B Interrupt Enable"]
    #[inline(always)]
    pub fn faultb(&mut self) -> FAULTB_W {
        FAULTB_W { w: self }
    }
    #[doc = "Bit 14 - Non-Recoverable Fault 0 Interrupt Enable"]
    #[inline(always)]
    pub fn fault0(&mut self) -> FAULT0_W {
        FAULT0_W { w: self }
    }
    #[doc = "Bit 15 - Non-Recoverable Fault 1 Interrupt Enable"]
    #[inline(always)]
    pub fn fault1(&mut self) -> FAULT1_W {
        FAULT1_W { w: self }
    }
    #[doc = "Bit 16 - Match or Capture Channel 0 Interrupt Enable"]
    #[inline(always)]
    pub fn mc0(&mut self) -> MC0_W {
        MC0_W { w: self }
    }
    #[doc = "Bit 17 - Match or Capture Channel 1 Interrupt Enable"]
    #[inline(always)]
    pub fn mc1(&mut self) -> MC1_W {
        MC1_W { w: self }
    }
    #[doc = "Bit 18 - Match or Capture Channel 2 Interrupt Enable"]
    #[inline(always)]
    pub fn mc2(&mut self) -> MC2_W {
        MC2_W { w: self }
    }
    #[doc = "Bit 19 - Match or Capture Channel 3 Interrupt Enable"]
    #[inline(always)]
    pub fn mc3(&mut self) -> MC3_W {
        MC3_W { w: self }
    }
    #[doc = "Bit 20 - Match or Capture Channel 4 Interrupt Enable"]
    #[inline(always)]
    pub fn mc4(&mut self) -> MC4_W {
        MC4_W { w: self }
    }
    #[doc = "Bit 21 - Match or Capture Channel 5 Interrupt Enable"]
    #[inline(always)]
    pub fn mc5(&mut self) -> MC5_W {
        MC5_W { w: self }
    }
}
