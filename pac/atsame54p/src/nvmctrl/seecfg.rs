#[doc = "Register `SEECFG` reader"]
pub struct R(crate::R<SEECFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SEECFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SEECFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SEECFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SEECFG` writer"]
pub struct W(crate::W<SEECFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SEECFG_SPEC>;
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
impl From<crate::W<SEECFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SEECFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WMODE` reader - Write Mode"]
pub type WMODE_R = crate::BitReader<WMODESELECT_A>;
#[doc = "Write Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WMODESELECT_A {
    #[doc = "0: A NVM write command is issued after each write in the pagebuffer"]
    UNBUFFERED = 0,
    #[doc = "1: A NVM write command is issued when a write to a new page is requested"]
    BUFFERED = 1,
}
impl From<WMODESELECT_A> for bool {
    #[inline(always)]
    fn from(variant: WMODESELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl WMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WMODESELECT_A {
        match self.bits {
            false => WMODESELECT_A::UNBUFFERED,
            true => WMODESELECT_A::BUFFERED,
        }
    }
    #[doc = "Checks if the value of the field is `UNBUFFERED`"]
    #[inline(always)]
    pub fn is_unbuffered(&self) -> bool {
        *self == WMODESELECT_A::UNBUFFERED
    }
    #[doc = "Checks if the value of the field is `BUFFERED`"]
    #[inline(always)]
    pub fn is_buffered(&self) -> bool {
        *self == WMODESELECT_A::BUFFERED
    }
}
#[doc = "Field `WMODE` writer - Write Mode"]
pub type WMODE_W<'a, const O: u8> = crate::BitWriter<'a, u8, SEECFG_SPEC, WMODESELECT_A, O>;
impl<'a, const O: u8> WMODE_W<'a, O> {
    #[doc = "A NVM write command is issued after each write in the pagebuffer"]
    #[inline(always)]
    pub fn unbuffered(self) -> &'a mut W {
        self.variant(WMODESELECT_A::UNBUFFERED)
    }
    #[doc = "A NVM write command is issued when a write to a new page is requested"]
    #[inline(always)]
    pub fn buffered(self) -> &'a mut W {
        self.variant(WMODESELECT_A::BUFFERED)
    }
}
#[doc = "Field `APRDIS` reader - Automatic Page Reallocation Disable"]
pub type APRDIS_R = crate::BitReader<bool>;
#[doc = "Field `APRDIS` writer - Automatic Page Reallocation Disable"]
pub type APRDIS_W<'a, const O: u8> = crate::BitWriter<'a, u8, SEECFG_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Write Mode"]
    #[inline(always)]
    pub fn wmode(&self) -> WMODE_R {
        WMODE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Automatic Page Reallocation Disable"]
    #[inline(always)]
    pub fn aprdis(&self) -> APRDIS_R {
        APRDIS_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Write Mode"]
    #[inline(always)]
    #[must_use]
    pub fn wmode(&mut self) -> WMODE_W<0> {
        WMODE_W::new(self)
    }
    #[doc = "Bit 1 - Automatic Page Reallocation Disable"]
    #[inline(always)]
    #[must_use]
    pub fn aprdis(&mut self) -> APRDIS_W<1> {
        APRDIS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SmartEEPROM Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [seecfg](index.html) module"]
pub struct SEECFG_SPEC;
impl crate::RegisterSpec for SEECFG_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [seecfg::R](R) reader structure"]
impl crate::Readable for SEECFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [seecfg::W](W) writer structure"]
impl crate::Writable for SEECFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SEECFG to value 0"]
impl crate::Resettable for SEECFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
