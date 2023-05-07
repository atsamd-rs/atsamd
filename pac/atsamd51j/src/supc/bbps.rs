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
pub type CONF_R = crate::BitReader<CONFSELECT_A>;
#[doc = "Battery Backup Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CONFSELECT_A {
    #[doc = "0: The power switch is handled by the BOD33"]
    BOD33 = 0,
    #[doc = "1: In Backup Domain, the backup domain is always supplied by battery backup power"]
    FORCED = 1,
}
impl From<CONFSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: CONFSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl CONF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CONFSELECT_A {
        match self.bits {
            false => CONFSELECT_A::BOD33,
            true => CONFSELECT_A::FORCED,
        }
    }
    #[doc = "Checks if the value of the field is `BOD33`"]
    #[inline(always)]
    pub fn is_bod33(&self) -> bool {
        *self == CONFSELECT_A::BOD33
    }
    #[doc = "Checks if the value of the field is `FORCED`"]
    #[inline(always)]
    pub fn is_forced(&self) -> bool {
        *self == CONFSELECT_A::FORCED
    }
}
#[doc = "Field `CONF` writer - Battery Backup Configuration"]
pub type CONF_W<'a, const O: u8> = crate::BitWriter<'a, u32, BBPS_SPEC, CONFSELECT_A, O>;
impl<'a, const O: u8> CONF_W<'a, O> {
    #[doc = "The power switch is handled by the BOD33"]
    #[inline(always)]
    pub fn bod33(self) -> &'a mut W {
        self.variant(CONFSELECT_A::BOD33)
    }
    #[doc = "In Backup Domain, the backup domain is always supplied by battery backup power"]
    #[inline(always)]
    pub fn forced(self) -> &'a mut W {
        self.variant(CONFSELECT_A::FORCED)
    }
}
#[doc = "Field `WAKEEN` reader - Wake Enable"]
pub type WAKEEN_R = crate::BitReader<bool>;
#[doc = "Field `WAKEEN` writer - Wake Enable"]
pub type WAKEEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, BBPS_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Battery Backup Configuration"]
    #[inline(always)]
    pub fn conf(&self) -> CONF_R {
        CONF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Wake Enable"]
    #[inline(always)]
    pub fn wakeen(&self) -> WAKEEN_R {
        WAKEEN_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Battery Backup Configuration"]
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
