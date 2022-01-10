#[doc = "Register `EXTCTRL` reader"]
pub struct R(crate::R<EXTCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EXTCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EXTCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EXTCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EXTCTRL` writer"]
pub struct W(crate::W<EXTCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EXTCTRL_SPEC>;
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
impl From<crate::W<EXTCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EXTCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SETDIS` reader - External Reset Disable"]
pub struct SETDIS_R(crate::FieldReader<bool, bool>);
impl SETDIS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SETDIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SETDIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SETDIS` writer - External Reset Disable"]
pub struct SETDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> SETDIS_W<'a> {
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
impl R {
    #[doc = "Bit 0 - External Reset Disable"]
    #[inline(always)]
    pub fn setdis(&self) -> SETDIS_R {
        SETDIS_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - External Reset Disable"]
    #[inline(always)]
    pub fn setdis(&mut self) -> SETDIS_W {
        SETDIS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "External Reset Controller\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [extctrl](index.html) module"]
pub struct EXTCTRL_SPEC;
impl crate::RegisterSpec for EXTCTRL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [extctrl::R](R) reader structure"]
impl crate::Readable for EXTCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [extctrl::W](W) writer structure"]
impl crate::Writable for EXTCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EXTCTRL to value 0"]
impl crate::Resettable for EXTCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
