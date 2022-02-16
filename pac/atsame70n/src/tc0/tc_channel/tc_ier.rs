#[doc = "Register `TC_IER` writer"]
pub struct W(crate::W<TC_IER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TC_IER_SPEC>;
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
impl From<crate::W<TC_IER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TC_IER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COVFS` writer - Counter Overflow"]
pub struct COVFS_W<'a> {
    w: &'a mut W,
}
impl<'a> COVFS_W<'a> {
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
#[doc = "Field `LOVRS` writer - Load Overrun"]
pub struct LOVRS_W<'a> {
    w: &'a mut W,
}
impl<'a> LOVRS_W<'a> {
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
#[doc = "Field `CPAS` writer - RA Compare"]
pub struct CPAS_W<'a> {
    w: &'a mut W,
}
impl<'a> CPAS_W<'a> {
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
#[doc = "Field `CPBS` writer - RB Compare"]
pub struct CPBS_W<'a> {
    w: &'a mut W,
}
impl<'a> CPBS_W<'a> {
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
#[doc = "Field `CPCS` writer - RC Compare"]
pub struct CPCS_W<'a> {
    w: &'a mut W,
}
impl<'a> CPCS_W<'a> {
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
#[doc = "Field `LDRAS` writer - RA Loading"]
pub struct LDRAS_W<'a> {
    w: &'a mut W,
}
impl<'a> LDRAS_W<'a> {
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
#[doc = "Field `LDRBS` writer - RB Loading"]
pub struct LDRBS_W<'a> {
    w: &'a mut W,
}
impl<'a> LDRBS_W<'a> {
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
#[doc = "Field `ETRGS` writer - External Trigger"]
pub struct ETRGS_W<'a> {
    w: &'a mut W,
}
impl<'a> ETRGS_W<'a> {
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
impl W {
    #[doc = "Bit 0 - Counter Overflow"]
    #[inline(always)]
    pub fn covfs(&mut self) -> COVFS_W {
        COVFS_W { w: self }
    }
    #[doc = "Bit 1 - Load Overrun"]
    #[inline(always)]
    pub fn lovrs(&mut self) -> LOVRS_W {
        LOVRS_W { w: self }
    }
    #[doc = "Bit 2 - RA Compare"]
    #[inline(always)]
    pub fn cpas(&mut self) -> CPAS_W {
        CPAS_W { w: self }
    }
    #[doc = "Bit 3 - RB Compare"]
    #[inline(always)]
    pub fn cpbs(&mut self) -> CPBS_W {
        CPBS_W { w: self }
    }
    #[doc = "Bit 4 - RC Compare"]
    #[inline(always)]
    pub fn cpcs(&mut self) -> CPCS_W {
        CPCS_W { w: self }
    }
    #[doc = "Bit 5 - RA Loading"]
    #[inline(always)]
    pub fn ldras(&mut self) -> LDRAS_W {
        LDRAS_W { w: self }
    }
    #[doc = "Bit 6 - RB Loading"]
    #[inline(always)]
    pub fn ldrbs(&mut self) -> LDRBS_W {
        LDRBS_W { w: self }
    }
    #[doc = "Bit 7 - External Trigger"]
    #[inline(always)]
    pub fn etrgs(&mut self) -> ETRGS_W {
        ETRGS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Enable Register (channel = 0)\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tc_ier](index.html) module"]
pub struct TC_IER_SPEC;
impl crate::RegisterSpec for TC_IER_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [tc_ier::W](W) writer structure"]
impl crate::Writable for TC_IER_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TC_IER to value 0"]
impl crate::Resettable for TC_IER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
