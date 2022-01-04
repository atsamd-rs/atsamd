#[doc = "Register `VREG` reader"]
pub struct R(crate::R<VREG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VREG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VREG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VREG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `VREG` writer"]
pub struct W(crate::W<VREG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<VREG_SPEC>;
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
impl From<crate::W<VREG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<VREG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RUNSTDBY` reader - Run in Standby"]
pub struct RUNSTDBY_R(crate::FieldReader<bool, bool>);
impl RUNSTDBY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RUNSTDBY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RUNSTDBY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RUNSTDBY` writer - Run in Standby"]
pub struct RUNSTDBY_W<'a> {
    w: &'a mut W,
}
impl<'a> RUNSTDBY_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u16 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `FORCELDO` reader - Force LDO Voltage Regulator"]
pub struct FORCELDO_R(crate::FieldReader<bool, bool>);
impl FORCELDO_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FORCELDO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FORCELDO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FORCELDO` writer - Force LDO Voltage Regulator"]
pub struct FORCELDO_W<'a> {
    w: &'a mut W,
}
impl<'a> FORCELDO_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u16 & 0x01) << 13);
        self.w
    }
}
impl R {
    #[doc = "Bit 6 - Run in Standby"]
    #[inline(always)]
    pub fn runstdby(&self) -> RUNSTDBY_R {
        RUNSTDBY_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Force LDO Voltage Regulator"]
    #[inline(always)]
    pub fn forceldo(&self) -> FORCELDO_R {
        FORCELDO_R::new(((self.bits >> 13) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 6 - Run in Standby"]
    #[inline(always)]
    pub fn runstdby(&mut self) -> RUNSTDBY_W {
        RUNSTDBY_W { w: self }
    }
    #[doc = "Bit 13 - Force LDO Voltage Regulator"]
    #[inline(always)]
    pub fn forceldo(&mut self) -> FORCELDO_W {
        FORCELDO_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Voltage Regulator System (VREG) Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vreg](index.html) module"]
pub struct VREG_SPEC;
impl crate::RegisterSpec for VREG_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [vreg::R](R) reader structure"]
impl crate::Readable for VREG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [vreg::W](W) writer structure"]
impl crate::Writable for VREG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets VREG to value 0"]
impl crate::Resettable for VREG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
