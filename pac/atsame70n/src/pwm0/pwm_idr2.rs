#[doc = "Register `PWM_IDR2` writer"]
pub struct W(crate::W<PWM_IDR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWM_IDR2_SPEC>;
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
impl From<crate::W<PWM_IDR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PWM_IDR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WRDY` writer - Write Ready for Synchronous Channels Update Interrupt Disable"]
pub struct WRDY_W<'a> {
    w: &'a mut W,
}
impl<'a> WRDY_W<'a> {
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
#[doc = "Field `UNRE` writer - Synchronous Channels Update Underrun Error Interrupt Disable"]
pub struct UNRE_W<'a> {
    w: &'a mut W,
}
impl<'a> UNRE_W<'a> {
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
#[doc = "Field `CMPM0` writer - Comparison 0 Match Interrupt Disable"]
pub struct CMPM0_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPM0_W<'a> {
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
#[doc = "Field `CMPM1` writer - Comparison 1 Match Interrupt Disable"]
pub struct CMPM1_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPM1_W<'a> {
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
#[doc = "Field `CMPM2` writer - Comparison 2 Match Interrupt Disable"]
pub struct CMPM2_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPM2_W<'a> {
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
#[doc = "Field `CMPM3` writer - Comparison 3 Match Interrupt Disable"]
pub struct CMPM3_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPM3_W<'a> {
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
#[doc = "Field `CMPM4` writer - Comparison 4 Match Interrupt Disable"]
pub struct CMPM4_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPM4_W<'a> {
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
#[doc = "Field `CMPM5` writer - Comparison 5 Match Interrupt Disable"]
pub struct CMPM5_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPM5_W<'a> {
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
#[doc = "Field `CMPM6` writer - Comparison 6 Match Interrupt Disable"]
pub struct CMPM6_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPM6_W<'a> {
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
#[doc = "Field `CMPM7` writer - Comparison 7 Match Interrupt Disable"]
pub struct CMPM7_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPM7_W<'a> {
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
#[doc = "Field `CMPU0` writer - Comparison 0 Update Interrupt Disable"]
pub struct CMPU0_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPU0_W<'a> {
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
#[doc = "Field `CMPU1` writer - Comparison 1 Update Interrupt Disable"]
pub struct CMPU1_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPU1_W<'a> {
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
#[doc = "Field `CMPU2` writer - Comparison 2 Update Interrupt Disable"]
pub struct CMPU2_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPU2_W<'a> {
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
#[doc = "Field `CMPU3` writer - Comparison 3 Update Interrupt Disable"]
pub struct CMPU3_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPU3_W<'a> {
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
#[doc = "Field `CMPU4` writer - Comparison 4 Update Interrupt Disable"]
pub struct CMPU4_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPU4_W<'a> {
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
#[doc = "Field `CMPU5` writer - Comparison 5 Update Interrupt Disable"]
pub struct CMPU5_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPU5_W<'a> {
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
#[doc = "Field `CMPU6` writer - Comparison 6 Update Interrupt Disable"]
pub struct CMPU6_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPU6_W<'a> {
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
#[doc = "Field `CMPU7` writer - Comparison 7 Update Interrupt Disable"]
pub struct CMPU7_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPU7_W<'a> {
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
    #[doc = "Bit 0 - Write Ready for Synchronous Channels Update Interrupt Disable"]
    #[inline(always)]
    pub fn wrdy(&mut self) -> WRDY_W {
        WRDY_W { w: self }
    }
    #[doc = "Bit 3 - Synchronous Channels Update Underrun Error Interrupt Disable"]
    #[inline(always)]
    pub fn unre(&mut self) -> UNRE_W {
        UNRE_W { w: self }
    }
    #[doc = "Bit 8 - Comparison 0 Match Interrupt Disable"]
    #[inline(always)]
    pub fn cmpm0(&mut self) -> CMPM0_W {
        CMPM0_W { w: self }
    }
    #[doc = "Bit 9 - Comparison 1 Match Interrupt Disable"]
    #[inline(always)]
    pub fn cmpm1(&mut self) -> CMPM1_W {
        CMPM1_W { w: self }
    }
    #[doc = "Bit 10 - Comparison 2 Match Interrupt Disable"]
    #[inline(always)]
    pub fn cmpm2(&mut self) -> CMPM2_W {
        CMPM2_W { w: self }
    }
    #[doc = "Bit 11 - Comparison 3 Match Interrupt Disable"]
    #[inline(always)]
    pub fn cmpm3(&mut self) -> CMPM3_W {
        CMPM3_W { w: self }
    }
    #[doc = "Bit 12 - Comparison 4 Match Interrupt Disable"]
    #[inline(always)]
    pub fn cmpm4(&mut self) -> CMPM4_W {
        CMPM4_W { w: self }
    }
    #[doc = "Bit 13 - Comparison 5 Match Interrupt Disable"]
    #[inline(always)]
    pub fn cmpm5(&mut self) -> CMPM5_W {
        CMPM5_W { w: self }
    }
    #[doc = "Bit 14 - Comparison 6 Match Interrupt Disable"]
    #[inline(always)]
    pub fn cmpm6(&mut self) -> CMPM6_W {
        CMPM6_W { w: self }
    }
    #[doc = "Bit 15 - Comparison 7 Match Interrupt Disable"]
    #[inline(always)]
    pub fn cmpm7(&mut self) -> CMPM7_W {
        CMPM7_W { w: self }
    }
    #[doc = "Bit 16 - Comparison 0 Update Interrupt Disable"]
    #[inline(always)]
    pub fn cmpu0(&mut self) -> CMPU0_W {
        CMPU0_W { w: self }
    }
    #[doc = "Bit 17 - Comparison 1 Update Interrupt Disable"]
    #[inline(always)]
    pub fn cmpu1(&mut self) -> CMPU1_W {
        CMPU1_W { w: self }
    }
    #[doc = "Bit 18 - Comparison 2 Update Interrupt Disable"]
    #[inline(always)]
    pub fn cmpu2(&mut self) -> CMPU2_W {
        CMPU2_W { w: self }
    }
    #[doc = "Bit 19 - Comparison 3 Update Interrupt Disable"]
    #[inline(always)]
    pub fn cmpu3(&mut self) -> CMPU3_W {
        CMPU3_W { w: self }
    }
    #[doc = "Bit 20 - Comparison 4 Update Interrupt Disable"]
    #[inline(always)]
    pub fn cmpu4(&mut self) -> CMPU4_W {
        CMPU4_W { w: self }
    }
    #[doc = "Bit 21 - Comparison 5 Update Interrupt Disable"]
    #[inline(always)]
    pub fn cmpu5(&mut self) -> CMPU5_W {
        CMPU5_W { w: self }
    }
    #[doc = "Bit 22 - Comparison 6 Update Interrupt Disable"]
    #[inline(always)]
    pub fn cmpu6(&mut self) -> CMPU6_W {
        CMPU6_W { w: self }
    }
    #[doc = "Bit 23 - Comparison 7 Update Interrupt Disable"]
    #[inline(always)]
    pub fn cmpu7(&mut self) -> CMPU7_W {
        CMPU7_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWM Interrupt Disable Register 2\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwm_idr2](index.html) module"]
pub struct PWM_IDR2_SPEC;
impl crate::RegisterSpec for PWM_IDR2_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [pwm_idr2::W](W) writer structure"]
impl crate::Writable for PWM_IDR2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PWM_IDR2 to value 0"]
impl crate::Resettable for PWM_IDR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
