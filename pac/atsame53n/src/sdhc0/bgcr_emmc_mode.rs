#[doc = "Register `BGCR_EMMC_MODE` reader"]
pub struct R(crate::R<BGCR_EMMC_MODE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BGCR_EMMC_MODE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BGCR_EMMC_MODE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BGCR_EMMC_MODE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BGCR_EMMC_MODE` writer"]
pub struct W(crate::W<BGCR_EMMC_MODE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BGCR_EMMC_MODE_SPEC>;
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
impl From<crate::W<BGCR_EMMC_MODE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BGCR_EMMC_MODE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Stop at Block Gap Request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STPBGR_A {
    #[doc = "0: Transfer"]
    TRANSFER = 0,
    #[doc = "1: Stop"]
    STOP = 1,
}
impl From<STPBGR_A> for bool {
    #[inline(always)]
    fn from(variant: STPBGR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STPBGR` reader - Stop at Block Gap Request"]
pub struct STPBGR_R(crate::FieldReader<bool, STPBGR_A>);
impl STPBGR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        STPBGR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STPBGR_A {
        match self.bits {
            false => STPBGR_A::TRANSFER,
            true => STPBGR_A::STOP,
        }
    }
    #[doc = "Checks if the value of the field is `TRANSFER`"]
    #[inline(always)]
    pub fn is_transfer(&self) -> bool {
        **self == STPBGR_A::TRANSFER
    }
    #[doc = "Checks if the value of the field is `STOP`"]
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        **self == STPBGR_A::STOP
    }
}
impl core::ops::Deref for STPBGR_R {
    type Target = crate::FieldReader<bool, STPBGR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STPBGR` writer - Stop at Block Gap Request"]
pub struct STPBGR_W<'a> {
    w: &'a mut W,
}
impl<'a> STPBGR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STPBGR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Transfer"]
    #[inline(always)]
    pub fn transfer(self) -> &'a mut W {
        self.variant(STPBGR_A::TRANSFER)
    }
    #[doc = "Stop"]
    #[inline(always)]
    pub fn stop(self) -> &'a mut W {
        self.variant(STPBGR_A::STOP)
    }
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
#[doc = "Continue Request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CONTR_A {
    #[doc = "0: Not affected"]
    GO_ON = 0,
    #[doc = "1: Restart"]
    RESTART = 1,
}
impl From<CONTR_A> for bool {
    #[inline(always)]
    fn from(variant: CONTR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CONTR` reader - Continue Request"]
pub struct CONTR_R(crate::FieldReader<bool, CONTR_A>);
impl CONTR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CONTR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CONTR_A {
        match self.bits {
            false => CONTR_A::GO_ON,
            true => CONTR_A::RESTART,
        }
    }
    #[doc = "Checks if the value of the field is `GO_ON`"]
    #[inline(always)]
    pub fn is_go_on(&self) -> bool {
        **self == CONTR_A::GO_ON
    }
    #[doc = "Checks if the value of the field is `RESTART`"]
    #[inline(always)]
    pub fn is_restart(&self) -> bool {
        **self == CONTR_A::RESTART
    }
}
impl core::ops::Deref for CONTR_R {
    type Target = crate::FieldReader<bool, CONTR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CONTR` writer - Continue Request"]
pub struct CONTR_W<'a> {
    w: &'a mut W,
}
impl<'a> CONTR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CONTR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Not affected"]
    #[inline(always)]
    pub fn go_on(self) -> &'a mut W {
        self.variant(CONTR_A::GO_ON)
    }
    #[doc = "Restart"]
    #[inline(always)]
    pub fn restart(self) -> &'a mut W {
        self.variant(CONTR_A::RESTART)
    }
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
    #[doc = "Bit 0 - Stop at Block Gap Request"]
    #[inline(always)]
    pub fn stpbgr(&self) -> STPBGR_R {
        STPBGR_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Continue Request"]
    #[inline(always)]
    pub fn contr(&self) -> CONTR_R {
        CONTR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Stop at Block Gap Request"]
    #[inline(always)]
    pub fn stpbgr(&mut self) -> STPBGR_W {
        STPBGR_W { w: self }
    }
    #[doc = "Bit 1 - Continue Request"]
    #[inline(always)]
    pub fn contr(&mut self) -> CONTR_W {
        CONTR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Block Gap Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bgcr_emmc_mode](index.html) module"]
pub struct BGCR_EMMC_MODE_SPEC;
impl crate::RegisterSpec for BGCR_EMMC_MODE_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [bgcr_emmc_mode::R](R) reader structure"]
impl crate::Readable for BGCR_EMMC_MODE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bgcr_emmc_mode::W](W) writer structure"]
impl crate::Writable for BGCR_EMMC_MODE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BGCR_EMMC_MODE to value 0"]
impl crate::Resettable for BGCR_EMMC_MODE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
