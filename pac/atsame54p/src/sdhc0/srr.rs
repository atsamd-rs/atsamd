#[doc = "Register `SRR` reader"]
pub struct R(crate::R<SRR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SRR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SRR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SRR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SRR` writer"]
pub struct W(crate::W<SRR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SRR_SPEC>;
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
impl From<crate::W<SRR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SRR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SWRSTALL` reader - Software Reset For All"]
pub type SWRSTALL_R = crate::BitReader<SWRSTALLSELECT_A>;
#[doc = "Software Reset For All\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SWRSTALLSELECT_A {
    #[doc = "0: Work"]
    WORK = 0,
    #[doc = "1: Reset"]
    RESET = 1,
}
impl From<SWRSTALLSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: SWRSTALLSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl SWRSTALL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SWRSTALLSELECT_A {
        match self.bits {
            false => SWRSTALLSELECT_A::WORK,
            true => SWRSTALLSELECT_A::RESET,
        }
    }
    #[doc = "Checks if the value of the field is `WORK`"]
    #[inline(always)]
    pub fn is_work(&self) -> bool {
        *self == SWRSTALLSELECT_A::WORK
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == SWRSTALLSELECT_A::RESET
    }
}
#[doc = "Field `SWRSTALL` writer - Software Reset For All"]
pub type SWRSTALL_W<'a, const O: u8> = crate::BitWriter<'a, u8, SRR_SPEC, SWRSTALLSELECT_A, O>;
impl<'a, const O: u8> SWRSTALL_W<'a, O> {
    #[doc = "Work"]
    #[inline(always)]
    pub fn work(self) -> &'a mut W {
        self.variant(SWRSTALLSELECT_A::WORK)
    }
    #[doc = "Reset"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(SWRSTALLSELECT_A::RESET)
    }
}
#[doc = "Field `SWRSTCMD` reader - Software Reset For CMD Line"]
pub type SWRSTCMD_R = crate::BitReader<SWRSTCMDSELECT_A>;
#[doc = "Software Reset For CMD Line\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SWRSTCMDSELECT_A {
    #[doc = "0: Work"]
    WORK = 0,
    #[doc = "1: Reset"]
    RESET = 1,
}
impl From<SWRSTCMDSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: SWRSTCMDSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl SWRSTCMD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SWRSTCMDSELECT_A {
        match self.bits {
            false => SWRSTCMDSELECT_A::WORK,
            true => SWRSTCMDSELECT_A::RESET,
        }
    }
    #[doc = "Checks if the value of the field is `WORK`"]
    #[inline(always)]
    pub fn is_work(&self) -> bool {
        *self == SWRSTCMDSELECT_A::WORK
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == SWRSTCMDSELECT_A::RESET
    }
}
#[doc = "Field `SWRSTCMD` writer - Software Reset For CMD Line"]
pub type SWRSTCMD_W<'a, const O: u8> = crate::BitWriter<'a, u8, SRR_SPEC, SWRSTCMDSELECT_A, O>;
impl<'a, const O: u8> SWRSTCMD_W<'a, O> {
    #[doc = "Work"]
    #[inline(always)]
    pub fn work(self) -> &'a mut W {
        self.variant(SWRSTCMDSELECT_A::WORK)
    }
    #[doc = "Reset"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(SWRSTCMDSELECT_A::RESET)
    }
}
#[doc = "Field `SWRSTDAT` reader - Software Reset For DAT Line"]
pub type SWRSTDAT_R = crate::BitReader<SWRSTDATSELECT_A>;
#[doc = "Software Reset For DAT Line\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SWRSTDATSELECT_A {
    #[doc = "0: Work"]
    WORK = 0,
    #[doc = "1: Reset"]
    RESET = 1,
}
impl From<SWRSTDATSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: SWRSTDATSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl SWRSTDAT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SWRSTDATSELECT_A {
        match self.bits {
            false => SWRSTDATSELECT_A::WORK,
            true => SWRSTDATSELECT_A::RESET,
        }
    }
    #[doc = "Checks if the value of the field is `WORK`"]
    #[inline(always)]
    pub fn is_work(&self) -> bool {
        *self == SWRSTDATSELECT_A::WORK
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == SWRSTDATSELECT_A::RESET
    }
}
#[doc = "Field `SWRSTDAT` writer - Software Reset For DAT Line"]
pub type SWRSTDAT_W<'a, const O: u8> = crate::BitWriter<'a, u8, SRR_SPEC, SWRSTDATSELECT_A, O>;
impl<'a, const O: u8> SWRSTDAT_W<'a, O> {
    #[doc = "Work"]
    #[inline(always)]
    pub fn work(self) -> &'a mut W {
        self.variant(SWRSTDATSELECT_A::WORK)
    }
    #[doc = "Reset"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(SWRSTDATSELECT_A::RESET)
    }
}
impl R {
    #[doc = "Bit 0 - Software Reset For All"]
    #[inline(always)]
    pub fn swrstall(&self) -> SWRSTALL_R {
        SWRSTALL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Software Reset For CMD Line"]
    #[inline(always)]
    pub fn swrstcmd(&self) -> SWRSTCMD_R {
        SWRSTCMD_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Software Reset For DAT Line"]
    #[inline(always)]
    pub fn swrstdat(&self) -> SWRSTDAT_R {
        SWRSTDAT_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Software Reset For All"]
    #[inline(always)]
    #[must_use]
    pub fn swrstall(&mut self) -> SWRSTALL_W<0> {
        SWRSTALL_W::new(self)
    }
    #[doc = "Bit 1 - Software Reset For CMD Line"]
    #[inline(always)]
    #[must_use]
    pub fn swrstcmd(&mut self) -> SWRSTCMD_W<1> {
        SWRSTCMD_W::new(self)
    }
    #[doc = "Bit 2 - Software Reset For DAT Line"]
    #[inline(always)]
    #[must_use]
    pub fn swrstdat(&mut self) -> SWRSTDAT_W<2> {
        SWRSTDAT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Software Reset\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [srr](index.html) module"]
pub struct SRR_SPEC;
impl crate::RegisterSpec for SRR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [srr::R](R) reader structure"]
impl crate::Readable for SRR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [srr::W](W) writer structure"]
impl crate::Writable for SRR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SRR to value 0"]
impl crate::Resettable for SRR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
