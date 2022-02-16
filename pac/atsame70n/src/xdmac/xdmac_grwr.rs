#[doc = "Register `XDMAC_GRWR` writer"]
pub struct W(crate::W<XDMAC_GRWR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<XDMAC_GRWR_SPEC>;
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
impl From<crate::W<XDMAC_GRWR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<XDMAC_GRWR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RWR0` writer - XDMAC Channel 0 Read Write Resume Bit"]
pub struct RWR0_W<'a> {
    w: &'a mut W,
}
impl<'a> RWR0_W<'a> {
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
#[doc = "Field `RWR1` writer - XDMAC Channel 1 Read Write Resume Bit"]
pub struct RWR1_W<'a> {
    w: &'a mut W,
}
impl<'a> RWR1_W<'a> {
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
#[doc = "Field `RWR2` writer - XDMAC Channel 2 Read Write Resume Bit"]
pub struct RWR2_W<'a> {
    w: &'a mut W,
}
impl<'a> RWR2_W<'a> {
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
#[doc = "Field `RWR3` writer - XDMAC Channel 3 Read Write Resume Bit"]
pub struct RWR3_W<'a> {
    w: &'a mut W,
}
impl<'a> RWR3_W<'a> {
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
#[doc = "Field `RWR4` writer - XDMAC Channel 4 Read Write Resume Bit"]
pub struct RWR4_W<'a> {
    w: &'a mut W,
}
impl<'a> RWR4_W<'a> {
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
#[doc = "Field `RWR5` writer - XDMAC Channel 5 Read Write Resume Bit"]
pub struct RWR5_W<'a> {
    w: &'a mut W,
}
impl<'a> RWR5_W<'a> {
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
#[doc = "Field `RWR6` writer - XDMAC Channel 6 Read Write Resume Bit"]
pub struct RWR6_W<'a> {
    w: &'a mut W,
}
impl<'a> RWR6_W<'a> {
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
#[doc = "Field `RWR7` writer - XDMAC Channel 7 Read Write Resume Bit"]
pub struct RWR7_W<'a> {
    w: &'a mut W,
}
impl<'a> RWR7_W<'a> {
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
#[doc = "Field `RWR8` writer - XDMAC Channel 8 Read Write Resume Bit"]
pub struct RWR8_W<'a> {
    w: &'a mut W,
}
impl<'a> RWR8_W<'a> {
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
#[doc = "Field `RWR9` writer - XDMAC Channel 9 Read Write Resume Bit"]
pub struct RWR9_W<'a> {
    w: &'a mut W,
}
impl<'a> RWR9_W<'a> {
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
#[doc = "Field `RWR10` writer - XDMAC Channel 10 Read Write Resume Bit"]
pub struct RWR10_W<'a> {
    w: &'a mut W,
}
impl<'a> RWR10_W<'a> {
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
#[doc = "Field `RWR11` writer - XDMAC Channel 11 Read Write Resume Bit"]
pub struct RWR11_W<'a> {
    w: &'a mut W,
}
impl<'a> RWR11_W<'a> {
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
#[doc = "Field `RWR12` writer - XDMAC Channel 12 Read Write Resume Bit"]
pub struct RWR12_W<'a> {
    w: &'a mut W,
}
impl<'a> RWR12_W<'a> {
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
#[doc = "Field `RWR13` writer - XDMAC Channel 13 Read Write Resume Bit"]
pub struct RWR13_W<'a> {
    w: &'a mut W,
}
impl<'a> RWR13_W<'a> {
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
#[doc = "Field `RWR14` writer - XDMAC Channel 14 Read Write Resume Bit"]
pub struct RWR14_W<'a> {
    w: &'a mut W,
}
impl<'a> RWR14_W<'a> {
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
#[doc = "Field `RWR15` writer - XDMAC Channel 15 Read Write Resume Bit"]
pub struct RWR15_W<'a> {
    w: &'a mut W,
}
impl<'a> RWR15_W<'a> {
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
#[doc = "Field `RWR16` writer - XDMAC Channel 16 Read Write Resume Bit"]
pub struct RWR16_W<'a> {
    w: &'a mut W,
}
impl<'a> RWR16_W<'a> {
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
#[doc = "Field `RWR17` writer - XDMAC Channel 17 Read Write Resume Bit"]
pub struct RWR17_W<'a> {
    w: &'a mut W,
}
impl<'a> RWR17_W<'a> {
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
#[doc = "Field `RWR18` writer - XDMAC Channel 18 Read Write Resume Bit"]
pub struct RWR18_W<'a> {
    w: &'a mut W,
}
impl<'a> RWR18_W<'a> {
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
#[doc = "Field `RWR19` writer - XDMAC Channel 19 Read Write Resume Bit"]
pub struct RWR19_W<'a> {
    w: &'a mut W,
}
impl<'a> RWR19_W<'a> {
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
#[doc = "Field `RWR20` writer - XDMAC Channel 20 Read Write Resume Bit"]
pub struct RWR20_W<'a> {
    w: &'a mut W,
}
impl<'a> RWR20_W<'a> {
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
#[doc = "Field `RWR21` writer - XDMAC Channel 21 Read Write Resume Bit"]
pub struct RWR21_W<'a> {
    w: &'a mut W,
}
impl<'a> RWR21_W<'a> {
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
#[doc = "Field `RWR22` writer - XDMAC Channel 22 Read Write Resume Bit"]
pub struct RWR22_W<'a> {
    w: &'a mut W,
}
impl<'a> RWR22_W<'a> {
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
#[doc = "Field `RWR23` writer - XDMAC Channel 23 Read Write Resume Bit"]
pub struct RWR23_W<'a> {
    w: &'a mut W,
}
impl<'a> RWR23_W<'a> {
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
    #[doc = "Bit 0 - XDMAC Channel 0 Read Write Resume Bit"]
    #[inline(always)]
    pub fn rwr0(&mut self) -> RWR0_W {
        RWR0_W { w: self }
    }
    #[doc = "Bit 1 - XDMAC Channel 1 Read Write Resume Bit"]
    #[inline(always)]
    pub fn rwr1(&mut self) -> RWR1_W {
        RWR1_W { w: self }
    }
    #[doc = "Bit 2 - XDMAC Channel 2 Read Write Resume Bit"]
    #[inline(always)]
    pub fn rwr2(&mut self) -> RWR2_W {
        RWR2_W { w: self }
    }
    #[doc = "Bit 3 - XDMAC Channel 3 Read Write Resume Bit"]
    #[inline(always)]
    pub fn rwr3(&mut self) -> RWR3_W {
        RWR3_W { w: self }
    }
    #[doc = "Bit 4 - XDMAC Channel 4 Read Write Resume Bit"]
    #[inline(always)]
    pub fn rwr4(&mut self) -> RWR4_W {
        RWR4_W { w: self }
    }
    #[doc = "Bit 5 - XDMAC Channel 5 Read Write Resume Bit"]
    #[inline(always)]
    pub fn rwr5(&mut self) -> RWR5_W {
        RWR5_W { w: self }
    }
    #[doc = "Bit 6 - XDMAC Channel 6 Read Write Resume Bit"]
    #[inline(always)]
    pub fn rwr6(&mut self) -> RWR6_W {
        RWR6_W { w: self }
    }
    #[doc = "Bit 7 - XDMAC Channel 7 Read Write Resume Bit"]
    #[inline(always)]
    pub fn rwr7(&mut self) -> RWR7_W {
        RWR7_W { w: self }
    }
    #[doc = "Bit 8 - XDMAC Channel 8 Read Write Resume Bit"]
    #[inline(always)]
    pub fn rwr8(&mut self) -> RWR8_W {
        RWR8_W { w: self }
    }
    #[doc = "Bit 9 - XDMAC Channel 9 Read Write Resume Bit"]
    #[inline(always)]
    pub fn rwr9(&mut self) -> RWR9_W {
        RWR9_W { w: self }
    }
    #[doc = "Bit 10 - XDMAC Channel 10 Read Write Resume Bit"]
    #[inline(always)]
    pub fn rwr10(&mut self) -> RWR10_W {
        RWR10_W { w: self }
    }
    #[doc = "Bit 11 - XDMAC Channel 11 Read Write Resume Bit"]
    #[inline(always)]
    pub fn rwr11(&mut self) -> RWR11_W {
        RWR11_W { w: self }
    }
    #[doc = "Bit 12 - XDMAC Channel 12 Read Write Resume Bit"]
    #[inline(always)]
    pub fn rwr12(&mut self) -> RWR12_W {
        RWR12_W { w: self }
    }
    #[doc = "Bit 13 - XDMAC Channel 13 Read Write Resume Bit"]
    #[inline(always)]
    pub fn rwr13(&mut self) -> RWR13_W {
        RWR13_W { w: self }
    }
    #[doc = "Bit 14 - XDMAC Channel 14 Read Write Resume Bit"]
    #[inline(always)]
    pub fn rwr14(&mut self) -> RWR14_W {
        RWR14_W { w: self }
    }
    #[doc = "Bit 15 - XDMAC Channel 15 Read Write Resume Bit"]
    #[inline(always)]
    pub fn rwr15(&mut self) -> RWR15_W {
        RWR15_W { w: self }
    }
    #[doc = "Bit 16 - XDMAC Channel 16 Read Write Resume Bit"]
    #[inline(always)]
    pub fn rwr16(&mut self) -> RWR16_W {
        RWR16_W { w: self }
    }
    #[doc = "Bit 17 - XDMAC Channel 17 Read Write Resume Bit"]
    #[inline(always)]
    pub fn rwr17(&mut self) -> RWR17_W {
        RWR17_W { w: self }
    }
    #[doc = "Bit 18 - XDMAC Channel 18 Read Write Resume Bit"]
    #[inline(always)]
    pub fn rwr18(&mut self) -> RWR18_W {
        RWR18_W { w: self }
    }
    #[doc = "Bit 19 - XDMAC Channel 19 Read Write Resume Bit"]
    #[inline(always)]
    pub fn rwr19(&mut self) -> RWR19_W {
        RWR19_W { w: self }
    }
    #[doc = "Bit 20 - XDMAC Channel 20 Read Write Resume Bit"]
    #[inline(always)]
    pub fn rwr20(&mut self) -> RWR20_W {
        RWR20_W { w: self }
    }
    #[doc = "Bit 21 - XDMAC Channel 21 Read Write Resume Bit"]
    #[inline(always)]
    pub fn rwr21(&mut self) -> RWR21_W {
        RWR21_W { w: self }
    }
    #[doc = "Bit 22 - XDMAC Channel 22 Read Write Resume Bit"]
    #[inline(always)]
    pub fn rwr22(&mut self) -> RWR22_W {
        RWR22_W { w: self }
    }
    #[doc = "Bit 23 - XDMAC Channel 23 Read Write Resume Bit"]
    #[inline(always)]
    pub fn rwr23(&mut self) -> RWR23_W {
        RWR23_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Global Channel Read Write Resume Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [xdmac_grwr](index.html) module"]
pub struct XDMAC_GRWR_SPEC;
impl crate::RegisterSpec for XDMAC_GRWR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [xdmac_grwr::W](W) writer structure"]
impl crate::Writable for XDMAC_GRWR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets XDMAC_GRWR to value 0"]
impl crate::Resettable for XDMAC_GRWR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
