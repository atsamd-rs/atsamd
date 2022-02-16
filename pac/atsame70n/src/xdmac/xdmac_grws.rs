#[doc = "Register `XDMAC_GRWS` writer"]
pub struct W(crate::W<XDMAC_GRWS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<XDMAC_GRWS_SPEC>;
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
impl From<crate::W<XDMAC_GRWS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<XDMAC_GRWS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RWS0` writer - XDMAC Channel 0 Read Write Suspend Bit"]
pub struct RWS0_W<'a> {
    w: &'a mut W,
}
impl<'a> RWS0_W<'a> {
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
#[doc = "Field `RWS1` writer - XDMAC Channel 1 Read Write Suspend Bit"]
pub struct RWS1_W<'a> {
    w: &'a mut W,
}
impl<'a> RWS1_W<'a> {
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
#[doc = "Field `RWS2` writer - XDMAC Channel 2 Read Write Suspend Bit"]
pub struct RWS2_W<'a> {
    w: &'a mut W,
}
impl<'a> RWS2_W<'a> {
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
#[doc = "Field `RWS3` writer - XDMAC Channel 3 Read Write Suspend Bit"]
pub struct RWS3_W<'a> {
    w: &'a mut W,
}
impl<'a> RWS3_W<'a> {
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
#[doc = "Field `RWS4` writer - XDMAC Channel 4 Read Write Suspend Bit"]
pub struct RWS4_W<'a> {
    w: &'a mut W,
}
impl<'a> RWS4_W<'a> {
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
#[doc = "Field `RWS5` writer - XDMAC Channel 5 Read Write Suspend Bit"]
pub struct RWS5_W<'a> {
    w: &'a mut W,
}
impl<'a> RWS5_W<'a> {
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
#[doc = "Field `RWS6` writer - XDMAC Channel 6 Read Write Suspend Bit"]
pub struct RWS6_W<'a> {
    w: &'a mut W,
}
impl<'a> RWS6_W<'a> {
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
#[doc = "Field `RWS7` writer - XDMAC Channel 7 Read Write Suspend Bit"]
pub struct RWS7_W<'a> {
    w: &'a mut W,
}
impl<'a> RWS7_W<'a> {
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
#[doc = "Field `RWS8` writer - XDMAC Channel 8 Read Write Suspend Bit"]
pub struct RWS8_W<'a> {
    w: &'a mut W,
}
impl<'a> RWS8_W<'a> {
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
#[doc = "Field `RWS9` writer - XDMAC Channel 9 Read Write Suspend Bit"]
pub struct RWS9_W<'a> {
    w: &'a mut W,
}
impl<'a> RWS9_W<'a> {
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
#[doc = "Field `RWS10` writer - XDMAC Channel 10 Read Write Suspend Bit"]
pub struct RWS10_W<'a> {
    w: &'a mut W,
}
impl<'a> RWS10_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "Field `RWS11` writer - XDMAC Channel 11 Read Write Suspend Bit"]
pub struct RWS11_W<'a> {
    w: &'a mut W,
}
impl<'a> RWS11_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "Field `RWS12` writer - XDMAC Channel 12 Read Write Suspend Bit"]
pub struct RWS12_W<'a> {
    w: &'a mut W,
}
impl<'a> RWS12_W<'a> {
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
#[doc = "Field `RWS13` writer - XDMAC Channel 13 Read Write Suspend Bit"]
pub struct RWS13_W<'a> {
    w: &'a mut W,
}
impl<'a> RWS13_W<'a> {
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
#[doc = "Field `RWS14` writer - XDMAC Channel 14 Read Write Suspend Bit"]
pub struct RWS14_W<'a> {
    w: &'a mut W,
}
impl<'a> RWS14_W<'a> {
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
#[doc = "Field `RWS15` writer - XDMAC Channel 15 Read Write Suspend Bit"]
pub struct RWS15_W<'a> {
    w: &'a mut W,
}
impl<'a> RWS15_W<'a> {
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
#[doc = "Field `RWS16` writer - XDMAC Channel 16 Read Write Suspend Bit"]
pub struct RWS16_W<'a> {
    w: &'a mut W,
}
impl<'a> RWS16_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Field `RWS17` writer - XDMAC Channel 17 Read Write Suspend Bit"]
pub struct RWS17_W<'a> {
    w: &'a mut W,
}
impl<'a> RWS17_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "Field `RWS18` writer - XDMAC Channel 18 Read Write Suspend Bit"]
pub struct RWS18_W<'a> {
    w: &'a mut W,
}
impl<'a> RWS18_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
#[doc = "Field `RWS19` writer - XDMAC Channel 19 Read Write Suspend Bit"]
pub struct RWS19_W<'a> {
    w: &'a mut W,
}
impl<'a> RWS19_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
#[doc = "Field `RWS20` writer - XDMAC Channel 20 Read Write Suspend Bit"]
pub struct RWS20_W<'a> {
    w: &'a mut W,
}
impl<'a> RWS20_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
        self.w
    }
}
#[doc = "Field `RWS21` writer - XDMAC Channel 21 Read Write Suspend Bit"]
pub struct RWS21_W<'a> {
    w: &'a mut W,
}
impl<'a> RWS21_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | ((value as u32 & 0x01) << 21);
        self.w
    }
}
#[doc = "Field `RWS22` writer - XDMAC Channel 22 Read Write Suspend Bit"]
pub struct RWS22_W<'a> {
    w: &'a mut W,
}
impl<'a> RWS22_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | ((value as u32 & 0x01) << 22);
        self.w
    }
}
#[doc = "Field `RWS23` writer - XDMAC Channel 23 Read Write Suspend Bit"]
pub struct RWS23_W<'a> {
    w: &'a mut W,
}
impl<'a> RWS23_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | ((value as u32 & 0x01) << 23);
        self.w
    }
}
impl W {
    #[doc = "Bit 0 - XDMAC Channel 0 Read Write Suspend Bit"]
    #[inline(always)]
    pub fn rws0(&mut self) -> RWS0_W {
        RWS0_W { w: self }
    }
    #[doc = "Bit 1 - XDMAC Channel 1 Read Write Suspend Bit"]
    #[inline(always)]
    pub fn rws1(&mut self) -> RWS1_W {
        RWS1_W { w: self }
    }
    #[doc = "Bit 2 - XDMAC Channel 2 Read Write Suspend Bit"]
    #[inline(always)]
    pub fn rws2(&mut self) -> RWS2_W {
        RWS2_W { w: self }
    }
    #[doc = "Bit 3 - XDMAC Channel 3 Read Write Suspend Bit"]
    #[inline(always)]
    pub fn rws3(&mut self) -> RWS3_W {
        RWS3_W { w: self }
    }
    #[doc = "Bit 4 - XDMAC Channel 4 Read Write Suspend Bit"]
    #[inline(always)]
    pub fn rws4(&mut self) -> RWS4_W {
        RWS4_W { w: self }
    }
    #[doc = "Bit 5 - XDMAC Channel 5 Read Write Suspend Bit"]
    #[inline(always)]
    pub fn rws5(&mut self) -> RWS5_W {
        RWS5_W { w: self }
    }
    #[doc = "Bit 6 - XDMAC Channel 6 Read Write Suspend Bit"]
    #[inline(always)]
    pub fn rws6(&mut self) -> RWS6_W {
        RWS6_W { w: self }
    }
    #[doc = "Bit 7 - XDMAC Channel 7 Read Write Suspend Bit"]
    #[inline(always)]
    pub fn rws7(&mut self) -> RWS7_W {
        RWS7_W { w: self }
    }
    #[doc = "Bit 8 - XDMAC Channel 8 Read Write Suspend Bit"]
    #[inline(always)]
    pub fn rws8(&mut self) -> RWS8_W {
        RWS8_W { w: self }
    }
    #[doc = "Bit 9 - XDMAC Channel 9 Read Write Suspend Bit"]
    #[inline(always)]
    pub fn rws9(&mut self) -> RWS9_W {
        RWS9_W { w: self }
    }
    #[doc = "Bit 10 - XDMAC Channel 10 Read Write Suspend Bit"]
    #[inline(always)]
    pub fn rws10(&mut self) -> RWS10_W {
        RWS10_W { w: self }
    }
    #[doc = "Bit 11 - XDMAC Channel 11 Read Write Suspend Bit"]
    #[inline(always)]
    pub fn rws11(&mut self) -> RWS11_W {
        RWS11_W { w: self }
    }
    #[doc = "Bit 12 - XDMAC Channel 12 Read Write Suspend Bit"]
    #[inline(always)]
    pub fn rws12(&mut self) -> RWS12_W {
        RWS12_W { w: self }
    }
    #[doc = "Bit 13 - XDMAC Channel 13 Read Write Suspend Bit"]
    #[inline(always)]
    pub fn rws13(&mut self) -> RWS13_W {
        RWS13_W { w: self }
    }
    #[doc = "Bit 14 - XDMAC Channel 14 Read Write Suspend Bit"]
    #[inline(always)]
    pub fn rws14(&mut self) -> RWS14_W {
        RWS14_W { w: self }
    }
    #[doc = "Bit 15 - XDMAC Channel 15 Read Write Suspend Bit"]
    #[inline(always)]
    pub fn rws15(&mut self) -> RWS15_W {
        RWS15_W { w: self }
    }
    #[doc = "Bit 16 - XDMAC Channel 16 Read Write Suspend Bit"]
    #[inline(always)]
    pub fn rws16(&mut self) -> RWS16_W {
        RWS16_W { w: self }
    }
    #[doc = "Bit 17 - XDMAC Channel 17 Read Write Suspend Bit"]
    #[inline(always)]
    pub fn rws17(&mut self) -> RWS17_W {
        RWS17_W { w: self }
    }
    #[doc = "Bit 18 - XDMAC Channel 18 Read Write Suspend Bit"]
    #[inline(always)]
    pub fn rws18(&mut self) -> RWS18_W {
        RWS18_W { w: self }
    }
    #[doc = "Bit 19 - XDMAC Channel 19 Read Write Suspend Bit"]
    #[inline(always)]
    pub fn rws19(&mut self) -> RWS19_W {
        RWS19_W { w: self }
    }
    #[doc = "Bit 20 - XDMAC Channel 20 Read Write Suspend Bit"]
    #[inline(always)]
    pub fn rws20(&mut self) -> RWS20_W {
        RWS20_W { w: self }
    }
    #[doc = "Bit 21 - XDMAC Channel 21 Read Write Suspend Bit"]
    #[inline(always)]
    pub fn rws21(&mut self) -> RWS21_W {
        RWS21_W { w: self }
    }
    #[doc = "Bit 22 - XDMAC Channel 22 Read Write Suspend Bit"]
    #[inline(always)]
    pub fn rws22(&mut self) -> RWS22_W {
        RWS22_W { w: self }
    }
    #[doc = "Bit 23 - XDMAC Channel 23 Read Write Suspend Bit"]
    #[inline(always)]
    pub fn rws23(&mut self) -> RWS23_W {
        RWS23_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Global Channel Read Write Suspend Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [xdmac_grws](index.html) module"]
pub struct XDMAC_GRWS_SPEC;
impl crate::RegisterSpec for XDMAC_GRWS_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [xdmac_grws::W](W) writer structure"]
impl crate::Writable for XDMAC_GRWS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets XDMAC_GRWS to value 0"]
impl crate::Resettable for XDMAC_GRWS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
