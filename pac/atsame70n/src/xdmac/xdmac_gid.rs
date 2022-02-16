#[doc = "Register `XDMAC_GID` writer"]
pub struct W(crate::W<XDMAC_GID_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<XDMAC_GID_SPEC>;
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
impl From<crate::W<XDMAC_GID_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<XDMAC_GID_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ID0` writer - XDMAC Channel 0 Interrupt Disable Bit"]
pub struct ID0_W<'a> {
    w: &'a mut W,
}
impl<'a> ID0_W<'a> {
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
#[doc = "Field `ID1` writer - XDMAC Channel 1 Interrupt Disable Bit"]
pub struct ID1_W<'a> {
    w: &'a mut W,
}
impl<'a> ID1_W<'a> {
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
#[doc = "Field `ID2` writer - XDMAC Channel 2 Interrupt Disable Bit"]
pub struct ID2_W<'a> {
    w: &'a mut W,
}
impl<'a> ID2_W<'a> {
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
#[doc = "Field `ID3` writer - XDMAC Channel 3 Interrupt Disable Bit"]
pub struct ID3_W<'a> {
    w: &'a mut W,
}
impl<'a> ID3_W<'a> {
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
#[doc = "Field `ID4` writer - XDMAC Channel 4 Interrupt Disable Bit"]
pub struct ID4_W<'a> {
    w: &'a mut W,
}
impl<'a> ID4_W<'a> {
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
#[doc = "Field `ID5` writer - XDMAC Channel 5 Interrupt Disable Bit"]
pub struct ID5_W<'a> {
    w: &'a mut W,
}
impl<'a> ID5_W<'a> {
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
#[doc = "Field `ID6` writer - XDMAC Channel 6 Interrupt Disable Bit"]
pub struct ID6_W<'a> {
    w: &'a mut W,
}
impl<'a> ID6_W<'a> {
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
#[doc = "Field `ID7` writer - XDMAC Channel 7 Interrupt Disable Bit"]
pub struct ID7_W<'a> {
    w: &'a mut W,
}
impl<'a> ID7_W<'a> {
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
#[doc = "Field `ID8` writer - XDMAC Channel 8 Interrupt Disable Bit"]
pub struct ID8_W<'a> {
    w: &'a mut W,
}
impl<'a> ID8_W<'a> {
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
#[doc = "Field `ID9` writer - XDMAC Channel 9 Interrupt Disable Bit"]
pub struct ID9_W<'a> {
    w: &'a mut W,
}
impl<'a> ID9_W<'a> {
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
#[doc = "Field `ID10` writer - XDMAC Channel 10 Interrupt Disable Bit"]
pub struct ID10_W<'a> {
    w: &'a mut W,
}
impl<'a> ID10_W<'a> {
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
#[doc = "Field `ID11` writer - XDMAC Channel 11 Interrupt Disable Bit"]
pub struct ID11_W<'a> {
    w: &'a mut W,
}
impl<'a> ID11_W<'a> {
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
#[doc = "Field `ID12` writer - XDMAC Channel 12 Interrupt Disable Bit"]
pub struct ID12_W<'a> {
    w: &'a mut W,
}
impl<'a> ID12_W<'a> {
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
#[doc = "Field `ID13` writer - XDMAC Channel 13 Interrupt Disable Bit"]
pub struct ID13_W<'a> {
    w: &'a mut W,
}
impl<'a> ID13_W<'a> {
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
#[doc = "Field `ID14` writer - XDMAC Channel 14 Interrupt Disable Bit"]
pub struct ID14_W<'a> {
    w: &'a mut W,
}
impl<'a> ID14_W<'a> {
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
#[doc = "Field `ID15` writer - XDMAC Channel 15 Interrupt Disable Bit"]
pub struct ID15_W<'a> {
    w: &'a mut W,
}
impl<'a> ID15_W<'a> {
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
#[doc = "Field `ID16` writer - XDMAC Channel 16 Interrupt Disable Bit"]
pub struct ID16_W<'a> {
    w: &'a mut W,
}
impl<'a> ID16_W<'a> {
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
#[doc = "Field `ID17` writer - XDMAC Channel 17 Interrupt Disable Bit"]
pub struct ID17_W<'a> {
    w: &'a mut W,
}
impl<'a> ID17_W<'a> {
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
#[doc = "Field `ID18` writer - XDMAC Channel 18 Interrupt Disable Bit"]
pub struct ID18_W<'a> {
    w: &'a mut W,
}
impl<'a> ID18_W<'a> {
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
#[doc = "Field `ID19` writer - XDMAC Channel 19 Interrupt Disable Bit"]
pub struct ID19_W<'a> {
    w: &'a mut W,
}
impl<'a> ID19_W<'a> {
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
#[doc = "Field `ID20` writer - XDMAC Channel 20 Interrupt Disable Bit"]
pub struct ID20_W<'a> {
    w: &'a mut W,
}
impl<'a> ID20_W<'a> {
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
#[doc = "Field `ID21` writer - XDMAC Channel 21 Interrupt Disable Bit"]
pub struct ID21_W<'a> {
    w: &'a mut W,
}
impl<'a> ID21_W<'a> {
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
#[doc = "Field `ID22` writer - XDMAC Channel 22 Interrupt Disable Bit"]
pub struct ID22_W<'a> {
    w: &'a mut W,
}
impl<'a> ID22_W<'a> {
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
#[doc = "Field `ID23` writer - XDMAC Channel 23 Interrupt Disable Bit"]
pub struct ID23_W<'a> {
    w: &'a mut W,
}
impl<'a> ID23_W<'a> {
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
    #[doc = "Bit 0 - XDMAC Channel 0 Interrupt Disable Bit"]
    #[inline(always)]
    pub fn id0(&mut self) -> ID0_W {
        ID0_W { w: self }
    }
    #[doc = "Bit 1 - XDMAC Channel 1 Interrupt Disable Bit"]
    #[inline(always)]
    pub fn id1(&mut self) -> ID1_W {
        ID1_W { w: self }
    }
    #[doc = "Bit 2 - XDMAC Channel 2 Interrupt Disable Bit"]
    #[inline(always)]
    pub fn id2(&mut self) -> ID2_W {
        ID2_W { w: self }
    }
    #[doc = "Bit 3 - XDMAC Channel 3 Interrupt Disable Bit"]
    #[inline(always)]
    pub fn id3(&mut self) -> ID3_W {
        ID3_W { w: self }
    }
    #[doc = "Bit 4 - XDMAC Channel 4 Interrupt Disable Bit"]
    #[inline(always)]
    pub fn id4(&mut self) -> ID4_W {
        ID4_W { w: self }
    }
    #[doc = "Bit 5 - XDMAC Channel 5 Interrupt Disable Bit"]
    #[inline(always)]
    pub fn id5(&mut self) -> ID5_W {
        ID5_W { w: self }
    }
    #[doc = "Bit 6 - XDMAC Channel 6 Interrupt Disable Bit"]
    #[inline(always)]
    pub fn id6(&mut self) -> ID6_W {
        ID6_W { w: self }
    }
    #[doc = "Bit 7 - XDMAC Channel 7 Interrupt Disable Bit"]
    #[inline(always)]
    pub fn id7(&mut self) -> ID7_W {
        ID7_W { w: self }
    }
    #[doc = "Bit 8 - XDMAC Channel 8 Interrupt Disable Bit"]
    #[inline(always)]
    pub fn id8(&mut self) -> ID8_W {
        ID8_W { w: self }
    }
    #[doc = "Bit 9 - XDMAC Channel 9 Interrupt Disable Bit"]
    #[inline(always)]
    pub fn id9(&mut self) -> ID9_W {
        ID9_W { w: self }
    }
    #[doc = "Bit 10 - XDMAC Channel 10 Interrupt Disable Bit"]
    #[inline(always)]
    pub fn id10(&mut self) -> ID10_W {
        ID10_W { w: self }
    }
    #[doc = "Bit 11 - XDMAC Channel 11 Interrupt Disable Bit"]
    #[inline(always)]
    pub fn id11(&mut self) -> ID11_W {
        ID11_W { w: self }
    }
    #[doc = "Bit 12 - XDMAC Channel 12 Interrupt Disable Bit"]
    #[inline(always)]
    pub fn id12(&mut self) -> ID12_W {
        ID12_W { w: self }
    }
    #[doc = "Bit 13 - XDMAC Channel 13 Interrupt Disable Bit"]
    #[inline(always)]
    pub fn id13(&mut self) -> ID13_W {
        ID13_W { w: self }
    }
    #[doc = "Bit 14 - XDMAC Channel 14 Interrupt Disable Bit"]
    #[inline(always)]
    pub fn id14(&mut self) -> ID14_W {
        ID14_W { w: self }
    }
    #[doc = "Bit 15 - XDMAC Channel 15 Interrupt Disable Bit"]
    #[inline(always)]
    pub fn id15(&mut self) -> ID15_W {
        ID15_W { w: self }
    }
    #[doc = "Bit 16 - XDMAC Channel 16 Interrupt Disable Bit"]
    #[inline(always)]
    pub fn id16(&mut self) -> ID16_W {
        ID16_W { w: self }
    }
    #[doc = "Bit 17 - XDMAC Channel 17 Interrupt Disable Bit"]
    #[inline(always)]
    pub fn id17(&mut self) -> ID17_W {
        ID17_W { w: self }
    }
    #[doc = "Bit 18 - XDMAC Channel 18 Interrupt Disable Bit"]
    #[inline(always)]
    pub fn id18(&mut self) -> ID18_W {
        ID18_W { w: self }
    }
    #[doc = "Bit 19 - XDMAC Channel 19 Interrupt Disable Bit"]
    #[inline(always)]
    pub fn id19(&mut self) -> ID19_W {
        ID19_W { w: self }
    }
    #[doc = "Bit 20 - XDMAC Channel 20 Interrupt Disable Bit"]
    #[inline(always)]
    pub fn id20(&mut self) -> ID20_W {
        ID20_W { w: self }
    }
    #[doc = "Bit 21 - XDMAC Channel 21 Interrupt Disable Bit"]
    #[inline(always)]
    pub fn id21(&mut self) -> ID21_W {
        ID21_W { w: self }
    }
    #[doc = "Bit 22 - XDMAC Channel 22 Interrupt Disable Bit"]
    #[inline(always)]
    pub fn id22(&mut self) -> ID22_W {
        ID22_W { w: self }
    }
    #[doc = "Bit 23 - XDMAC Channel 23 Interrupt Disable Bit"]
    #[inline(always)]
    pub fn id23(&mut self) -> ID23_W {
        ID23_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Global Interrupt Disable Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [xdmac_gid](index.html) module"]
pub struct XDMAC_GID_SPEC;
impl crate::RegisterSpec for XDMAC_GID_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [xdmac_gid::W](W) writer structure"]
impl crate::Writable for XDMAC_GID_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets XDMAC_GID to value 0"]
impl crate::Resettable for XDMAC_GID_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
