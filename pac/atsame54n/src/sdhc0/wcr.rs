#[doc = "Register `WCR` reader"]
pub struct R(crate::R<WCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WCR` writer"]
pub struct W(crate::W<WCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WCR_SPEC>;
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
impl From<crate::W<WCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WKENCINT` reader - Wakeup Event Enable on Card Interrupt"]
pub type WKENCINT_R = crate::BitReader<WKENCINTSELECT_A>;
#[doc = "Wakeup Event Enable on Card Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WKENCINTSELECT_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Enable"]
    ENABLE = 1,
}
impl From<WKENCINTSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: WKENCINTSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl WKENCINT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WKENCINTSELECT_A {
        match self.bits {
            false => WKENCINTSELECT_A::DISABLE,
            true => WKENCINTSELECT_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == WKENCINTSELECT_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == WKENCINTSELECT_A::ENABLE
    }
}
#[doc = "Field `WKENCINT` writer - Wakeup Event Enable on Card Interrupt"]
pub type WKENCINT_W<'a, const O: u8> = crate::BitWriter<'a, u8, WCR_SPEC, WKENCINTSELECT_A, O>;
impl<'a, const O: u8> WKENCINT_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(WKENCINTSELECT_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(WKENCINTSELECT_A::ENABLE)
    }
}
#[doc = "Field `WKENCINS` reader - Wakeup Event Enable on Card Insertion"]
pub type WKENCINS_R = crate::BitReader<WKENCINSSELECT_A>;
#[doc = "Wakeup Event Enable on Card Insertion\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WKENCINSSELECT_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Enable"]
    ENABLE = 1,
}
impl From<WKENCINSSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: WKENCINSSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl WKENCINS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WKENCINSSELECT_A {
        match self.bits {
            false => WKENCINSSELECT_A::DISABLE,
            true => WKENCINSSELECT_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == WKENCINSSELECT_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == WKENCINSSELECT_A::ENABLE
    }
}
#[doc = "Field `WKENCINS` writer - Wakeup Event Enable on Card Insertion"]
pub type WKENCINS_W<'a, const O: u8> = crate::BitWriter<'a, u8, WCR_SPEC, WKENCINSSELECT_A, O>;
impl<'a, const O: u8> WKENCINS_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(WKENCINSSELECT_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(WKENCINSSELECT_A::ENABLE)
    }
}
#[doc = "Field `WKENCREM` reader - Wakeup Event Enable on Card Removal"]
pub type WKENCREM_R = crate::BitReader<WKENCREMSELECT_A>;
#[doc = "Wakeup Event Enable on Card Removal\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WKENCREMSELECT_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Enable"]
    ENABLE = 1,
}
impl From<WKENCREMSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: WKENCREMSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl WKENCREM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WKENCREMSELECT_A {
        match self.bits {
            false => WKENCREMSELECT_A::DISABLE,
            true => WKENCREMSELECT_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == WKENCREMSELECT_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == WKENCREMSELECT_A::ENABLE
    }
}
#[doc = "Field `WKENCREM` writer - Wakeup Event Enable on Card Removal"]
pub type WKENCREM_W<'a, const O: u8> = crate::BitWriter<'a, u8, WCR_SPEC, WKENCREMSELECT_A, O>;
impl<'a, const O: u8> WKENCREM_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(WKENCREMSELECT_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(WKENCREMSELECT_A::ENABLE)
    }
}
impl R {
    #[doc = "Bit 0 - Wakeup Event Enable on Card Interrupt"]
    #[inline(always)]
    pub fn wkencint(&self) -> WKENCINT_R {
        WKENCINT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Wakeup Event Enable on Card Insertion"]
    #[inline(always)]
    pub fn wkencins(&self) -> WKENCINS_R {
        WKENCINS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Wakeup Event Enable on Card Removal"]
    #[inline(always)]
    pub fn wkencrem(&self) -> WKENCREM_R {
        WKENCREM_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Wakeup Event Enable on Card Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn wkencint(&mut self) -> WKENCINT_W<0> {
        WKENCINT_W::new(self)
    }
    #[doc = "Bit 1 - Wakeup Event Enable on Card Insertion"]
    #[inline(always)]
    #[must_use]
    pub fn wkencins(&mut self) -> WKENCINS_W<1> {
        WKENCINS_W::new(self)
    }
    #[doc = "Bit 2 - Wakeup Event Enable on Card Removal"]
    #[inline(always)]
    #[must_use]
    pub fn wkencrem(&mut self) -> WKENCREM_W<2> {
        WKENCREM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Wakeup Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wcr](index.html) module"]
pub struct WCR_SPEC;
impl crate::RegisterSpec for WCR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [wcr::R](R) reader structure"]
impl crate::Readable for WCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wcr::W](W) writer structure"]
impl crate::Writable for WCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WCR to value 0"]
impl crate::Resettable for WCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
