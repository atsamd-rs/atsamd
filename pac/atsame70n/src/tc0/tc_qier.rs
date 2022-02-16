#[doc = "Register `TC_QIER` writer"]
pub struct W(crate::W<TC_QIER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TC_QIER_SPEC>;
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
impl From<crate::W<TC_QIER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TC_QIER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IDX` writer - Index"]
pub struct IDX_W<'a> {
    w: &'a mut W,
}
impl<'a> IDX_W<'a> {
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
#[doc = "Field `DIRCHG` writer - Direction Change"]
pub struct DIRCHG_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRCHG_W<'a> {
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
#[doc = "Field `QERR` writer - Quadrature Error"]
pub struct QERR_W<'a> {
    w: &'a mut W,
}
impl<'a> QERR_W<'a> {
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
#[doc = "Field `MPE` writer - Consecutive Missing Pulse Error"]
pub struct MPE_W<'a> {
    w: &'a mut W,
}
impl<'a> MPE_W<'a> {
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
impl W {
    #[doc = "Bit 0 - Index"]
    #[inline(always)]
    pub fn idx(&mut self) -> IDX_W {
        IDX_W { w: self }
    }
    #[doc = "Bit 1 - Direction Change"]
    #[inline(always)]
    pub fn dirchg(&mut self) -> DIRCHG_W {
        DIRCHG_W { w: self }
    }
    #[doc = "Bit 2 - Quadrature Error"]
    #[inline(always)]
    pub fn qerr(&mut self) -> QERR_W {
        QERR_W { w: self }
    }
    #[doc = "Bit 3 - Consecutive Missing Pulse Error"]
    #[inline(always)]
    pub fn mpe(&mut self) -> MPE_W {
        MPE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "QDEC Interrupt Enable Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tc_qier](index.html) module"]
pub struct TC_QIER_SPEC;
impl crate::RegisterSpec for TC_QIER_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [tc_qier::W](W) writer structure"]
impl crate::Writable for TC_QIER_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TC_QIER to value 0"]
impl crate::Resettable for TC_QIER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
