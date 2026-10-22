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
#[doc = "Field `CONF` reader - Battery Backup Configuration"]
pub type CONF_R = crate::FieldReader<u8, CONFSELECT_A>;
#[doc = "Battery Backup Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CONFSELECT_A {
    #[doc = "0: The backup domain is always supplied by main power"]
    NONE = 0,
    #[doc = "1: The power switch is handled by the automatic power switch"]
    APWS = 1,
    #[doc = "2: The backup domain is always supplied by battery backup power"]
    FORCED = 2,
    #[doc = "3: The power switch is handled by the BOD33"]
    BOD33 = 3,
}
impl From<CONFSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: CONFSELECT_A) -> Self {
        variant as _
    }
}
impl CONF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CONFSELECT_A {
        match self.bits {
            0 => CONFSELECT_A::NONE,
            1 => CONFSELECT_A::APWS,
            2 => CONFSELECT_A::FORCED,
            3 => CONFSELECT_A::BOD33,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == CONFSELECT_A::NONE
    }
    #[doc = "Checks if the value of the field is `APWS`"]
    #[inline(always)]
    pub fn is_apws(&self) -> bool {
        *self == CONFSELECT_A::APWS
    }
    #[doc = "Checks if the value of the field is `FORCED`"]
    #[inline(always)]
    pub fn is_forced(&self) -> bool {
        *self == CONFSELECT_A::FORCED
    }
    #[doc = "Checks if the value of the field is `BOD33`"]
    #[inline(always)]
    pub fn is_bod33(&self) -> bool {
        *self == CONFSELECT_A::BOD33
    }
}
#[doc = "Field `CONF` writer - Battery Backup Configuration"]
pub type CONF_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, BBPS_SPEC, u8, CONFSELECT_A, 2, O>;
impl<'a, const O: u8> CONF_W<'a, O> {
    #[doc = "The backup domain is always supplied by main power"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(CONFSELECT_A::NONE)
    }
    #[doc = "The power switch is handled by the automatic power switch"]
    #[inline(always)]
    pub fn apws(self) -> &'a mut W {
        self.variant(CONFSELECT_A::APWS)
    }
    #[doc = "The backup domain is always supplied by battery backup power"]
    #[inline(always)]
    pub fn forced(self) -> &'a mut W {
        self.variant(CONFSELECT_A::FORCED)
    }
    #[doc = "The power switch is handled by the BOD33"]
    #[inline(always)]
    pub fn bod33(self) -> &'a mut W {
        self.variant(CONFSELECT_A::BOD33)
    }
}
#[doc = "Field `WAKEEN` reader - Wake Enable"]
pub type WAKEEN_R = crate::BitReader<bool>;
#[doc = "Field `WAKEEN` writer - Wake Enable"]
pub type WAKEEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, BBPS_SPEC, bool, O>;
#[doc = "Field `PSOKEN` reader - Power Supply OK Enable"]
pub type PSOKEN_R = crate::BitReader<bool>;
#[doc = "Field `PSOKEN` writer - Power Supply OK Enable"]
pub type PSOKEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, BBPS_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:1 - Battery Backup Configuration"]
    #[inline(always)]
    pub fn conf(&self) -> CONF_R {
        CONF_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Wake Enable"]
    #[inline(always)]
    pub fn wakeen(&self) -> WAKEEN_R {
        WAKEEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Power Supply OK Enable"]
    #[inline(always)]
    pub fn psoken(&self) -> PSOKEN_R {
        PSOKEN_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Battery Backup Configuration"]
    #[inline(always)]
    #[must_use]
    pub fn conf(&mut self) -> CONF_W<0> {
        CONF_W::new(self)
    }
    #[doc = "Bit 2 - Wake Enable"]
    #[inline(always)]
    #[must_use]
    pub fn wakeen(&mut self) -> WAKEEN_W<2> {
        WAKEEN_W::new(self)
    }
    #[doc = "Bit 3 - Power Supply OK Enable"]
    #[inline(always)]
    #[must_use]
    pub fn psoken(&mut self) -> PSOKEN_W<3> {
        PSOKEN_W::new(self)
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BBPS to value 0"]
impl crate::Resettable for BBPS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
