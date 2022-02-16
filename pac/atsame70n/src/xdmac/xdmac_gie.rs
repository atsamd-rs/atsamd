#[doc = "Register `XDMAC_GIE` writer"]
pub struct W(crate::W<XDMAC_GIE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<XDMAC_GIE_SPEC>;
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
impl From<crate::W<XDMAC_GIE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<XDMAC_GIE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IE0` writer - XDMAC Channel 0 Interrupt Enable Bit"]
pub struct IE0_W<'a> {
    w: &'a mut W,
}
impl<'a> IE0_W<'a> {
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
#[doc = "Field `IE1` writer - XDMAC Channel 1 Interrupt Enable Bit"]
pub struct IE1_W<'a> {
    w: &'a mut W,
}
impl<'a> IE1_W<'a> {
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
#[doc = "Field `IE2` writer - XDMAC Channel 2 Interrupt Enable Bit"]
pub struct IE2_W<'a> {
    w: &'a mut W,
}
impl<'a> IE2_W<'a> {
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
#[doc = "Field `IE3` writer - XDMAC Channel 3 Interrupt Enable Bit"]
pub struct IE3_W<'a> {
    w: &'a mut W,
}
impl<'a> IE3_W<'a> {
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
#[doc = "Field `IE4` writer - XDMAC Channel 4 Interrupt Enable Bit"]
pub struct IE4_W<'a> {
    w: &'a mut W,
}
impl<'a> IE4_W<'a> {
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
#[doc = "Field `IE5` writer - XDMAC Channel 5 Interrupt Enable Bit"]
pub struct IE5_W<'a> {
    w: &'a mut W,
}
impl<'a> IE5_W<'a> {
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
#[doc = "Field `IE6` writer - XDMAC Channel 6 Interrupt Enable Bit"]
pub struct IE6_W<'a> {
    w: &'a mut W,
}
impl<'a> IE6_W<'a> {
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
#[doc = "Field `IE7` writer - XDMAC Channel 7 Interrupt Enable Bit"]
pub struct IE7_W<'a> {
    w: &'a mut W,
}
impl<'a> IE7_W<'a> {
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
#[doc = "Field `IE8` writer - XDMAC Channel 8 Interrupt Enable Bit"]
pub struct IE8_W<'a> {
    w: &'a mut W,
}
impl<'a> IE8_W<'a> {
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
#[doc = "Field `IE9` writer - XDMAC Channel 9 Interrupt Enable Bit"]
pub struct IE9_W<'a> {
    w: &'a mut W,
}
impl<'a> IE9_W<'a> {
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
#[doc = "Field `IE10` writer - XDMAC Channel 10 Interrupt Enable Bit"]
pub struct IE10_W<'a> {
    w: &'a mut W,
}
impl<'a> IE10_W<'a> {
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
#[doc = "Field `IE11` writer - XDMAC Channel 11 Interrupt Enable Bit"]
pub struct IE11_W<'a> {
    w: &'a mut W,
}
impl<'a> IE11_W<'a> {
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
#[doc = "Field `IE12` writer - XDMAC Channel 12 Interrupt Enable Bit"]
pub struct IE12_W<'a> {
    w: &'a mut W,
}
impl<'a> IE12_W<'a> {
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
#[doc = "Field `IE13` writer - XDMAC Channel 13 Interrupt Enable Bit"]
pub struct IE13_W<'a> {
    w: &'a mut W,
}
impl<'a> IE13_W<'a> {
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
#[doc = "Field `IE14` writer - XDMAC Channel 14 Interrupt Enable Bit"]
pub struct IE14_W<'a> {
    w: &'a mut W,
}
impl<'a> IE14_W<'a> {
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
#[doc = "Field `IE15` writer - XDMAC Channel 15 Interrupt Enable Bit"]
pub struct IE15_W<'a> {
    w: &'a mut W,
}
impl<'a> IE15_W<'a> {
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
#[doc = "Field `IE16` writer - XDMAC Channel 16 Interrupt Enable Bit"]
pub struct IE16_W<'a> {
    w: &'a mut W,
}
impl<'a> IE16_W<'a> {
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
#[doc = "Field `IE17` writer - XDMAC Channel 17 Interrupt Enable Bit"]
pub struct IE17_W<'a> {
    w: &'a mut W,
}
impl<'a> IE17_W<'a> {
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
#[doc = "Field `IE18` writer - XDMAC Channel 18 Interrupt Enable Bit"]
pub struct IE18_W<'a> {
    w: &'a mut W,
}
impl<'a> IE18_W<'a> {
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
#[doc = "Field `IE19` writer - XDMAC Channel 19 Interrupt Enable Bit"]
pub struct IE19_W<'a> {
    w: &'a mut W,
}
impl<'a> IE19_W<'a> {
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
#[doc = "Field `IE20` writer - XDMAC Channel 20 Interrupt Enable Bit"]
pub struct IE20_W<'a> {
    w: &'a mut W,
}
impl<'a> IE20_W<'a> {
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
#[doc = "Field `IE21` writer - XDMAC Channel 21 Interrupt Enable Bit"]
pub struct IE21_W<'a> {
    w: &'a mut W,
}
impl<'a> IE21_W<'a> {
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
#[doc = "Field `IE22` writer - XDMAC Channel 22 Interrupt Enable Bit"]
pub struct IE22_W<'a> {
    w: &'a mut W,
}
impl<'a> IE22_W<'a> {
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
#[doc = "Field `IE23` writer - XDMAC Channel 23 Interrupt Enable Bit"]
pub struct IE23_W<'a> {
    w: &'a mut W,
}
impl<'a> IE23_W<'a> {
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
    #[doc = "Bit 0 - XDMAC Channel 0 Interrupt Enable Bit"]
    #[inline(always)]
    pub fn ie0(&mut self) -> IE0_W {
        IE0_W { w: self }
    }
    #[doc = "Bit 1 - XDMAC Channel 1 Interrupt Enable Bit"]
    #[inline(always)]
    pub fn ie1(&mut self) -> IE1_W {
        IE1_W { w: self }
    }
    #[doc = "Bit 2 - XDMAC Channel 2 Interrupt Enable Bit"]
    #[inline(always)]
    pub fn ie2(&mut self) -> IE2_W {
        IE2_W { w: self }
    }
    #[doc = "Bit 3 - XDMAC Channel 3 Interrupt Enable Bit"]
    #[inline(always)]
    pub fn ie3(&mut self) -> IE3_W {
        IE3_W { w: self }
    }
    #[doc = "Bit 4 - XDMAC Channel 4 Interrupt Enable Bit"]
    #[inline(always)]
    pub fn ie4(&mut self) -> IE4_W {
        IE4_W { w: self }
    }
    #[doc = "Bit 5 - XDMAC Channel 5 Interrupt Enable Bit"]
    #[inline(always)]
    pub fn ie5(&mut self) -> IE5_W {
        IE5_W { w: self }
    }
    #[doc = "Bit 6 - XDMAC Channel 6 Interrupt Enable Bit"]
    #[inline(always)]
    pub fn ie6(&mut self) -> IE6_W {
        IE6_W { w: self }
    }
    #[doc = "Bit 7 - XDMAC Channel 7 Interrupt Enable Bit"]
    #[inline(always)]
    pub fn ie7(&mut self) -> IE7_W {
        IE7_W { w: self }
    }
    #[doc = "Bit 8 - XDMAC Channel 8 Interrupt Enable Bit"]
    #[inline(always)]
    pub fn ie8(&mut self) -> IE8_W {
        IE8_W { w: self }
    }
    #[doc = "Bit 9 - XDMAC Channel 9 Interrupt Enable Bit"]
    #[inline(always)]
    pub fn ie9(&mut self) -> IE9_W {
        IE9_W { w: self }
    }
    #[doc = "Bit 10 - XDMAC Channel 10 Interrupt Enable Bit"]
    #[inline(always)]
    pub fn ie10(&mut self) -> IE10_W {
        IE10_W { w: self }
    }
    #[doc = "Bit 11 - XDMAC Channel 11 Interrupt Enable Bit"]
    #[inline(always)]
    pub fn ie11(&mut self) -> IE11_W {
        IE11_W { w: self }
    }
    #[doc = "Bit 12 - XDMAC Channel 12 Interrupt Enable Bit"]
    #[inline(always)]
    pub fn ie12(&mut self) -> IE12_W {
        IE12_W { w: self }
    }
    #[doc = "Bit 13 - XDMAC Channel 13 Interrupt Enable Bit"]
    #[inline(always)]
    pub fn ie13(&mut self) -> IE13_W {
        IE13_W { w: self }
    }
    #[doc = "Bit 14 - XDMAC Channel 14 Interrupt Enable Bit"]
    #[inline(always)]
    pub fn ie14(&mut self) -> IE14_W {
        IE14_W { w: self }
    }
    #[doc = "Bit 15 - XDMAC Channel 15 Interrupt Enable Bit"]
    #[inline(always)]
    pub fn ie15(&mut self) -> IE15_W {
        IE15_W { w: self }
    }
    #[doc = "Bit 16 - XDMAC Channel 16 Interrupt Enable Bit"]
    #[inline(always)]
    pub fn ie16(&mut self) -> IE16_W {
        IE16_W { w: self }
    }
    #[doc = "Bit 17 - XDMAC Channel 17 Interrupt Enable Bit"]
    #[inline(always)]
    pub fn ie17(&mut self) -> IE17_W {
        IE17_W { w: self }
    }
    #[doc = "Bit 18 - XDMAC Channel 18 Interrupt Enable Bit"]
    #[inline(always)]
    pub fn ie18(&mut self) -> IE18_W {
        IE18_W { w: self }
    }
    #[doc = "Bit 19 - XDMAC Channel 19 Interrupt Enable Bit"]
    #[inline(always)]
    pub fn ie19(&mut self) -> IE19_W {
        IE19_W { w: self }
    }
    #[doc = "Bit 20 - XDMAC Channel 20 Interrupt Enable Bit"]
    #[inline(always)]
    pub fn ie20(&mut self) -> IE20_W {
        IE20_W { w: self }
    }
    #[doc = "Bit 21 - XDMAC Channel 21 Interrupt Enable Bit"]
    #[inline(always)]
    pub fn ie21(&mut self) -> IE21_W {
        IE21_W { w: self }
    }
    #[doc = "Bit 22 - XDMAC Channel 22 Interrupt Enable Bit"]
    #[inline(always)]
    pub fn ie22(&mut self) -> IE22_W {
        IE22_W { w: self }
    }
    #[doc = "Bit 23 - XDMAC Channel 23 Interrupt Enable Bit"]
    #[inline(always)]
    pub fn ie23(&mut self) -> IE23_W {
        IE23_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Global Interrupt Enable Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [xdmac_gie](index.html) module"]
pub struct XDMAC_GIE_SPEC;
impl crate::RegisterSpec for XDMAC_GIE_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [xdmac_gie::W](W) writer structure"]
impl crate::Writable for XDMAC_GIE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets XDMAC_GIE to value 0"]
impl crate::Resettable for XDMAC_GIE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
