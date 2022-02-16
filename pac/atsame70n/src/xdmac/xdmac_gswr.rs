#[doc = "Register `XDMAC_GSWR` writer"]
pub struct W(crate::W<XDMAC_GSWR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<XDMAC_GSWR_SPEC>;
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
impl From<crate::W<XDMAC_GSWR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<XDMAC_GSWR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SWREQ0` writer - XDMAC Channel 0 Software Request Bit"]
pub struct SWREQ0_W<'a> {
    w: &'a mut W,
}
impl<'a> SWREQ0_W<'a> {
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
#[doc = "Field `SWREQ1` writer - XDMAC Channel 1 Software Request Bit"]
pub struct SWREQ1_W<'a> {
    w: &'a mut W,
}
impl<'a> SWREQ1_W<'a> {
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
#[doc = "Field `SWREQ2` writer - XDMAC Channel 2 Software Request Bit"]
pub struct SWREQ2_W<'a> {
    w: &'a mut W,
}
impl<'a> SWREQ2_W<'a> {
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
#[doc = "Field `SWREQ3` writer - XDMAC Channel 3 Software Request Bit"]
pub struct SWREQ3_W<'a> {
    w: &'a mut W,
}
impl<'a> SWREQ3_W<'a> {
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
#[doc = "Field `SWREQ4` writer - XDMAC Channel 4 Software Request Bit"]
pub struct SWREQ4_W<'a> {
    w: &'a mut W,
}
impl<'a> SWREQ4_W<'a> {
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
#[doc = "Field `SWREQ5` writer - XDMAC Channel 5 Software Request Bit"]
pub struct SWREQ5_W<'a> {
    w: &'a mut W,
}
impl<'a> SWREQ5_W<'a> {
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
#[doc = "Field `SWREQ6` writer - XDMAC Channel 6 Software Request Bit"]
pub struct SWREQ6_W<'a> {
    w: &'a mut W,
}
impl<'a> SWREQ6_W<'a> {
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
#[doc = "Field `SWREQ7` writer - XDMAC Channel 7 Software Request Bit"]
pub struct SWREQ7_W<'a> {
    w: &'a mut W,
}
impl<'a> SWREQ7_W<'a> {
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
#[doc = "Field `SWREQ8` writer - XDMAC Channel 8 Software Request Bit"]
pub struct SWREQ8_W<'a> {
    w: &'a mut W,
}
impl<'a> SWREQ8_W<'a> {
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
#[doc = "Field `SWREQ9` writer - XDMAC Channel 9 Software Request Bit"]
pub struct SWREQ9_W<'a> {
    w: &'a mut W,
}
impl<'a> SWREQ9_W<'a> {
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
#[doc = "Field `SWREQ10` writer - XDMAC Channel 10 Software Request Bit"]
pub struct SWREQ10_W<'a> {
    w: &'a mut W,
}
impl<'a> SWREQ10_W<'a> {
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
#[doc = "Field `SWREQ11` writer - XDMAC Channel 11 Software Request Bit"]
pub struct SWREQ11_W<'a> {
    w: &'a mut W,
}
impl<'a> SWREQ11_W<'a> {
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
#[doc = "Field `SWREQ12` writer - XDMAC Channel 12 Software Request Bit"]
pub struct SWREQ12_W<'a> {
    w: &'a mut W,
}
impl<'a> SWREQ12_W<'a> {
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
#[doc = "Field `SWREQ13` writer - XDMAC Channel 13 Software Request Bit"]
pub struct SWREQ13_W<'a> {
    w: &'a mut W,
}
impl<'a> SWREQ13_W<'a> {
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
#[doc = "Field `SWREQ14` writer - XDMAC Channel 14 Software Request Bit"]
pub struct SWREQ14_W<'a> {
    w: &'a mut W,
}
impl<'a> SWREQ14_W<'a> {
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
#[doc = "Field `SWREQ15` writer - XDMAC Channel 15 Software Request Bit"]
pub struct SWREQ15_W<'a> {
    w: &'a mut W,
}
impl<'a> SWREQ15_W<'a> {
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
#[doc = "Field `SWREQ16` writer - XDMAC Channel 16 Software Request Bit"]
pub struct SWREQ16_W<'a> {
    w: &'a mut W,
}
impl<'a> SWREQ16_W<'a> {
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
#[doc = "Field `SWREQ17` writer - XDMAC Channel 17 Software Request Bit"]
pub struct SWREQ17_W<'a> {
    w: &'a mut W,
}
impl<'a> SWREQ17_W<'a> {
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
#[doc = "Field `SWREQ18` writer - XDMAC Channel 18 Software Request Bit"]
pub struct SWREQ18_W<'a> {
    w: &'a mut W,
}
impl<'a> SWREQ18_W<'a> {
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
#[doc = "Field `SWREQ19` writer - XDMAC Channel 19 Software Request Bit"]
pub struct SWREQ19_W<'a> {
    w: &'a mut W,
}
impl<'a> SWREQ19_W<'a> {
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
#[doc = "Field `SWREQ20` writer - XDMAC Channel 20 Software Request Bit"]
pub struct SWREQ20_W<'a> {
    w: &'a mut W,
}
impl<'a> SWREQ20_W<'a> {
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
#[doc = "Field `SWREQ21` writer - XDMAC Channel 21 Software Request Bit"]
pub struct SWREQ21_W<'a> {
    w: &'a mut W,
}
impl<'a> SWREQ21_W<'a> {
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
#[doc = "Field `SWREQ22` writer - XDMAC Channel 22 Software Request Bit"]
pub struct SWREQ22_W<'a> {
    w: &'a mut W,
}
impl<'a> SWREQ22_W<'a> {
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
#[doc = "Field `SWREQ23` writer - XDMAC Channel 23 Software Request Bit"]
pub struct SWREQ23_W<'a> {
    w: &'a mut W,
}
impl<'a> SWREQ23_W<'a> {
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
    #[doc = "Bit 0 - XDMAC Channel 0 Software Request Bit"]
    #[inline(always)]
    pub fn swreq0(&mut self) -> SWREQ0_W {
        SWREQ0_W { w: self }
    }
    #[doc = "Bit 1 - XDMAC Channel 1 Software Request Bit"]
    #[inline(always)]
    pub fn swreq1(&mut self) -> SWREQ1_W {
        SWREQ1_W { w: self }
    }
    #[doc = "Bit 2 - XDMAC Channel 2 Software Request Bit"]
    #[inline(always)]
    pub fn swreq2(&mut self) -> SWREQ2_W {
        SWREQ2_W { w: self }
    }
    #[doc = "Bit 3 - XDMAC Channel 3 Software Request Bit"]
    #[inline(always)]
    pub fn swreq3(&mut self) -> SWREQ3_W {
        SWREQ3_W { w: self }
    }
    #[doc = "Bit 4 - XDMAC Channel 4 Software Request Bit"]
    #[inline(always)]
    pub fn swreq4(&mut self) -> SWREQ4_W {
        SWREQ4_W { w: self }
    }
    #[doc = "Bit 5 - XDMAC Channel 5 Software Request Bit"]
    #[inline(always)]
    pub fn swreq5(&mut self) -> SWREQ5_W {
        SWREQ5_W { w: self }
    }
    #[doc = "Bit 6 - XDMAC Channel 6 Software Request Bit"]
    #[inline(always)]
    pub fn swreq6(&mut self) -> SWREQ6_W {
        SWREQ6_W { w: self }
    }
    #[doc = "Bit 7 - XDMAC Channel 7 Software Request Bit"]
    #[inline(always)]
    pub fn swreq7(&mut self) -> SWREQ7_W {
        SWREQ7_W { w: self }
    }
    #[doc = "Bit 8 - XDMAC Channel 8 Software Request Bit"]
    #[inline(always)]
    pub fn swreq8(&mut self) -> SWREQ8_W {
        SWREQ8_W { w: self }
    }
    #[doc = "Bit 9 - XDMAC Channel 9 Software Request Bit"]
    #[inline(always)]
    pub fn swreq9(&mut self) -> SWREQ9_W {
        SWREQ9_W { w: self }
    }
    #[doc = "Bit 10 - XDMAC Channel 10 Software Request Bit"]
    #[inline(always)]
    pub fn swreq10(&mut self) -> SWREQ10_W {
        SWREQ10_W { w: self }
    }
    #[doc = "Bit 11 - XDMAC Channel 11 Software Request Bit"]
    #[inline(always)]
    pub fn swreq11(&mut self) -> SWREQ11_W {
        SWREQ11_W { w: self }
    }
    #[doc = "Bit 12 - XDMAC Channel 12 Software Request Bit"]
    #[inline(always)]
    pub fn swreq12(&mut self) -> SWREQ12_W {
        SWREQ12_W { w: self }
    }
    #[doc = "Bit 13 - XDMAC Channel 13 Software Request Bit"]
    #[inline(always)]
    pub fn swreq13(&mut self) -> SWREQ13_W {
        SWREQ13_W { w: self }
    }
    #[doc = "Bit 14 - XDMAC Channel 14 Software Request Bit"]
    #[inline(always)]
    pub fn swreq14(&mut self) -> SWREQ14_W {
        SWREQ14_W { w: self }
    }
    #[doc = "Bit 15 - XDMAC Channel 15 Software Request Bit"]
    #[inline(always)]
    pub fn swreq15(&mut self) -> SWREQ15_W {
        SWREQ15_W { w: self }
    }
    #[doc = "Bit 16 - XDMAC Channel 16 Software Request Bit"]
    #[inline(always)]
    pub fn swreq16(&mut self) -> SWREQ16_W {
        SWREQ16_W { w: self }
    }
    #[doc = "Bit 17 - XDMAC Channel 17 Software Request Bit"]
    #[inline(always)]
    pub fn swreq17(&mut self) -> SWREQ17_W {
        SWREQ17_W { w: self }
    }
    #[doc = "Bit 18 - XDMAC Channel 18 Software Request Bit"]
    #[inline(always)]
    pub fn swreq18(&mut self) -> SWREQ18_W {
        SWREQ18_W { w: self }
    }
    #[doc = "Bit 19 - XDMAC Channel 19 Software Request Bit"]
    #[inline(always)]
    pub fn swreq19(&mut self) -> SWREQ19_W {
        SWREQ19_W { w: self }
    }
    #[doc = "Bit 20 - XDMAC Channel 20 Software Request Bit"]
    #[inline(always)]
    pub fn swreq20(&mut self) -> SWREQ20_W {
        SWREQ20_W { w: self }
    }
    #[doc = "Bit 21 - XDMAC Channel 21 Software Request Bit"]
    #[inline(always)]
    pub fn swreq21(&mut self) -> SWREQ21_W {
        SWREQ21_W { w: self }
    }
    #[doc = "Bit 22 - XDMAC Channel 22 Software Request Bit"]
    #[inline(always)]
    pub fn swreq22(&mut self) -> SWREQ22_W {
        SWREQ22_W { w: self }
    }
    #[doc = "Bit 23 - XDMAC Channel 23 Software Request Bit"]
    #[inline(always)]
    pub fn swreq23(&mut self) -> SWREQ23_W {
        SWREQ23_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Global Channel Software Request Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [xdmac_gswr](index.html) module"]
pub struct XDMAC_GSWR_SPEC;
impl crate::RegisterSpec for XDMAC_GSWR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [xdmac_gswr::W](W) writer structure"]
impl crate::Writable for XDMAC_GSWR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets XDMAC_GSWR to value 0"]
impl crate::Resettable for XDMAC_GSWR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
