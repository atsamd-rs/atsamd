#[doc = "Reader of register STATUS"]
pub type R = crate::R<u32, super::STATUS>;
#[doc = "Writer for register STATUS"]
pub type W = crate::W<u32, super::STATUS>;
#[doc = "Register STATUS `reset()`'s with value 0x01"]
impl crate::ResetValue for super::STATUS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x01
    }
}
#[doc = "Reader of field `STOP`"]
pub type STOP_R = crate::R<bool, bool>;
#[doc = "Reader of field `IDX`"]
pub type IDX_R = crate::R<bool, bool>;
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `SLAVE`"]
pub type SLAVE_R = crate::R<bool, bool>;
#[doc = "Reader of field `PATTBV`"]
pub type PATTBV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PATTBV`"]
pub struct PATTBV_W<'a> {
    w: &'a mut W,
}
impl<'a> PATTBV_W<'a> {
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
#[doc = "Reader of field `WAVEBV`"]
pub type WAVEBV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WAVEBV`"]
pub struct WAVEBV_W<'a> {
    w: &'a mut W,
}
impl<'a> WAVEBV_W<'a> {
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
#[doc = "Reader of field `PERBV`"]
pub type PERBV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PERBV`"]
pub struct PERBV_W<'a> {
    w: &'a mut W,
}
impl<'a> PERBV_W<'a> {
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
#[doc = "Reader of field `FAULTAIN`"]
pub type FAULTAIN_R = crate::R<bool, bool>;
#[doc = "Reader of field `FAULTBIN`"]
pub type FAULTBIN_R = crate::R<bool, bool>;
#[doc = "Reader of field `FAULT0IN`"]
pub type FAULT0IN_R = crate::R<bool, bool>;
#[doc = "Reader of field `FAULT1IN`"]
pub type FAULT1IN_R = crate::R<bool, bool>;
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
#[doc = "Reader of field `CCBV0`"]
pub type CCBV0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CCBV0`"]
pub struct CCBV0_W<'a> {
    w: &'a mut W,
}
impl<'a> CCBV0_W<'a> {
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
#[doc = "Reader of field `CCBV1`"]
pub type CCBV1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CCBV1`"]
pub struct CCBV1_W<'a> {
    w: &'a mut W,
}
impl<'a> CCBV1_W<'a> {
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
#[doc = "Reader of field `CCBV2`"]
pub type CCBV2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CCBV2`"]
pub struct CCBV2_W<'a> {
    w: &'a mut W,
}
impl<'a> CCBV2_W<'a> {
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
#[doc = "Reader of field `CCBV3`"]
pub type CCBV3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CCBV3`"]
pub struct CCBV3_W<'a> {
    w: &'a mut W,
}
impl<'a> CCBV3_W<'a> {
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
#[doc = "Reader of field `CMP0`"]
pub type CMP0_R = crate::R<bool, bool>;
#[doc = "Reader of field `CMP1`"]
pub type CMP1_R = crate::R<bool, bool>;
#[doc = "Reader of field `CMP2`"]
pub type CMP2_R = crate::R<bool, bool>;
#[doc = "Reader of field `CMP3`"]
pub type CMP3_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Stop"]
    #[inline(always)]
    pub fn stop(&self) -> STOP_R {
        STOP_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Ramp"]
    #[inline(always)]
    pub fn idx(&self) -> IDX_R {
        IDX_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Non-Recoverable Debug Fault State"]
    #[inline(always)]
    pub fn dfs(&self) -> DFS_R {
        DFS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Slave"]
    #[inline(always)]
    pub fn slave(&self) -> SLAVE_R {
        SLAVE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Pattern Buffer Valid"]
    #[inline(always)]
    pub fn pattbv(&self) -> PATTBV_R {
        PATTBV_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Wave Buffer Valid"]
    #[inline(always)]
    pub fn wavebv(&self) -> WAVEBV_R {
        WAVEBV_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Period Buffer Valid"]
    #[inline(always)]
    pub fn perbv(&self) -> PERBV_R {
        PERBV_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Recoverable Fault A Input"]
    #[inline(always)]
    pub fn faultain(&self) -> FAULTAIN_R {
        FAULTAIN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Recoverable Fault B Input"]
    #[inline(always)]
    pub fn faultbin(&self) -> FAULTBIN_R {
        FAULTBIN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Non-Recoverable Fault0 Input"]
    #[inline(always)]
    pub fn fault0in(&self) -> FAULT0IN_R {
        FAULT0IN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Non-Recoverable Fault1 Input"]
    #[inline(always)]
    pub fn fault1in(&self) -> FAULT1IN_R {
        FAULT1IN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Recoverable Fault A State"]
    #[inline(always)]
    pub fn faulta(&self) -> FAULTA_R {
        FAULTA_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Recoverable Fault B State"]
    #[inline(always)]
    pub fn faultb(&self) -> FAULTB_R {
        FAULTB_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Non-Recoverable Fault 0 State"]
    #[inline(always)]
    pub fn fault0(&self) -> FAULT0_R {
        FAULT0_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Non-Recoverable Fault 1 State"]
    #[inline(always)]
    pub fn fault1(&self) -> FAULT1_R {
        FAULT1_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Compare Channel 0 Buffer Valid"]
    #[inline(always)]
    pub fn ccbv0(&self) -> CCBV0_R {
        CCBV0_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Compare Channel 1 Buffer Valid"]
    #[inline(always)]
    pub fn ccbv1(&self) -> CCBV1_R {
        CCBV1_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Compare Channel 2 Buffer Valid"]
    #[inline(always)]
    pub fn ccbv2(&self) -> CCBV2_R {
        CCBV2_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Compare Channel 3 Buffer Valid"]
    #[inline(always)]
    pub fn ccbv3(&self) -> CCBV3_R {
        CCBV3_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Compare Channel 0 Value"]
    #[inline(always)]
    pub fn cmp0(&self) -> CMP0_R {
        CMP0_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Compare Channel 1 Value"]
    #[inline(always)]
    pub fn cmp1(&self) -> CMP1_R {
        CMP1_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Compare Channel 2 Value"]
    #[inline(always)]
    pub fn cmp2(&self) -> CMP2_R {
        CMP2_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Compare Channel 3 Value"]
    #[inline(always)]
    pub fn cmp3(&self) -> CMP3_R {
        CMP3_R::new(((self.bits >> 27) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - Non-Recoverable Debug Fault State"]
    #[inline(always)]
    pub fn dfs(&mut self) -> DFS_W {
        DFS_W { w: self }
    }
    #[doc = "Bit 5 - Pattern Buffer Valid"]
    #[inline(always)]
    pub fn pattbv(&mut self) -> PATTBV_W {
        PATTBV_W { w: self }
    }
    #[doc = "Bit 6 - Wave Buffer Valid"]
    #[inline(always)]
    pub fn wavebv(&mut self) -> WAVEBV_W {
        WAVEBV_W { w: self }
    }
    #[doc = "Bit 7 - Period Buffer Valid"]
    #[inline(always)]
    pub fn perbv(&mut self) -> PERBV_W {
        PERBV_W { w: self }
    }
    #[doc = "Bit 12 - Recoverable Fault A State"]
    #[inline(always)]
    pub fn faulta(&mut self) -> FAULTA_W {
        FAULTA_W { w: self }
    }
    #[doc = "Bit 13 - Recoverable Fault B State"]
    #[inline(always)]
    pub fn faultb(&mut self) -> FAULTB_W {
        FAULTB_W { w: self }
    }
    #[doc = "Bit 14 - Non-Recoverable Fault 0 State"]
    #[inline(always)]
    pub fn fault0(&mut self) -> FAULT0_W {
        FAULT0_W { w: self }
    }
    #[doc = "Bit 15 - Non-Recoverable Fault 1 State"]
    #[inline(always)]
    pub fn fault1(&mut self) -> FAULT1_W {
        FAULT1_W { w: self }
    }
    #[doc = "Bit 16 - Compare Channel 0 Buffer Valid"]
    #[inline(always)]
    pub fn ccbv0(&mut self) -> CCBV0_W {
        CCBV0_W { w: self }
    }
    #[doc = "Bit 17 - Compare Channel 1 Buffer Valid"]
    #[inline(always)]
    pub fn ccbv1(&mut self) -> CCBV1_W {
        CCBV1_W { w: self }
    }
    #[doc = "Bit 18 - Compare Channel 2 Buffer Valid"]
    #[inline(always)]
    pub fn ccbv2(&mut self) -> CCBV2_W {
        CCBV2_W { w: self }
    }
    #[doc = "Bit 19 - Compare Channel 3 Buffer Valid"]
    #[inline(always)]
    pub fn ccbv3(&mut self) -> CCBV3_W {
        CCBV3_W { w: self }
    }
}
