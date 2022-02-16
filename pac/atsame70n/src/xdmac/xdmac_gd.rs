#[doc = "Register `XDMAC_GD` writer"]
pub struct W(crate::W<XDMAC_GD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<XDMAC_GD_SPEC>;
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
impl From<crate::W<XDMAC_GD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<XDMAC_GD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DI0` writer - XDMAC Channel 0 Disable Bit"]
pub struct DI0_W<'a> {
    w: &'a mut W,
}
impl<'a> DI0_W<'a> {
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
#[doc = "Field `DI1` writer - XDMAC Channel 1 Disable Bit"]
pub struct DI1_W<'a> {
    w: &'a mut W,
}
impl<'a> DI1_W<'a> {
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
#[doc = "Field `DI2` writer - XDMAC Channel 2 Disable Bit"]
pub struct DI2_W<'a> {
    w: &'a mut W,
}
impl<'a> DI2_W<'a> {
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
#[doc = "Field `DI3` writer - XDMAC Channel 3 Disable Bit"]
pub struct DI3_W<'a> {
    w: &'a mut W,
}
impl<'a> DI3_W<'a> {
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
#[doc = "Field `DI4` writer - XDMAC Channel 4 Disable Bit"]
pub struct DI4_W<'a> {
    w: &'a mut W,
}
impl<'a> DI4_W<'a> {
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
#[doc = "Field `DI5` writer - XDMAC Channel 5 Disable Bit"]
pub struct DI5_W<'a> {
    w: &'a mut W,
}
impl<'a> DI5_W<'a> {
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
#[doc = "Field `DI6` writer - XDMAC Channel 6 Disable Bit"]
pub struct DI6_W<'a> {
    w: &'a mut W,
}
impl<'a> DI6_W<'a> {
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
#[doc = "Field `DI7` writer - XDMAC Channel 7 Disable Bit"]
pub struct DI7_W<'a> {
    w: &'a mut W,
}
impl<'a> DI7_W<'a> {
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
#[doc = "Field `DI8` writer - XDMAC Channel 8 Disable Bit"]
pub struct DI8_W<'a> {
    w: &'a mut W,
}
impl<'a> DI8_W<'a> {
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
#[doc = "Field `DI9` writer - XDMAC Channel 9 Disable Bit"]
pub struct DI9_W<'a> {
    w: &'a mut W,
}
impl<'a> DI9_W<'a> {
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
#[doc = "Field `DI10` writer - XDMAC Channel 10 Disable Bit"]
pub struct DI10_W<'a> {
    w: &'a mut W,
}
impl<'a> DI10_W<'a> {
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
#[doc = "Field `DI11` writer - XDMAC Channel 11 Disable Bit"]
pub struct DI11_W<'a> {
    w: &'a mut W,
}
impl<'a> DI11_W<'a> {
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
#[doc = "Field `DI12` writer - XDMAC Channel 12 Disable Bit"]
pub struct DI12_W<'a> {
    w: &'a mut W,
}
impl<'a> DI12_W<'a> {
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
#[doc = "Field `DI13` writer - XDMAC Channel 13 Disable Bit"]
pub struct DI13_W<'a> {
    w: &'a mut W,
}
impl<'a> DI13_W<'a> {
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
#[doc = "Field `DI14` writer - XDMAC Channel 14 Disable Bit"]
pub struct DI14_W<'a> {
    w: &'a mut W,
}
impl<'a> DI14_W<'a> {
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
#[doc = "Field `DI15` writer - XDMAC Channel 15 Disable Bit"]
pub struct DI15_W<'a> {
    w: &'a mut W,
}
impl<'a> DI15_W<'a> {
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
#[doc = "Field `DI16` writer - XDMAC Channel 16 Disable Bit"]
pub struct DI16_W<'a> {
    w: &'a mut W,
}
impl<'a> DI16_W<'a> {
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
#[doc = "Field `DI17` writer - XDMAC Channel 17 Disable Bit"]
pub struct DI17_W<'a> {
    w: &'a mut W,
}
impl<'a> DI17_W<'a> {
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
#[doc = "Field `DI18` writer - XDMAC Channel 18 Disable Bit"]
pub struct DI18_W<'a> {
    w: &'a mut W,
}
impl<'a> DI18_W<'a> {
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
#[doc = "Field `DI19` writer - XDMAC Channel 19 Disable Bit"]
pub struct DI19_W<'a> {
    w: &'a mut W,
}
impl<'a> DI19_W<'a> {
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
#[doc = "Field `DI20` writer - XDMAC Channel 20 Disable Bit"]
pub struct DI20_W<'a> {
    w: &'a mut W,
}
impl<'a> DI20_W<'a> {
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
#[doc = "Field `DI21` writer - XDMAC Channel 21 Disable Bit"]
pub struct DI21_W<'a> {
    w: &'a mut W,
}
impl<'a> DI21_W<'a> {
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
#[doc = "Field `DI22` writer - XDMAC Channel 22 Disable Bit"]
pub struct DI22_W<'a> {
    w: &'a mut W,
}
impl<'a> DI22_W<'a> {
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
#[doc = "Field `DI23` writer - XDMAC Channel 23 Disable Bit"]
pub struct DI23_W<'a> {
    w: &'a mut W,
}
impl<'a> DI23_W<'a> {
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
    #[doc = "Bit 0 - XDMAC Channel 0 Disable Bit"]
    #[inline(always)]
    pub fn di0(&mut self) -> DI0_W {
        DI0_W { w: self }
    }
    #[doc = "Bit 1 - XDMAC Channel 1 Disable Bit"]
    #[inline(always)]
    pub fn di1(&mut self) -> DI1_W {
        DI1_W { w: self }
    }
    #[doc = "Bit 2 - XDMAC Channel 2 Disable Bit"]
    #[inline(always)]
    pub fn di2(&mut self) -> DI2_W {
        DI2_W { w: self }
    }
    #[doc = "Bit 3 - XDMAC Channel 3 Disable Bit"]
    #[inline(always)]
    pub fn di3(&mut self) -> DI3_W {
        DI3_W { w: self }
    }
    #[doc = "Bit 4 - XDMAC Channel 4 Disable Bit"]
    #[inline(always)]
    pub fn di4(&mut self) -> DI4_W {
        DI4_W { w: self }
    }
    #[doc = "Bit 5 - XDMAC Channel 5 Disable Bit"]
    #[inline(always)]
    pub fn di5(&mut self) -> DI5_W {
        DI5_W { w: self }
    }
    #[doc = "Bit 6 - XDMAC Channel 6 Disable Bit"]
    #[inline(always)]
    pub fn di6(&mut self) -> DI6_W {
        DI6_W { w: self }
    }
    #[doc = "Bit 7 - XDMAC Channel 7 Disable Bit"]
    #[inline(always)]
    pub fn di7(&mut self) -> DI7_W {
        DI7_W { w: self }
    }
    #[doc = "Bit 8 - XDMAC Channel 8 Disable Bit"]
    #[inline(always)]
    pub fn di8(&mut self) -> DI8_W {
        DI8_W { w: self }
    }
    #[doc = "Bit 9 - XDMAC Channel 9 Disable Bit"]
    #[inline(always)]
    pub fn di9(&mut self) -> DI9_W {
        DI9_W { w: self }
    }
    #[doc = "Bit 10 - XDMAC Channel 10 Disable Bit"]
    #[inline(always)]
    pub fn di10(&mut self) -> DI10_W {
        DI10_W { w: self }
    }
    #[doc = "Bit 11 - XDMAC Channel 11 Disable Bit"]
    #[inline(always)]
    pub fn di11(&mut self) -> DI11_W {
        DI11_W { w: self }
    }
    #[doc = "Bit 12 - XDMAC Channel 12 Disable Bit"]
    #[inline(always)]
    pub fn di12(&mut self) -> DI12_W {
        DI12_W { w: self }
    }
    #[doc = "Bit 13 - XDMAC Channel 13 Disable Bit"]
    #[inline(always)]
    pub fn di13(&mut self) -> DI13_W {
        DI13_W { w: self }
    }
    #[doc = "Bit 14 - XDMAC Channel 14 Disable Bit"]
    #[inline(always)]
    pub fn di14(&mut self) -> DI14_W {
        DI14_W { w: self }
    }
    #[doc = "Bit 15 - XDMAC Channel 15 Disable Bit"]
    #[inline(always)]
    pub fn di15(&mut self) -> DI15_W {
        DI15_W { w: self }
    }
    #[doc = "Bit 16 - XDMAC Channel 16 Disable Bit"]
    #[inline(always)]
    pub fn di16(&mut self) -> DI16_W {
        DI16_W { w: self }
    }
    #[doc = "Bit 17 - XDMAC Channel 17 Disable Bit"]
    #[inline(always)]
    pub fn di17(&mut self) -> DI17_W {
        DI17_W { w: self }
    }
    #[doc = "Bit 18 - XDMAC Channel 18 Disable Bit"]
    #[inline(always)]
    pub fn di18(&mut self) -> DI18_W {
        DI18_W { w: self }
    }
    #[doc = "Bit 19 - XDMAC Channel 19 Disable Bit"]
    #[inline(always)]
    pub fn di19(&mut self) -> DI19_W {
        DI19_W { w: self }
    }
    #[doc = "Bit 20 - XDMAC Channel 20 Disable Bit"]
    #[inline(always)]
    pub fn di20(&mut self) -> DI20_W {
        DI20_W { w: self }
    }
    #[doc = "Bit 21 - XDMAC Channel 21 Disable Bit"]
    #[inline(always)]
    pub fn di21(&mut self) -> DI21_W {
        DI21_W { w: self }
    }
    #[doc = "Bit 22 - XDMAC Channel 22 Disable Bit"]
    #[inline(always)]
    pub fn di22(&mut self) -> DI22_W {
        DI22_W { w: self }
    }
    #[doc = "Bit 23 - XDMAC Channel 23 Disable Bit"]
    #[inline(always)]
    pub fn di23(&mut self) -> DI23_W {
        DI23_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Global Channel Disable Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [xdmac_gd](index.html) module"]
pub struct XDMAC_GD_SPEC;
impl crate::RegisterSpec for XDMAC_GD_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [xdmac_gd::W](W) writer structure"]
impl crate::Writable for XDMAC_GD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets XDMAC_GD to value 0"]
impl crate::Resettable for XDMAC_GD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
