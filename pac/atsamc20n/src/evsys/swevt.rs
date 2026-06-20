#[doc = "Register `SWEVT` writer"]
pub struct W(crate::W<SWEVT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SWEVT_SPEC>;
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
impl From<crate::W<SWEVT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SWEVT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CHANNEL0` writer - Channel 0 Software Selection"]
pub struct CHANNEL0_W<'a> {
    w: &'a mut W,
}
impl<'a> CHANNEL0_W<'a> {
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
#[doc = "Field `CHANNEL1` writer - Channel 1 Software Selection"]
pub struct CHANNEL1_W<'a> {
    w: &'a mut W,
}
impl<'a> CHANNEL1_W<'a> {
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
#[doc = "Field `CHANNEL2` writer - Channel 2 Software Selection"]
pub struct CHANNEL2_W<'a> {
    w: &'a mut W,
}
impl<'a> CHANNEL2_W<'a> {
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
#[doc = "Field `CHANNEL3` writer - Channel 3 Software Selection"]
pub struct CHANNEL3_W<'a> {
    w: &'a mut W,
}
impl<'a> CHANNEL3_W<'a> {
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
#[doc = "Field `CHANNEL4` writer - Channel 4 Software Selection"]
pub struct CHANNEL4_W<'a> {
    w: &'a mut W,
}
impl<'a> CHANNEL4_W<'a> {
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
#[doc = "Field `CHANNEL5` writer - Channel 5 Software Selection"]
pub struct CHANNEL5_W<'a> {
    w: &'a mut W,
}
impl<'a> CHANNEL5_W<'a> {
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
#[doc = "Field `CHANNEL6` writer - Channel 6 Software Selection"]
pub struct CHANNEL6_W<'a> {
    w: &'a mut W,
}
impl<'a> CHANNEL6_W<'a> {
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
#[doc = "Field `CHANNEL7` writer - Channel 7 Software Selection"]
pub struct CHANNEL7_W<'a> {
    w: &'a mut W,
}
impl<'a> CHANNEL7_W<'a> {
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
#[doc = "Field `CHANNEL8` writer - Channel 8 Software Selection"]
pub struct CHANNEL8_W<'a> {
    w: &'a mut W,
}
impl<'a> CHANNEL8_W<'a> {
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
#[doc = "Field `CHANNEL9` writer - Channel 9 Software Selection"]
pub struct CHANNEL9_W<'a> {
    w: &'a mut W,
}
impl<'a> CHANNEL9_W<'a> {
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
#[doc = "Field `CHANNEL10` writer - Channel 10 Software Selection"]
pub struct CHANNEL10_W<'a> {
    w: &'a mut W,
}
impl<'a> CHANNEL10_W<'a> {
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
#[doc = "Field `CHANNEL11` writer - Channel 11 Software Selection"]
pub struct CHANNEL11_W<'a> {
    w: &'a mut W,
}
impl<'a> CHANNEL11_W<'a> {
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
impl W {
    #[doc = "Bit 0 - Channel 0 Software Selection"]
    #[inline(always)]
    pub fn channel0(&mut self) -> CHANNEL0_W {
        CHANNEL0_W { w: self }
    }
    #[doc = "Bit 1 - Channel 1 Software Selection"]
    #[inline(always)]
    pub fn channel1(&mut self) -> CHANNEL1_W {
        CHANNEL1_W { w: self }
    }
    #[doc = "Bit 2 - Channel 2 Software Selection"]
    #[inline(always)]
    pub fn channel2(&mut self) -> CHANNEL2_W {
        CHANNEL2_W { w: self }
    }
    #[doc = "Bit 3 - Channel 3 Software Selection"]
    #[inline(always)]
    pub fn channel3(&mut self) -> CHANNEL3_W {
        CHANNEL3_W { w: self }
    }
    #[doc = "Bit 4 - Channel 4 Software Selection"]
    #[inline(always)]
    pub fn channel4(&mut self) -> CHANNEL4_W {
        CHANNEL4_W { w: self }
    }
    #[doc = "Bit 5 - Channel 5 Software Selection"]
    #[inline(always)]
    pub fn channel5(&mut self) -> CHANNEL5_W {
        CHANNEL5_W { w: self }
    }
    #[doc = "Bit 6 - Channel 6 Software Selection"]
    #[inline(always)]
    pub fn channel6(&mut self) -> CHANNEL6_W {
        CHANNEL6_W { w: self }
    }
    #[doc = "Bit 7 - Channel 7 Software Selection"]
    #[inline(always)]
    pub fn channel7(&mut self) -> CHANNEL7_W {
        CHANNEL7_W { w: self }
    }
    #[doc = "Bit 8 - Channel 8 Software Selection"]
    #[inline(always)]
    pub fn channel8(&mut self) -> CHANNEL8_W {
        CHANNEL8_W { w: self }
    }
    #[doc = "Bit 9 - Channel 9 Software Selection"]
    #[inline(always)]
    pub fn channel9(&mut self) -> CHANNEL9_W {
        CHANNEL9_W { w: self }
    }
    #[doc = "Bit 10 - Channel 10 Software Selection"]
    #[inline(always)]
    pub fn channel10(&mut self) -> CHANNEL10_W {
        CHANNEL10_W { w: self }
    }
    #[doc = "Bit 11 - Channel 11 Software Selection"]
    #[inline(always)]
    pub fn channel11(&mut self) -> CHANNEL11_W {
        CHANNEL11_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Software Event\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [swevt](index.html) module"]
pub struct SWEVT_SPEC;
impl crate::RegisterSpec for SWEVT_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [swevt::W](W) writer structure"]
impl crate::Writable for SWEVT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SWEVT to value 0"]
impl crate::Resettable for SWEVT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
