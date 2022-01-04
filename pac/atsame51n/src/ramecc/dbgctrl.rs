#[doc = "Register `DBGCTRL` reader"]
pub struct R(crate::R<DBGCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DBGCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DBGCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DBGCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DBGCTRL` writer"]
pub struct W(crate::W<DBGCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DBGCTRL_SPEC>;
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
impl From<crate::W<DBGCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DBGCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ECCDIS` reader - ECC Disable"]
pub struct ECCDIS_R(crate::FieldReader<bool, bool>);
impl ECCDIS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ECCDIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ECCDIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ECCDIS` writer - ECC Disable"]
pub struct ECCDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> ECCDIS_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | (value as u8 & 0x01);
        self.w
    }
}
#[doc = "Field `ECCELOG` reader - ECC Error Log"]
pub struct ECCELOG_R(crate::FieldReader<bool, bool>);
impl ECCELOG_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ECCELOG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ECCELOG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ECCELOG` writer - ECC Error Log"]
pub struct ECCELOG_W<'a> {
    w: &'a mut W,
}
impl<'a> ECCELOG_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u8 & 0x01) << 1);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - ECC Disable"]
    #[inline(always)]
    pub fn eccdis(&self) -> ECCDIS_R {
        ECCDIS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - ECC Error Log"]
    #[inline(always)]
    pub fn eccelog(&self) -> ECCELOG_R {
        ECCELOG_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ECC Disable"]
    #[inline(always)]
    pub fn eccdis(&mut self) -> ECCDIS_W {
        ECCDIS_W { w: self }
    }
    #[doc = "Bit 1 - ECC Error Log"]
    #[inline(always)]
    pub fn eccelog(&mut self) -> ECCELOG_W {
        ECCELOG_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Debug Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dbgctrl](index.html) module"]
pub struct DBGCTRL_SPEC;
impl crate::RegisterSpec for DBGCTRL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [dbgctrl::R](R) reader structure"]
impl crate::Readable for DBGCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dbgctrl::W](W) writer structure"]
impl crate::Writable for DBGCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DBGCTRL to value 0"]
impl crate::Resettable for DBGCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
