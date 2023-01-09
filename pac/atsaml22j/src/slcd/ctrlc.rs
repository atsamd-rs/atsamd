#[doc = "Register `CTRLC` reader"]
pub struct R(crate::R<CTRLC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRLC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRLC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRLC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRLC` writer"]
pub struct W(crate::W<CTRLC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRLC_SPEC>;
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
impl From<crate::W<CTRLC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRLC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLEAR` reader - Clear Display Memory"]
pub type CLEAR_R = crate::BitReader<bool>;
#[doc = "Field `CLEAR` writer - Clear Display Memory"]
pub type CLEAR_W<'a, const O: u8> = crate::BitWriter<'a, u16, CTRLC_SPEC, bool, O>;
#[doc = "Field `LOCK` reader - Lock Shadow Memory"]
pub type LOCK_R = crate::BitReader<bool>;
#[doc = "Field `LOCK` writer - Lock Shadow Memory"]
pub type LOCK_W<'a, const O: u8> = crate::BitWriter<'a, u16, CTRLC_SPEC, bool, O>;
#[doc = "Field `ABMEN` reader - Automated Bit Mapping Enable"]
pub type ABMEN_R = crate::BitReader<bool>;
#[doc = "Field `ABMEN` writer - Automated Bit Mapping Enable"]
pub type ABMEN_W<'a, const O: u8> = crate::BitWriter<'a, u16, CTRLC_SPEC, bool, O>;
#[doc = "Field `ACMEN` reader - Automated Character Mapping Enable"]
pub type ACMEN_R = crate::BitReader<bool>;
#[doc = "Field `ACMEN` writer - Automated Character Mapping Enable"]
pub type ACMEN_W<'a, const O: u8> = crate::BitWriter<'a, u16, CTRLC_SPEC, bool, O>;
#[doc = "Field `CTST` reader - Contrast Adjustment"]
pub type CTST_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CTST` writer - Contrast Adjustment"]
pub type CTST_W<'a, const O: u8> = crate::FieldWriter<'a, u16, CTRLC_SPEC, u8, u8, 4, O>;
#[doc = "Field `LPPM` reader - LCD Power Macro Power mode"]
pub type LPPM_R = crate::FieldReader<u8, LPPMSELECT_A>;
#[doc = "LCD Power Macro Power mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LPPMSELECT_A {
    #[doc = "0: LCD power automatically select regualation mode or pump mode"]
    AUTO = 0,
    #[doc = "1: LCD power use step-up pump loop only"]
    STEPUP = 1,
    #[doc = "2: LCD power use step-down drop-out regulation loop only"]
    STEPDOWN = 2,
}
impl From<LPPMSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: LPPMSELECT_A) -> Self {
        variant as _
    }
}
impl LPPM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<LPPMSELECT_A> {
        match self.bits {
            0 => Some(LPPMSELECT_A::AUTO),
            1 => Some(LPPMSELECT_A::STEPUP),
            2 => Some(LPPMSELECT_A::STEPDOWN),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `AUTO`"]
    #[inline(always)]
    pub fn is_auto(&self) -> bool {
        *self == LPPMSELECT_A::AUTO
    }
    #[doc = "Checks if the value of the field is `STEPUP`"]
    #[inline(always)]
    pub fn is_stepup(&self) -> bool {
        *self == LPPMSELECT_A::STEPUP
    }
    #[doc = "Checks if the value of the field is `STEPDOWN`"]
    #[inline(always)]
    pub fn is_stepdown(&self) -> bool {
        *self == LPPMSELECT_A::STEPDOWN
    }
}
#[doc = "Field `LPPM` writer - LCD Power Macro Power mode"]
pub type LPPM_W<'a, const O: u8> = crate::FieldWriter<'a, u16, CTRLC_SPEC, u8, LPPMSELECT_A, 2, O>;
impl<'a, const O: u8> LPPM_W<'a, O> {
    #[doc = "LCD power automatically select regualation mode or pump mode"]
    #[inline(always)]
    pub fn auto(self) -> &'a mut W {
        self.variant(LPPMSELECT_A::AUTO)
    }
    #[doc = "LCD power use step-up pump loop only"]
    #[inline(always)]
    pub fn stepup(self) -> &'a mut W {
        self.variant(LPPMSELECT_A::STEPUP)
    }
    #[doc = "LCD power use step-down drop-out regulation loop only"]
    #[inline(always)]
    pub fn stepdown(self) -> &'a mut W {
        self.variant(LPPMSELECT_A::STEPDOWN)
    }
}
impl R {
    #[doc = "Bit 0 - Clear Display Memory"]
    #[inline(always)]
    pub fn clear(&self) -> CLEAR_R {
        CLEAR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Lock Shadow Memory"]
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Automated Bit Mapping Enable"]
    #[inline(always)]
    pub fn abmen(&self) -> ABMEN_R {
        ABMEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Automated Character Mapping Enable"]
    #[inline(always)]
    pub fn acmen(&self) -> ACMEN_R {
        ACMEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - Contrast Adjustment"]
    #[inline(always)]
    pub fn ctst(&self) -> CTST_R {
        CTST_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:9 - LCD Power Macro Power mode"]
    #[inline(always)]
    pub fn lppm(&self) -> LPPM_R {
        LPPM_R::new(((self.bits >> 8) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Clear Display Memory"]
    #[inline(always)]
    #[must_use]
    pub fn clear(&mut self) -> CLEAR_W<0> {
        CLEAR_W::new(self)
    }
    #[doc = "Bit 1 - Lock Shadow Memory"]
    #[inline(always)]
    #[must_use]
    pub fn lock(&mut self) -> LOCK_W<1> {
        LOCK_W::new(self)
    }
    #[doc = "Bit 2 - Automated Bit Mapping Enable"]
    #[inline(always)]
    #[must_use]
    pub fn abmen(&mut self) -> ABMEN_W<2> {
        ABMEN_W::new(self)
    }
    #[doc = "Bit 3 - Automated Character Mapping Enable"]
    #[inline(always)]
    #[must_use]
    pub fn acmen(&mut self) -> ACMEN_W<3> {
        ACMEN_W::new(self)
    }
    #[doc = "Bits 4:7 - Contrast Adjustment"]
    #[inline(always)]
    #[must_use]
    pub fn ctst(&mut self) -> CTST_W<4> {
        CTST_W::new(self)
    }
    #[doc = "Bits 8:9 - LCD Power Macro Power mode"]
    #[inline(always)]
    #[must_use]
    pub fn lppm(&mut self) -> LPPM_W<8> {
        LPPM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control C\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrlc](index.html) module"]
pub struct CTRLC_SPEC;
impl crate::RegisterSpec for CTRLC_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [ctrlc::R](R) reader structure"]
impl crate::Readable for CTRLC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrlc::W](W) writer structure"]
impl crate::Writable for CTRLC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRLC to value 0"]
impl crate::Resettable for CTRLC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
