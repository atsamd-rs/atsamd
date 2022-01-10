#[doc = "Register `BGCR` reader"]
pub struct R(crate::R<BGCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BGCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BGCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BGCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BGCR` writer"]
pub struct W(crate::W<BGCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BGCR_SPEC>;
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
impl From<crate::W<BGCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BGCR_SPEC>) -> Self {
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
#[doc = "Read Wait Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RWCTRL_A {
    #[doc = "0: Disable Read Wait Control"]
    DISABLE = 0,
    #[doc = "1: Enable Read Wait Control"]
    ENABLE = 1,
}
impl From<RWCTRL_A> for bool {
    #[inline(always)]
    fn from(variant: RWCTRL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RWCTRL` reader - Read Wait Control"]
pub struct RWCTRL_R(crate::FieldReader<bool, RWCTRL_A>);
impl RWCTRL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RWCTRL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RWCTRL_A {
        match self.bits {
            false => RWCTRL_A::DISABLE,
            true => RWCTRL_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == RWCTRL_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == RWCTRL_A::ENABLE
    }
}
impl core::ops::Deref for RWCTRL_R {
    type Target = crate::FieldReader<bool, RWCTRL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RWCTRL` writer - Read Wait Control"]
pub struct RWCTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> RWCTRL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RWCTRL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable Read Wait Control"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(RWCTRL_A::DISABLE)
    }
    #[doc = "Enable Read Wait Control"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(RWCTRL_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u8 & 0x01) << 2);
        self.w
    }
}
#[doc = "Interrupt at Block Gap\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INTBG_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<INTBG_A> for bool {
    #[inline(always)]
    fn from(variant: INTBG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INTBG` reader - Interrupt at Block Gap"]
pub struct INTBG_R(crate::FieldReader<bool, INTBG_A>);
impl INTBG_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        INTBG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTBG_A {
        match self.bits {
            false => INTBG_A::DISABLED,
            true => INTBG_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == INTBG_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == INTBG_A::ENABLED
    }
}
impl core::ops::Deref for INTBG_R {
    type Target = crate::FieldReader<bool, INTBG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INTBG` writer - Interrupt at Block Gap"]
pub struct INTBG_W<'a> {
    w: &'a mut W,
}
impl<'a> INTBG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INTBG_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(INTBG_A::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(INTBG_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u8 & 0x01) << 3);
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
    #[doc = "Bit 2 - Read Wait Control"]
    #[inline(always)]
    pub fn rwctrl(&self) -> RWCTRL_R {
        RWCTRL_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Interrupt at Block Gap"]
    #[inline(always)]
    pub fn intbg(&self) -> INTBG_R {
        INTBG_R::new(((self.bits >> 3) & 0x01) != 0)
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
    #[doc = "Bit 2 - Read Wait Control"]
    #[inline(always)]
    pub fn rwctrl(&mut self) -> RWCTRL_W {
        RWCTRL_W { w: self }
    }
    #[doc = "Bit 3 - Interrupt at Block Gap"]
    #[inline(always)]
    pub fn intbg(&mut self) -> INTBG_W {
        INTBG_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Block Gap Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bgcr](index.html) module"]
pub struct BGCR_SPEC;
impl crate::RegisterSpec for BGCR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [bgcr::R](R) reader structure"]
impl crate::Readable for BGCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bgcr::W](W) writer structure"]
impl crate::Writable for BGCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BGCR to value 0"]
impl crate::Resettable for BGCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
