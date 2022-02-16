#[doc = "Register `XDMAC_GSWF` writer"]
pub struct W(crate::W<XDMAC_GSWF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<XDMAC_GSWF_SPEC>;
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
impl From<crate::W<XDMAC_GSWF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<XDMAC_GSWF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SWF0` writer - XDMAC Channel 0 Software Flush Request Bit"]
pub struct SWF0_W<'a> {
    w: &'a mut W,
}
impl<'a> SWF0_W<'a> {
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
#[doc = "Field `SWF1` writer - XDMAC Channel 1 Software Flush Request Bit"]
pub struct SWF1_W<'a> {
    w: &'a mut W,
}
impl<'a> SWF1_W<'a> {
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
#[doc = "Field `SWF2` writer - XDMAC Channel 2 Software Flush Request Bit"]
pub struct SWF2_W<'a> {
    w: &'a mut W,
}
impl<'a> SWF2_W<'a> {
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
#[doc = "Field `SWF3` writer - XDMAC Channel 3 Software Flush Request Bit"]
pub struct SWF3_W<'a> {
    w: &'a mut W,
}
impl<'a> SWF3_W<'a> {
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
#[doc = "Field `SWF4` writer - XDMAC Channel 4 Software Flush Request Bit"]
pub struct SWF4_W<'a> {
    w: &'a mut W,
}
impl<'a> SWF4_W<'a> {
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
#[doc = "Field `SWF5` writer - XDMAC Channel 5 Software Flush Request Bit"]
pub struct SWF5_W<'a> {
    w: &'a mut W,
}
impl<'a> SWF5_W<'a> {
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
#[doc = "Field `SWF6` writer - XDMAC Channel 6 Software Flush Request Bit"]
pub struct SWF6_W<'a> {
    w: &'a mut W,
}
impl<'a> SWF6_W<'a> {
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
#[doc = "Field `SWF7` writer - XDMAC Channel 7 Software Flush Request Bit"]
pub struct SWF7_W<'a> {
    w: &'a mut W,
}
impl<'a> SWF7_W<'a> {
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
#[doc = "Field `SWF8` writer - XDMAC Channel 8 Software Flush Request Bit"]
pub struct SWF8_W<'a> {
    w: &'a mut W,
}
impl<'a> SWF8_W<'a> {
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
#[doc = "Field `SWF9` writer - XDMAC Channel 9 Software Flush Request Bit"]
pub struct SWF9_W<'a> {
    w: &'a mut W,
}
impl<'a> SWF9_W<'a> {
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
#[doc = "Field `SWF10` writer - XDMAC Channel 10 Software Flush Request Bit"]
pub struct SWF10_W<'a> {
    w: &'a mut W,
}
impl<'a> SWF10_W<'a> {
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
#[doc = "Field `SWF11` writer - XDMAC Channel 11 Software Flush Request Bit"]
pub struct SWF11_W<'a> {
    w: &'a mut W,
}
impl<'a> SWF11_W<'a> {
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
#[doc = "Field `SWF12` writer - XDMAC Channel 12 Software Flush Request Bit"]
pub struct SWF12_W<'a> {
    w: &'a mut W,
}
impl<'a> SWF12_W<'a> {
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
#[doc = "Field `SWF13` writer - XDMAC Channel 13 Software Flush Request Bit"]
pub struct SWF13_W<'a> {
    w: &'a mut W,
}
impl<'a> SWF13_W<'a> {
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
#[doc = "Field `SWF14` writer - XDMAC Channel 14 Software Flush Request Bit"]
pub struct SWF14_W<'a> {
    w: &'a mut W,
}
impl<'a> SWF14_W<'a> {
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
#[doc = "Field `SWF15` writer - XDMAC Channel 15 Software Flush Request Bit"]
pub struct SWF15_W<'a> {
    w: &'a mut W,
}
impl<'a> SWF15_W<'a> {
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
#[doc = "Field `SWF16` writer - XDMAC Channel 16 Software Flush Request Bit"]
pub struct SWF16_W<'a> {
    w: &'a mut W,
}
impl<'a> SWF16_W<'a> {
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
#[doc = "Field `SWF17` writer - XDMAC Channel 17 Software Flush Request Bit"]
pub struct SWF17_W<'a> {
    w: &'a mut W,
}
impl<'a> SWF17_W<'a> {
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
#[doc = "Field `SWF18` writer - XDMAC Channel 18 Software Flush Request Bit"]
pub struct SWF18_W<'a> {
    w: &'a mut W,
}
impl<'a> SWF18_W<'a> {
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
#[doc = "Field `SWF19` writer - XDMAC Channel 19 Software Flush Request Bit"]
pub struct SWF19_W<'a> {
    w: &'a mut W,
}
impl<'a> SWF19_W<'a> {
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
#[doc = "Field `SWF20` writer - XDMAC Channel 20 Software Flush Request Bit"]
pub struct SWF20_W<'a> {
    w: &'a mut W,
}
impl<'a> SWF20_W<'a> {
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
#[doc = "Field `SWF21` writer - XDMAC Channel 21 Software Flush Request Bit"]
pub struct SWF21_W<'a> {
    w: &'a mut W,
}
impl<'a> SWF21_W<'a> {
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
#[doc = "Field `SWF22` writer - XDMAC Channel 22 Software Flush Request Bit"]
pub struct SWF22_W<'a> {
    w: &'a mut W,
}
impl<'a> SWF22_W<'a> {
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
#[doc = "Field `SWF23` writer - XDMAC Channel 23 Software Flush Request Bit"]
pub struct SWF23_W<'a> {
    w: &'a mut W,
}
impl<'a> SWF23_W<'a> {
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
    #[doc = "Bit 0 - XDMAC Channel 0 Software Flush Request Bit"]
    #[inline(always)]
    pub fn swf0(&mut self) -> SWF0_W {
        SWF0_W { w: self }
    }
    #[doc = "Bit 1 - XDMAC Channel 1 Software Flush Request Bit"]
    #[inline(always)]
    pub fn swf1(&mut self) -> SWF1_W {
        SWF1_W { w: self }
    }
    #[doc = "Bit 2 - XDMAC Channel 2 Software Flush Request Bit"]
    #[inline(always)]
    pub fn swf2(&mut self) -> SWF2_W {
        SWF2_W { w: self }
    }
    #[doc = "Bit 3 - XDMAC Channel 3 Software Flush Request Bit"]
    #[inline(always)]
    pub fn swf3(&mut self) -> SWF3_W {
        SWF3_W { w: self }
    }
    #[doc = "Bit 4 - XDMAC Channel 4 Software Flush Request Bit"]
    #[inline(always)]
    pub fn swf4(&mut self) -> SWF4_W {
        SWF4_W { w: self }
    }
    #[doc = "Bit 5 - XDMAC Channel 5 Software Flush Request Bit"]
    #[inline(always)]
    pub fn swf5(&mut self) -> SWF5_W {
        SWF5_W { w: self }
    }
    #[doc = "Bit 6 - XDMAC Channel 6 Software Flush Request Bit"]
    #[inline(always)]
    pub fn swf6(&mut self) -> SWF6_W {
        SWF6_W { w: self }
    }
    #[doc = "Bit 7 - XDMAC Channel 7 Software Flush Request Bit"]
    #[inline(always)]
    pub fn swf7(&mut self) -> SWF7_W {
        SWF7_W { w: self }
    }
    #[doc = "Bit 8 - XDMAC Channel 8 Software Flush Request Bit"]
    #[inline(always)]
    pub fn swf8(&mut self) -> SWF8_W {
        SWF8_W { w: self }
    }
    #[doc = "Bit 9 - XDMAC Channel 9 Software Flush Request Bit"]
    #[inline(always)]
    pub fn swf9(&mut self) -> SWF9_W {
        SWF9_W { w: self }
    }
    #[doc = "Bit 10 - XDMAC Channel 10 Software Flush Request Bit"]
    #[inline(always)]
    pub fn swf10(&mut self) -> SWF10_W {
        SWF10_W { w: self }
    }
    #[doc = "Bit 11 - XDMAC Channel 11 Software Flush Request Bit"]
    #[inline(always)]
    pub fn swf11(&mut self) -> SWF11_W {
        SWF11_W { w: self }
    }
    #[doc = "Bit 12 - XDMAC Channel 12 Software Flush Request Bit"]
    #[inline(always)]
    pub fn swf12(&mut self) -> SWF12_W {
        SWF12_W { w: self }
    }
    #[doc = "Bit 13 - XDMAC Channel 13 Software Flush Request Bit"]
    #[inline(always)]
    pub fn swf13(&mut self) -> SWF13_W {
        SWF13_W { w: self }
    }
    #[doc = "Bit 14 - XDMAC Channel 14 Software Flush Request Bit"]
    #[inline(always)]
    pub fn swf14(&mut self) -> SWF14_W {
        SWF14_W { w: self }
    }
    #[doc = "Bit 15 - XDMAC Channel 15 Software Flush Request Bit"]
    #[inline(always)]
    pub fn swf15(&mut self) -> SWF15_W {
        SWF15_W { w: self }
    }
    #[doc = "Bit 16 - XDMAC Channel 16 Software Flush Request Bit"]
    #[inline(always)]
    pub fn swf16(&mut self) -> SWF16_W {
        SWF16_W { w: self }
    }
    #[doc = "Bit 17 - XDMAC Channel 17 Software Flush Request Bit"]
    #[inline(always)]
    pub fn swf17(&mut self) -> SWF17_W {
        SWF17_W { w: self }
    }
    #[doc = "Bit 18 - XDMAC Channel 18 Software Flush Request Bit"]
    #[inline(always)]
    pub fn swf18(&mut self) -> SWF18_W {
        SWF18_W { w: self }
    }
    #[doc = "Bit 19 - XDMAC Channel 19 Software Flush Request Bit"]
    #[inline(always)]
    pub fn swf19(&mut self) -> SWF19_W {
        SWF19_W { w: self }
    }
    #[doc = "Bit 20 - XDMAC Channel 20 Software Flush Request Bit"]
    #[inline(always)]
    pub fn swf20(&mut self) -> SWF20_W {
        SWF20_W { w: self }
    }
    #[doc = "Bit 21 - XDMAC Channel 21 Software Flush Request Bit"]
    #[inline(always)]
    pub fn swf21(&mut self) -> SWF21_W {
        SWF21_W { w: self }
    }
    #[doc = "Bit 22 - XDMAC Channel 22 Software Flush Request Bit"]
    #[inline(always)]
    pub fn swf22(&mut self) -> SWF22_W {
        SWF22_W { w: self }
    }
    #[doc = "Bit 23 - XDMAC Channel 23 Software Flush Request Bit"]
    #[inline(always)]
    pub fn swf23(&mut self) -> SWF23_W {
        SWF23_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Global Channel Software Flush Request Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [xdmac_gswf](index.html) module"]
pub struct XDMAC_GSWF_SPEC;
impl crate::RegisterSpec for XDMAC_GSWF_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [xdmac_gswf::W](W) writer structure"]
impl crate::Writable for XDMAC_GSWF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets XDMAC_GSWF to value 0"]
impl crate::Resettable for XDMAC_GSWF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
