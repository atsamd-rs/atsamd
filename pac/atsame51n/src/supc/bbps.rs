#[doc = "Register `BBPS` reader"]
pub struct R(crate::R<BBPS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BBPS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BBPS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BBPS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BBPS` writer"]
pub struct W(crate::W<BBPS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BBPS_SPEC>;
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
impl From<crate::W<BBPS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BBPS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Battery Backup Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CONF_A {
    #[doc = "0: The power switch is handled by the BOD33"]
    BOD33 = 0,
    #[doc = "1: In Backup Domain, the backup domain is always supplied by battery backup power"]
    FORCED = 1,
}
impl From<CONF_A> for bool {
    #[inline(always)]
    fn from(variant: CONF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CONF` reader - Battery Backup Configuration"]
pub struct CONF_R(crate::FieldReader<bool, CONF_A>);
impl CONF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CONF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CONF_A {
        match self.bits {
            false => CONF_A::BOD33,
            true => CONF_A::FORCED,
        }
    }
    #[doc = "Checks if the value of the field is `BOD33`"]
    #[inline(always)]
    pub fn is_bod33(&self) -> bool {
        **self == CONF_A::BOD33
    }
    #[doc = "Checks if the value of the field is `FORCED`"]
    #[inline(always)]
    pub fn is_forced(&self) -> bool {
        **self == CONF_A::FORCED
    }
}
impl core::ops::Deref for CONF_R {
    type Target = crate::FieldReader<bool, CONF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CONF` writer - Battery Backup Configuration"]
pub struct CONF_W<'a> {
    w: &'a mut W,
}
impl<'a> CONF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CONF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The power switch is handled by the BOD33"]
    #[inline(always)]
    pub fn bod33(self) -> &'a mut W {
        self.variant(CONF_A::BOD33)
    }
    #[doc = "In Backup Domain, the backup domain is always supplied by battery backup power"]
    #[inline(always)]
    pub fn forced(self) -> &'a mut W {
        self.variant(CONF_A::FORCED)
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Field `WAKEEN` reader - Wake Enable"]
pub struct WAKEEN_R(crate::FieldReader<bool, bool>);
impl WAKEEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WAKEEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WAKEEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WAKEEN` writer - Wake Enable"]
pub struct WAKEEN_W<'a> {
    w: &'a mut W,
}
impl<'a> WAKEEN_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Battery Backup Configuration"]
    #[inline(always)]
    pub fn conf(&self) -> CONF_R {
        CONF_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 2 - Wake Enable"]
    #[inline(always)]
    pub fn wakeen(&self) -> WAKEEN_R {
        WAKEEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Battery Backup Configuration"]
    #[inline(always)]
    pub fn conf(&mut self) -> CONF_W {
        CONF_W { w: self }
    }
    #[doc = "Bit 2 - Wake Enable"]
    #[inline(always)]
    pub fn wakeen(&mut self) -> WAKEEN_W {
        WAKEEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Battery Backup Power Switch\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bbps](index.html) module"]
pub struct BBPS_SPEC;
impl crate::RegisterSpec for BBPS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bbps::R](R) reader structure"]
impl crate::Readable for BBPS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bbps::W](W) writer structure"]
impl crate::Writable for BBPS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BBPS to value 0"]
impl crate::Resettable for BBPS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
