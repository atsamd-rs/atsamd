#[doc = "Register `XDMAC_GE` writer"]
pub struct W(crate::W<XDMAC_GE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<XDMAC_GE_SPEC>;
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
impl From<crate::W<XDMAC_GE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<XDMAC_GE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EN0` writer - XDMAC Channel 0 Enable Bit"]
pub struct EN0_W<'a> {
    w: &'a mut W,
}
impl<'a> EN0_W<'a> {
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
#[doc = "Field `EN1` writer - XDMAC Channel 1 Enable Bit"]
pub struct EN1_W<'a> {
    w: &'a mut W,
}
impl<'a> EN1_W<'a> {
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
#[doc = "Field `EN2` writer - XDMAC Channel 2 Enable Bit"]
pub struct EN2_W<'a> {
    w: &'a mut W,
}
impl<'a> EN2_W<'a> {
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
#[doc = "Field `EN3` writer - XDMAC Channel 3 Enable Bit"]
pub struct EN3_W<'a> {
    w: &'a mut W,
}
impl<'a> EN3_W<'a> {
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
#[doc = "Field `EN4` writer - XDMAC Channel 4 Enable Bit"]
pub struct EN4_W<'a> {
    w: &'a mut W,
}
impl<'a> EN4_W<'a> {
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
#[doc = "Field `EN5` writer - XDMAC Channel 5 Enable Bit"]
pub struct EN5_W<'a> {
    w: &'a mut W,
}
impl<'a> EN5_W<'a> {
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
#[doc = "Field `EN6` writer - XDMAC Channel 6 Enable Bit"]
pub struct EN6_W<'a> {
    w: &'a mut W,
}
impl<'a> EN6_W<'a> {
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
#[doc = "Field `EN7` writer - XDMAC Channel 7 Enable Bit"]
pub struct EN7_W<'a> {
    w: &'a mut W,
}
impl<'a> EN7_W<'a> {
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
#[doc = "Field `EN8` writer - XDMAC Channel 8 Enable Bit"]
pub struct EN8_W<'a> {
    w: &'a mut W,
}
impl<'a> EN8_W<'a> {
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
#[doc = "Field `EN9` writer - XDMAC Channel 9 Enable Bit"]
pub struct EN9_W<'a> {
    w: &'a mut W,
}
impl<'a> EN9_W<'a> {
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
#[doc = "Field `EN10` writer - XDMAC Channel 10 Enable Bit"]
pub struct EN10_W<'a> {
    w: &'a mut W,
}
impl<'a> EN10_W<'a> {
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
#[doc = "Field `EN11` writer - XDMAC Channel 11 Enable Bit"]
pub struct EN11_W<'a> {
    w: &'a mut W,
}
impl<'a> EN11_W<'a> {
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
#[doc = "Field `EN12` writer - XDMAC Channel 12 Enable Bit"]
pub struct EN12_W<'a> {
    w: &'a mut W,
}
impl<'a> EN12_W<'a> {
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
#[doc = "Field `EN13` writer - XDMAC Channel 13 Enable Bit"]
pub struct EN13_W<'a> {
    w: &'a mut W,
}
impl<'a> EN13_W<'a> {
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
#[doc = "Field `EN14` writer - XDMAC Channel 14 Enable Bit"]
pub struct EN14_W<'a> {
    w: &'a mut W,
}
impl<'a> EN14_W<'a> {
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
#[doc = "Field `EN15` writer - XDMAC Channel 15 Enable Bit"]
pub struct EN15_W<'a> {
    w: &'a mut W,
}
impl<'a> EN15_W<'a> {
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
#[doc = "Field `EN16` writer - XDMAC Channel 16 Enable Bit"]
pub struct EN16_W<'a> {
    w: &'a mut W,
}
impl<'a> EN16_W<'a> {
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
#[doc = "Field `EN17` writer - XDMAC Channel 17 Enable Bit"]
pub struct EN17_W<'a> {
    w: &'a mut W,
}
impl<'a> EN17_W<'a> {
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
#[doc = "Field `EN18` writer - XDMAC Channel 18 Enable Bit"]
pub struct EN18_W<'a> {
    w: &'a mut W,
}
impl<'a> EN18_W<'a> {
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
#[doc = "Field `EN19` writer - XDMAC Channel 19 Enable Bit"]
pub struct EN19_W<'a> {
    w: &'a mut W,
}
impl<'a> EN19_W<'a> {
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
#[doc = "Field `EN20` writer - XDMAC Channel 20 Enable Bit"]
pub struct EN20_W<'a> {
    w: &'a mut W,
}
impl<'a> EN20_W<'a> {
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
#[doc = "Field `EN21` writer - XDMAC Channel 21 Enable Bit"]
pub struct EN21_W<'a> {
    w: &'a mut W,
}
impl<'a> EN21_W<'a> {
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
#[doc = "Field `EN22` writer - XDMAC Channel 22 Enable Bit"]
pub struct EN22_W<'a> {
    w: &'a mut W,
}
impl<'a> EN22_W<'a> {
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
#[doc = "Field `EN23` writer - XDMAC Channel 23 Enable Bit"]
pub struct EN23_W<'a> {
    w: &'a mut W,
}
impl<'a> EN23_W<'a> {
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
    #[doc = "Bit 0 - XDMAC Channel 0 Enable Bit"]
    #[inline(always)]
    pub fn en0(&mut self) -> EN0_W {
        EN0_W { w: self }
    }
    #[doc = "Bit 1 - XDMAC Channel 1 Enable Bit"]
    #[inline(always)]
    pub fn en1(&mut self) -> EN1_W {
        EN1_W { w: self }
    }
    #[doc = "Bit 2 - XDMAC Channel 2 Enable Bit"]
    #[inline(always)]
    pub fn en2(&mut self) -> EN2_W {
        EN2_W { w: self }
    }
    #[doc = "Bit 3 - XDMAC Channel 3 Enable Bit"]
    #[inline(always)]
    pub fn en3(&mut self) -> EN3_W {
        EN3_W { w: self }
    }
    #[doc = "Bit 4 - XDMAC Channel 4 Enable Bit"]
    #[inline(always)]
    pub fn en4(&mut self) -> EN4_W {
        EN4_W { w: self }
    }
    #[doc = "Bit 5 - XDMAC Channel 5 Enable Bit"]
    #[inline(always)]
    pub fn en5(&mut self) -> EN5_W {
        EN5_W { w: self }
    }
    #[doc = "Bit 6 - XDMAC Channel 6 Enable Bit"]
    #[inline(always)]
    pub fn en6(&mut self) -> EN6_W {
        EN6_W { w: self }
    }
    #[doc = "Bit 7 - XDMAC Channel 7 Enable Bit"]
    #[inline(always)]
    pub fn en7(&mut self) -> EN7_W {
        EN7_W { w: self }
    }
    #[doc = "Bit 8 - XDMAC Channel 8 Enable Bit"]
    #[inline(always)]
    pub fn en8(&mut self) -> EN8_W {
        EN8_W { w: self }
    }
    #[doc = "Bit 9 - XDMAC Channel 9 Enable Bit"]
    #[inline(always)]
    pub fn en9(&mut self) -> EN9_W {
        EN9_W { w: self }
    }
    #[doc = "Bit 10 - XDMAC Channel 10 Enable Bit"]
    #[inline(always)]
    pub fn en10(&mut self) -> EN10_W {
        EN10_W { w: self }
    }
    #[doc = "Bit 11 - XDMAC Channel 11 Enable Bit"]
    #[inline(always)]
    pub fn en11(&mut self) -> EN11_W {
        EN11_W { w: self }
    }
    #[doc = "Bit 12 - XDMAC Channel 12 Enable Bit"]
    #[inline(always)]
    pub fn en12(&mut self) -> EN12_W {
        EN12_W { w: self }
    }
    #[doc = "Bit 13 - XDMAC Channel 13 Enable Bit"]
    #[inline(always)]
    pub fn en13(&mut self) -> EN13_W {
        EN13_W { w: self }
    }
    #[doc = "Bit 14 - XDMAC Channel 14 Enable Bit"]
    #[inline(always)]
    pub fn en14(&mut self) -> EN14_W {
        EN14_W { w: self }
    }
    #[doc = "Bit 15 - XDMAC Channel 15 Enable Bit"]
    #[inline(always)]
    pub fn en15(&mut self) -> EN15_W {
        EN15_W { w: self }
    }
    #[doc = "Bit 16 - XDMAC Channel 16 Enable Bit"]
    #[inline(always)]
    pub fn en16(&mut self) -> EN16_W {
        EN16_W { w: self }
    }
    #[doc = "Bit 17 - XDMAC Channel 17 Enable Bit"]
    #[inline(always)]
    pub fn en17(&mut self) -> EN17_W {
        EN17_W { w: self }
    }
    #[doc = "Bit 18 - XDMAC Channel 18 Enable Bit"]
    #[inline(always)]
    pub fn en18(&mut self) -> EN18_W {
        EN18_W { w: self }
    }
    #[doc = "Bit 19 - XDMAC Channel 19 Enable Bit"]
    #[inline(always)]
    pub fn en19(&mut self) -> EN19_W {
        EN19_W { w: self }
    }
    #[doc = "Bit 20 - XDMAC Channel 20 Enable Bit"]
    #[inline(always)]
    pub fn en20(&mut self) -> EN20_W {
        EN20_W { w: self }
    }
    #[doc = "Bit 21 - XDMAC Channel 21 Enable Bit"]
    #[inline(always)]
    pub fn en21(&mut self) -> EN21_W {
        EN21_W { w: self }
    }
    #[doc = "Bit 22 - XDMAC Channel 22 Enable Bit"]
    #[inline(always)]
    pub fn en22(&mut self) -> EN22_W {
        EN22_W { w: self }
    }
    #[doc = "Bit 23 - XDMAC Channel 23 Enable Bit"]
    #[inline(always)]
    pub fn en23(&mut self) -> EN23_W {
        EN23_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Global Channel Enable Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [xdmac_ge](index.html) module"]
pub struct XDMAC_GE_SPEC;
impl crate::RegisterSpec for XDMAC_GE_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [xdmac_ge::W](W) writer structure"]
impl crate::Writable for XDMAC_GE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets XDMAC_GE to value 0"]
impl crate::Resettable for XDMAC_GE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
